//! In-memory network the relay benchmark runs on: four nodes and the channels a two-relay path
//! through the benchmarked node needs.

use std::{collections::HashMap, sync::Arc, time::Duration};

use futures::stream::{self, BoxStream};
use hopr_lib::api::{
    PeerId,
    chain::{
        ChainInfo, ChainKeyOperations, ChainReadChannelOperations, ChainReadTicketOperations,
        ChainValues, ChannelEntry, ChannelId, ChannelSelector, DomainSeparators, HoprKeyIdent,
        KeyIdMapping, RedemptionStats,
    },
    types::{
        crypto::prelude::{ChainKeypair, Hash, Keypair, OffchainKeypair, OffchainPublicKey},
        internal::prelude::{ChannelBuilder, CoreTypesError, WinningProbability},
        primitive::prelude::{Address, Balance, Currency, HoprBalance},
    },
};

/// Error for chain queries the packet codec never makes.
#[derive(Debug, thiserror::Error)]
#[error("not available in the synthetic relay benchmark network")]
pub struct Unsupported;

/// Identity of a synthetic node.
pub struct SyntheticNode {
    pub chain_key: ChainKeypair,
    pub packet_key: OffchainKeypair,
}

impl SyntheticNode {
    fn random() -> Self {
        Self {
            chain_key: ChainKeypair::random(),
            packet_key: OffchainKeypair::random(),
        }
    }

    pub fn address(&self) -> Address {
        self.chain_key.public().to_address()
    }

    pub fn public_key(&self) -> &OffchainPublicKey {
        self.packet_key.public()
    }

    pub fn peer_id(&self) -> PeerId {
        self.packet_key.public().into()
    }
}

/// Network of `sender -> relay -> next -> exit`, where `relay` is the node under benchmark.
pub struct SyntheticNetwork {
    pub sender: SyntheticNode,
    pub relay: SyntheticNode,
    pub next: SyntheticNode,
    pub exit: SyntheticNode,
    state: Arc<NetworkState>,
}

struct NetworkState {
    addresses: HashMap<Address, OffchainPublicKey>,
    packet_keys: HashMap<OffchainPublicKey, Address>,
    mapper: KeyMapper,
    channels: HashMap<ChannelId, ChannelEntry>,
}

impl SyntheticNetwork {
    pub const TICKET_WIN_PROB: WinningProbability = WinningProbability::ALWAYS;

    pub fn ticket_price() -> HoprBalance {
        HoprBalance::from(1_000_000_000_000_u64)
    }

    pub fn random() -> Result<Self, CoreTypesError> {
        let [sender, relay, next, exit] = std::array::from_fn(|_| SyntheticNode::random());

        let nodes = [&sender, &relay, &next, &exit];
        let addresses = nodes
            .iter()
            .map(|n| (n.address(), *n.public_key()))
            .collect();
        let packet_keys = nodes
            .iter()
            .map(|n| (*n.public_key(), n.address()))
            .collect();
        let mapper = KeyMapper(Arc::new(nodes.iter().map(|n| *n.public_key()).collect()));

        // The last hop needs no channel: it is paid with a zero-hop ticket.
        let channels = [(&sender, &relay), (&relay, &next)]
            .into_iter()
            .map(|(src, dst)| {
                let channel = ChannelBuilder::default()
                    .between(src.address(), dst.address())
                    .balance(HoprBalance::new_base(1_000_000_u64))
                    .build()?;
                Ok((*channel.get_id(), channel))
            })
            .collect::<Result<_, CoreTypesError>>()?;

        Ok(Self {
            state: Arc::new(NetworkState {
                addresses,
                packet_keys,
                mapper,
                channels,
            }),
            sender,
            relay,
            next,
            exit,
        })
    }

    /// Chain view of the network as seen by `node`.
    pub fn chain_of(&self, node: &SyntheticNode) -> SyntheticChain {
        SyntheticChain {
            me: node.address(),
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct KeyMapper(Arc<Vec<OffchainPublicKey>>);

impl KeyIdMapping<HoprKeyIdent, OffchainPublicKey> for KeyMapper {
    fn map_key_to_id(&self, key: &OffchainPublicKey) -> Option<HoprKeyIdent> {
        self.0
            .iter()
            .position(|k| k == key)
            .map(|i| HoprKeyIdent::from(i as u32))
    }

    fn map_id_to_public(&self, id: &HoprKeyIdent) -> Option<OffchainPublicKey> {
        self.0.get(u32::from(*id) as usize).copied()
    }
}

/// Read-only chain API over a [`SyntheticNetwork`].
#[derive(Clone)]
pub struct SyntheticChain {
    me: Address,
    state: Arc<NetworkState>,
}

impl ChainKeyOperations for SyntheticChain {
    type Error = Unsupported;
    type Mapper = KeyMapper;

    fn chain_key_to_packet_key(
        &self,
        chain: &Address,
    ) -> Result<Option<OffchainPublicKey>, Self::Error> {
        Ok(self.state.addresses.get(chain).copied())
    }

    fn packet_key_to_chain_key(
        &self,
        packet: &OffchainPublicKey,
    ) -> Result<Option<Address>, Self::Error> {
        Ok(self.state.packet_keys.get(packet).copied())
    }

    fn key_id_mapper_ref(&self) -> &Self::Mapper {
        &self.state.mapper
    }
}

impl ChainReadChannelOperations for SyntheticChain {
    type Error = Unsupported;

    fn me(&self) -> &Address {
        &self.me
    }

    fn channel_by_id(&self, channel_id: &ChannelId) -> Result<Option<ChannelEntry>, Self::Error> {
        Ok(self.state.channels.get(channel_id).copied())
    }

    fn stream_channels<'a>(
        &'a self,
        selector: ChannelSelector,
    ) -> Result<BoxStream<'a, ChannelEntry>, Self::Error> {
        Ok(Box::pin(stream::iter(
            self.state
                .channels
                .values()
                .filter(move |c| selector.satisfies(c))
                .copied(),
        )))
    }
}

impl ChainReadTicketOperations for SyntheticChain {
    type Error = Unsupported;

    fn incoming_ticket_values(&self) -> Result<(WinningProbability, HoprBalance), Self::Error> {
        Ok((
            SyntheticNetwork::TICKET_WIN_PROB,
            SyntheticNetwork::ticket_price(),
        ))
    }
}

#[async_trait::async_trait]
impl ChainValues for SyntheticChain {
    type Error = Unsupported;

    async fn balance<C: Currency, A: Into<Address> + Send>(
        &self,
        _address: A,
    ) -> Result<Balance<C>, Self::Error> {
        Err(Unsupported)
    }

    async fn domain_separators(&self) -> Result<DomainSeparators, Self::Error> {
        Ok(DomainSeparators {
            ledger: Hash::default(),
            safe_registry: Hash::default(),
            channel: Hash::default(),
        })
    }

    async fn minimum_incoming_ticket_win_prob(&self) -> Result<WinningProbability, Self::Error> {
        Ok(SyntheticNetwork::TICKET_WIN_PROB)
    }

    async fn minimum_ticket_price(&self) -> Result<HoprBalance, Self::Error> {
        Ok(SyntheticNetwork::ticket_price())
    }

    async fn key_binding_fee(&self) -> Result<HoprBalance, Self::Error> {
        Err(Unsupported)
    }

    async fn channel_closure_notice_period(&self) -> Result<Duration, Self::Error> {
        Err(Unsupported)
    }

    async fn chain_info(&self) -> Result<ChainInfo, Self::Error> {
        Err(Unsupported)
    }

    async fn redemption_stats<A: Into<Address> + Send>(
        &self,
        _safe_addr: A,
    ) -> Result<RedemptionStats, Self::Error> {
        Err(Unsupported)
    }

    async fn typical_resolution_time(&self) -> Result<Duration, Self::Error> {
        Err(Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use hopr_lib::api::{
        chain::ChainPathResolver,
        types::internal::prelude::{Path, ValidatedPath},
    };

    use super::*;

    #[test]
    fn key_mapper_should_round_trip_every_node_key() -> anyhow::Result<()> {
        let network = SyntheticNetwork::random()?;
        let chain = network.chain_of(&network.relay);
        let mapper = chain.key_id_mapper_ref();

        for node in [
            &network.sender,
            &network.relay,
            &network.next,
            &network.exit,
        ] {
            let id = mapper
                .map_key_to_id(node.public_key())
                .ok_or_else(|| anyhow::anyhow!("node key has no id"))?;
            assert_eq!(mapper.map_id_to_public(&id), Some(*node.public_key()));
        }
        Ok(())
    }

    #[tokio::test]
    async fn the_benchmark_path_should_validate_through_the_relay() -> anyhow::Result<()> {
        let network = SyntheticNetwork::random()?;
        let chain = network.chain_of(&network.sender);

        let path = ValidatedPath::new(
            network.sender.address(),
            vec![
                network.relay.address(),
                network.next.address(),
                network.exit.address(),
            ],
            &ChainPathResolver::from(&chain),
        )
        .await?;

        assert_eq!(path.num_hops(), 3);
        Ok(())
    }
}
