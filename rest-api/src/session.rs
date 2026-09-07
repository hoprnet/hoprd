use std::{fmt::Formatter, hash::Hash, net::IpAddr, str::FromStr, sync::Arc};

use axum::{
    extract::{Json, Path, State},
    http::status::StatusCode,
    response::{IntoResponse, Response},
};
use base64::Engine;
use hopr_lib::api::chain::ChainKeyOperations;
use hopr_lib::api::node::HasChainApi;
use hopr_lib::{
    HopRouting, HoprSessionClientConfig,
    api::types::primitive::{errors::GeneralError, prelude::Address, traits::ToHex},
    errors::HoprLibError,
    exports::transport::{
        HoprPixSpec, PixParams, SESSION_MTU, SURB_SIZE, ServiceId, SessionCapabilities, SessionId,
        SessionTarget, SurbBalancerConfig,
    },
};
#[allow(deprecated)]
use hopr_lib::{HoprSessionClientExplicitPathConfig, api::types::internal::NodeId};
use hopr_utils_session::{
    ListenerId, build_binding_host, create_tcp_client_binding, create_udp_client_binding,
};
use serde::{Deserialize, Serialize};
// Imported for some IDEs to not treat the `json!` macro inside the `schema` macro as an error
#[allow(unused_imports)]
use serde_json::json;
use serde_with::{DisplayFromStr, serde_as};

use crate::{ApiError, ApiErrorStatus, BASE_PATH, InternalState};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
#[schema(
    example = json!({"Plain": "example.com:80"}),
    example = json!({"Sealed": "SGVsbG9Xb3JsZA"}), // base64 for "HelloWorld"
    example = json!({"Service": 0})
)]
/// Session target specification.
pub enum SessionTargetSpec {
    Plain(String),
    Sealed(#[serde_as(as = "serde_with::base64::Base64")] Vec<u8>),
    #[schema(value_type = u32)]
    Service(ServiceId),
}

impl std::fmt::Display for SessionTargetSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionTargetSpec::Plain(t) => write!(f, "{t}"),
            SessionTargetSpec::Sealed(t) => {
                write!(f, "$${}", base64::prelude::BASE64_URL_SAFE.encode(t))
            }
            SessionTargetSpec::Service(t) => write!(f, "#{t}"),
        }
    }
}

impl FromStr for SessionTargetSpec {
    type Err = HoprLibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(stripped) = s.strip_prefix("$$") {
            Self::Sealed(
                base64::prelude::BASE64_URL_SAFE
                    .decode(stripped)
                    .map_err(|e| HoprLibError::GeneralError(e.to_string()))?,
            )
        } else if let Some(stripped) = s.strip_prefix("#") {
            Self::Service(
                stripped
                    .parse()
                    .map_err(|_| HoprLibError::GeneralError("cannot parse service id".into()))?,
            )
        } else {
            Self::Plain(s.to_owned())
        })
    }
}

impl From<SessionTargetSpec> for hopr_utils_session::SessionTargetSpec {
    fn from(spec: SessionTargetSpec) -> Self {
        match spec {
            SessionTargetSpec::Plain(t) => Self::Plain(t),
            SessionTargetSpec::Sealed(t) => Self::Sealed(t),
            SessionTargetSpec::Service(t) => Self::Service(t),
        }
    }
}

#[repr(u8)]
#[derive(
    Debug,
    Clone,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
    Serialize,
    Deserialize,
    PartialEq,
    utoipa::ToSchema,
)]
#[schema(example = "Segmentation")]
/// Session capabilities that can be negotiated with the target peer.
pub enum SessionCapability {
    /// Frame segmentation
    Segmentation,
    /// Frame retransmission (ACK and NACK-based)
    Retransmission,
    /// Frame retransmission (only ACK-based)
    RetransmissionAckOnly,
    /// Disable packet buffering
    NoDelay,
    /// Disable SURB-based egress rate control at the Exit.
    NoRateControl,
    /// Use the Protocol for Incentivization of eXits (PIX).
    UsePIX,
}

impl From<SessionCapability> for SessionCapabilities {
    fn from(cap: SessionCapability) -> SessionCapabilities {
        match cap {
            SessionCapability::Segmentation => {
                hopr_lib::exports::transport::SessionCapability::Segmentation.into()
            }
            SessionCapability::Retransmission => {
                hopr_lib::exports::transport::SessionCapability::RetransmissionNack
                    | hopr_lib::exports::transport::SessionCapability::RetransmissionAck
            }
            SessionCapability::RetransmissionAckOnly => {
                hopr_lib::exports::transport::SessionCapability::RetransmissionAck.into()
            }
            SessionCapability::NoDelay => {
                hopr_lib::exports::transport::SessionCapability::NoDelay.into()
            }
            SessionCapability::NoRateControl => {
                hopr_lib::exports::transport::SessionCapability::NoRateControl.into()
            }
            SessionCapability::UsePIX => {
                hopr_lib::exports::transport::SessionCapability::UsePIX.into()
            }
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({ "Hops": 1 }))]
pub enum RoutingOptions {
    Hops(usize),
    IntermediatePath(Vec<String>),
}

impl TryFrom<RoutingOptions> for hopr_lib::HopRouting {
    type Error = GeneralError;

    /// Converts API routing options into protocol-level hop routing.
    fn try_from(value: RoutingOptions) -> Result<Self, Self::Error> {
        match value {
            RoutingOptions::Hops(hops) => HopRouting::try_from(hops),
            RoutingOptions::IntermediatePath(_) => Err(GeneralError::ParseError(
                "explicit path routing is only supported on /session/{protocol}/explicit-path"
                    .into(),
            )),
        }
    }
}

impl From<hopr_lib::HopRouting> for RoutingOptions {
    fn from(opts: hopr_lib::HopRouting) -> Self {
        RoutingOptions::Hops(opts.hop_count())
    }
}

impl From<hopr_lib::api::types::internal::routing::RoutingOptions> for RoutingOptions {
    fn from(opts: hopr_lib::api::types::internal::routing::RoutingOptions) -> Self {
        match opts {
            hopr_lib::api::types::internal::routing::RoutingOptions::Hops(hops) => {
                RoutingOptions::Hops(usize::from(hops))
            }
            hopr_lib::api::types::internal::routing::RoutingOptions::IntermediatePath(path) => {
                RoutingOptions::IntermediatePath(
                    path.into_iter().map(|id| id.to_string()).collect(),
                )
            }
        }
    }
}

/// The three SSA dimensions a PIX Session is priced against.
///
/// Named fields rather than the positional triple this used to be, for the reason
/// [`PixParams`] gives for its own shape: `polysPerSsa` and `sharesPerPoly` are interchangeable
/// to any type checker and are *not* interchangeable to the protocol, while their product —
/// which is all the Exit compares — is identical either way. A transposed pair therefore
/// announces valid-looking dimensions against a correct quota. Names are what close that, and
/// they close it in every consumer of the spec, not just the Rust ones.
///
/// The field names are [`PixParams`]' own, so one vocabulary runs from the JSON body to the
/// packed protocol word.
//
// The per-field `value_type`/`maximum` pairs describe each Rust type's own range, not the
// protocol's: `usize` drops utoipa's `format: int32` (which would otherwise make every dimension
// a signed `i32` downstream) and the bound is what typify then uses to pick the width back up, so
// the generated client lands on the exact `u16`/`u8`/`u8` this struct declares. The narrower
// protocol limits — `MAX_POLYS_PER_SSA`, `MIN_POLY_THRESHOLD` — stay with
// `PixParams::try_new_for`, which rejects them by name and says what the node expects; a schema
// bound could only produce an untyped 422. `//` rather than `///`: this explains the attributes,
// and utoipa would otherwise publish it as the type's description.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[schema(example = json!({"polysPerSsa": 8, "sharesPerPoly": 4, "surplusShares": 2}))]
pub(crate) struct PixSsaQuota {
    /// Polynomials per SSA.
    #[schema(value_type = usize, maximum = 65535)]
    pub polys_per_ssa: u16,
    /// Shares per polynomial, i.e. the reconstruction threshold.
    #[schema(value_type = usize, maximum = 255)]
    pub shares_per_poly: u8,
    /// Shares beyond the threshold.
    ///
    /// Priced like the other two: the per-SSA quota is
    /// `polysPerSsa × (sharesPerPoly + surplusShares) × PAYLOAD_SIZE`, so a surplus that
    /// disagreed with the Exit would size every deposit against a quota the node never
    /// agreed to.
    #[schema(value_type = usize, maximum = 255)]
    pub surplus_shares: u8,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({
        "destination": "0x1B482420Afa04aeC1Ef0e4a00C18451E84466c75",
        "forwardPath": { "Hops": 1 },
        "returnPath": { "Hops": 1 },
        "target": {"Plain": "localhost:8080"},
        "listenHost": "127.0.0.1:10000",
        "capabilities": ["Retransmission", "Segmentation"],
        "responseBuffer": "2 MB",
        "maxSurbUpstream": "2000 kb/s",
        "sessionPool": 0,
        "maxClientSessions": 2
    }))]
#[serde(rename_all = "camelCase")]
/// Request body for creating a new client session.
pub(crate) struct SessionClientRequest {
    /// Address of the Exit node.
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String)]
    pub destination: Address,
    /// The forward path for the Session.
    pub forward_path: RoutingOptions,
    /// The return path for the Session.
    pub return_path: RoutingOptions,
    /// Target for the Session.
    pub target: SessionTargetSpec,
    /// Listen host (`ip:port`) for the Session socket at the Entry node.
    ///
    /// Supports also partial specification (only `ip` or only `:port`) with the
    /// respective part replaced by the node's configured default.
    pub listen_host: Option<String>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    /// Capabilities for the Session protocol.
    ///
    /// Defaults to `Segmentation` and `Retransmission` for TCP and nothing for UDP.
    pub capabilities: Option<Vec<SessionCapability>>,
    /// The amount of response data the Session counterparty can deliver back to us,
    /// without us sending any SURBs to them.
    ///
    /// In other words, this size is recalculated to a number of SURBs delivered
    /// to the counterparty upfront and then maintained.
    /// The maintenance is dynamic, based on the number of responses we receive.
    ///
    /// All syntaxes like "2 MB", "128 kiB", "3MiB" are supported. The value must be
    /// at least the size of 2 Session packet payloads.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[schema(value_type = Option<String>)]
    pub response_buffer: Option<bytesize::ByteSize>,
    /// The maximum throughput at which artificial SURBs might be generated and sent
    /// to the recipient of the Session.
    ///
    /// On Sessions that rarely send data but receive a lot (= Exit node has high SURB consumption),
    /// this should roughly match the maximum retrieval throughput.
    ///
    /// All syntaxes like "2 MBps", "1.2Mbps", "300 kb/s", "1.23 Mb/s" are supported.
    #[serde(default)]
    #[serde(with = "human_bandwidth::option")]
    #[schema(value_type = Option<String>)]
    pub max_surb_upstream: Option<human_bandwidth::re::bandwidth::Bandwidth>,
    /// How many Sessions to pool for clients.
    ///
    /// If no sessions are pooled, they will be opened ad-hoc when a client connects.
    /// It has no effect on UDP sessions in the current implementation.
    ///
    /// Currently, the maximum value is 5.
    pub session_pool: Option<usize>,
    /// The maximum number of client sessions that the listener can spawn.
    ///
    /// This currently applies only to the TCP sessions, as UDP sessions cannot
    /// handle multiple clients (and spawn therefore always only a single session).
    ///
    /// If this value is smaller than the value specified in `session_pool`, it will
    /// be set to that value.
    ///
    /// The default value is 5.
    pub max_client_sessions: Option<usize>,
    /// PIX SSA parameters.
    ///
    /// When set, the Session will use the PIX protocol with the given parameters. When
    /// not set, PIX is not advertised to the Exit node.
    ///
    /// All three dimensions have to match this node's own installed share generator, or the
    /// Session is refused at setup.
    ///
    /// [`PixParams`] is a quadruple — the fourth element is the curve suite, and it is
    /// deliberately not here. The suite is a property of this build, fixed by the
    /// `pix-bjj`/`pix-secp256k1` feature that selects `HoprPixSpec`, not something an API
    /// caller may pick: shares are produced under one curve and announcing another would
    /// describe a generator this node does not have. It is supplied by
    /// `PixParams::try_new_for::<HoprPixSpec>` so it comes from the same place the shares do.
    #[serde(default)]
    pub pix_ssa_quota: Option<PixSsaQuota>,
    /// Flow-control (AIMD send-window) profile for this session: `off` | `clean` | `robust`.
    ///
    /// Flow control paces the entry (sending) side of the session. When omitted, the node's
    /// configured default (`api.session_flow_control`) is used. `robust` is the tail-tolerance
    /// profile for throttled / high-latency multi-hop paths.
    #[serde(default)]
    pub flow_control: Option<crate::config::SessionFlowControl>,
}

/// Maps the wire-form quota onto [`PixParams`] for this build's curve suite.
///
/// Shared by both session request types because the conversion carries the whole validation
/// contract of `pixSsaQuota` — the node refuses any Session whose three dimensions disagree with
/// its installed generator — and two hand-synchronised copies of that is one copy too many.
///
/// The error text is kept. It names which dimension is wrong and what the node expects, and it is
/// what stands between a caller whose generator disagrees and a bare `400 INVALID_INPUT`.
fn pix_params_from_quota(quota: Option<PixSsaQuota>) -> Result<Option<PixParams>, ApiErrorStatus> {
    quota
        .map(|q| {
            let PixSsaQuota {
                polys_per_ssa,
                shares_per_poly,
                surplus_shares,
            } = q;
            PixParams::try_new_for::<HoprPixSpec>(polys_per_ssa, shares_per_poly, surplus_shares)
                .map_err(|e| {
                    ApiErrorStatus::InvalidInputDetail(format!(
                        "invalid pixSsaQuota {{polysPerSsa: {polys_per_ssa}, sharesPerPoly: \
                         {shares_per_poly}, surplusShares: {surplus_shares}}}: {e}"
                    ))
                })
        })
        .transpose()
}

/// Rejects a `pixSsaQuota` and a `UsePIX` capability that do not arrive together.
///
/// Neither half is caught anywhere else, and both are silent failures rather than errors:
///
/// - Quota without the capability builds `PixParams` into a capability set that never advertises
///   PIX. The Session opens, the Exit relays with no deposit expectation, and the caller believes
///   it opened a paid Session.
/// - The capability without a quota announces PIX with no negotiated parameters. What the Exit
///   does with that is decided outside this crate, and either way it is a configuration error
///   that should be reported here, where each field still has a name attached.
fn check_pix_consistency(
    capabilities: SessionCapabilities,
    quota: Option<PixSsaQuota>,
) -> Result<(), ApiErrorStatus> {
    // The protocol capability, not this module's same-named API enum: `capabilities` is already
    // the converted flag set.
    let advertises_pix =
        capabilities.contains(hopr_lib::exports::transport::SessionCapability::UsePIX);
    if advertises_pix != quota.is_some() {
        return Err(ApiErrorStatus::InvalidInputDetail(
            "`pixSsaQuota` and the `UsePIX` capability must be supplied together: a quota without \
             the capability opens an unpaid Session, and the capability without a quota \
             advertises PIX with no negotiated parameters"
                .into(),
        ));
    }
    Ok(())
}

impl SessionClientRequest {
    /// Converts the API client session request into protocol-level session configuration.
    pub(crate) async fn into_protocol_session_config(
        self,
        target_protocol: IpProtocol,
        flow_control: Option<hopr_lib::exports::transport::FlowControlConfig>,
    ) -> Result<(Address, SessionTarget, HoprSessionClientConfig), ApiErrorStatus> {
        let target_spec: hopr_utils_session::SessionTargetSpec = self.target.clone().into();
        let capabilities = self
            .capabilities
            .map(|vs| {
                let mut caps = SessionCapabilities::empty();
                caps.extend(vs.into_iter().map(SessionCapabilities::from));
                caps
            })
            .unwrap_or_else(|| match target_protocol {
                IpProtocol::TCP => {
                    hopr_lib::exports::transport::SessionCapability::RetransmissionAck
                        | hopr_lib::exports::transport::SessionCapability::RetransmissionNack
                        | hopr_lib::exports::transport::SessionCapability::Segmentation
                }
                // Only Segmentation capability for UDP per default
                _ => SessionCapability::Segmentation.into(),
            });
        check_pix_consistency(capabilities, self.pix_ssa_quota)?;

        Ok((
            self.destination,
            target_spec.into_target(target_protocol.into())?,
            HoprSessionClientConfig {
                forward_path: self.forward_path.try_into()?,
                return_path: self.return_path.try_into()?,
                capabilities,
                surb_management: SessionConfig {
                    response_buffer: self.response_buffer,
                    max_surb_upstream: self.max_surb_upstream,
                }
                .into(),
                pix_ssa_quota: pix_params_from_quota(self.pix_ssa_quota)?,
                // Per-request profile overrides the node default when present.
                flow_control: self
                    .flow_control
                    .map(crate::config::SessionFlowControl::to_config)
                    .unwrap_or(flow_control),
                ..Default::default()
            },
        ))
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({
        "destination": "0x1B482420Afa04aeC1Ef0e4a00C18451E84466c75",
        "forwardPath": ["0x1111111111111111111111111111111111111111", "0x2222222222222222222222222222222222222222"],
        "returnPath": ["0x1111111111111111111111111111111111111111", "0x2222222222222222222222222222222222222222"],
        "target": {"Plain": "localhost:8080"},
        "listenHost": "127.0.0.1:10000",
        "capabilities": ["Retransmission", "Segmentation"],
        "responseBuffer": "2 MB",
        "maxSurbUpstream": "2000 kb/s",
        "sessionPool": 0,
        "maxClientSessions": 2
    }))]
#[serde(rename_all = "camelCase")]
pub(crate) struct SessionClientExplicitPathRequest {
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String)]
    pub destination: Address,
    pub forward_path: Vec<String>,
    pub return_path: Vec<String>,
    pub target: SessionTargetSpec,
    pub listen_host: Option<String>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    pub capabilities: Option<Vec<SessionCapability>>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[schema(value_type = Option<String>)]
    pub response_buffer: Option<bytesize::ByteSize>,
    #[serde(default)]
    #[serde(with = "human_bandwidth::option")]
    #[schema(value_type = Option<String>)]
    pub max_surb_upstream: Option<human_bandwidth::re::bandwidth::Bandwidth>,
    pub session_pool: Option<usize>,
    pub max_client_sessions: Option<usize>,
    /// PIX SSA parameters.
    ///
    /// Same meaning and same constraints as on
    /// [`SessionClientRequest`](SessionClientRequest::pix_ssa_quota).
    #[serde(default)]
    pub pix_ssa_quota: Option<PixSsaQuota>,
}

impl SessionClientExplicitPathRequest {
    #[allow(deprecated)]
    fn into_protocol_session_explicit_config<H>(
        self,
        hopr: &H,
        target_protocol: IpProtocol,
        flow_control: Option<hopr_lib::exports::transport::FlowControlConfig>,
    ) -> Result<
        (
            Address,
            SessionTarget,
            HoprSessionClientExplicitPathConfig,
            RoutingOptions,
            RoutingOptions,
        ),
        ApiErrorStatus,
    >
    where
        H: HasChainApi<ChainError = HoprLibError>,
    {
        let parse_node = |node: String| -> Result<NodeId, ApiErrorStatus> {
            let address = Address::from_str(&node).map_err(|err| {
                ApiErrorStatus::UnknownFailure(format!(
                    "invalid intermediate path node address '{node}': {err}"
                ))
            })?;
            let offchain_key = hopr
                .chain_api()
                .chain_key_to_packet_key(&address)
                .map_err(|err| {
                    ApiErrorStatus::UnknownFailure(format!(
                        "failed to resolve intermediate path node address '{node}': {err}"
                    ))
                })?
                .ok_or_else(|| {
                    ApiErrorStatus::UnknownFailure(format!(
                        "unknown intermediate path node address '{node}'"
                    ))
                })?;
            Ok(NodeId::from(offchain_key))
        };
        let forward_path = self
            .forward_path
            .clone()
            .into_iter()
            .map(parse_node)
            .collect::<Result<Vec<_>, _>>()?;
        let return_path = self
            .return_path
            .clone()
            .into_iter()
            .map(parse_node)
            .collect::<Result<Vec<_>, _>>()?;

        let target_spec: hopr_utils_session::SessionTargetSpec = self.target.clone().into();
        let capabilities = self
            .capabilities
            .map(|vs| {
                let mut caps = SessionCapabilities::empty();
                caps.extend(vs.into_iter().map(SessionCapabilities::from));
                caps
            })
            .unwrap_or_else(|| match target_protocol {
                IpProtocol::TCP => {
                    hopr_lib::exports::transport::SessionCapability::RetransmissionAck
                        | hopr_lib::exports::transport::SessionCapability::RetransmissionNack
                        | hopr_lib::exports::transport::SessionCapability::Segmentation
                }
                _ => SessionCapability::Segmentation.into(),
            });
        check_pix_consistency(capabilities, self.pix_ssa_quota)?;

        Ok((
            self.destination,
            target_spec.into_target(target_protocol.into())?,
            #[allow(deprecated)]
            {
                HoprSessionClientExplicitPathConfig {
                    forward_path,
                    return_path,
                    capabilities,
                    surb_management: SessionConfig {
                        response_buffer: self.response_buffer,
                        max_surb_upstream: self.max_surb_upstream,
                    }
                    .into(),
                    pix_ssa_quota: pix_params_from_quota(self.pix_ssa_quota)?,
                    // The deprecated explicit-path endpoint has no per-request override, so it
                    // always uses the node default profile.
                    flow_control,
                    ..Default::default()
                }
            },
            RoutingOptions::IntermediatePath(self.forward_path),
            RoutingOptions::IntermediatePath(self.return_path),
        ))
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({
        "target": "example.com:80",
        "destination": "0x5112D584a1C72Fc250176B57aEba5fFbbB287D8F",
        "forwardPath": { "Hops": 1 },
        "returnPath": { "Hops": 1 },
        "protocol": "tcp",
        "ip": "127.0.0.1",
        "port": 5542,
        "hoprMtu": 1002,
        "surbLen": 398,
        "activeClients": [],
        "maxClientSessions": 2,
        "maxSurbUpstream": "2000 kb/s",
        "responseBuffer": "2 MB",
        "sessionPool": 0
    }))]
#[serde(rename_all = "camelCase")]
/// Response body for creating a new client session.
pub(crate) struct SessionClientResponse {
    #[schema(example = "example.com:80")]
    /// Target of the Session.
    pub target: String,
    /// Destination node (exit node) of the Session.
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String)]
    pub destination: Address,
    /// Forward routing path.
    pub forward_path: RoutingOptions,
    /// Return routing path.
    pub return_path: RoutingOptions,
    /// IP protocol used by Session's listening socket.
    #[serde_as(as = "DisplayFromStr")]
    #[schema(example = "tcp")]
    pub protocol: IpProtocol,
    /// Listening IP address of the Session's socket.
    #[schema(example = "127.0.0.1")]
    pub ip: String,
    #[schema(example = 5542)]
    /// Listening port of the Session's socket.
    pub port: u16,
    /// MTU used by the underlying HOPR transport.
    pub hopr_mtu: usize,
    /// Size of a Single Use Reply Block used by the protocol.
    ///
    /// This is useful for SURB balancing calculations.
    pub surb_len: usize,
    /// Lists Session IDs of all active clients.
    ///
    /// Can contain multiple entries on TCP sessions, but currently
    /// always only a single entry on UDP sessions.
    pub active_clients: Vec<String>,
    /// The maximum number of client sessions that the listener can spawn.
    ///
    /// This currently applies only to the TCP sessions, as UDP sessions cannot
    /// have multiple clients (defaults to 1 for UDP).
    pub max_client_sessions: usize,
    /// The maximum throughput at which artificial SURBs might be generated and sent
    /// to the recipient of the Session.
    #[serde(default)]
    #[serde(with = "human_bandwidth::option")]
    #[schema(value_type = Option<String>)]
    pub max_surb_upstream: Option<human_bandwidth::re::bandwidth::Bandwidth>,
    /// The amount of response data the Session counterparty can deliver back to us, without us
    /// sending any SURBs to them.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[schema(value_type = Option<String>)]
    pub response_buffer: Option<bytesize::ByteSize>,
    /// How many Sessions to pool for clients.
    pub session_pool: Option<usize>,
}

/// Creates a new client session returning the given session listening host and port over TCP or UDP.
/// If no listening port is given in the request, the socket will be bound to a random free
/// port and returned in the response.
/// Different capabilities can be configured for the session, such as data segmentation or
/// retransmission.
///
/// Once the host and port are bound, it is possible to use the socket for bidirectional read/write
/// communication over the selected IP protocol and HOPR network routing with the given destination.
/// The destination HOPR node forwards all the data to the given target over the selected IP protocol.
///
/// Various services require different types of socket communications:
/// - services running over UDP usually do not require data retransmission, as it is already expected
/// that UDP does not provide these and is therefore handled at the application layer.
/// - On the contrary, services running over TCP *almost always* expect data segmentation and
/// retransmission capabilities, so these should be configured while creating a session that passes
/// TCP data.
#[utoipa::path(
        post,
        path = const_format::formatcp!("{BASE_PATH}/session/{{protocol}}"),
        description = "Creates a new client HOPR session that will start listening on a dedicated port. Once the port is bound, it is possible to use the socket for bidirectional read and write communication.",
        params(
            ("protocol" = String, Path, description = "IP transport protocol", example = "tcp"),
        ),
        request_body(
            content = SessionClientRequest,
            description = "Creates a new client HOPR session that will start listening on a dedicated port. Once the port is bound, it is possible to use the socket for bidirectional read and write communication.",
            content_type = "application/json"),
        responses(
            (status = 200, description = "Successfully created a new client session.", body = SessionClientResponse),
            (status = 400, description = "Invalid IP protocol.", body = ApiError),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 409, description = "Listening address and port already in use.", body = ApiError),
            (status = 422, description = "Unknown failure", body = ApiError),
        ),
        security(
            ("api_token" = []),
            ("bearer_token" = [])
        ),
        tag = "Session"
    )]
pub(crate) async fn create_client<H: crate::RestApiSessionFactory>(
    State(state): State<Arc<InternalState<H>>>,
    Path(protocol): Path<IpProtocol>,
    Json(args): Json<SessionClientRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    create_client_impl(state, protocol, args).await
}

#[deprecated(note = "Use POST /session/{protocol} with hop-based routing.")]
#[utoipa::path(
        post,
        path = const_format::formatcp!("{BASE_PATH}/session/{{protocol}}/explicit-path"),
        description = "Deprecated: creates a client HOPR session using explicit routing paths.",
        params(
            ("protocol" = String, Path, description = "IP transport protocol", example = "tcp"),
        ),
        request_body(
            content = SessionClientExplicitPathRequest,
            description = "Deprecated explicit-path session creation endpoint.",
            content_type = "application/json"),
        responses(
            (status = 200, description = "Successfully created a new client session.", body = SessionClientResponse),
            (status = 400, description = "Invalid input.", body = ApiError),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 409, description = "Listening address and port already in use.", body = ApiError),
            (status = 422, description = "Unknown failure", body = ApiError),
        ),
        security(
            ("api_token" = []),
            ("bearer_token" = [])
        ),
        tag = "Session"
    )]
pub(crate) async fn create_client_explicit_path<
    H: crate::RestApiSessionFactory + HasChainApi<ChainError = HoprLibError>,
>(
    State(state): State<Arc<InternalState<H>>>,
    Path(protocol): Path<IpProtocol>,
    Json(args): Json<SessionClientExplicitPathRequest>,
) -> Result<Response, (StatusCode, ApiErrorStatus)> {
    create_client_explicit_path_impl(state, protocol, args).await
}

async fn create_client_explicit_path_impl<
    H: crate::RestApiSessionFactory + HasChainApi<ChainError = HoprLibError>,
>(
    state: Arc<InternalState<H>>,
    protocol: IpProtocol,
    args: SessionClientExplicitPathRequest,
) -> Result<Response, (StatusCode, ApiErrorStatus)> {
    let bind_host: std::net::SocketAddr =
        build_binding_host(args.listen_host.as_deref(), state.default_listen_host);

    let listener_id = ListenerId(protocol.into(), bind_host);
    if bind_host.port() > 0 && state.open_listeners.0.contains_key(&listener_id) {
        return Err((StatusCode::CONFLICT, ApiErrorStatus::ListenHostAlreadyUsed));
    }

    let port_range = std::env::var(crate::env::HOPRD_SESSION_PORT_RANGE).ok();
    tracing::debug!(%protocol, %bind_host, ?port_range, "binding explicit-path session listening socket");

    match protocol {
        IpProtocol::TCP => {
            let session_pool = args.session_pool;
            let max_client_sessions = args.max_client_sessions;
            let target_spec: hopr_utils_session::SessionTargetSpec = args.target.clone().into();
            let (destination, _target, config, forward_path, return_path) = args
                .clone()
                .into_protocol_session_explicit_config(
                    &*state.hopr,
                    IpProtocol::TCP,
                    state.session_flow_control.to_config(),
                )
                .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))?;

            let (bound_host, udp_session_id, max_client_sessions) = create_tcp_client_binding(
                bind_host,
                port_range,
                H::explicit_path_session_factory(state.hopr.clone()),
                state.open_listeners.clone(),
                destination,
                target_spec,
                config,
                session_pool,
                max_client_sessions,
            )
            .await
            .map_err(|e| match e {
                hopr_utils_session::BindError::ListenHostAlreadyUsed => {
                    (StatusCode::CONFLICT, ApiErrorStatus::ListenHostAlreadyUsed)
                }
                hopr_utils_session::BindError::UnknownFailure(_) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ApiErrorStatus::UnknownFailure(format!(
                        "failed to start TCP listener on {bind_host}: {e}"
                    )),
                ),
            })?;

            Ok::<_, (StatusCode, ApiErrorStatus)>(
                (
                    StatusCode::OK,
                    Json(SessionClientResponse {
                        protocol,
                        ip: bound_host.ip().to_string(),
                        port: bound_host.port(),
                        target: args.target.to_string(),
                        destination: args.destination,
                        forward_path,
                        return_path,
                        hopr_mtu: SESSION_MTU,
                        surb_len: SURB_SIZE,
                        active_clients: udp_session_id.into_iter().map(|s| s.to_string()).collect(),
                        max_client_sessions,
                        max_surb_upstream: args.max_surb_upstream,
                        response_buffer: args.response_buffer,
                        session_pool: args.session_pool,
                    }),
                )
                    .into_response(),
            )
        }
        IpProtocol::UDP => {
            let target_spec: hopr_utils_session::SessionTargetSpec = args.target.clone().into();
            let (destination, _target, config, forward_path, return_path) = args
                .clone()
                .into_protocol_session_explicit_config(
                    &*state.hopr,
                    IpProtocol::UDP,
                    state.session_flow_control.to_config(),
                )
                .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))?;

            let (bound_host, udp_session_id, max_client_sessions) = create_udp_client_binding(
                bind_host,
                port_range,
                H::explicit_path_session_factory(state.hopr.clone()),
                state.open_listeners.clone(),
                destination,
                target_spec,
                config,
            )
            .await
            .map_err(|e| match e {
                hopr_utils_session::BindError::ListenHostAlreadyUsed => {
                    (StatusCode::CONFLICT, ApiErrorStatus::ListenHostAlreadyUsed)
                }
                hopr_utils_session::BindError::UnknownFailure(_) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ApiErrorStatus::UnknownFailure(format!(
                        "failed to start UDP listener on {bind_host}: {e}"
                    )),
                ),
            })?;

            Ok::<_, (StatusCode, ApiErrorStatus)>(
                (
                    StatusCode::OK,
                    Json(SessionClientResponse {
                        protocol,
                        ip: bound_host.ip().to_string(),
                        port: bound_host.port(),
                        target: args.target.to_string(),
                        destination: args.destination,
                        forward_path,
                        return_path,
                        hopr_mtu: SESSION_MTU,
                        surb_len: SURB_SIZE,
                        active_clients: udp_session_id.into_iter().map(|s| s.to_string()).collect(),
                        max_client_sessions,
                        max_surb_upstream: args.max_surb_upstream,
                        response_buffer: args.response_buffer,
                        session_pool: args.session_pool,
                    }),
                )
                    .into_response(),
            )
        }
    }
}

async fn create_client_impl<H: crate::RestApiSessionFactory>(
    state: Arc<InternalState<H>>,
    protocol: IpProtocol,
    args: SessionClientRequest,
) -> Result<Response, (StatusCode, ApiErrorStatus)> {
    let bind_host: std::net::SocketAddr =
        build_binding_host(args.listen_host.as_deref(), state.default_listen_host);

    let listener_id = ListenerId(protocol.into(), bind_host);
    if bind_host.port() > 0 && state.open_listeners.0.contains_key(&listener_id) {
        return Err((StatusCode::CONFLICT, ApiErrorStatus::ListenHostAlreadyUsed));
    }

    let port_range = std::env::var(crate::env::HOPRD_SESSION_PORT_RANGE).ok();
    tracing::debug!(%protocol, %bind_host, ?port_range, "binding session listening socket");

    let (bound_host, udp_session_id, max_clients) = match protocol {
        IpProtocol::TCP => {
            let session_pool = args.session_pool;
            let max_client_sessions = args.max_client_sessions;
            let target_spec: hopr_utils_session::SessionTargetSpec = args.target.clone().into();
            let (destination, _target, config) = args
                .clone()
                .into_protocol_session_config(
                    IpProtocol::TCP,
                    state.session_flow_control.to_config(),
                )
                .await
                .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))?;

            create_tcp_client_binding(
                bind_host,
                port_range,
                H::hop_session_factory(state.hopr.clone()),
                state.open_listeners.clone(),
                destination,
                target_spec,
                config,
                session_pool,
                max_client_sessions,
            )
            .await
            .map_err(|e| match e {
                hopr_utils_session::BindError::ListenHostAlreadyUsed => {
                    (StatusCode::CONFLICT, ApiErrorStatus::ListenHostAlreadyUsed)
                }
                hopr_utils_session::BindError::UnknownFailure(_) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ApiErrorStatus::UnknownFailure(format!(
                        "failed to start TCP listener on {bind_host}: {e}"
                    )),
                ),
            })?
        }
        IpProtocol::UDP => {
            let target_spec: hopr_utils_session::SessionTargetSpec = args.target.clone().into();
            let (destination, _target, config) = args
                .clone()
                .into_protocol_session_config(
                    IpProtocol::UDP,
                    state.session_flow_control.to_config(),
                )
                .await
                .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))?;

            create_udp_client_binding(
                bind_host,
                port_range,
                H::hop_session_factory(state.hopr.clone()),
                state.open_listeners.clone(),
                destination,
                target_spec,
                config,
            )
            .await
            .map_err(|e| match e {
                hopr_utils_session::BindError::ListenHostAlreadyUsed => {
                    (StatusCode::CONFLICT, ApiErrorStatus::ListenHostAlreadyUsed)
                }
                hopr_utils_session::BindError::UnknownFailure(_) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ApiErrorStatus::UnknownFailure(format!(
                        "failed to start UDP listener on {bind_host}: {e}"
                    )),
                ),
            })?
        }
    };

    Ok::<_, (StatusCode, ApiErrorStatus)>(
        (
            StatusCode::OK,
            Json(SessionClientResponse {
                protocol,
                ip: bound_host.ip().to_string(),
                port: bound_host.port(),
                target: args.target.to_string(),
                destination: args.destination,
                forward_path: args.forward_path.clone(),
                return_path: args.return_path.clone(),
                hopr_mtu: SESSION_MTU,
                surb_len: SURB_SIZE,
                active_clients: udp_session_id.into_iter().map(|s| s.to_string()).collect(),
                max_client_sessions: max_clients,
                max_surb_upstream: args.max_surb_upstream,
                response_buffer: args.response_buffer,
                session_pool: args.session_pool,
            }),
        )
            .into_response(),
    )
}

/// Lists existing Session listeners for the given IP protocol.
#[utoipa::path(
    get,
    path = const_format::formatcp!("{BASE_PATH}/session/{{protocol}}"),
    description = "Lists existing Session listeners for the given IP protocol.",
    params(
        ("protocol" = String, Path, description = "IP transport protocol", example = "tcp"),
    ),
    responses(
        (status = 200, description = "Opened session listeners for the given IP protocol.", body = Vec<SessionClientResponse>, example = json!([
            {
                "target": "example.com:80",
                "destination": "0x5112D584a1C72Fc250176B57aEba5fFbbB287D8F",
                "forwardPath": { "Hops": 1 },
                "returnPath": { "Hops": 1 },
                "protocol": "tcp",
                "ip": "127.0.0.1",
                "port": 5542,
                "surbLen": 400,
                "hoprMtu": 1020,
                "activeClients": [],
                "maxClientSessions": 2,
                "maxSurbUpstream": "2000 kb/s",
                "responseBuffer": "2 MB",
                "sessionPool": 0
            }
        ])),
        (status = 400, description = "Invalid IP protocol.", body = ApiError),
        (status = 401, description = "Invalid authorization token.", body = ApiError),
        (status = 422, description = "Unknown failure", body = ApiError)
    ),
    security(
        ("api_token" = []),
        ("bearer_token" = [])
    ),
    tag = "Session",
)]
pub(crate) async fn list_clients<H: Send + Sync + 'static>(
    State(state): State<Arc<InternalState<H>>>,
    Path(protocol): Path<IpProtocol>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let response = state
        .open_listeners
        .0
        .iter()
        .filter(|v| v.key().0 == protocol.into())
        .map(|v| {
            let ListenerId(_, addr) = *v.key();
            let entry = v.value();
            let forward_path = entry.forward_path.clone();
            let return_path = entry.return_path.clone();
            SessionClientResponse {
                protocol,
                ip: addr.ip().to_string(),
                port: addr.port(),
                target: entry.target.to_string(),
                forward_path: forward_path.into(),
                return_path: return_path.into(),
                destination: entry.destination,
                hopr_mtu: SESSION_MTU,
                surb_len: SURB_SIZE,
                active_clients: entry
                    .get_clients()
                    .iter()
                    .map(|e| e.key().to_string())
                    .collect(),
                max_client_sessions: entry.max_client_sessions,
                max_surb_upstream: entry.max_surb_upstream,
                response_buffer: entry.response_buffer,
                session_pool: entry.session_pool,
            }
        })
        .collect::<Vec<_>>();

    Ok::<_, (StatusCode, ApiErrorStatus)>((StatusCode::OK, Json(response)).into_response())
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({
    "responseBuffer": "2 MB",
    "maxSurbUpstream": "2 Mbps"
}))]
#[serde(rename_all = "camelCase")]
pub(crate) struct SessionConfig {
    /// The amount of response data the Session counterparty can deliver back to us,
    /// without us sending any SURBs to them.
    ///
    /// In other words, this size is recalculated to a number of SURBs delivered
    /// to the counterparty upfront and then maintained.
    /// The maintenance is dynamic, based on the number of responses we receive.
    ///
    /// All syntaxes like "2 MB", "128 kiB", "3MiB" are supported. The value must be
    /// at least the size of 2 Session packet payloads.
    #[serde(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[schema(value_type = String)]
    pub response_buffer: Option<bytesize::ByteSize>,
    /// The maximum throughput at which artificial SURBs might be generated and sent
    /// to the recipient of the Session.
    ///
    /// On Sessions that rarely send data but receive a lot (= Exit node has high SURB consumption),
    /// this should roughly match the maximum retrieval throughput.
    ///
    /// All syntaxes like "2 MBps", "1.2Mbps", "300 kb/s", "1.23 Mb/s" are supported.
    #[serde(default)]
    #[serde(with = "human_bandwidth::option")]
    #[schema(value_type = String)]
    pub max_surb_upstream: Option<human_bandwidth::re::bandwidth::Bandwidth>,
}

impl From<SessionConfig> for Option<SurbBalancerConfig> {
    /// Converts the API session config into protocol-level SURB balancer config.
    fn from(value: SessionConfig) -> Self {
        match value.response_buffer {
            // Buffer worth at least 2 reply packets
            Some(buffer_size) if buffer_size.as_u64() >= 2 * SESSION_MTU as u64 => {
                Some(SurbBalancerConfig {
                    target_surb_buffer_size: buffer_size.as_u64() / SESSION_MTU as u64,
                    max_surbs_per_sec: value
                        .max_surb_upstream
                        .map(|b| (b.as_bps() as usize / (8 * SURB_SIZE)) as u64)
                        .unwrap_or_else(|| SurbBalancerConfig::default().max_surbs_per_sec),
                    ..Default::default()
                })
            }
            // No additional SURBs are set up and maintained, useful for high-send low-reply sessions
            Some(_) => None,
            // Use defaults otherwise
            None => Some(SurbBalancerConfig::default()),
        }
    }
}

impl From<SurbBalancerConfig> for SessionConfig {
    /// Converts protocol-level SURB balancer config into the API session config format.
    fn from(value: SurbBalancerConfig) -> Self {
        Self {
            response_buffer: Some(bytesize::ByteSize::b(
                value.target_surb_buffer_size * SESSION_MTU as u64,
            )),
            max_surb_upstream: Some(human_bandwidth::re::bandwidth::Bandwidth::from_bps(
                value.max_surbs_per_sec * (8 * SURB_SIZE) as u64,
            )),
        }
    }
}

#[utoipa::path(
    post,
    path = const_format::formatcp!("{BASE_PATH}/session/config/{{id}}"),
    description = "Updates configuration of an existing active session.",
    params(
        ("id" = String, Path, description = "Session ID", example = "0x5112D584a1C72Fc25017:487"),
    ),
    request_body(
            content = SessionConfig,
            description = "Allows updating of several parameters of an existing active session.",
            content_type = "application/json"),
    responses(
            (status = 204, description = "Successfully updated the configuration"),
            (status = 400, description = "Invalid configuration.", body = ApiError),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 404, description = "Given session ID does not refer to an existing Session", body = ApiError),
            (status = 406, description = "Session cannot be reconfigured.", body = ApiError),
            (status = 422, description = "Unknown failure", body = ApiError),
    ),
    security(
            ("api_token" = []),
            ("bearer_token" = [])
    ),
    tag = "Session"
)]
pub(crate) async fn adjust_session<H: Send + Sync + 'static>(
    State(state): State<Arc<InternalState<H>>>,
    Path(session_id): Path<String>,
    Json(args): Json<SessionConfig>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let session_id = SessionId::from_hex(&session_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, ApiErrorStatus::InvalidSessionId))?;

    if let Some(cfg) = Option::<SurbBalancerConfig>::from(args) {
        let configurator = state.open_listeners.find_configurator(&session_id);

        match configurator {
            Some(configurator) => match configurator.update_surb_balancer_config(cfg) {
                Ok(_) => Ok::<_, (StatusCode, ApiErrorStatus)>(
                    (StatusCode::NO_CONTENT, "").into_response(),
                ),
                Err(e) => Err((
                    StatusCode::NOT_ACCEPTABLE,
                    ApiErrorStatus::UnknownFailure(e.to_string()),
                )),
            },
            None => Err((StatusCode::NOT_FOUND, ApiErrorStatus::SessionNotFound)),
        }
    } else {
        Err::<_, (StatusCode, ApiErrorStatus)>((
            StatusCode::BAD_REQUEST,
            ApiErrorStatus::InvalidInput,
        ))
    }
}

#[utoipa::path(
    get,
    path = const_format::formatcp!("{BASE_PATH}/session/config/{{id}}"),
    description = "Gets configuration of an existing active session.",
    params(
        ("id" = String, Path, description = "Session ID", example = "0x5112D584a1C72Fc25017:487"),
    ),
    responses(
            (status = 200, description = "Retrieved session configuration.", body = SessionConfig),
            (status = 400, description = "Invalid session ID.", body = ApiError),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 404, description = "Given session ID does not refer to an existing Session", body = ApiError),
            (status = 422, description = "Unknown failure", body = ApiError),
    ),
    security(
            ("api_token" = []),
            ("bearer_token" = [])
    ),
    tag = "Session"
)]
pub(crate) async fn session_config<H: Send + Sync + 'static>(
    State(state): State<Arc<InternalState<H>>>,
    Path(session_id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let session_id = SessionId::from_hex(&session_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, ApiErrorStatus::InvalidSessionId))?;

    // Find the configurator for this session across all listeners
    let configurator = state.open_listeners.0.iter().find_map(|entry| {
        entry
            .value()
            .get_clients()
            .get(&session_id)
            .map(|client| client.value().configurator.clone())
    });

    match configurator {
        Some(configurator) => match configurator.get_surb_balancer_config() {
            Ok(Some(cfg)) => Ok::<_, (StatusCode, ApiErrorStatus)>(
                (StatusCode::OK, Json(SessionConfig::from(cfg))).into_response(),
            ),
            Ok(None) => Err((StatusCode::NOT_FOUND, ApiErrorStatus::SessionNotFound)),
            Err(e) => Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiErrorStatus::UnknownFailure(e.to_string()),
            )),
        },
        None => Err((StatusCode::NOT_FOUND, ApiErrorStatus::SessionNotFound)),
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    utoipa::ToSchema,
)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
#[serde(rename_all = "lowercase")]
#[schema(example = "tcp")]
/// IP transport protocol
pub enum IpProtocol {
    #[allow(clippy::upper_case_acronyms)]
    TCP,
    #[allow(clippy::upper_case_acronyms)]
    UDP,
}

impl From<IpProtocol> for hopr_lib::exports::network::types::prelude::IpProtocol {
    fn from(protocol: IpProtocol) -> hopr_lib::exports::network::types::prelude::IpProtocol {
        match protocol {
            IpProtocol::TCP => hopr_lib::exports::network::types::prelude::IpProtocol::TCP,
            IpProtocol::UDP => hopr_lib::exports::network::types::prelude::IpProtocol::UDP,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct SessionCloseClientQuery {
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String, example = "tcp")]
    /// IP transport protocol
    pub protocol: IpProtocol,

    /// Listening IP address of the Session.
    #[schema(example = "127.0.0.1:8545")]
    pub ip: String,

    /// Session port used for the listener.
    #[schema(value_type = u16, example = 10101)]
    pub port: u16,
}

/// Closes an existing Session listener.
/// The listener must've been previously created and bound for the given IP protocol.
/// Once a listener is closed, no more socket connections can be made to it.
/// If the passed port number is 0, listeners on all ports of the given listening IP and protocol
/// will be closed.
#[utoipa::path(
    delete,
    path = const_format::formatcp!("{BASE_PATH}/session/{{protocol}}/{{ip}}/{{port}}"),
    description = "Closes an existing Session listener.",
    params(SessionCloseClientQuery),
    responses(
            (status = 204, description = "Listener closed successfully"),
            (status = 400, description = "Invalid IP protocol or port.", body = ApiError),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 404, description = "Listener not found.", body = ApiError),
            (status = 422, description = "Unknown failure", body = ApiError)
    ),
    security(
            ("api_token" = []),
            ("bearer_token" = [])
    ),
    tag = "Session",
)]
pub(crate) async fn close_client<H: Send + Sync + 'static>(
    State(state): State<Arc<InternalState<H>>>,
    Path(SessionCloseClientQuery { protocol, ip, port }): Path<SessionCloseClientQuery>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let listening_ip: IpAddr = ip
        .parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, ApiErrorStatus::InvalidInput))?;

    {
        let open_listeners = &state.open_listeners.0;

        let mut to_remove = Vec::new();
        let protocol: hopr_lib::exports::network::types::prelude::IpProtocol = protocol.into();

        // Find all listeners with protocol, listening IP and optionally port number (if > 0)
        open_listeners
            .iter()
            .filter(|v| {
                let ListenerId(proto, addr) = v.key();
                protocol == *proto
                    && addr.ip() == listening_ip
                    && (addr.port() == port || port == 0)
            })
            .for_each(|v| to_remove.push(*v.key()));

        if to_remove.is_empty() {
            return Err((StatusCode::NOT_FOUND, ApiErrorStatus::InvalidInput));
        }

        for bound_addr in to_remove {
            let (_, entry) = open_listeners
                .remove(&bound_addr)
                .ok_or((StatusCode::NOT_FOUND, ApiErrorStatus::InvalidInput))?;

            // Explicitly close every client session bound to this listener so the
            // SessionManager invalidates its cache entries immediately. Otherwise
            // the entries linger until idle-timeout / LRU eviction and per-session
            // state (frame reassembly buffers, etc.) accumulates.
            let configurators: Vec<_> = entry
                .get_clients()
                .iter()
                .map(|c| c.value().configurator.clone())
                .collect();

            for cfg in &configurators {
                cfg.close();
            }

            entry.abort_handle.abort();
        }
    }

    Ok::<_, (StatusCode, ApiErrorStatus)>((StatusCode::NO_CONTENT, "").into_response())
}

#[cfg(test)]
mod tests {
    use axum::{Router, body::Body, http::Request, routing::get};
    use tower::ServiceExt;

    use super::*;
    use crate::testing::NoopNode;

    /// Flow-control precedence, exercised through the production function rather than a
    /// re-statement of it: a test that re-implements
    /// `self.flow_control.map(to_config).unwrap_or(node_default)` cannot catch that expression
    /// changing, which is the only thing worth pinning here.
    ///
    /// The `off` case is the subtle one. It nests to `Some(None)`, so an explicit per-request
    /// `off` must disable flow control even though the node default enables it; a flatten in the
    /// wrong place would silently re-enable it.
    #[tokio::test]
    async fn per_request_flow_control_should_override_the_node_default() -> anyhow::Result<()> {
        use crate::config::SessionFlowControl;

        let node_default = SessionFlowControl::Robust.to_config();

        let request = |per_request: Option<SessionFlowControl>| SessionClientRequest {
            destination: Address::from([1u8; 20]),
            forward_path: RoutingOptions::Hops(1),
            return_path: RoutingOptions::Hops(1),
            target: SessionTargetSpec::Plain("127.0.0.1:8080".into()),
            listen_host: None,
            capabilities: None,
            response_buffer: None,
            max_surb_upstream: None,
            session_pool: None,
            max_client_sessions: None,
            pix_ssa_quota: None,
            flow_control: per_request,
        };

        let resolve = async |per_request| -> anyhow::Result<_> {
            let (_, _, cfg) = request(per_request)
                .into_protocol_session_config(IpProtocol::UDP, node_default)
                .await
                .map_err(|e| anyhow::anyhow!("{e:?}"))?;
            Ok(cfg.flow_control)
        };

        assert_eq!(
            node_default,
            resolve(None).await?,
            "no per-request profile falls back to the node default"
        );
        assert_eq!(
            SessionFlowControl::Clean.to_config(),
            resolve(Some(SessionFlowControl::Clean)).await?,
            "an explicit profile wins over the node default"
        );
        assert_eq!(
            None,
            resolve(Some(SessionFlowControl::Off)).await?,
            "an explicit `off` disables flow control despite a node default that enables it"
        );
        Ok(())
    }

    fn session_router() -> Router {
        let state: Arc<InternalState<NoopNode>> = Arc::new(InternalState {
            version: "test-version".to_string(),
            hoprd_cfg: serde_json::json!({}),
            auth: Arc::new(crate::config::Auth::None),
            hopr: Arc::new(NoopNode),
            open_listeners: Arc::new(hopr_utils_session::ListenerJoinHandles::default()),
            default_listen_host: "127.0.0.1:0".parse().unwrap(),
            session_flow_control: Default::default(),
        });
        Router::new()
            .route("/session/{protocol}", get(list_clients::<NoopNode>))
            .with_state(state)
    }

    #[test]
    fn session_id_to_string_round_trips_via_from_hex() {
        use hopr_lib::api::types::crypto_random::Randomizable;
        let id = SessionId::random();
        let hex = id.to_string();
        let parsed = SessionId::from_hex(&hex).expect("from_hex must accept to_string output");
        assert_eq!(id, parsed);
    }

    #[tokio::test]
    async fn list_clients_should_return_empty_when_no_sessions() -> anyhow::Result<()> {
        let app = session_router();
        let resp = app
            .oneshot(Request::get("/session/tcp").body(Body::empty())?)
            .await?;
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
        let body: serde_json::Value = serde_json::from_slice(&bytes)?;
        assert_eq!(body.as_array().unwrap().len(), 0);
        Ok(())
    }

    #[test]
    fn use_pix_capability_maps_correctly() {
        let caps: SessionCapabilities = SessionCapability::UsePIX.into();
        assert!(caps.contains(hopr_lib::exports::transport::SessionCapability::UsePIX));
    }

    #[test]
    fn pix_quota_and_capability_must_arrive_together() {
        let with_pix: SessionCapabilities = SessionCapability::UsePIX.into();
        let without_pix: SessionCapabilities = SessionCapability::Segmentation.into();

        assert!(check_pix_consistency(with_pix, Some(quota(8, 4, 2))).is_ok());
        assert!(check_pix_consistency(without_pix, None).is_ok());

        // A quota with no capability is the dangerous half: the Session would open and the Exit
        // would relay with no deposit expectation.
        let quota_alone = check_pix_consistency(without_pix, Some(quota(8, 4, 2)));
        assert!(matches!(
            quota_alone,
            Err(ApiErrorStatus::InvalidInputDetail(_))
        ));

        let capability_alone = check_pix_consistency(with_pix, None);
        assert!(matches!(
            capability_alone,
            Err(ApiErrorStatus::InvalidInputDetail(_))
        ));
    }

    /// A caller whose dimensions disagree with the generator has to be told which one, and the
    /// status code has to stay 400 while that happens.
    #[test]
    fn pix_params_conversion_keeps_the_validation_message() {
        assert!(matches!(pix_params_from_quota(None), Ok(None)));

        // 0 polynomials cannot describe a generator, whichever spec this build installed.
        match pix_params_from_quota(Some(quota(0, 0, 0))) {
            Err(ApiErrorStatus::InvalidInputDetail(detail)) => {
                assert!(
                    detail.starts_with(
                        "invalid pixSsaQuota {polysPerSsa: 0, sharesPerPoly: 0, surplusShares: \
                         0}: "
                    ),
                    "detail should name each offending dimension, got {detail:?}"
                );
            }
            other => panic!("expected a detailed InvalidInput, got {other:?}"),
        }
    }

    fn quota(polys_per_ssa: u16, shares_per_poly: u8, surplus_shares: u8) -> PixSsaQuota {
        PixSsaQuota {
            polys_per_ssa,
            shares_per_poly,
            surplus_shares,
        }
    }

    #[test]
    fn pix_ssa_quota_roundtrips_via_json() {
        let req = SessionClientRequest {
            destination: Address::default(),
            forward_path: RoutingOptions::Hops(1),
            return_path: RoutingOptions::Hops(1),
            target: SessionTargetSpec::Plain("127.0.0.1:8080".to_string()),
            listen_host: None,
            capabilities: None,
            response_buffer: None,
            max_surb_upstream: None,
            session_pool: None,
            max_client_sessions: None,
            pix_ssa_quota: Some(quota(8, 4, 2)),
            flow_control: None,
        };
        let serialized = serde_json::to_value(&req).expect("serialize");
        assert_eq!(
            serialized["pixSsaQuota"],
            serde_json::json!({"polysPerSsa": 8, "sharesPerPoly": 4, "surplusShares": 2})
        );
        let deserialized: SessionClientRequest =
            serde_json::from_value(serialized).expect("deserialize");
        assert_eq!(deserialized.pix_ssa_quota, Some(quota(8, 4, 2)));
    }

    /// Every dimension is named, required, and spelled the one way.
    ///
    /// What the names buy is that a caller cannot *quietly* transpose two of them: under the
    /// positional triple `[4, 8, 2]` was a well-formed request for dimensions nobody meant, and
    /// only the downstream generator check — which is about something else — stood any chance of
    /// noticing. Swapping two values in the object form now requires swapping their keys too,
    /// which is a thing a reader can see.
    #[test]
    fn pix_ssa_quota_rejects_names_it_does_not_know() {
        let ok = serde_json::json!({"polysPerSsa": 8, "sharesPerPoly": 4, "surplusShares": 2});
        assert_eq!(
            serde_json::from_value::<PixSsaQuota>(ok).expect("named triple deserializes"),
            quota(8, 4, 2)
        );

        // serde's derived visitor also accepts the positional form, so the pre-rename wire
        // shape keeps deserializing. That is left alone deliberately: it is not something this
        // struct introduces but how *every* derived `Deserialize` in this API behaves —
        // `SessionClientRequest` itself accepts its own fields as a bare array — so singling
        // this one out for a hand-written visitor would buy uniformity nowhere and cost the
        // field-level error messages that make a 422 on this body readable.
        assert_eq!(
            serde_json::from_str::<PixSsaQuota>("[8, 4, 2]").expect("positional form still parses"),
            quota(8, 4, 2)
        );

        // A misspelling is caught here rather than defaulting the dimension to zero.
        let typo = serde_json::json!({"polysPerSSA": 8, "sharesPerPoly": 4, "surplusShares": 2});
        assert!(serde_json::from_value::<PixSsaQuota>(typo).is_err());

        // Every dimension is required: an omitted one is not a zero.
        let short = serde_json::json!({"polysPerSsa": 8, "sharesPerPoly": 4});
        assert!(serde_json::from_value::<PixSsaQuota>(short).is_err());
    }

    /// The published schema has to name the three dimensions, on both request types.
    ///
    /// Asserted on the schema rather than on the generated client, which is where the effect is
    /// visible (a named `PixSsaQuota` struct instead of `Option<Vec<i32>>`): a test over
    /// generated Rust would additionally fail on every progenitor or typify bump, and nothing
    /// regenerates that file in CI anyway. Both request types are checked because the
    /// explicit-path one never reaches `ApiDoc::openapi()` — only the spec served when
    /// `enable_explicit_path_sessions` is on — so a spec-level test could not see it.
    #[test]
    fn pix_ssa_quota_schema_names_its_three_dimensions() {
        let value = serde_json::to_value(<PixSsaQuota as utoipa::PartialSchema>::schema())
            .expect("schema serializes");
        assert_eq!(value["type"], "object");
        let mut required: Vec<_> = value["required"]
            .as_array()
            .expect("all three dimensions are required")
            .iter()
            .map(|v| v.as_str().expect("field name").to_string())
            .collect();
        required.sort();
        assert_eq!(required, ["polysPerSsa", "sharesPerPoly", "surplusShares"]);

        // Each request type has to reference that component rather than inlining a shape of its
        // own, or the two could drift apart while both look correct.
        for (name, schema) in [
            (
                "SessionClientRequest",
                <SessionClientRequest as utoipa::PartialSchema>::schema(),
            ),
            (
                "SessionClientExplicitPathRequest",
                <SessionClientExplicitPathRequest as utoipa::PartialSchema>::schema(),
            ),
        ] {
            let value = serde_json::to_value(schema).expect("schema serializes");
            let quota = serde_json::to_string(&value["properties"]["pixSsaQuota"])
                .expect("property serializes");
            assert!(
                quota.contains("PixSsaQuota"),
                "{name} should reference the PixSsaQuota component, got {quota}"
            );
        }
    }
}
