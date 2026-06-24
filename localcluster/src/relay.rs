//! Userspace UDP relay that injects artificial latency between cluster nodes.
//!
//! When latency is enabled, each node `X` announces the relay's port `P_X` on chain
//! instead of its real listen port `R_X` (and disables its own announce). Peers
//! therefore dial `P_X`; the relay forwards datagrams to `R_X` after a per-link delay
//! and relays the node's replies back. Because libp2p-quic reuses its listen socket
//! for outbound traffic, a peer `Y`'s source UDP port equals its listen port
//! `p2p_port_base + Y`, which lets the relay identify the sender and apply a delay
//! specific to the directed link `Y → X`.
//!
//! All traffic destined for `X` traverses `relay_X`, so a single relay per node gives
//! full per-link coverage. The forward leg (`Y → X`) is shaped by `resolve(Y, X)` and
//! the return leg (`X → Y`) by `resolve(X, Y)`, allowing independent directional delays.
//!
//! **Ordering — modelled physically.** Each datagram is released at
//! `arrival + sampled_delay` and delivered in release-time order (a per-flow worker with a
//! release-time min-heap). This mirrors a real link:
//! - a *fixed* delay shifts every packet equally, so order is preserved;
//! - *jitter* lets a packet that drew a smaller delay overtake an earlier one, so packets
//!   reorder — exactly as they do on the real internet.
//!
//! Ordering is therefore a consequence of the delays, never forced. (A naive
//! task-per-datagram timer using a *relative* sleep would also reorder under a *fixed*
//! delay because equal timer deadlines wake in arbitrary order — an artefact this avoids.)

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    future,
    net::SocketAddr,
    sync::Arc,
};

use anyhow::Context;
use tokio::{
    net::UdpSocket,
    sync::mpsc,
    task::JoinHandle,
    time::{Instant, sleep_until},
};
use tracing::warn;

use crate::latency::{DelayDist, LatencyConfig};

/// Largest datagram the relay will buffer. QUIC datagrams stay well under this.
const MAX_DATAGRAM: usize = 64 * 1024;

/// Sentinel peer index used when a datagram's source port is not a known node port.
/// It never matches a `per_link`/`per_node` entry, so such traffic falls back to the
/// global default delay (or no delay).
const UNKNOWN_PEER: usize = usize::MAX;

/// A datagram tagged with the instant it arrived at the relay, so the sender worker can
/// schedule its release relative to arrival rather than to when it is dequeued.
type Pending = (Instant, Vec<u8>);

/// Parameters for a single node's relay.
pub struct RelayConfig {
    /// Index of the node this relay fronts (used as the `dst` for inbound traffic).
    pub node_id: usize,
    /// Address the relay binds and announces (`P_X`).
    pub listen: SocketAddr,
    /// The node's real listen address packets are forwarded to (`R_X`).
    pub target: SocketAddr,
    /// Base P2P port, used to recover a peer's node index from its source port.
    pub p2p_port_base: u16,
    /// Latency model resolved per directed link.
    pub latency: Arc<LatencyConfig>,
}

/// Handle to a running relay; aborts the relay task on [`RelayHandle::abort`].
pub struct RelayHandle {
    listen_addr: SocketAddr,
    handle: JoinHandle<()>,
}

impl RelayHandle {
    /// The address the relay is actually bound to (resolved, post-bind).
    pub fn listen_addr(&self) -> SocketAddr {
        self.listen_addr
    }

    /// Stop the relay.
    pub fn abort(&self) {
        self.handle.abort();
    }
}

/// Bind the relay's listen socket and spawn its forwarding loop.
///
/// Binding happens synchronously so the caller can guarantee the relay is ready
/// before nodes start dialing it.
pub async fn spawn_relay(cfg: RelayConfig) -> anyhow::Result<RelayHandle> {
    let listen = Arc::new(
        UdpSocket::bind(cfg.listen)
            .await
            .with_context(|| format!("failed to bind relay listen socket {}", cfg.listen))?,
    );
    let listen_addr = listen.local_addr().context("relay listen addr")?;
    let handle = tokio::spawn(run_relay(Arc::new(cfg), listen));
    Ok(RelayHandle {
        listen_addr,
        handle,
    })
}

async fn run_relay(cfg: Arc<RelayConfig>, listen: Arc<UdpSocket>) {
    // One forward queue per peer; the node then sees each peer as a stable distinct source.
    let mut sessions: HashMap<SocketAddr, mpsc::UnboundedSender<Pending>> = HashMap::new();
    let mut buf = vec![0u8; MAX_DATAGRAM];

    loop {
        let (len, peer) = match listen.recv_from(&mut buf).await {
            Ok(v) => v,
            Err(e) => {
                warn!(error = %e, "relay listen socket recv failed; stopping relay");
                return;
            }
        };

        // Not the entry API: creating the value is an async, fallible call (bind/connect)
        // that may `continue` on error, which `entry().or_insert_with` can't express.
        #[allow(clippy::map_entry)]
        if !sessions.contains_key(&peer) {
            match new_upstream(cfg.clone(), listen.clone(), peer).await {
                Ok(tx) => {
                    sessions.insert(peer, tx);
                }
                Err(e) => {
                    warn!(error = %e, %peer, "relay failed to create upstream socket");
                    continue;
                }
            }
        }

        let datagram = buf[..len].to_vec();
        // Forward leg: peer Y -> node X, shaped by resolve(Y, X).
        if sessions[&peer].send((Instant::now(), datagram)).is_err() {
            sessions.remove(&peer);
        }
    }
}

/// Create an upstream socket toward the node and spawn the return-path reader. Returns
/// the queue that the forward leg (peer → node) feeds.
async fn new_upstream(
    cfg: Arc<RelayConfig>,
    listen: Arc<UdpSocket>,
    peer: SocketAddr,
) -> anyhow::Result<mpsc::UnboundedSender<Pending>> {
    let bind_addr: SocketAddr = if cfg.target.is_ipv6() {
        "[::]:0".parse().expect("valid v6 wildcard")
    } else {
        "0.0.0.0:0".parse().expect("valid v4 wildcard")
    };
    let upstream = Arc::new(
        UdpSocket::bind(bind_addr)
            .await
            .context("bind relay upstream socket")?,
    );
    upstream
        .connect(cfg.target)
        .await
        .with_context(|| format!("connect relay upstream to {}", cfg.target))?;

    let peer_idx = peer_index(&cfg, peer);
    let forward_delay = cfg.latency.resolve(peer_idx, cfg.node_id);
    let return_delay = cfg.latency.resolve(cfg.node_id, peer_idx);

    // Return leg: node X -> peer Y, sent from the listen socket back to the peer.
    let return_tx = spawn_sender(listen, ForwardTarget::SendTo(peer), return_delay);
    let reader = upstream.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; MAX_DATAGRAM];
        loop {
            match reader.recv(&mut buf).await {
                Ok(len) => {
                    if return_tx
                        .send((Instant::now(), buf[..len].to_vec()))
                        .is_err()
                    {
                        return;
                    }
                }
                Err(_) => return,
            }
        }
    });

    Ok(spawn_sender(
        upstream,
        ForwardTarget::Connected,
        forward_delay,
    ))
}

#[derive(Clone, Copy)]
enum ForwardTarget {
    /// Send on a `connect`-ed socket (relay → node).
    Connected,
    /// Send to an explicit address (relay → peer).
    SendTo(SocketAddr),
}

/// Spawn a delaying forwarder for one directed flow.
///
/// Each datagram's release time is `arrival + sampled_delay`; datagrams are sent in
/// release-time order via a min-heap. A fixed delay preserves order; jitter lets a packet
/// with a smaller delay overtake an earlier one, so reordering happens naturally.
fn spawn_sender(
    socket: Arc<UdpSocket>,
    target: ForwardTarget,
    delay: Option<DelayDist>,
) -> mpsc::UnboundedSender<Pending> {
    let (tx, mut rx) = mpsc::unbounded_channel::<Pending>();
    tokio::spawn(async move {
        // Ordered by release instant; `seq` breaks ties so payloads are never compared.
        let mut heap: BinaryHeap<Reverse<(Instant, u64, Vec<u8>)>> = BinaryHeap::new();
        let mut seq: u64 = 0;

        loop {
            let now = Instant::now();
            while heap.peek().is_some_and(|Reverse((r, _, _))| *r <= now) {
                let Reverse((_, _, datagram)) = heap.pop().expect("peeked");
                let result = match target {
                    ForwardTarget::Connected => socket.send(&datagram).await.map(|_| ()),
                    ForwardTarget::SendTo(addr) => {
                        socket.send_to(&datagram, addr).await.map(|_| ())
                    }
                };
                if let Err(e) = result {
                    warn!(error = %e, "relay forward failed");
                }
            }

            let next_release = heap.peek().map(|Reverse((r, _, _))| *r);
            tokio::select! {
                maybe = rx.recv() => match maybe {
                    Some((arrival, datagram)) => {
                        let release = arrival + delay.map(|d| d.sample()).unwrap_or_default();
                        heap.push(Reverse((release, seq, datagram)));
                        seq += 1;
                    }
                    None => return,
                },
                _ = async {
                    match next_release {
                        Some(r) => sleep_until(r).await,
                        None => future::pending::<()>().await,
                    }
                } => {}
            }
        }
    });
    tx
}

/// Recover a peer's node index from its source port, or [`UNKNOWN_PEER`].
fn peer_index(cfg: &RelayConfig, addr: SocketAddr) -> usize {
    let port = addr.port();
    if port >= cfg.p2p_port_base {
        (port - cfg.p2p_port_base) as usize
    } else {
        UNKNOWN_PEER
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant as StdInstant};

    use super::*;

    fn relay_cfg(node_id: usize, target: SocketAddr, latency: LatencyConfig) -> RelayConfig {
        RelayConfig {
            node_id,
            listen: "127.0.0.1:0".parse().unwrap(),
            target,
            p2p_port_base: 9000,
            latency: Arc::new(latency),
        }
    }

    #[tokio::test]
    async fn relay_forwards_both_directions_with_delay() {
        let node = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let node_addr = node.local_addr().unwrap();

        let relay = spawn_relay(relay_cfg(
            0,
            node_addr,
            LatencyConfig::global(DelayDist::Fixed(Duration::from_millis(80))),
        ))
        .await
        .unwrap();

        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let start = StdInstant::now();
        client.send_to(b"ping", relay.listen_addr()).await.unwrap();

        let mut buf = [0u8; 16];
        let (n, from) = node.recv_from(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"ping");
        assert!(
            start.elapsed() >= Duration::from_millis(70),
            "forward leg was not delayed: {:?}",
            start.elapsed()
        );

        node.send_to(b"pong", from).await.unwrap();
        let (n, _) = client.recv_from(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"pong");
        assert!(
            start.elapsed() >= Duration::from_millis(150),
            "return leg was not delayed: {:?}",
            start.elapsed()
        );

        relay.abort();
    }

    #[tokio::test]
    async fn per_link_delay_is_keyed_by_source_port() {
        let p2p_port_base = 49_000u16;
        let node = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let node_addr = node.local_addr().unwrap();

        let mut latency = LatencyConfig::default();
        latency
            .per_link
            .insert((0, 5), DelayDist::Fixed(Duration::from_millis(30)));
        latency
            .per_link
            .insert((1, 5), DelayDist::Fixed(Duration::from_millis(200)));

        let relay = spawn_relay(RelayConfig {
            node_id: 5,
            listen: "127.0.0.1:0".parse().unwrap(),
            target: node_addr,
            p2p_port_base,
            latency: Arc::new(latency),
        })
        .await
        .unwrap();
        let relay_addr = relay.listen_addr();

        let client0 = UdpSocket::bind(("127.0.0.1", p2p_port_base)).await.unwrap();
        let client1 = UdpSocket::bind(("127.0.0.1", p2p_port_base + 1))
            .await
            .unwrap();

        let start = StdInstant::now();
        client1.send_to(b"from1", relay_addr).await.unwrap();
        client0.send_to(b"from0", relay_addr).await.unwrap();

        let mut buf = [0u8; 8];
        let (n, _) = node.recv_from(&mut buf).await.unwrap();
        let fast_at = start.elapsed();
        assert_eq!(
            &buf[..n],
            b"from0",
            "fast link (0 -> 5) should arrive first"
        );

        let (n, _) = node.recv_from(&mut buf).await.unwrap();
        let slow_at = start.elapsed();
        assert_eq!(
            &buf[..n],
            b"from1",
            "slow link (1 -> 5) should arrive second"
        );
        // Assert the relative lag (configured delta is 200ms - 30ms = 170ms) rather than
        // absolute arrival times, so global scheduling overhead on a loaded runner can't
        // make this flaky.
        assert!(
            slow_at - fast_at >= Duration::from_millis(150),
            "slow link should lag fast by ~170ms: fast={fast_at:?} slow={slow_at:?}"
        );

        relay.abort();
    }

    #[tokio::test]
    async fn preserves_order_under_burst() {
        // A burst of datagrams at a fixed delay must arrive in send order — a reordered
        // segment would break HOPR session frame reassembly.
        let node = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let node_addr = node.local_addr().unwrap();

        let relay = spawn_relay(relay_cfg(
            0,
            node_addr,
            LatencyConfig::global(DelayDist::Fixed(Duration::from_millis(30))),
        ))
        .await
        .unwrap();

        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        for i in 0u8..32 {
            client.send_to(&[i], relay.listen_addr()).await.unwrap();
        }

        let mut got = Vec::new();
        for _ in 0..32 {
            let mut b = [0u8; 1];
            let (_, _) = node.recv_from(&mut b).await.unwrap();
            got.push(b[0]);
        }
        assert_eq!(
            got,
            (0u8..32).collect::<Vec<_>>(),
            "datagrams were reordered"
        );

        relay.abort();
    }
}
