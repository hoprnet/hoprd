//! Log volume and collection cost of the periodic component state dump.
//!
//! Builds the component types hoprd runs (the trustful Blokli chain connector, the channel graph
//! and the libp2p network), fills them with synthetic node profiles, and runs the three upstream
//! reporters that `ComponentStateReporter` joins. Lines are formatted as hoprd formats them, both as
//! text and with `HOPRD_LOG_FORMAT=json`, under the filter recommended for nodes that keep other
//! logs at WARN. The chain data comes from a static Blokli test client instead of a live indexer.
//!
//! ```sh
//! nix develop -c cargo bench -p hoprd --bench state_dump --profile candidate
//! ```
//!
//! `HOPR_STATE_DUMP_INTERVAL` is forced to 5 s so the run takes seconds; record content does not
//! depend on the interval. Daily volume is extrapolated for longer intervals.
use std::{
    collections::BTreeMap,
    io,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

use anyhow::Context;
use hopr_chain_connector::{
    BlockchainConnectorConfig, HoprBlockchainSafeConnector, create_trustful_hopr_blokli_connector,
    testing::{BlokliTestClient, BlokliTestStateBuilder, StaticState},
};
use hopr_lib::api::{
    graph::{
        NetworkGraphTraverse as _, NetworkGraphUpdate as _, NetworkGraphView as _,
        NetworkGraphWrite as _,
        function::EdgeValueFn,
        traits::{Balance, EdgeObservableWrite as _, EdgeWeightType},
    },
    types::{
        crypto::prelude::{ChainKeypair, Keypair, OffchainKeypair},
        internal::prelude::{ChannelEntry, ChannelStatus},
        primitive::prelude::{Address, HoprBalance, WxHOPR, XDaiBalance},
    },
};
use hopr_network_graph::{ChannelGraph, SharedChannelGraph};
use hopr_transport_p2p::{HoprLibp2pNetworkBuilder, HoprNetwork};
use serde_json::Value;
use tracing_subscriber::{EnvFilter, fmt::MakeWriter, prelude::*};

/// Keeps every other log at WARN and the state records at INFO.
const FILTER: &str = "warn,hopr_chain_connector::state_reporting=info,\
                      hopr_network_graph::state_reporting=info,hopr_transport_p2p::state_reporting=info";
const COMPONENTS: [&str; 3] = ["chain", "graph", "transport"];
/// Reporting intervals the daily volume is extrapolated for, in seconds. 60 is the default.
const INTERVALS: [u64; 4] = [60, 300, 900, 3600];
/// Upstream cap on the serialized `state` field of one record.
const MAX_STATE_BYTES: usize = 32 * 1024;
const RECORD_DEADLINE: Duration = Duration::from_secs(30);

/// One synthetic node. Every peer has one outgoing and one incoming channel with this node.
struct Profile {
    name: &'static str,
    peers: usize,
    /// Open outgoing channels holding less than one ticket face value.
    underfunded: usize,
    /// Outgoing channels waiting for closure.
    pending_to_close: usize,
    /// Graph nodes, including this node and its peers.
    graph_nodes: usize,
    /// Whether path searches have run, which retains search samples in the graph record.
    searches: bool,
}

const PROFILES: [Profile; 4] = [
    Profile {
        name: "fresh",
        peers: 2,
        underfunded: 0,
        pending_to_close: 0,
        graph_nodes: 3,
        searches: false,
    },
    Profile {
        name: "typical",
        peers: 30,
        underfunded: 3,
        pending_to_close: 1,
        graph_nodes: 300,
        searches: true,
    },
    Profile {
        name: "busy",
        peers: 300,
        underfunded: 40,
        pending_to_close: 5,
        graph_nodes: 3_000,
        searches: true,
    },
    Profile {
        name: "stress",
        peers: 3_000,
        underfunded: 40,
        pending_to_close: 5,
        graph_nodes: 3_000,
        searches: true,
    },
];

type BenchConnector = HoprBlockchainSafeConnector<BlokliTestClient<StaticState>>;

struct Node {
    profile: &'static Profile,
    instance: String,
    chain: BenchConnector,
    graph: SharedChannelGraph,
    graph_edges: usize,
    safe: Address,
}

/// What one component's records cost in the log, maximum over the records seen.
#[derive(Default)]
struct LineCost {
    text_bytes: usize,
    text_no_color_bytes: usize,
    json_bytes: usize,
    state_bytes: usize,
    collection_ms: u64,
    reason: String,
}

#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Vec<u8>>>);

impl io::Write for Capture {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .map_err(|_| io::Error::other("capture lock poisoned"))?
            .extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Capture {
    type Writer = Capture;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

impl Capture {
    fn lines(&self) -> anyhow::Result<Vec<String>> {
        let bytes = self
            .0
            .lock()
            .map_err(|_| anyhow::anyhow!("capture lock poisoned"))?;
        Ok(String::from_utf8_lossy(&bytes)
            .lines()
            .map(str::to_owned)
            .collect())
    }
}

fn main() -> anyhow::Result<()> {
    // SAFETY: no other thread exists yet; the runtime is built below.
    unsafe { std::env::set_var("HOPR_STATE_DUMP_INTERVAL", "5") };

    let (text, json) = (Capture::default(), Capture::default());
    tracing_subscriber::registry()
        .with(EnvFilter::new(FILTER))
        .with(hoprd_fmt_layer().with_writer(text.clone()))
        .with(hoprd_fmt_layer().json().with_writer(json.clone()))
        .try_init()
        .context("install the capturing subscriber")?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(run(&text, &json))
}

/// The same fmt settings as hoprd's `install_base_subscriber`.
fn hoprd_fmt_layer<S>() -> tracing_subscriber::fmt::Layer<S>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(false)
}

async fn run(text: &Capture, json: &Capture) -> anyhow::Result<()> {
    let face_value = Balance::from(100_000_000_000_000_000u64);
    let mut nodes = Vec::new();
    for profile in &PROFILES {
        let started = Instant::now();
        nodes.push(build_node(profile, face_value).await?);
        eprintln!("built profile {} in {:?}", profile.name, started.elapsed());
    }
    let (network, _network_process) = HoprLibp2pNetworkBuilder::new(futures::stream::empty())
        .build(
            &OffchainKeypair::random(),
            vec![],
            "/hopr/state-dump-bench/1",
            true,
        )
        .await
        .map_err(|error| anyhow::anyhow!("build the P2P network: {error}"))?;

    let costs = emit_records(&nodes, &network, text, json).await?;
    let timings = time_collection(&nodes, &network).await?;
    print_report(&nodes, &costs, &timings);
    Ok(())
}

async fn build_node(profile: &'static Profile, face_value: Balance) -> anyhow::Result<Node> {
    let key = ChainKeypair::from_secret(&[0x11; 32])?;
    let me = key.public().to_address();
    let safe = Address::from([0x88; 20]);
    let peers: Vec<Address> = (0..profile.peers).map(peer_address).collect();

    let funded = face_value * Balance::from(10u64);
    let underfunded = face_value / Balance::from(2u64);
    let closes_at = SystemTime::now() + Duration::from_secs(3600);
    let mut channels = Vec::with_capacity(2 * peers.len());
    for (i, peer) in peers.iter().enumerate() {
        let (amount, status) = if i < profile.underfunded {
            (underfunded, ChannelStatus::Open)
        } else if i < profile.underfunded + profile.pending_to_close {
            (funded, ChannelStatus::PendingToClose(closes_at))
        } else {
            (funded, ChannelStatus::Open)
        };
        channels.push(
            ChannelEntry::builder()
                .between(me, *peer)
                .amount(amount)
                .status(status)
                .build()?,
        );
        channels.push(
            ChannelEntry::builder()
                .between(*peer, me)
                .amount(funded)
                .status(ChannelStatus::Open)
                .build()?,
        );
    }

    let accounts: Vec<&Address> = std::iter::once(&me).chain(peers.iter()).collect();
    let client = BlokliTestStateBuilder::default()
        .with_generated_accounts(
            &accounts,
            false,
            XDaiBalance::new_base(1),
            HoprBalance::new_base(10),
        )
        .with_channels(channels)
        .with_balances::<WxHOPR>([(safe, HoprBalance::new_base(1_000))])
        .with_safe_allowances([(safe, HoprBalance::new_base(500))])
        .build_static_client();
    let mut chain = create_trustful_hopr_blokli_connector(
        &key,
        BlockchainConnectorConfig {
            connection_sync_timeout: Duration::from_mins(5),
            sync_tolerance: 100,
            ..Default::default()
        },
        client,
        Address::from([0x99; 20]),
    )
    .await?;
    chain.connect().await?;

    let (graph, graph_edges) = build_graph(profile, face_value)?;
    Ok(Node {
        profile,
        instance: format!("bench-{}", profile.name),
        chain,
        graph,
        graph_edges,
        safe,
    })
}

/// Channels between this node and its peers carry full observations; the rest of the graph is a
/// sparse mesh of funded, probed edges so that multi-hop searches find paths.
fn build_graph(
    profile: &Profile,
    face_value: Balance,
) -> anyhow::Result<(SharedChannelGraph, usize)> {
    let keys = (0..profile.graph_nodes)
        .map(|i| packet_key(i).map(|key| *key.public()))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let me = keys[0];
    let graph: SharedChannelGraph = Arc::new(ChannelGraph::new(me));
    graph.set_ticket_face_value(face_value);
    for key in &keys {
        graph.add_node(*key);
    }

    let funded = Some(face_value * Balance::from(10u64));
    let mut edges = 0;
    for (i, peer) in keys.iter().enumerate().skip(1).take(profile.peers) {
        for (src, dst) in [(&me, peer), (peer, &me)] {
            graph.upsert_edge(src, dst, |o| {
                o.record(EdgeWeightType::Connected(true));
                o.record(EdgeWeightType::Balance(funded));
                o.record(EdgeWeightType::Immediate(Ok(Duration::from_millis(
                    20 + i as u64 % 80,
                ))));
                o.record(EdgeWeightType::Intermediate(Ok(Duration::from_millis(
                    60 + i as u64 % 120,
                ))));
                o.record(EdgeWeightType::ImmediateProtocolConformance {
                    num_packets: 1_000,
                    num_acks: 950 + i as u64 % 50,
                });
            });
            edges += 1;
        }
    }
    let n = keys.len();
    for i in 1..n {
        for step in [7, 13, 31] {
            let j = 1 + (i * step + 5) % (n - 1);
            if j != i {
                graph.upsert_edge(&keys[i], &keys[j], |o| {
                    o.record(EdgeWeightType::Balance(funded));
                    o.record(EdgeWeightType::Intermediate(Ok(Duration::from_millis(80))));
                });
                edges += 1;
            }
        }
    }

    if profile.searches {
        // An unreachable destination makes a failed search, which keeps its rejection evidence.
        let isolated = *packet_key(usize::MAX)?.public();
        graph.add_node(isolated);
        let length = NonZeroUsize::new(3).context("non-zero path length")?;
        for _ in 0..3 {
            graph.simple_loopback_to_self(3, Some(8));
            graph.simple_paths(
                &me,
                &isolated,
                3,
                Some(8),
                EdgeValueFn::forward(length, 0.5, 0.1, Some(face_value)),
            );
        }
    }
    Ok((graph, edges))
}

fn peer_address(i: usize) -> Address {
    let mut bytes = [0xbe; 20];
    bytes[12..].copy_from_slice(&(i as u64).to_be_bytes());
    Address::from(bytes)
}

fn packet_key(i: usize) -> anyhow::Result<OffchainKeypair> {
    let mut secret = [0x5a; 32];
    secret[..8].copy_from_slice(&(i as u64).to_be_bytes());
    Ok(OffchainKeypair::from_secret(&secret)?)
}

/// Runs the reporters as `ComponentStateReporter` does until each has logged a record.
async fn emit_records(
    nodes: &[Node],
    network: &HoprNetwork,
    text: &Capture,
    json: &Capture,
) -> anyhow::Result<BTreeMap<(String, &'static str), LineCost>> {
    let reporters = futures::future::join_all(nodes.iter().map(|node| async move {
        futures::join!(
            node.chain
                .run_state_reporting_with_funding_policy(&node.instance, node.safe, || node
                    .graph
                    .ticket_face_value()),
            node.graph.run_state_reporting(&node.instance, Vec::new),
            network.run_state_reporting(&node.instance),
        )
    }));
    let deadline = Instant::now() + RECORD_DEADLINE;
    let all_reported = async {
        loop {
            tokio::time::sleep(Duration::from_millis(250)).await;
            let costs = parse_records(text, json)?;
            let complete = nodes.iter().all(|node| {
                COMPONENTS.iter().all(|component| {
                    costs
                        .get(&(node.instance.clone(), *component))
                        .is_some_and(|cost| cost.json_bytes > 0 && cost.text_bytes > 0)
                })
            });
            if complete {
                return Ok(costs);
            }
            anyhow::ensure!(
                Instant::now() < deadline,
                "not every component reported within {RECORD_DEADLINE:?}; is the filter right?"
            );
        }
    };
    tokio::select! {
        results = reporters => anyhow::bail!("reporters stopped before reporting: {results:?}"),
        costs = all_reported => costs,
    }
}

fn parse_records(
    text: &Capture,
    json: &Capture,
) -> anyhow::Result<BTreeMap<(String, &'static str), LineCost>> {
    let mut costs: BTreeMap<(String, &'static str), LineCost> = BTreeMap::new();
    for line in json.lines()? {
        let record: Value = serde_json::from_str(&line)?;
        let fields = &record["fields"];
        if fields["event"] != "component_state" {
            continue;
        }
        let (Some(instance), Some(component)) = (
            fields["instance"].as_str(),
            component_name(fields["component"].as_str()),
        ) else {
            continue;
        };
        let state = fields["state"].as_str().unwrap_or_default();
        let envelope: Value = serde_json::from_str(state).unwrap_or(Value::Null);
        let cost = costs.entry((instance.to_owned(), component)).or_default();
        cost.json_bytes = cost.json_bytes.max(line.len() + 1);
        cost.state_bytes = cost.state_bytes.max(state.len());
        cost.collection_ms = cost
            .collection_ms
            .max(envelope["collection_ms"].as_u64().unwrap_or_default());
        cost.reason = fields["reason"].as_str().unwrap_or_default().to_owned();
    }
    for line in text.lines()? {
        let plain = strip_ansi(&line);
        if !plain.contains("event=\"component_state\"") {
            continue;
        }
        let (Some(instance), Some(component)) = (
            text_field(&plain, "instance"),
            component_name(text_field(&plain, "component")),
        ) else {
            continue;
        };
        let cost = costs.entry((instance.to_owned(), component)).or_default();
        cost.text_bytes = cost.text_bytes.max(line.len() + 1);
        cost.text_no_color_bytes = cost.text_no_color_bytes.max(plain.len() + 1);
    }
    Ok(costs)
}

fn component_name(name: Option<&str>) -> Option<&'static str> {
    COMPONENTS.into_iter().find(|c| Some(*c) == name)
}

/// The value of `name=` in a text line, without quotes. Values here contain no spaces.
fn text_field<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let start = line.find(&format!(" {name}="))? + name.len() + 2;
    let value = line[start..].split(' ').next()?;
    Some(value.trim_matches('"'))
}

fn strip_ansi(line: &str) -> String {
    let mut plain = String::with_capacity(line.len());
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            chars.by_ref().find(|c| *c == 'm');
        } else {
            plain.push(c);
        }
    }
    plain
}

/// Median and maximum wall time of one collection, per profile and component.
type Timings = BTreeMap<(String, &'static str), (Duration, Duration)>;

/// Times the component reads directly; record `collection_ms` only has millisecond resolution.
async fn time_collection(nodes: &[Node], network: &HoprNetwork) -> anyhow::Result<Timings> {
    let mut timings = Timings::new();
    for node in nodes {
        let face_value = node.graph.ticket_face_value();
        let mut chain = Vec::new();
        for _ in 0..20 {
            let started = Instant::now();
            let state = node
                .chain
                .state(node.safe, face_value)
                .await
                .map_err(anyhow::Error::msg)?;
            std::hint::black_box(serde_json::to_vec(&state)?);
            chain.push(started.elapsed());
        }
        let mut graph = Vec::new();
        for _ in 0..200 {
            let started = Instant::now();
            std::hint::black_box(serde_json::to_vec(&node.graph.state(Some(&[]))?)?);
            graph.push(started.elapsed());
        }
        let mut transport = Vec::new();
        for _ in 0..200 {
            let started = Instant::now();
            let state = network.state().await.map_err(anyhow::Error::msg)?;
            std::hint::black_box(serde_json::to_vec(&state)?);
            transport.push(started.elapsed());
        }
        for (component, mut samples) in
            [("chain", chain), ("graph", graph), ("transport", transport)]
        {
            samples.sort();
            let median = samples[samples.len() / 2];
            let max = samples.last().copied().unwrap_or_default();
            timings.insert((node.instance.clone(), component), (median, max));
        }
    }
    Ok(timings)
}

fn print_report(
    nodes: &[Node],
    costs: &BTreeMap<(String, &'static str), LineCost>,
    timings: &Timings,
) {
    println!("# State dump benchmark\n");
    println!("Filter: `{FILTER}`\n");

    println!("## Node profiles\n");
    println!(
        "| profile | local channels | underfunded | pending to close | graph nodes | graph edges | searches |"
    );
    println!("|---|--:|--:|--:|--:|--:|---|");
    for node in nodes {
        let p = node.profile;
        println!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            p.name,
            2 * p.peers,
            p.underfunded,
            p.pending_to_close,
            p.graph_nodes,
            node.graph_edges,
            if p.searches { "yes" } else { "no" },
        );
    }

    println!("\n## Bytes per log line\n");
    println!(
        "| profile | component | text (hoprd default) | text (`NO_COLOR=1`) | JSON | `state` field | reason |"
    );
    println!("|---|---|--:|--:|--:|--:|---|");
    for node in nodes {
        for component in COMPONENTS {
            let Some(cost) = costs.get(&(node.instance.clone(), component)) else {
                continue;
            };
            println!(
                "| {} | {component} | {} | {} | {} | {} | `{}` |",
                node.profile.name,
                cost.text_bytes,
                cost.text_no_color_bytes,
                cost.json_bytes,
                cost.state_bytes,
                cost.reason,
            );
        }
    }

    println!("\n## Log volume per node per day (three records per interval)\n");
    print!("| profile | format |");
    for interval in INTERVALS {
        print!(" every {interval} s |");
    }
    println!();
    println!("|---|---|{}", "--:|".repeat(INTERVALS.len()));
    for node in nodes {
        let total = |bytes: fn(&LineCost) -> usize| -> usize {
            COMPONENTS
                .iter()
                .filter_map(|c| costs.get(&(node.instance.clone(), *c)))
                .map(bytes)
                .sum()
        };
        for (format, per_interval) in [
            ("text", total(|c| c.text_bytes)),
            ("text, NO_COLOR", total(|c| c.text_no_color_bytes)),
            ("JSON", total(|c| c.json_bytes)),
        ] {
            print!("| {} | {format} |", node.profile.name);
            for interval in INTERVALS {
                let per_day = per_interval as f64 * (86_400 / interval) as f64;
                print!(" {} |", human_bytes(per_day));
            }
            println!();
        }
    }

    println!("\n## Collection cost per record\n");
    println!("| profile | component | median | max | `collection_ms` in record |");
    println!("|---|---|--:|--:|--:|");
    for node in nodes {
        for component in COMPONENTS {
            let key = (node.instance.clone(), component);
            let (Some((median, max)), Some(cost)) = (timings.get(&key), costs.get(&key)) else {
                continue;
            };
            println!(
                "| {} | {component} | {median:.1?} | {max:.1?} | {} |",
                node.profile.name, cost.collection_ms,
            );
        }
    }

    let over_budget: Vec<_> = costs
        .iter()
        .filter(|(_, cost)| cost.state_bytes > MAX_STATE_BYTES)
        .map(|((instance, component), _)| format!("{instance}/{component}"))
        .collect();
    if !over_budget.is_empty() {
        println!("\nOver the {MAX_STATE_BYTES}-byte state budget: {over_budget:?}");
    }
}

fn human_bytes(bytes: f64) -> String {
    const MIB: f64 = 1024.0 * 1024.0;
    if bytes >= MIB {
        format!("{:.1} MiB", bytes / MIB)
    } else {
        format!("{:.0} KiB", bytes / 1024.0)
    }
}
