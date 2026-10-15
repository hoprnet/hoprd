//! Relay throughput benchmark.
//!
//! Replays the relay path of the HOPR packet pipeline at full speed on the node's shared CPU pool:
//! SPHINX peel with ticket validation and re-issue, acknowledgement back to the previous hop and
//! resolution of the next hop's acknowledgements. Traffic runs over a [`SyntheticNetwork`], so the
//! node's keys, channels, tickets, transport and packet metrics are never touched.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use bytes::Bytes;
use futures::{SinkExt, StreamExt, TryStreamExt, channel::mpsc};
use hopr_crypto_packet::{HoprPixSpec, HoprSurb, prelude::HoprPacket};
use hopr_lib::{
    api::{
        PeerId,
        chain::{ChainPathResolver, ChainReadChannelOperations},
        types::{
            crypto::prelude::{
                ChainKeypair, HalfKey, HalfKeyChallenge, Hash, Keypair, OffchainKeypair,
                OffchainPublicKey,
            },
            crypto_random::{Randomizable, random_fill},
            internal::{
                prelude::{
                    Acknowledgement, HoprPseudonym, UnacknowledgedTicket, ValidatedPath,
                    VerifiedAcknowledgement,
                },
                routing::ResolvedTransportRouting,
            },
        },
    },
    exports::transport::protocol::AcknowledgementPipelineConfig,
    utils::parallelize::cpu::{self, rayon::prelude::*},
};
use hopr_protocol_hopr::{
    HoprCodecConfig, HoprDecoder, HoprEncoder, HoprUnacknowledgedTicketProcessorConfig,
    IncomingForwardedPacket, IncomingPacket, IncomingPacketError, MAX_ACKNOWLEDGEMENTS_BATCH_SIZE,
    MemorySurbStore, PacketDecoder, PacketEncoder, SurbStoreConfig,
};
use hopr_protocol_pix::{
    SsaGeneratorConfig, SsaShareGenerator, ack_verify::verify_expected_acknowledgements,
};
use hopr_ticket_manager::{HoprTicketFactory, HoprTicketManager, MemoryStore, MemoryTicketQueue};

use super::synthetic::{SyntheticChain, SyntheticNetwork, SyntheticNode};

/// Mirrors the pipeline's per-CPU decode queue depth (`PIPELINE_CONCURRENCY_PER_CPU`).
const PIPELINE_CONCURRENCY_PER_CPU: usize = 8;
/// Mirrors the pipeline's `DEFAULT_ACK_{INPUT,OUTPUT}_CONCURRENCY`.
const DEFAULT_ACK_CONCURRENCY: usize = 10;

type Factory = Arc<HoprTicketFactory<MemoryStore>>;
type Encoder =
    HoprEncoder<SyntheticChain, SsaShareGenerator<HoprPixSpec>, MemorySurbStore, Factory>;
type Decoder = HoprDecoder<SyntheticChain, MemorySurbStore, Factory>;

/// Parameters of a relay benchmark run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelayBenchmarkConfig {
    /// Number of packets to relay.
    pub packets: usize,
    /// Number of packets decoded concurrently.
    pub concurrency: usize,
}

impl RelayBenchmarkConfig {
    /// Decode concurrency the packet pipeline runs with by default.
    pub fn default_concurrency() -> usize {
        std::thread::available_parallelism().map_or(1, |n| n.get()) * PIPELINE_CONCURRENCY_PER_CPU
    }
}

/// Stage a running benchmark is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayBenchmarkPhase {
    /// Generating the traffic to relay; not measured.
    Preparing,
    /// Relaying the prepared traffic; measured.
    Relaying,
}

/// Outcome of a relay benchmark run.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RelayBenchmarkReport {
    pub packets: usize,
    pub relayed: usize,
    pub rejected: usize,
    pub relayed_bytes: usize,
    pub acks_sent: usize,
    pub ack_packets_sent: usize,
    pub tickets_acknowledged: usize,
    pub preparation: Duration,
    pub duration: Duration,
    /// First failure seen while relaying, if any.
    pub first_error: Option<String>,
}

impl RelayBenchmarkReport {
    pub fn packets_per_second(&self) -> f64 {
        per_second(self.relayed, self.duration)
    }

    pub fn bytes_per_second(&self) -> f64 {
        per_second(self.relayed_bytes, self.duration)
    }
}

fn per_second(count: usize, duration: Duration) -> f64 {
    if duration.is_zero() {
        0.0
    } else {
        count as f64 / duration.as_secs_f64()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RelayBenchmarkError {
    #[error("failed to set up the synthetic network: {0}")]
    Setup(String),
    #[error("failed to prepare benchmark traffic: {0}")]
    Preparation(String),
}

/// Prepares `cfg.packets` packets addressed through a synthetic relay, then relays them as fast
/// as the node's CPU pool allows.
pub async fn run_relay_benchmark(
    cfg: RelayBenchmarkConfig,
    on_phase: impl Fn(RelayBenchmarkPhase),
) -> Result<RelayBenchmarkReport, RelayBenchmarkError> {
    let network =
        SyntheticNetwork::random().map_err(|e| RelayBenchmarkError::Setup(e.to_string()))?;

    on_phase(RelayBenchmarkPhase::Preparing);
    let started = Instant::now();
    let traffic = Traffic::prepare(&network, cfg.packets).await?;
    let preparation = started.elapsed();

    on_phase(RelayBenchmarkPhase::Relaying);
    let mut report = RelayUnderTest::new(&network)
        .relay(traffic, cfg.concurrency.max(1))
        .await;
    report.preparation = preparation;
    Ok(report)
}

/// Codec of one synthetic node.
struct NodeCodec {
    encoder: Encoder,
    decoder: Decoder,
    // Keeps the factory reporting unrealized channel value like it does on a running node.
    _tickets: HoprTicketManager<MemoryStore, MemoryTicketQueue>,
}

impl NodeCodec {
    fn new(network: &SyntheticNetwork, node: &SyntheticNode) -> Self {
        let chain = network.chain_of(node);
        let surbs = MemorySurbStore::new(SurbStoreConfig::default());
        let (tickets, factory) = HoprTicketManager::new_with_factory(MemoryStore::default());
        let factory = Arc::new(factory);
        let cfg = HoprCodecConfig::default();

        Self {
            encoder: HoprEncoder::new(
                node.chain_key.clone(),
                chain.clone(),
                surbs.clone(),
                factory.clone(),
                Hash::default(),
                SsaShareGenerator::new(SsaGeneratorConfig::default()),
                cfg,
            ),
            decoder: HoprDecoder::new(
                (node.packet_key.clone(), node.chain_key.clone()),
                chain,
                surbs,
                factory,
                Hash::default(),
                cfg,
            ),
            _tickets: tickets,
        }
    }
}

fn into_forwarded<E: std::error::Error>(
    decoded: Result<IncomingPacket, IncomingPacketError<E>>,
) -> Result<IncomingForwardedPacket, String> {
    match decoded {
        Ok(IncomingPacket::Forwarded(packet)) => Ok(*packet),
        Ok(_) => Err("synthetic packet was not forwarded".into()),
        Err(error) => Err(error.to_string()),
    }
}

/// Packets the sender pushes through the relay, with the next hop's acknowledgements of them.
struct Traffic {
    chunks: Vec<TrafficChunk>,
}

struct TrafficChunk {
    packets: Vec<Bytes>,
    /// Single packet from the next hop acknowledging all of `packets`.
    acknowledgements: Bytes,
}

/// Everything around the relay under test, used only to generate [`Traffic`].
struct TrafficGenerator {
    sender: NodeCodec,
    // Separate from the relay under test, whose replay filter must not have seen the packets.
    relay: NodeCodec,
    next: NodeCodec,
    next_key: OffchainKeypair,
    sender_peer: PeerId,
    relay_peer: PeerId,
    relay_key: OffchainPublicKey,
    routing: ResolvedTransportRouting<HoprSurb>,
    payload: Bytes,
}

impl Traffic {
    async fn prepare(
        network: &SyntheticNetwork,
        packets: usize,
    ) -> Result<Self, RelayBenchmarkError> {
        let path = ValidatedPath::new(
            network.sender.address(),
            vec![
                network.relay.address(),
                network.next.address(),
                network.exit.address(),
            ],
            &ChainPathResolver::from(&network.chain_of(&network.sender)),
        )
        .await
        .map_err(|e| RelayBenchmarkError::Setup(e.to_string()))?;

        let mut payload = vec![0_u8; HoprPacket::max_message_with_surbs(0)];
        random_fill(&mut payload);

        let generator = Arc::new(TrafficGenerator {
            sender: NodeCodec::new(network, &network.sender),
            relay: NodeCodec::new(network, &network.relay),
            next: NodeCodec::new(network, &network.next),
            next_key: network.next.packet_key.clone(),
            sender_peer: network.sender.peer_id(),
            relay_peer: network.relay.peer_id(),
            relay_key: *network.relay.public_key(),
            routing: ResolvedTransportRouting::Forward {
                pseudonym: HoprPseudonym::random(),
                forward_path: path,
                return_paths: vec![],
            },
            payload: payload.into(),
        });

        let chunk_sizes = (0..packets)
            .step_by(MAX_ACKNOWLEDGEMENTS_BATCH_SIZE)
            .map(|start| MAX_ACKNOWLEDGEMENTS_BATCH_SIZE.min(packets - start));

        let chunks = futures::stream::iter(chunk_sizes)
            .map(|size| {
                let generator = generator.clone();
                async move {
                    cpu::spawn_fifo_blocking(
                        move || generator.chunk(size),
                        "relay_benchmark_prepare",
                    )
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(|chunk| chunk)
                }
            })
            .buffered(cpu::rayon::current_num_threads().max(1) * 2)
            .try_collect()
            .await
            .map_err(RelayBenchmarkError::Preparation)?;

        Ok(Self { chunks })
    }
}

impl TrafficGenerator {
    fn chunk(&self, size: usize) -> Result<TrafficChunk, String> {
        let mut packets = Vec::with_capacity(size);
        let mut acks = Vec::with_capacity(size);
        for _ in 0..size {
            let packet = self
                .sender
                .encoder
                .encode_packet(self.payload.clone(), self.routing.clone(), None, None)
                .map_err(|e| e.to_string())?;
            let at_relay = into_forwarded(
                self.relay
                    .decoder
                    .decode(self.sender_peer, packet.data.clone()),
            )?;
            let at_next = into_forwarded(self.next.decoder.decode(self.relay_peer, at_relay.data))?;

            acks.push(VerifiedAcknowledgement::new(
                at_next.ack_key_prev_hop,
                &self.next_key,
            ));
            packets.push(packet.data);
        }

        let acknowledgements = self
            .next
            .encoder
            .encode_acknowledgements(&acks, &self.relay_key)
            .map_err(|e| e.to_string())?;

        Ok(TrafficChunk {
            packets,
            acknowledgements: acknowledgements.data,
        })
    }
}

/// Unacknowledged-ticket bookkeeping of `HoprUnacknowledgedTicketProcessor`, minus its metrics so
/// benchmark tickets never show up in the node's gauges.
struct AckTracker {
    awaiting: moka::sync::Cache<
        OffchainPublicKey,
        moka::sync::Cache<HalfKeyChallenge, UnacknowledgedTicket>,
    >,
    chain: SyntheticChain,
    chain_key: ChainKeypair,
    cfg: HoprUnacknowledgedTicketProcessorConfig,
}

impl AckTracker {
    fn insert(
        &self,
        next_hop: &OffchainPublicKey,
        challenge: HalfKeyChallenge,
        ticket: UnacknowledgedTicket,
    ) {
        self.awaiting
            .get_with_by_ref(next_hop, || {
                moka::sync::Cache::builder()
                    .time_to_live(self.cfg.unack_ticket_timeout)
                    .max_capacity(self.cfg.max_unack_tickets as u64)
                    .build()
            })
            .insert(challenge, ticket);
    }

    /// Returns how many tickets the acknowledgements turned into winning tickets.
    fn acknowledge(&self, peer: OffchainPublicKey, acks: Vec<Acknowledgement>) -> usize {
        let Some((awaiting, half_keys)) = verify_expected_acknowledgements(
            peer,
            acks,
            &self.awaiting,
            self.cfg.use_batch_verification,
        ) else {
            return 0;
        };

        let me = self.chain_key.public().to_address();
        let tickets = half_keys
            .into_iter()
            .filter_map(|(half_key, challenge)| {
                awaiting.remove(&challenge).map(|ticket| (half_key, ticket))
            })
            .filter(|(_, ticket)| {
                self.chain
                    .channel_by_parties(ticket.ticket.verified_issuer(), &me)
                    .ok()
                    .flatten()
                    .is_some_and(|c| c.channel_epoch == ticket.verified_ticket().channel_epoch)
            })
            .collect::<Vec<_>>();

        tickets
            .into_par_iter()
            .filter_map(|(half_key, ticket)| ticket.acknowledge(&half_key).ok())
            .filter(|ticket| {
                ticket
                    .into_redeemable(&self.chain_key, &Hash::default())
                    .is_ok()
            })
            .count()
    }
}

/// The relay node whose throughput is measured.
struct RelayUnderTest {
    codec: NodeCodec,
    tracker: AckTracker,
    packet_key: OffchainKeypair,
    sender_peer: PeerId,
    sender_key: OffchainPublicKey,
    next_peer: PeerId,
}

#[derive(Default)]
struct AckEgress {
    acks: usize,
    packets: usize,
    first_error: Option<String>,
}

impl RelayUnderTest {
    fn new(network: &SyntheticNetwork) -> Arc<Self> {
        Arc::new(Self {
            codec: NodeCodec::new(network, &network.relay),
            tracker: AckTracker {
                awaiting: moka::sync::Cache::builder()
                    .time_to_idle(
                        HoprUnacknowledgedTicketProcessorConfig::default().unack_ticket_timeout,
                    )
                    .max_capacity(100_000)
                    .build(),
                chain: network.chain_of(&network.relay),
                chain_key: network.relay.chain_key.clone(),
                cfg: HoprUnacknowledgedTicketProcessorConfig::default(),
            },
            packet_key: network.relay.packet_key.clone(),
            sender_peer: network.sender.peer_id(),
            sender_key: *network.sender.public_key(),
            next_peer: network.next.peer_id(),
        })
    }

    /// Feeds the traffic to the relay like the packet pipeline does and waits until every packet,
    /// outgoing acknowledgement and incoming acknowledgement is processed.
    async fn relay(self: Arc<Self>, traffic: Traffic, concurrency: usize) -> RelayBenchmarkReport {
        let ack_cfg = AcknowledgementPipelineConfig::default();
        let mut report = RelayBenchmarkReport::default();

        let mut forward = Vec::new();
        let mut pending = Vec::with_capacity(traffic.chunks.len());
        let mut acknowledgements = Vec::with_capacity(traffic.chunks.len());
        for (index, chunk) in traffic.chunks.into_iter().enumerate() {
            pending.push(chunk.packets.len());
            forward.extend(
                chunk
                    .packets
                    .into_iter()
                    .map(|data| (Some(index), self.sender_peer, data)),
            );
            acknowledgements.push(Some(chunk.acknowledgements));
        }
        report.packets = forward.len();
        let mut remaining_chunks = pending.len();

        // Holds the next hop's acknowledgements until the packets they acknowledge are relayed.
        let (acks_in_tx, acks_in_rx) = mpsc::channel(remaining_chunks);
        let mut acks_in_tx = Some(acks_in_tx);
        let (mut ack_out_tx, ack_out_rx) = mpsc::channel::<HalfKey>(ack_cfg.ack_out_buffer_size);
        let (mut ack_in_tx, ack_in_rx) = mpsc::channel(ack_cfg.ticket_ack_buffer_size);

        let started = Instant::now();

        let egress = tokio::spawn(
            self.clone().send_acknowledgements(
                ack_out_rx,
                ack_cfg
                    .ack_output_concurrency
                    .filter(|&n| n > 0)
                    .unwrap_or(DEFAULT_ACK_CONCURRENCY),
            ),
        );
        let ingress = tokio::spawn(
            self.clone().receive_acknowledgements(
                ack_in_rx,
                ack_cfg
                    .ack_input_concurrency
                    .filter(|&n| n > 0)
                    .unwrap_or(DEFAULT_ACK_CONCURRENCY),
            ),
        );

        let decoder = self.clone();
        let mut decoded = futures::stream::select(futures::stream::iter(forward), acks_in_rx)
            .map(move |(chunk, peer, data)| {
                let relay = decoder.clone();
                async move {
                    let result = cpu::spawn_decode_blocking(
                        move || relay.codec.decoder.decode(peer, data),
                        "relay_benchmark_decode",
                    )
                    .await;
                    (chunk, result)
                }
            })
            .buffered(concurrency);

        while let Some((chunk, result)) = decoded.next().await {
            let failure = match result {
                Ok(Ok(IncomingPacket::Forwarded(packet))) => {
                    let IncomingForwardedPacket {
                        next_hop,
                        data,
                        ack_challenge,
                        received_ticket,
                        ack_key_prev_hop,
                        ..
                    } = *packet;
                    self.tracker
                        .insert(&next_hop, ack_challenge, received_ticket);
                    report.relayed += 1;
                    report.relayed_bytes += data.len();
                    ack_out_tx
                        .send(ack_key_prev_hop)
                        .await
                        .err()
                        .map(|e| e.to_string())
                }
                Ok(Ok(IncomingPacket::Acknowledgement(ack))) => ack_in_tx
                    .send((ack.previous_hop, ack.received_acks))
                    .await
                    .err()
                    .map(|e| e.to_string()),
                Ok(Ok(IncomingPacket::Final(_))) => {
                    Some("synthetic packet terminated at the relay".into())
                }
                Ok(Err(error)) => Some(error.to_string()),
                Err(error) => Some(error.to_string()),
            };

            if let Some(error) = failure {
                if chunk.is_some() {
                    report.rejected += 1;
                }
                report.first_error.get_or_insert(error);
            }

            if let Some(index) = chunk {
                pending[index] -= 1;
                if pending[index] == 0 {
                    if let (Some(tx), Some(acks)) =
                        (acks_in_tx.as_mut(), acknowledgements[index].take())
                    {
                        let _ = tx.send((None, self.next_peer, acks)).await;
                    }
                    remaining_chunks -= 1;
                    if remaining_chunks == 0 {
                        acks_in_tx = None;
                    }
                }
            }
        }

        drop(ack_out_tx);
        drop(ack_in_tx);
        let (egress, ingress) = futures::join!(egress, ingress);
        report.duration = started.elapsed();

        match egress {
            Ok(egress) => {
                report.acks_sent = egress.acks;
                report.ack_packets_sent = egress.packets;
                if let Some(error) = egress.first_error {
                    report.first_error.get_or_insert(error);
                }
            }
            Err(error) => {
                report.first_error.get_or_insert(error.to_string());
            }
        }
        match ingress {
            Ok(acknowledged) => report.tickets_acknowledged = acknowledged,
            Err(error) => {
                report.first_error.get_or_insert(error.to_string());
            }
        }

        report
    }

    async fn send_acknowledgements(
        self: Arc<Self>,
        ack_keys: mpsc::Receiver<HalfKey>,
        concurrency: usize,
    ) -> AckEgress {
        let packet_key = self.packet_key.clone();
        ack_keys
            .map(move |ack_key| VerifiedAcknowledgement::new(ack_key, &packet_key))
            .chunks(MAX_ACKNOWLEDGEMENTS_BATCH_SIZE)
            .map(|acks| {
                let relay = self.clone();
                async move {
                    let count = acks.len();
                    let encoded = cpu::spawn_fifo_blocking(
                        move || {
                            relay
                                .codec
                                .encoder
                                .encode_acknowledgements(&acks, &relay.sender_key)
                        },
                        "relay_benchmark_ack_encode",
                    )
                    .await;
                    (count, encoded)
                }
            })
            .buffer_unordered(concurrency)
            .fold(AckEgress::default(), |mut egress, (count, encoded)| {
                match encoded {
                    Ok(Ok(_)) => {
                        egress.acks += count;
                        egress.packets += 1;
                    }
                    Ok(Err(error)) => {
                        egress.first_error.get_or_insert(error.to_string());
                    }
                    Err(error) => {
                        egress.first_error.get_or_insert(error.to_string());
                    }
                }
                futures::future::ready(egress)
            })
            .await
    }

    async fn receive_acknowledgements(
        self: Arc<Self>,
        acks: mpsc::Receiver<(OffchainPublicKey, Vec<Acknowledgement>)>,
        concurrency: usize,
    ) -> usize {
        acks.map(|(peer, acks)| {
            let relay = self.clone();
            cpu::spawn_fifo_blocking(
                move || relay.tracker.acknowledge(peer, acks),
                "relay_benchmark_ack_decode",
            )
        })
        .buffer_unordered(concurrency)
        .fold(0, |total, acknowledged| {
            futures::future::ready(total + acknowledged.unwrap_or_default())
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn every_packet_should_be_relayed_and_acknowledged() -> anyhow::Result<()> {
        let packets = MAX_ACKNOWLEDGEMENTS_BATCH_SIZE * 2 + 3;
        let report = run_relay_benchmark(
            RelayBenchmarkConfig {
                packets,
                concurrency: 4,
            },
            |_| {},
        )
        .await?;

        assert_eq!(report.first_error, None);
        assert_eq!(report.packets, packets);
        assert_eq!(report.relayed, packets);
        assert_eq!(report.rejected, 0);
        assert_eq!(report.relayed_bytes, packets * HoprPacket::SIZE);
        assert_eq!(report.acks_sent, packets);
        assert_eq!(report.ack_packets_sent, 3);
        assert_eq!(report.tickets_acknowledged, packets);
        assert!(report.packets_per_second() > 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn phases_should_be_reported_in_order() -> anyhow::Result<()> {
        let phases = parking_lot::Mutex::new(Vec::new());
        run_relay_benchmark(
            RelayBenchmarkConfig {
                packets: 1,
                concurrency: 1,
            },
            |phase| phases.lock().push(phase),
        )
        .await?;

        assert_eq!(
            *phases.lock(),
            vec![
                RelayBenchmarkPhase::Preparing,
                RelayBenchmarkPhase::Relaying
            ]
        );
        Ok(())
    }
}
