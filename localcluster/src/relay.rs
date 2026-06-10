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
//! A delay is sampled and applied per datagram via a spawned timer task; this can
//! reorder datagrams, which QUIC tolerates and which mirrors real networks.

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::Context;
use tokio::{net::UdpSocket, task::JoinHandle, time::sleep};
use tracing::warn;

use crate::latency::LatencyConfig;

/// Largest datagram the relay will buffer. QUIC datagrams stay well under this.
const MAX_DATAGRAM: usize = 64 * 1024;

/// Sentinel peer index used when a datagram's source port is not a known node port.
/// It never matches a `per_link`/`per_node` entry, so such traffic falls back to the
/// global default delay (or no delay).
const UNKNOWN_PEER: usize = usize::MAX;

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
    // One upstream socket per peer so the node sees each peer as a stable distinct source.
    let mut sessions: HashMap<SocketAddr, Arc<UdpSocket>> = HashMap::new();
    let mut buf = vec![0u8; MAX_DATAGRAM];

    loop {
        let (len, peer) = match listen.recv_from(&mut buf).await {
            Ok(v) => v,
            Err(e) => {
                warn!(error = %e, "relay listen socket recv failed; stopping relay");
                return;
            }
        };

        let upstream = match sessions.get(&peer) {
            Some(u) => u.clone(),
            None => match new_upstream(cfg.clone(), listen.clone(), peer).await {
                Ok(u) => {
                    sessions.insert(peer, u.clone());
                    u
                }
                Err(e) => {
                    warn!(error = %e, %peer, "relay failed to create upstream socket");
                    continue;
                }
            },
        };

        let datagram = buf[..len].to_vec();
        // Forward leg: peer Y -> node X, shaped by resolve(Y, X).
        let delay = cfg.latency.resolve(peer_index(&cfg, peer), cfg.node_id);
        forward(upstream, datagram, delay, ForwardTarget::Connected);
    }
}

/// Create an upstream socket toward the node and spawn the return-path reader.
async fn new_upstream(
    cfg: Arc<RelayConfig>,
    listen: Arc<UdpSocket>,
    peer: SocketAddr,
) -> anyhow::Result<Arc<UdpSocket>> {
    let bind_addr: SocketAddr = if cfg.target.is_ipv6() {
        "[::]:0".parse().expect("valid v6 wildcard")
    } else {
        "0.0.0.0:0".parse().expect("valid v4 wildcard")
    };
    let sock = UdpSocket::bind(bind_addr)
        .await
        .context("bind relay upstream socket")?;
    sock.connect(cfg.target)
        .await
        .with_context(|| format!("connect relay upstream to {}", cfg.target))?;
    let sock = Arc::new(sock);

    let reader = sock.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; MAX_DATAGRAM];
        loop {
            match reader.recv(&mut buf).await {
                Ok(len) => {
                    let datagram = buf[..len].to_vec();
                    // Return leg: node X -> peer Y, shaped by resolve(X, Y).
                    let delay = cfg.latency.resolve(cfg.node_id, peer_index(&cfg, peer));
                    forward(listen.clone(), datagram, delay, ForwardTarget::SendTo(peer));
                }
                Err(_) => return,
            }
        }
    });

    Ok(sock)
}

enum ForwardTarget {
    /// Send on a `connect`-ed socket (relay → node).
    Connected,
    /// Send to an explicit address (relay → peer).
    SendTo(SocketAddr),
}

/// Send `datagram` on `socket`, optionally after a sampled delay. A delayed send is
/// spawned so the relay's receive loop is never blocked waiting on the timer.
fn forward(
    socket: Arc<UdpSocket>,
    datagram: Vec<u8>,
    delay: Option<crate::latency::DelayDist>,
    target: ForwardTarget,
) {
    tokio::spawn(async move {
        if let Some(dist) = delay {
            sleep(dist.sample()).await;
        }
        let result = match target {
            ForwardTarget::Connected => socket.send(&datagram).await.map(|_| ()),
            ForwardTarget::SendTo(addr) => socket.send_to(&datagram, addr).await.map(|_| ()),
        };
        if let Err(e) = result {
            warn!(error = %e, "relay forward failed");
        }
    });
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
    use std::time::{Duration, Instant};

    use super::*;
    use crate::latency::DelayDist;

    #[tokio::test]
    async fn relay_forwards_both_directions_with_delay() {
        // Stand-in for the node: an echo socket.
        let node = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let node_addr = node.local_addr().unwrap();

        let cfg = RelayConfig {
            node_id: 0,
            listen: "127.0.0.1:0".parse().unwrap(),
            target: node_addr,
            p2p_port_base: 9000,
            latency: Arc::new(LatencyConfig::global(DelayDist::Fixed(
                Duration::from_millis(80),
            ))),
        };
        let relay = spawn_relay(cfg).await.unwrap();

        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let start = Instant::now();
        client.send_to(b"ping", relay.listen_addr()).await.unwrap();

        // Forward leg reaches the node after ~80ms.
        let mut buf = [0u8; 16];
        let (n, from) = node.recv_from(&mut buf).await.unwrap();
        let forward_elapsed = start.elapsed();
        assert_eq!(&buf[..n], b"ping");
        assert!(
            forward_elapsed >= Duration::from_millis(70),
            "forward leg was not delayed: {forward_elapsed:?}"
        );

        // Echo back; return leg reaches the client after another ~80ms.
        node.send_to(b"pong", from).await.unwrap();
        let mut buf = [0u8; 16];
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
        // Clients bind the fixed ports a node would use, so the relay recovers their
        // node index from the source port and applies the matching per-link delay.
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
        let client1 = UdpSocket::bind(("127.0.0.1", p2p_port_base + 1)).await.unwrap();

        let start = Instant::now();
        // Send the slow link first; the fast link must still arrive first.
        client1.send_to(b"from1", relay_addr).await.unwrap();
        client0.send_to(b"from0", relay_addr).await.unwrap();

        let mut buf = [0u8; 8];
        let (n, _) = node.recv_from(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"from0", "fast link (0 -> 5) should arrive first");
        assert!(
            start.elapsed() < Duration::from_millis(120),
            "fast link was too slow: {:?}",
            start.elapsed()
        );

        let (n, _) = node.recv_from(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"from1", "slow link (1 -> 5) should arrive second");
        assert!(
            start.elapsed() >= Duration::from_millis(180),
            "slow link was too fast: {:?}",
            start.elapsed()
        );

        relay.abort();
    }
}
