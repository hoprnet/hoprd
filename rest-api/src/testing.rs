//! Test mocks for REST API handler testing.
//!
//! Uses `mockall::mock!` on the narrowed trait interfaces that individual
//! handlers require. Each mock covers the minimal trait surface for one
//! group of endpoints.
//!
//! [`StubChain`] is a lightweight in-memory chain connector that implements
//! all 11 `HoprChainApi` sub-traits. Write operations return stub errors;
//! read operations return sensible zero-values.
//!
//! [`MockChainNode`] composes `StubChain` with a `NodeOnchainIdentity` and
//! implements `HasChainApi` so that REST API handlers bound on
//! `HasChainApi<ChainError = HoprLibError>` can be tested.

use std::{collections::HashSet, time::Duration};

use async_trait::async_trait;
use bimap::BiMap;
use futures::stream::{self, BoxStream};
use hopr_lib::{
    api::{
        Multiaddr, PeerId,
        chain::{self, *},
        network::{Health, NetworkEvent, NetworkView},
        node::{
            ComponentStatus, EventWaitResult, HasChainApi, HasNetworkView, HasTicketManagement,
            HoprNodeOperations, HoprState, NodeOnchainIdentity, TicketEvent,
        },
        tickets::{ChannelStats, RedemptionResult, TicketManagement},
        types::{
            crypto::{
                keypairs::{ChainKeypair, Keypair, OffchainKeypair},
                types::OffchainPublicKey,
            },
            internal::tickets::WinningProbability,
            primitive::prelude::{Address, Balance, Currency, HoprBalance, KeyIdMapping, KeyIdent},
        },
    },
    errors::HoprLibError,
};

// ---------------------------------------------------------------------------
// Mock for HoprNodeOperations (startedz)
// ---------------------------------------------------------------------------

mockall::mock! {
    pub NodeOps {}
    impl HoprNodeOperations for NodeOps {
        fn status(&self) -> HoprState;
    }
}

// ---------------------------------------------------------------------------
// Mock for NetworkView
// ---------------------------------------------------------------------------

mockall::mock! {
    pub NetView {}
    #[allow(refining_impl_trait)]
    impl NetworkView for NetView {
        fn listening_as(&self) -> HashSet<Multiaddr>;
        fn multiaddress_of(&self, peer: &PeerId) -> Option<HashSet<Multiaddr>>;
        fn discovered_peers(&self) -> HashSet<PeerId>;
        fn connected_peers(&self) -> HashSet<PeerId>;
        fn is_connected(&self, peer: &PeerId) -> bool;
        fn health(&self) -> Health;
        fn subscribe_network_events(&self) -> futures::stream::Empty<NetworkEvent>;
    }
}

// ---------------------------------------------------------------------------
// Composite mock for HoprNodeOperations + HasNetworkView (readyz, healthyz)
// ---------------------------------------------------------------------------

/// Composite mock implementing both `HoprNodeOperations` and `HasNetworkView`.
///
/// mockall can't mock two traits with same-named methods (`status`) in one
/// `mock!` block, so we compose them manually.
pub struct ChecksNode {
    pub node_state: HoprState,
    pub net: MockNetView,
    pub chain: StubChain,
    pub identity: NodeOnchainIdentity,
}

impl ChecksNode {
    pub fn new(state: HoprState, health: Health) -> Self {
        let mut net = MockNetView::new();
        net.expect_health().returning(move || health);
        let chain_key = ChainKeypair::random();
        let offchain_key = OffchainKeypair::random();
        Self {
            node_state: state,
            net,
            identity: NodeOnchainIdentity {
                node_address: chain_key.public().to_address(),
                safe_address: Address::default(),
                module_address: Address::default(),
            },
            chain: StubChain::new(&offchain_key, &chain_key),
        }
    }
}

impl HoprNodeOperations for ChecksNode {
    fn status(&self) -> HoprState {
        self.node_state
    }
}

impl HasNetworkView for ChecksNode {
    type NetworkView = MockNetView;

    fn network_view(&self) -> &MockNetView {
        &self.net
    }

    fn status(&self) -> ComponentStatus {
        ComponentStatus::Ready
    }
}

impl HasChainApi for ChecksNode {
    type ChainApi = StubChain;
    type ChainError = HoprLibError;

    fn identity(&self) -> &NodeOnchainIdentity {
        &self.identity
    }

    fn chain_api(&self) -> &StubChain {
        &self.chain
    }

    fn status(&self) -> ComponentStatus {
        ComponentStatus::Ready
    }

    fn wait_for_on_chain_event<F>(
        &self,
        _predicate: F,
        _context: String,
        _timeout: Duration,
    ) -> EventWaitResult<
        <Self::ChainApi as hopr_lib::api::chain::HoprChainApi>::ChainError,
        Self::ChainError,
    >
    where
        F: Fn(&hopr_lib::api::chain::ChainEvent) -> bool + Send + Sync + 'static,
    {
        unimplemented!("not needed for checks tests")
    }
}

// ---------------------------------------------------------------------------
// Bare unit type for handlers that don't use hopr at all
// ---------------------------------------------------------------------------

/// For handlers bound only on `Send + Sync + 'static`
/// (`configuration`, `list_clients`, `close_client`, `authenticate`).
pub struct NoopNode;

// ===========================================================================
// StubChain — lightweight in-memory chain connector for test doubles
// ===========================================================================

/// Stub error type for test doubles.
#[derive(Debug, Clone, thiserror::Error)]
#[error("stub: {0}")]
pub struct StubError(pub String);

/// Stub chain connector satisfying all `HoprChainApi` sub-trait bounds.
///
/// Read operations return sensible zero-values.
/// Write operations return [`StubError`].
#[derive(Debug, Clone)]
pub struct StubChain {
    me_addr: Address,
    keys: BiMap<Address, OffchainPublicKey>,
    mapper: StubKeyMapper,
}

impl StubChain {
    pub fn new(offchain: &OffchainKeypair, chain: &ChainKeypair) -> Self {
        let addr = chain.public().to_address();
        let mut keys = BiMap::new();
        keys.insert(addr, *offchain.public());
        Self {
            me_addr: addr,
            keys,
            mapper: StubKeyMapper,
        }
    }

    /// Registers a peer so `packet_key_to_chain_key` can resolve it.
    pub fn register_peer(&mut self, packet: OffchainPublicKey, address: Address) {
        self.keys.insert(address, packet);
    }
}

// --- ChainReadChannelOperations ---

impl ChainReadChannelOperations for StubChain {
    type Error = StubError;

    fn me(&self) -> &Address {
        &self.me_addr
    }

    fn channel_by_id(&self, _channel_id: &ChannelId) -> Result<Option<ChannelEntry>, Self::Error> {
        Ok(None)
    }

    fn stream_channels<'a>(
        &'a self,
        _selector: ChannelSelector,
    ) -> Result<BoxStream<'a, ChannelEntry>, Self::Error> {
        Ok(Box::pin(stream::empty()))
    }
}

// --- ChainWriteChannelOperations ---

#[async_trait]
impl ChainWriteChannelOperations for StubChain {
    type Error = StubError;

    async fn open_channel<'a>(
        &'a self,
        _dst: &'a Address,
        _amount: HoprBalance,
    ) -> Result<futures::future::BoxFuture<'a, Result<ChainReceipt, Self::Error>>, Self::Error>
    {
        Err(StubError("stub cannot open channels".into()))
    }

    async fn fund_channel<'a>(
        &'a self,
        _channel_id: &'a ChannelId,
        _amount: HoprBalance,
    ) -> Result<futures::future::BoxFuture<'a, Result<ChainReceipt, Self::Error>>, Self::Error>
    {
        Err(StubError("stub cannot fund channels".into()))
    }

    async fn close_channel<'a>(
        &'a self,
        _channel_id: &'a ChannelId,
    ) -> Result<futures::future::BoxFuture<'a, Result<ChainReceipt, Self::Error>>, Self::Error>
    {
        Err(StubError("stub cannot close channels".into()))
    }
}

// --- ChainReadAccountOperations ---

#[async_trait]
impl ChainReadAccountOperations for StubChain {
    type Error = StubError;

    fn stream_accounts<'a>(
        &'a self,
        _selector: AccountSelector,
    ) -> Result<BoxStream<'a, AccountEntry>, Self::Error> {
        Ok(Box::pin(stream::empty()))
    }

    async fn count_accounts(&self, _selector: AccountSelector) -> Result<usize, Self::Error> {
        Ok(0)
    }

    async fn await_key_binding(
        &self,
        _offchain_key: &OffchainPublicKey,
        _timeout: Duration,
    ) -> Result<AccountEntry, Self::Error> {
        Err(StubError("not implemented".into()))
    }
}

// --- ChainWriteAccountOperations ---

#[async_trait]
impl ChainWriteAccountOperations for StubChain {
    type Error = StubError;

    async fn announce(
        &self,
        _multiaddrs: &[Multiaddr],
        _key: &OffchainKeypair,
    ) -> Result<
        futures::future::BoxFuture<'life0, Result<ChainReceipt, Self::Error>>,
        AnnouncementError<Self::Error>,
    > {
        Err(AnnouncementError::ProcessingError(StubError(
            "stub cannot announce".into(),
        )))
    }

    async fn withdraw<C: Currency + Send>(
        &self,
        _balance: Balance<C>,
        _recipient: &Address,
    ) -> Result<futures::future::BoxFuture<'life0, Result<ChainReceipt, Self::Error>>, Self::Error>
    {
        Err(StubError("stub cannot withdraw".into()))
    }

    async fn withdraw_from_signer<C: Currency + Send>(
        &self,
        _key: &ChainKeypair,
        _balance: Balance<C>,
        _recipient: &Address,
    ) -> Result<futures::future::BoxFuture<'life0, Result<ChainReceipt, Self::Error>>, Self::Error>
    {
        Err(StubError("stub cannot withdraw from signer".into()))
    }

    async fn register_safe(
        &self,
        _safe_address: &Address,
    ) -> Result<
        futures::future::BoxFuture<'life0, Result<ChainReceipt, Self::Error>>,
        SafeRegistrationError<Self::Error>,
    > {
        Err(SafeRegistrationError::ProcessingError(StubError(
            "stub cannot register safe".into(),
        )))
    }
}

// --- ChainReadSafeOperations ---

#[async_trait]
impl ChainReadSafeOperations for StubChain {
    type Error = StubError;

    async fn safe_allowance<C: Currency, A: Into<Address> + Send>(
        &self,
        _safe_address: A,
    ) -> Result<Balance<C>, Self::Error> {
        Ok(Balance::zero())
    }

    async fn safe_info(
        &self,
        _selector: SafeSelector,
    ) -> Result<Option<DeployedSafe>, Self::Error> {
        Ok(None)
    }

    async fn await_safe_deployment(
        &self,
        _selector: SafeSelector,
        _timeout: Duration,
    ) -> Result<DeployedSafe, Self::Error> {
        Err(StubError("stub cannot await safe deployment".into()))
    }

    async fn predict_module_address(
        &self,
        _nonce: u64,
        _owner: &Address,
        _safe_address: &Address,
    ) -> Result<Address, Self::Error> {
        Err(StubError("stub cannot predict module address".into()))
    }
}

// --- ChainWriteSafeOperations ---

#[async_trait]
impl ChainWriteSafeOperations for StubChain {
    type Error = StubError;

    async fn deploy_safe<'a>(
        &'a self,
        _balance: HoprBalance,
    ) -> Result<futures::future::BoxFuture<'a, Result<ChainReceipt, Self::Error>>, Self::Error>
    {
        Err(StubError("stub cannot deploy safe".into()))
    }
}

// --- ChainEvents ---

impl ChainEvents for StubChain {
    type Error = StubError;

    fn subscribe_with_state_sync<I: IntoIterator<Item = StateSyncOptions>>(
        &self,
        _options: I,
    ) -> Result<impl futures::Stream<Item = ChainEvent> + Send + 'static, Self::Error> {
        Ok(stream::empty())
    }
}

// --- ChainKeyOperations ---

#[derive(Debug, Clone)]
pub struct StubKeyMapper;

impl KeyIdMapping<KeyIdent, OffchainPublicKey> for StubKeyMapper {
    fn map_key_to_id(&self, _key: &OffchainPublicKey) -> Option<KeyIdent> {
        None
    }

    fn map_id_to_public(&self, _id: &KeyIdent) -> Option<OffchainPublicKey> {
        None
    }
}

impl ChainKeyOperations for StubChain {
    type Error = StubError;
    type Mapper = StubKeyMapper;

    fn chain_key_to_packet_key(
        &self,
        chain: &Address,
    ) -> Result<Option<OffchainPublicKey>, Self::Error> {
        Ok(self.keys.get_by_left(chain).copied())
    }

    fn packet_key_to_chain_key(
        &self,
        packet: &OffchainPublicKey,
    ) -> Result<Option<Address>, Self::Error> {
        Ok(self.keys.get_by_right(packet).copied())
    }

    fn key_id_mapper_ref(&self) -> &Self::Mapper {
        &self.mapper
    }
}

// --- ChainValues ---

#[async_trait]
impl ChainValues for StubChain {
    type Error = StubError;

    async fn balance<C: Currency, A: Into<Address> + Send>(
        &self,
        _address: A,
    ) -> Result<Balance<C>, Self::Error> {
        Ok(Balance::zero())
    }

    async fn domain_separators(&self) -> Result<DomainSeparators, Self::Error> {
        Ok(DomainSeparators {
            ledger: hopr_lib::api::types::crypto::types::Hash::default(),
            safe_registry: hopr_lib::api::types::crypto::types::Hash::default(),
            channel: hopr_lib::api::types::crypto::types::Hash::default(),
        })
    }

    async fn minimum_incoming_ticket_win_prob(&self) -> Result<WinningProbability, Self::Error> {
        Ok(WinningProbability::ALWAYS)
    }

    async fn minimum_ticket_price(&self) -> Result<HoprBalance, Self::Error> {
        Ok(HoprBalance::zero())
    }

    async fn key_binding_fee(&self) -> Result<HoprBalance, Self::Error> {
        Ok(HoprBalance::zero())
    }

    async fn channel_closure_notice_period(&self) -> Result<Duration, Self::Error> {
        Ok(Duration::from_secs(60))
    }

    async fn chain_info(&self) -> Result<ChainInfo, Self::Error> {
        Ok(ChainInfo {
            chain_id: 100,
            hopr_network_name: "stub-test".into(),
            contract_addresses: Default::default(),
        })
    }

    async fn redemption_stats<A: Into<Address> + Send>(
        &self,
        _: A,
    ) -> Result<RedemptionStats, Self::Error> {
        Ok(RedemptionStats {
            redeemed_count: 0,
            redeemed_value: HoprBalance::zero(),
        })
    }

    async fn typical_resolution_time(&self) -> Result<Duration, Self::Error> {
        Ok(Duration::from_secs(30))
    }
}

// --- ChainReadTicketOperations ---

impl ChainReadTicketOperations for StubChain {
    type Error = StubError;

    fn outgoing_ticket_values(
        &self,
        configured_wp: Option<WinningProbability>,
        configured_price: Option<HoprBalance>,
    ) -> Result<(WinningProbability, HoprBalance), Self::Error> {
        Ok((
            configured_wp.unwrap_or(WinningProbability::ALWAYS),
            configured_price.unwrap_or(1.into()),
        ))
    }

    fn incoming_ticket_values(&self) -> Result<(WinningProbability, HoprBalance), Self::Error> {
        Ok((WinningProbability::ALWAYS, 1.into()))
    }
}

// --- ChainWriteTicketOperations ---

#[async_trait]
impl ChainWriteTicketOperations for StubChain {
    type Error = StubError;

    async fn redeem_ticket<'a>(
        &'a self,
        ticket: RedeemableTicket,
    ) -> Result<
        futures::future::BoxFuture<
            'a,
            Result<(VerifiedTicket, ChainReceipt), TicketRedeemError<Self::Error>>,
        >,
        TicketRedeemError<Self::Error>,
    > {
        Err(TicketRedeemError::ProcessingError(
            ticket.ticket,
            StubError("stubs do not redeem tickets".into()),
        ))
    }
}

impl hopr_lib::api::node::ComponentStatusReporter for StubChain {
    fn component_status(&self) -> ComponentStatus {
        ComponentStatus::Ready
    }
}

// ===========================================================================
// MockChainNode — HasChainApi composite for account/channel endpoint tests
// ===========================================================================

/// Composite struct implementing `HasChainApi` by composing a [`StubChain`]
/// with a [`NodeOnchainIdentity`].
///
/// Use [`MockChainNode::random()`] to construct an instance with fresh keypairs.
pub struct MockChainNode {
    pub identity: NodeOnchainIdentity,
    pub chain: StubChain,
    pub graph: StubGraph,
}

impl MockChainNode {
    /// Creates a new `MockChainNode` with random keypairs.
    pub fn random() -> Self {
        let offchain = OffchainKeypair::random();
        let chain_kp = ChainKeypair::random();
        let node_address = chain_kp.public().to_address();

        let safe_chain = ChainKeypair::random();
        let module_chain = ChainKeypair::random();

        Self {
            identity: NodeOnchainIdentity {
                node_address,
                safe_address: safe_chain.public().to_address(),
                module_address: module_chain.public().to_address(),
            },
            chain: StubChain::new(&offchain, &chain_kp),
            graph: StubGraph::new(*offchain.public()),
        }
    }
}

impl hopr_lib::api::node::HasGraphView for MockChainNode {
    type Graph = StubGraph;

    fn graph(&self) -> &StubGraph {
        &self.graph
    }

    fn status(&self) -> ComponentStatus {
        ComponentStatus::Ready
    }
}

impl HoprNodeOperations for MockChainNode {
    fn status(&self) -> HoprState {
        HoprState::Running
    }
}

impl HasChainApi for MockChainNode {
    type ChainApi = StubChain;
    type ChainError = HoprLibError;

    fn identity(&self) -> &NodeOnchainIdentity {
        &self.identity
    }

    fn chain_api(&self) -> &StubChain {
        &self.chain
    }

    fn status(&self) -> ComponentStatus {
        ComponentStatus::Ready
    }

    fn wait_for_on_chain_event<F>(
        &self,
        _predicate: F,
        _context: String,
        _timeout: Duration,
    ) -> EventWaitResult<<Self::ChainApi as chain::HoprChainApi>::ChainError, Self::ChainError>
    where
        F: Fn(&ChainEvent) -> bool + Send + Sync + 'static,
    {
        Err(StubError("not needed for REST API tests".into()))
    }
}

// ===========================================================================
// StubTicketManager — minimal TicketManagement for test doubles
// ===========================================================================

/// Stub ticket manager returning zero stats and empty streams.
#[derive(Debug, Clone)]
pub struct StubTicketManager;

impl TicketManagement for StubTicketManager {
    type Error = StubError;

    fn redeem_stream<C: ChainWriteTicketOperations + Send + Sync + 'static>(
        &self,
        _client: C,
        _channel_id: hopr_lib::api::chain::ChannelId,
        _min_amount: Option<HoprBalance>,
    ) -> Result<
        impl futures::Stream<Item = Result<RedemptionResult, Self::Error>> + Send,
        Self::Error,
    > {
        Ok(stream::empty())
    }

    fn neglect_tickets(
        &self,
        _channel_id: &hopr_lib::api::chain::ChannelId,
        _max_ticket_index: Option<u64>,
    ) -> Result<Vec<hopr_lib::api::tickets::VerifiedTicket>, Self::Error> {
        Ok(vec![])
    }

    fn ticket_stats(
        &self,
        _channel_id: Option<&hopr_lib::api::chain::ChannelId>,
    ) -> Result<ChannelStats, Self::Error> {
        Ok(ChannelStats::default())
    }

    fn insert_incoming_ticket(
        &self,
        _ticket: hopr_lib::api::types::internal::prelude::RedeemableTicket,
    ) -> Result<Vec<hopr_lib::api::tickets::VerifiedTicket>, Self::Error> {
        Ok(vec![])
    }
}

impl HasTicketManagement for MockChainNode {
    type TicketManager = StubTicketManager;

    fn ticket_management(&self) -> &StubTicketManager {
        &StubTicketManager
    }

    fn subscribe_ticket_events(&self) -> impl futures::Stream<Item = TicketEvent> + Send + 'static {
        stream::empty()
    }

    fn status(&self) -> ComponentStatus {
        ComponentStatus::Ready
    }
}

// ===========================================================================
// StubGraph — a controllable network graph for the `network` endpoints
// ===========================================================================

/// One observed edge, with every measurement independently settable.
///
/// The point of the fixture is that each observation can be *absent* as well as
/// present: `None` means never measured, which the endpoints must render
/// differently from a measured zero.
#[derive(Clone, Default)]
pub struct StubEdge {
    pub last_update: Duration,
    pub score: Option<f64>,
    pub immediate: Option<StubMeasurement>,
    pub intermediate: Option<StubMeasurement>,
}

/// One QoS stream on a [`StubEdge`].
#[derive(Clone, Default)]
pub struct StubMeasurement {
    pub connected: Option<bool>,
    pub probe_rate: Option<f64>,
    pub latency: Option<Duration>,
    pub ack_rate: Option<f64>,
    pub balance: Option<hopr_lib::api::graph::traits::Balance>,
}

/// A graph holding a fixed set of edges out of [`identity`](StubGraph::identity).
///
/// Deliberately not `Default`: the identity has to come from somewhere, and a defaulted one would
/// have to be invented — leaving every defaulted graph in the process sharing a single made-up key
/// belonging to no node.
#[derive(Clone)]
pub struct StubGraph {
    me: OffchainPublicKey,
    edges: Vec<(OffchainPublicKey, OffchainPublicKey, StubEdge)>,
}

impl StubGraph {
    /// A graph whose self-identity is `me` and which holds no edges.
    pub fn new(me: OffchainPublicKey) -> Self {
        Self {
            me,
            edges: Vec::new(),
        }
    }

    /// Adds a directed edge.
    pub fn with_edge(
        mut self,
        src: OffchainPublicKey,
        dst: OffchainPublicKey,
        edge: StubEdge,
    ) -> Self {
        self.edges.push((src, dst, edge));
        self
    }
}

impl hopr_lib::api::graph::NetworkGraphView for StubGraph {
    type NodeId = OffchainPublicKey;
    type Observed = StubEdge;

    fn ticket_face_value(&self) -> Option<hopr_lib::api::graph::traits::Balance> {
        None
    }

    fn path_slot(&self, _key: &Self::NodeId) -> Option<u64> {
        None
    }

    fn node_count(&self) -> usize {
        self.edges.len()
    }

    fn contains_node(&self, key: &Self::NodeId) -> bool {
        self.edges.iter().any(|(s, d, _)| s == key || d == key)
    }

    fn nodes(&self) -> BoxStream<'static, Self::NodeId> {
        let all: Vec<_> = self.edges.iter().flat_map(|(s, d, _)| [*s, *d]).collect();
        Box::pin(stream::iter(all))
    }

    fn edge(&self, src: &Self::NodeId, dest: &Self::NodeId) -> Option<Self::Observed> {
        self.edges
            .iter()
            .find(|(s, d, _)| s == src && d == dest)
            .map(|(_, _, e)| e.clone())
    }

    fn identity(&self) -> &Self::NodeId {
        &self.me
    }
}

impl hopr_lib::api::graph::NetworkGraphConnectivity for StubGraph {
    type NodeId = OffchainPublicKey;
    type Observed = StubEdge;

    fn connected_edges(&self) -> Vec<(Self::NodeId, Self::NodeId, Self::Observed)> {
        self.edges.clone()
    }

    fn reachable_edges(&self) -> Vec<(Self::NodeId, Self::NodeId, Self::Observed)> {
        self.edges.clone()
    }
}

impl hopr_lib::api::graph::NetworkGraphTraverse for StubGraph {
    type NodeId = OffchainPublicKey;
    type Observed = StubEdge;

    fn simple_paths<V: hopr_lib::api::graph::ValueFn<Weight = Self::Observed>>(
        &self,
        _source: &Self::NodeId,
        _destination: &Self::NodeId,
        _length: usize,
        _take_count: Option<usize>,
        _value_fn: V,
    ) -> Vec<(Vec<Self::NodeId>, [u64; 5], V::Value)> {
        Vec::new()
    }

    fn simple_paths_from<V: hopr_lib::api::graph::ValueFn<Weight = Self::Observed>>(
        &self,
        _source: &Self::NodeId,
        _length: usize,
        _take_count: Option<usize>,
        _value_fn: V,
    ) -> Vec<(Vec<Self::NodeId>, [u64; 5], V::Value)> {
        Vec::new()
    }

    fn simple_loopback_to_self(
        &self,
        _length: usize,
        _take_count: Option<usize>,
    ) -> Vec<(Vec<Self::NodeId>, [u64; 5])> {
        Vec::new()
    }
}

impl hopr_lib::api::graph::EdgeObservableRead for StubEdge {
    type ImmediateMeasurement = StubMeasurement;
    type IntermediateMeasurement = StubMeasurement;

    fn last_update(&self) -> Duration {
        self.last_update
    }

    fn immediate_qos(&self) -> Option<&Self::ImmediateMeasurement> {
        self.immediate.as_ref()
    }

    fn intermediate_qos(&self) -> Option<&Self::IntermediateMeasurement> {
        self.intermediate.as_ref()
    }

    fn score(&self) -> Option<f64> {
        self.score
    }
}

impl hopr_lib::api::graph::traits::EdgeObservableWrite for StubEdge {
    fn record(&mut self, _measurement: hopr_lib::api::graph::traits::EdgeWeightType) {}
}

impl hopr_lib::api::graph::EdgeLinkObservable for StubMeasurement {
    fn record(&mut self, _: hopr_lib::api::graph::traits::EdgeTransportMeasurement) {}

    fn average_latency(&self) -> Option<Duration> {
        self.latency
    }

    fn average_probe_rate(&self) -> Option<f64> {
        self.probe_rate
    }

    fn score(&self) -> Option<f64> {
        self.probe_rate
    }
}

impl hopr_lib::api::graph::traits::EdgeNetworkObservableRead for StubMeasurement {
    fn is_connected(&self) -> Option<bool> {
        self.connected
    }
}

impl hopr_lib::api::graph::EdgeImmediateProtocolObservable for StubMeasurement {
    fn ack_rate(&self) -> Option<f64> {
        self.ack_rate
    }
}

impl hopr_lib::api::graph::traits::EdgeProtocolObservable for StubMeasurement {
    fn balance(&self) -> Option<hopr_lib::api::graph::traits::Balance> {
        self.balance
    }
}
