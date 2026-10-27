#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    ///Contains the node's native addresses.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountAddressesResponse {
        pub native: ::std::string::String,
    }
    ///Contains all node's and safe's related balances.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AccountBalancesResponse {
        pub hopr: ::std::string::String,
        pub native: ::std::string::String,
        #[serde(rename = "safeHopr")]
        pub safe_hopr: ::std::string::String,
        #[serde(rename = "safeHoprAllowance")]
        pub safe_hopr_allowance: ::std::string::String,
        #[serde(rename = "safeNative")]
        pub safe_native: ::std::string::String,
    }
    ///A peer that has been announced.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AnnouncedPeerResponse {
        pub address: ::std::string::String,
        pub multiaddrs: ::std::vec::Vec<::std::string::String>,
        pub origin: AnnouncementOriginResponse,
    }
    ///How a peer announcement was discovered.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum AnnouncementOriginResponse {
        #[serde(rename = "chain")]
        Chain,
        #[serde(rename = "dht")]
        Dht,
    }
    impl ::std::fmt::Display for AnnouncementOriginResponse {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Chain => f.write_str("chain"),
                Self::Dht => f.write_str("dht"),
            }
        }
    }
    impl ::std::str::FromStr for AnnouncementOriginResponse {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "chain" => Ok(Self::Chain),
                "dht" => Ok(Self::Dht),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for AnnouncementOriginResponse {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for AnnouncementOriginResponse {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Standardized error response for the API
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiError {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        pub status: ::std::string::String,
    }
    ///Direction of a channel relative to this node.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum ChannelDirection {
        #[serde(rename = "outgoing")]
        Outgoing,
        #[serde(rename = "incoming")]
        Incoming,
    }
    impl ::std::fmt::Display for ChannelDirection {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Outgoing => f.write_str("outgoing"),
                Self::Incoming => f.write_str("incoming"),
            }
        }
    }
    impl ::std::str::FromStr for ChannelDirection {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "outgoing" => Ok(Self::Outgoing),
                "incoming" => Ok(Self::Incoming),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ChannelDirection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ChannelDirection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Query parameters selecting which channel (incoming or outgoing) to address.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct ChannelDirectionQuery {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub direction: ::std::option::Option<ChannelDirection>,
    }
    ///General information about a channel state.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChannelInfoResponse {
        pub balance: ::std::string::String,
        #[serde(rename = "channelEpoch")]
        pub channel_epoch: i32,
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
        #[serde(rename = "closureTime")]
        pub closure_time: i64,
        pub destination: ::std::string::String,
        pub source: ::std::string::String,
        pub status: ::std::string::String,
        #[serde(rename = "ticketIndex")]
        pub ticket_index: i64,
    }
    ///Represents a ticket in a channel.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ChannelTicket {
        pub amount: ::std::string::String,
        #[serde(rename = "channelEpoch")]
        pub channel_epoch: i32,
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
        pub index: i64,
        pub signature: ::std::string::String,
        #[serde(rename = "winProb")]
        pub win_prob: ::std::string::String,
    }
    ///Parameters for enumerating channels.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct ChannelsQueryRequest {
        ///Should all channels (not only the ones concerning this node) be enumerated?
        #[serde(rename = "fullTopology", default)]
        pub full_topology: bool,
        ///Should be the closed channels included?
        #[serde(rename = "includingClosed", default)]
        pub including_closed: bool,
    }
    ///Status of the channel after a close operation.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CloseChannelResponse {
        ///Receipt for the channel close transaction.
        pub receipt: ::std::string::String,
    }
    ///`ComponentStatusInfo`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ComponentStatusInfo {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub detail: ::std::option::Option<::std::string::String>,
        pub status: ::std::string::String,
    }
    ///`ComponentStatusesResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ComponentStatusesResponse {
        pub chain: ComponentStatusInfo,
        pub network: ComponentStatusInfo,
        pub transport: ComponentStatusInfo,
    }
    ///Immediate observation data for a connected peer.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ConnectedPeerResponse {
        pub address: ::std::string::String,
        ///Average latency in milliseconds, if available.
        #[serde(
            rename = "averageLatency",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_latency: ::std::option::Option<u64>,
        ///Epoch milliseconds of the last observation update.
        #[serde(rename = "lastUpdate")]
        pub last_update: u64,
        /**Average probe rate, if any probe has been measured.

Nullable for the same reason as `average_latency` below, which this field now matches:
hopr-api 2.0 reports an unmeasured observation as absent rather than as zero. Serialising
the absent case as `0.0` would be indistinguishable from a peer measured at zero — a peer
nothing has been learned about yet would read as one known to be failing.*/
        #[serde(
            rename = "probeRate",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub probe_rate: ::std::option::Option<f64>,
        ///Edge score, if one has been computed. Nullable as `probe_rate` above.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub score: ::std::option::Option<f64>,
    }
    ///Specifies the amount of HOPR tokens to fund a channel with.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FundBodyRequest {
        ///Amount of HOPR tokens to fund the channel with.
        pub amount: ::std::string::String,
    }
    ///Response body for funding a channel.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FundChannelResponse {
        pub hash: ::std::string::String,
    }
    ///IP transport protocol
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum IpProtocol {
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "udp")]
        Udp,
    }
    impl ::std::fmt::Display for IpProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tcp => f.write_str("tcp"),
                Self::Udp => f.write_str("udp"),
            }
        }
    }
    impl ::std::str::FromStr for IpProtocol {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tcp" => Ok(Self::Tcp),
                "udp" => Ok(Self::Udp),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for IpProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for IpProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A multiaddress paired with its discovery origin.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MultiaddressSource {
        pub multiaddress: ::std::string::String,
        pub origin: AnnouncementOriginResponse,
    }
    ///Channel information as seen by the node.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NodeChannel {
        pub balance: ::std::string::String,
        pub id: ::std::string::String,
        #[serde(rename = "peerAddress")]
        pub peer_address: ::std::string::String,
        pub status: ::std::string::String,
    }
    ///Listing of channels.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NodeChannelsResponse {
        ///Complete channel topology as seen by this node.
        pub all: ::std::vec::Vec<ChannelInfoResponse>,
        ///Channels incoming to this node.
        pub incoming: ::std::vec::Vec<NodeChannel>,
        ///Channels outgoing from this node.
        pub outgoing: ::std::vec::Vec<NodeChannel>,
    }
    /**Information about the current node. Covers network, addresses, eligibility, connectivity status, contracts addresses
and indexer state.*/
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NodeInfoResponse {
        #[serde(rename = "announcedAddress")]
        pub announced_address: ::std::vec::Vec<::std::string::String>,
        ///Chain/blokli connector status.
        #[serde(rename = "chainStatus")]
        pub chain_status: ::std::string::String,
        ///Channel closure period in seconds
        #[serde(rename = "channelClosurePeriod")]
        pub channel_closure_period: i64,
        #[serde(rename = "connectivityStatus")]
        pub connectivity_status: ::std::string::String,
        #[serde(rename = "hoprNetworkName")]
        pub hopr_network_name: ::std::string::String,
        #[serde(rename = "hoprNodeSafe")]
        pub hopr_node_safe: ::std::string::String,
        #[serde(rename = "listeningAddress")]
        pub listening_address: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "providerUrl")]
        pub provider_url: ::std::string::String,
    }
    ///Comprehensive information about a peer: multiaddresses, QoS observations, and channel state.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NodePeerInfoResponse {
        ///Announced multiaddresses grouped by discovery origin.
        #[serde(rename = "announcedSources")]
        pub announced_sources: ::std::vec::Vec<MultiaddressSource>,
        #[serde(
            rename = "incomingChannel",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub incoming_channel: ::std::option::Option<PeerChannelInfo>,
        pub observed: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "outgoingChannel",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub outgoing_channel: ::std::option::Option<PeerChannelInfo>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub qos: ::std::option::Option<PeerQosInfo>,
    }
    ///`NodeStatusResponse`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NodeStatusResponse {
        pub components: ComponentStatusesResponse,
        ///Current node lifecycle state.
        #[serde(rename = "nodeState")]
        pub node_state: ::std::string::String,
        ///Aggregated status across all components.
        pub overall: ::std::string::String,
    }
    ///Received tickets statistics.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NodeTicketStatisticsResponse {
        #[serde(rename = "neglectedValue")]
        pub neglected_value: ::std::string::String,
        #[serde(rename = "rejectedValue")]
        pub rejected_value: ::std::string::String,
        #[serde(rename = "unredeemedValue")]
        pub unredeemed_value: ::std::string::String,
        #[serde(rename = "winningCount")]
        pub winning_count: i64,
    }
    ///Running node version.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NodeVersionResponse {
        pub version: ::std::string::String,
    }
    ///Request body for opening a channel.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OpenChannelBodyRequest {
        ///Initial amount of stake in HOPR tokens.
        pub amount: ::std::string::String,
        ///On-chain address of the counterparty.
        pub destination: ::std::string::String,
    }
    ///Response body for opening a channel.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct OpenChannelResponse {
        ///ID of the new channel.
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
        ///Receipt of the channel open transaction.
        #[serde(rename = "transactionReceipt")]
        pub transaction_receipt: ::std::string::String,
    }
    ///Channel information for a specific peer.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PeerChannelInfo {
        pub balance: ::std::string::String,
        pub id: ::std::string::String,
        pub status: ::std::string::String,
    }
    ///QoS observation data for a peer.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PeerQosInfo {
        ///Average latency in milliseconds, if available.
        #[serde(
            rename = "averageLatency",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_latency: ::std::option::Option<u64>,
        ///Epoch milliseconds of the last observation update.
        #[serde(rename = "lastUpdate")]
        pub last_update: u64,
        /**Average probe rate, if any probe has been measured.

Nullable for the same reason as `average_latency` below, which this field now matches:
hopr-api 2.0 reports an unmeasured observation as absent rather than as zero, and reporting
the absent case as `0.0` would be indistinguishable from a peer measured at zero.*/
        #[serde(
            rename = "probeRate",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub probe_rate: ::std::option::Option<f64>,
        ///Edge score, if one has been computed. Nullable as `probe_rate` above.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub score: ::std::option::Option<f64>,
    }
    ///Contains the latency and the reported version of a peer that has been pinged.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PingResponse {
        pub latency: i64,
    }
    ///Request body for ticket redemption with optional fields.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct RedeemTicketsRequest {
        /**On-chain address of the counterparty whose incoming channel tickets to redeem.
If omitted, tickets in all channels are redeemed.*/
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub address: ::std::option::Option<::std::string::String>,
    }
    ///`RoutingOptions`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub enum RoutingOptions {
        Hops(u64),
        IntermediatePath(::std::vec::Vec<::std::string::String>),
    }
    impl ::std::convert::From<u64> for RoutingOptions {
        fn from(value: u64) -> Self {
            Self::Hops(value)
        }
    }
    impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
    for RoutingOptions {
        fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
            Self::IntermediatePath(value)
        }
    }
    ///Session capabilities that can be negotiated with the target peer.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum SessionCapability {
        Segmentation,
        Retransmission,
        RetransmissionAckOnly,
        NoDelay,
        NoRateControl,
        #[serde(rename = "UsePIX")]
        UsePix,
    }
    impl ::std::fmt::Display for SessionCapability {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Segmentation => f.write_str("Segmentation"),
                Self::Retransmission => f.write_str("Retransmission"),
                Self::RetransmissionAckOnly => f.write_str("RetransmissionAckOnly"),
                Self::NoDelay => f.write_str("NoDelay"),
                Self::NoRateControl => f.write_str("NoRateControl"),
                Self::UsePix => f.write_str("UsePIX"),
            }
        }
    }
    impl ::std::str::FromStr for SessionCapability {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Segmentation" => Ok(Self::Segmentation),
                "Retransmission" => Ok(Self::Retransmission),
                "RetransmissionAckOnly" => Ok(Self::RetransmissionAckOnly),
                "NoDelay" => Ok(Self::NoDelay),
                "NoRateControl" => Ok(Self::NoRateControl),
                "UsePIX" => Ok(Self::UsePix),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SessionCapability {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SessionCapability {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Request body for creating a new client session.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SessionClientRequest {
        /**Capabilities for the Session protocol.

Defaults to `Segmentation` and `Retransmission` for TCP and nothing for UDP.*/
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub capabilities: ::std::option::Option<::std::vec::Vec<SessionCapability>>,
        ///Address of the Exit node.
        pub destination: ::std::string::String,
        #[serde(
            rename = "flowControl",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub flow_control: ::std::option::Option<SessionFlowControl>,
        #[serde(rename = "forwardPath")]
        pub forward_path: RoutingOptions,
        /**Listen host (`ip:port`) for the Session socket at the Entry node.

Supports also partial specification (only `ip` or only `:port`) with the
respective part replaced by the node's configured default.*/
        #[serde(
            rename = "listenHost",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub listen_host: ::std::option::Option<::std::string::String>,
        /**The maximum number of client sessions that the listener can spawn.

This currently applies only to the TCP sessions, as UDP sessions cannot
handle multiple clients (and spawn therefore always only a single session).

If this value is smaller than the value specified in `session_pool`, it will
be set to that value.

The default value is 5.*/
        #[serde(
            rename = "maxClientSessions",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_client_sessions: ::std::option::Option<u64>,
        /**The maximum throughput at which artificial SURBs might be generated and sent
to the recipient of the Session.

On Sessions that rarely send data but receive a lot (= Exit node has high SURB consumption),
this should roughly match the maximum retrieval throughput.

All syntaxes like "2 MBps", "1.2Mbps", "300 kb/s", "1.23 Mb/s" are supported.*/
        #[serde(
            rename = "maxSurbUpstream",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_surb_upstream: ::std::option::Option<::std::string::String>,
        /**The amount of response data the Session counterparty can deliver back to us,
without us sending any SURBs to them.

In other words, this size is recalculated to a number of SURBs delivered
to the counterparty upfront and then maintained.
The maintenance is dynamic, based on the number of responses we receive.

All syntaxes like "2 MB", "128 kiB", "3MiB" are supported. The value must be
at least the size of 2 Session packet payloads.*/
        #[serde(
            rename = "responseBuffer",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub response_buffer: ::std::option::Option<::std::string::String>,
        #[serde(rename = "returnPath")]
        pub return_path: RoutingOptions,
        /**How many Sessions to pool for clients.

If no sessions are pooled, they will be opened ad-hoc when a client connects.
It has no effect on UDP sessions in the current implementation.

Currently, the maximum value is 5.*/
        #[serde(
            rename = "sessionPool",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub session_pool: ::std::option::Option<u64>,
        pub target: SessionTargetSpec,
    }
    ///Response body for creating a new client session.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct SessionClientResponse {
        /**Lists Session IDs of all active clients.

Can contain multiple entries on TCP sessions, but currently
always only a single entry on UDP sessions.*/
        #[serde(rename = "activeClients")]
        pub active_clients: ::std::vec::Vec<::std::string::String>,
        ///Destination node (exit node) of the Session.
        pub destination: ::std::string::String,
        #[serde(rename = "forwardPath")]
        pub forward_path: RoutingOptions,
        ///MTU used by the underlying HOPR transport.
        #[serde(rename = "hoprMtu")]
        pub hopr_mtu: u64,
        ///Listening IP address of the Session's socket.
        pub ip: ::std::string::String,
        /**The maximum number of client sessions that the listener can spawn.

This currently applies only to the TCP sessions, as UDP sessions cannot
have multiple clients (defaults to 1 for UDP).*/
        #[serde(rename = "maxClientSessions")]
        pub max_client_sessions: u64,
        /**The maximum throughput at which artificial SURBs might be generated and sent
to the recipient of the Session.*/
        #[serde(
            rename = "maxSurbUpstream",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_surb_upstream: ::std::option::Option<::std::string::String>,
        ///Listening port of the Session's socket.
        pub port: i32,
        pub protocol: IpProtocol,
        /**The amount of response data the Session counterparty can deliver back to us, without us
sending any SURBs to them.*/
        #[serde(
            rename = "responseBuffer",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub response_buffer: ::std::option::Option<::std::string::String>,
        #[serde(rename = "returnPath")]
        pub return_path: RoutingOptions,
        ///How many Sessions to pool for clients.
        #[serde(
            rename = "sessionPool",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub session_pool: ::std::option::Option<u64>,
        /**Size of a Single Use Reply Block used by the protocol.

This is useful for SURB balancing calculations.*/
        #[serde(rename = "surbLen")]
        pub surb_len: u64,
        ///Target of the Session.
        pub target: ::std::string::String,
    }
    ///`SessionConfig`
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
    pub struct SessionConfig {
        /**The maximum throughput at which artificial SURBs might be generated and sent
to the recipient of the Session.

On Sessions that rarely send data but receive a lot (= Exit node has high SURB consumption),
this should roughly match the maximum retrieval throughput.

All syntaxes like "2 MBps", "1.2Mbps", "300 kb/s", "1.23 Mb/s" are supported.*/
        #[serde(
            rename = "maxSurbUpstream",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_surb_upstream: ::std::option::Option<::std::string::String>,
        /**The amount of response data the Session counterparty can deliver back to us,
without us sending any SURBs to them.

In other words, this size is recalculated to a number of SURBs delivered
to the counterparty upfront and then maintained.
The maintenance is dynamic, based on the number of responses we receive.

All syntaxes like "2 MB", "128 kiB", "3MiB" are supported. The value must be
at least the size of 2 Session packet payloads.*/
        #[serde(
            rename = "responseBuffer",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub response_buffer: ::std::option::Option<::std::string::String>,
    }
    ///Flow-control profile for node-initiated sessions. See the `api.session_flow_control` config option.
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum SessionFlowControl {
        #[serde(rename = "off")]
        Off,
        #[serde(rename = "clean")]
        Clean,
        #[serde(rename = "robust")]
        Robust,
    }
    impl ::std::fmt::Display for SessionFlowControl {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Off => f.write_str("off"),
                Self::Clean => f.write_str("clean"),
                Self::Robust => f.write_str("robust"),
            }
        }
    }
    impl ::std::str::FromStr for SessionFlowControl {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "off" => Ok(Self::Off),
                "clean" => Ok(Self::Clean),
                "robust" => Ok(Self::Robust),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SessionFlowControl {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SessionFlowControl {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Session target specification.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub enum SessionTargetSpec {
        Plain(::std::string::String),
        Sealed(::std::vec::Vec<i32>),
        Service(i32),
    }
    impl ::std::convert::From<::std::vec::Vec<i32>> for SessionTargetSpec {
        fn from(value: ::std::vec::Vec<i32>) -> Self {
            Self::Sealed(value)
        }
    }
    impl ::std::convert::From<i32> for SessionTargetSpec {
        fn from(value: i32) -> Self {
            Self::Service(value)
        }
    }
    ///Contains the ticket price in HOPR tokens.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TicketPriceResponse {
        ///Price of the ticket in HOPR tokens.
        pub price: ::std::string::String,
    }
    ///Contains the winning probability of a ticket.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct TicketProbabilityResponse {
        ///Winning probability of a ticket.
        pub probability: f64,
    }
    ///Request body for the withdrawal endpoint.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct WithdrawBodyRequest {
        pub address: ::std::string::String,
        pub amount: ::std::string::String,
    }
    ///Response body for the withdrawal endpoint.
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct WithdrawResponse {
        pub receipt: ::std::string::String,
    }
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for hoprd-api

API enabling developers to interact with a hoprd node programatically through HTTP REST API.

Version: 4.14.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "4.14.0"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    /**Get node's native addresses

Sends a `GET` request to `/api/v4/account/addresses`

*/
    pub async fn addresses<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::AccountAddressesResponse>, Error<()>> {
        let url = format!("{}/api/v4/account/addresses", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "addresses",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get node's and associated Safe's HOPR and native balances as the allowance for HOPR
tokens to be drawn by HoprChannels from Safe

HOPR tokens from the Safe balance are used to fund the payment channels between this
node and other nodes on the network.
NATIVE balance of the Node is used to pay for the gas fees for the blockchain.

Sends a `GET` request to `/api/v4/account/balances`

*/
    pub async fn balances<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::AccountBalancesResponse>, Error<()>> {
        let url = format!("{}/api/v4/account/balances", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "balances",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Withdraw funds from this node to the ethereum wallet address

Withdraw funds from this node to the ethereum wallet address

Sends a `POST` request to `/api/v4/account/withdraw`

Arguments:
- `body`: Request body for the withdraw endpoint
*/
    pub async fn withdraw<'a>(
        &'a self,
        body: &'a types::WithdrawBodyRequest,
    ) -> Result<ResponseValue<types::WithdrawResponse>, Error<()>> {
        let url = format!("{}/api/v4/account/withdraw", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "withdraw",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Lists channels opened to/from this node. Alternatively, it can print all
the channels in the network as this node sees them

List channels opened to/from this node. Alternatively, it can print all the channels in the network as this node sees them.

Sends a `GET` request to `/api/v4/channels`

Arguments:
- `full_topology`: Should all channels (not only the ones concerning this node) be enumerated?
- `including_closed`: Should be the closed channels included?
*/
    pub async fn list_channels<'a>(
        &'a self,
        full_topology: Option<bool>,
        including_closed: Option<bool>,
    ) -> Result<ResponseValue<types::NodeChannelsResponse>, Error<()>> {
        let url = format!("{}/api/v4/channels", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("fullTopology", &full_topology))
            .query(
                &progenitor_client::QueryParam::new("includingClosed", &including_closed),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_channels",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Opens a channel to the given on-chain address with the given initial stake of HOPR tokens

Opens a channel to the given on-chain address with the given initial stake of HOPR tokens.

Sends a `POST` request to `/api/v4/channels`

Arguments:
- `body`: Open channel request specification: on-chain address of the counterparty and the initial HOPR token stake.
*/
    pub async fn open_channel<'a>(
        &'a self,
        body: &'a types::OpenChannelBodyRequest,
    ) -> Result<ResponseValue<types::OpenChannelResponse>, Error<()>> {
        let url = format!("{}/api/v4/channels", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "open_channel",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns information about the channel with the given counterparty address in the given direction

Returns information about the channel with the given counterparty address. Use the `direction` query parameter to choose between the outgoing (this node → counterparty, default) and incoming (counterparty → this node) channel.

Sends a `GET` request to `/api/v4/channels/{address}`

Arguments:
- `address`: On-chain address of the counterparty.
- `direction`: Direction of the channel relative to this node. Defaults to `outgoing`.
*/
    pub async fn show_channel<'a>(
        &'a self,
        address: &'a str,
        direction: Option<types::ChannelDirection>,
    ) -> Result<ResponseValue<types::ChannelInfoResponse>, Error<()>> {
        let url = format!(
            "{}/api/v4/channels/{}", self.baseurl, encode_path(& address.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("direction", &direction))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "show_channel",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Closes the channel with the given counterparty in the given direction

Closes the channel with the given counterparty. Use the `direction` query parameter to choose between the outgoing (this node → counterparty, default) and incoming (counterparty → this node) channel.

Sends a `DELETE` request to `/api/v4/channels/{address}`

Arguments:
- `address`: On-chain address of the counterparty.
- `direction`: Direction of the channel relative to this node. Defaults to `outgoing`.
*/
    pub async fn close_channel<'a>(
        &'a self,
        address: &'a str,
        direction: Option<types::ChannelDirection>,
    ) -> Result<ResponseValue<types::CloseChannelResponse>, Error<()>> {
        let url = format!(
            "{}/api/v4/channels/{}", self.baseurl, encode_path(& address.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("direction", &direction))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "close_channel",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Funds the outgoing channel to the given counterparty with the given amount of HOPR tokens

Funds the outgoing channel to the given counterparty with the given amount of HOPR tokens.

Sends a `POST` request to `/api/v4/channels/{address}/fund`

Arguments:
- `address`: On-chain address of the counterparty.
- `body`: Specifies the amount of HOPR tokens to fund a channel with.
*/
    pub async fn fund_channel<'a>(
        &'a self,
        address: &'a str,
        body: &'a types::FundBodyRequest,
    ) -> Result<ResponseValue<types::FundChannelResponse>, Error<()>> {
        let url = format!(
            "{}/api/v4/channels/{}/fund", self.baseurl, encode_path(& address
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "fund_channel",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Lists all announced peers

List all announced peers

Sends a `GET` request to `/api/v4/network/announced`

*/
    pub async fn announced<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::AnnouncedPeerResponse>>,
        Error<()>,
    > {
        let url = format!("{}/api/v4/network/announced", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "announced",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Lists peers with immediate observation data from the network graph

List connected peers with immediate observation data from the network graph

Sends a `GET` request to `/api/v4/network/connected`

*/
    pub async fn connected<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::ConnectedPeerResponse>>,
        Error<()>,
    > {
        let url = format!("{}/api/v4/network/connected", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "connected",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns the network graph in DOT (Graphviz) format

Get the network graph in DOT (Graphviz) format

Sends a `GET` request to `/api/v4/network/graph`

Arguments:
- `reachable_only`: When true, only include edges reachable from this node via directed
traversal. Disconnected subgraphs that cannot be routed through are excluded.
*/
    pub async fn graph<'a>(
        &'a self,
        reachable_only: Option<bool>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/api/v4/network/graph", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&progenitor_client::QueryParam::new("reachableOnly", &reachable_only))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "graph",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Gets the current ticket price

Get the current ticket price

Sends a `GET` request to `/api/v4/network/price`

*/
    pub async fn price<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::TicketPriceResponse>, Error<()>> {
        let url = format!("{}/api/v4/network/price", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "price",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Gets the current minimum incoming ticket winning probability defined by the network

Get the current minimum incoming ticket winning probability defined by the network

Sends a `GET` request to `/api/v4/network/probability`

*/
    pub async fn probability<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::TicketProbabilityResponse>, Error<()>> {
        let url = format!("{}/api/v4/network/probability", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "probability",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get the configuration of the running node

Get the configuration of the running node

Sends a `GET` request to `/api/v4/node/configuration`

*/
    pub async fn configuration<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<()>,
    > {
        let url = format!("{}/api/v4/node/configuration", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "configuration",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get information about this HOPR Node

Get information about this HOPR Node

Sends a `GET` request to `/api/v4/node/info`

*/
    pub async fn info<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::NodeInfoResponse>, Error<()>> {
        let url = format!("{}/api/v4/node/info", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get the aggregated status of this HOPR node and its individual components

Get the aggregated status of this HOPR node and its individual components

Sends a `GET` request to `/api/v4/node/status`

*/
    pub async fn status<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::NodeStatusResponse>, Error<()>> {
        let url = format!("{}/api/v4/node/status", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get the release version of the running node

Get the release version of the running node

Sends a `GET` request to `/api/v4/node/version`

*/
    pub async fn version<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::NodeVersionResponse>, Error<()>> {
        let url = format!("{}/api/v4/node/version", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "version",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns comprehensive information about the given peer

Includes announced and observed multiaddresses, QoS observation data from the
network graph, and the state of any channels between this node and the peer.

Sends a `GET` request to `/api/v4/peers/{address}`

Arguments:
- `address`: On-chain address of the requested peer
*/
    pub async fn show_peer_info<'a>(
        &'a self,
        address: &'a str,
    ) -> Result<ResponseValue<types::NodePeerInfoResponse>, Error<()>> {
        let url = format!(
            "{}/api/v4/peers/{}", self.baseurl, encode_path(& address.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "show_peer_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Directly pings the given peer

Directly ping the given peer

Sends a `POST` request to `/api/v4/peers/{address}/ping`

Arguments:
- `address`: On-chain address of the requested peer
*/
    pub async fn ping_peer<'a>(
        &'a self,
        address: &'a str,
    ) -> Result<ResponseValue<types::PingResponse>, Error<()>> {
        let url = format!(
            "{}/api/v4/peers/{}/ping", self.baseurl, encode_path(& address.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ping_peer",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Gets configuration of an existing active session.

Sends a `GET` request to `/api/v4/session/config/{id}`

Arguments:
- `id`: Session ID
*/
    pub async fn session_config<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<ResponseValue<types::SessionConfig>, Error<()>> {
        let url = format!(
            "{}/api/v4/session/config/{}", self.baseurl, encode_path(& id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "session_config",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Updates configuration of an existing active session.

Sends a `POST` request to `/api/v4/session/config/{id}`

Arguments:
- `id`: Session ID
- `body`: Allows updating of several parameters of an existing active session.
*/
    pub async fn adjust_session<'a>(
        &'a self,
        id: &'a str,
        body: &'a types::SessionConfig,
    ) -> Result<ResponseValue<ByteStream>, Error<types::ApiError>> {
        let url = format!(
            "{}/api/v4/session/config/{}", self.baseurl, encode_path(& id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "adjust_session",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Lists existing Session listeners for the given IP protocol

Lists existing Session listeners for the given IP protocol.

Sends a `GET` request to `/api/v4/session/{protocol}`

Arguments:
- `protocol`: IP transport protocol
*/
    pub async fn list_clients<'a>(
        &'a self,
        protocol: &'a str,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::SessionClientResponse>>,
        Error<()>,
    > {
        let url = format!(
            "{}/api/v4/session/{}", self.baseurl, encode_path(& protocol.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_clients",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Creates a new client session returning the given session listening host and port over TCP or UDP.
If no listening port is given in the request, the socket will be bound to a random free
port and returned in the response.
Different capabilities can be configured for the session, such as data segmentation or
retransmission

Creates a new client HOPR session that will start listening on a dedicated port. Once the port is bound, it is possible to use the socket for bidirectional read and write communication.

Sends a `POST` request to `/api/v4/session/{protocol}`

Arguments:
- `protocol`: IP transport protocol
- `body`: Creates a new client HOPR session that will start listening on a dedicated port. Once the port is bound, it is possible to use the socket for bidirectional read and write communication.
*/
    pub async fn create_client<'a>(
        &'a self,
        protocol: &'a str,
        body: &'a types::SessionClientRequest,
    ) -> Result<ResponseValue<types::SessionClientResponse>, Error<()>> {
        let url = format!(
            "{}/api/v4/session/{}", self.baseurl, encode_path(& protocol.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_client",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Closes an existing Session listener.
The listener must've been previously created and bound for the given IP protocol.
Once a listener is closed, no more socket connections can be made to it.
If the passed port number is 0, listeners on all ports of the given listening IP and protocol
will be closed

Closes an existing Session listener.

Sends a `DELETE` request to `/api/v4/session/{protocol}/{ip}/{port}`

Arguments:
- `protocol`: IP transport protocol
- `ip`: Listening IP address of the Session.
- `port`: Session port used for the listener.
*/
    pub async fn close_client<'a>(
        &'a self,
        protocol: types::IpProtocol,
        ip: &'a str,
        port: i32,
    ) -> Result<ResponseValue<ByteStream>, Error<types::ApiError>> {
        let url = format!(
            "{}/api/v4/session/{}/{}/{}", self.baseurl, encode_path(& protocol
            .to_string()), encode_path(& ip.to_string()), encode_path(& port
            .to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "close_client",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Starts redeeming tickets

Starts redeeming tickets. When a counterparty address is specified, only tickets from that counterparty are redeemed.

Sends a `POST` request to `/api/v4/tickets/redeem`

Arguments:
- `body`: Optional counterparty address to scope ticket redemption.
*/
    pub async fn redeem_tickets<'a>(
        &'a self,
        body: &'a types::RedeemTicketsRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<types::ApiError>> {
        let url = format!("{}/api/v4/tickets/redeem", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "redeem_tickets",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Returns current complete statistics on tickets

Returns current complete statistics on tickets. When a counterparty address is given, only the incoming channel from that counterparty is reported.

Sends a `GET` request to `/api/v4/tickets/statistics`

Arguments:
- `address`: On-chain address of the counterparty whose incoming channel to report on.
If omitted, statistics are aggregated across every channel.
*/
    pub async fn show_ticket_statistics<'a>(
        &'a self,
        address: Option<&'a str>,
    ) -> Result<ResponseValue<types::NodeTicketStatisticsResponse>, Error<()>> {
        let url = format!("{}/api/v4/tickets/statistics", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("address", &address))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "show_ticket_statistics",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Check whether the node is eligible in the network

Check whether the node is eligible in the network

Sends a `GET` request to `/eligiblez`

*/
    pub async fn eligiblez<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/eligiblez", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "eligiblez",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            412u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Check whether the node is **healthy**

Check whether the node is healthy

Sends a `GET` request to `/healthyz`

*/
    pub async fn healthyz<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/healthyz", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "healthyz",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            412u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Retrieve Prometheus metrics from the running node

Retrieve Prometheus metrics from the running node

Sends a `GET` request to `/metrics`

*/
    pub async fn metrics<'a>(&'a self) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/metrics", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "metrics",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Check whether the node is **ready** to accept connections

Check whether the node is ready to accept connections

Sends a `GET` request to `/readyz`

*/
    pub async fn readyz<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/readyz", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "readyz",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            412u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Check whether the node is started

Check whether the node is started

Sends a `GET` request to `/startedz`

*/
    pub async fn startedz<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/startedz", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "startedz",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            412u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
