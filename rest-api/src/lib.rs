//! REST API for the HOPRd node.
pub mod config;

mod account;
mod channels;
mod checks;
mod middleware;
mod network;
mod node;
mod peers;
mod root;
mod session;
#[cfg(test)]
pub(crate) mod testing;
mod tickets;

pub(crate) mod env {
    /// Name of the environment variable specifying automatic port range selection for Sessions.
    /// Expected format: "<start_port>:<end_port>" (e.g., "9091:9099")
    pub const HOPRD_SESSION_PORT_RANGE: &str = "HOPRD_SESSION_PORT_RANGE";
}

use std::{error::Error, sync::Arc};

use axum::{
    Router,
    extract::Json,
    http::{
        Method,
        header::{AUTHORIZATION, HeaderName},
        status::StatusCode,
    },
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};
use hopr_lib::{api::types::primitive::prelude::Address, errors::HoprLibError};
use hopr_utils_session::ListenerJoinHandles;
use serde::Serialize;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Auth;

pub(crate) const BASE_PATH: &str =
    const_format::formatcp!("/api/v{}", env!("CARGO_PKG_VERSION_MAJOR"));

/// Combined trait bound for the HOPR node type parameter used throughout the REST API.
///
/// Any type `H: HoprNode` can be used as the node implementation backing the API.
/// In production this is `Hopr<Chain, Graph, Net, TMgr>`; in tests it can be a mock.
pub trait HoprNode:
    hopr_lib::api::node::HoprNodeOperations
    + hopr_lib::api::node::HasChainApi<ChainError = hopr_lib::errors::HoprLibError>
    + hopr_lib::api::node::HasNetworkView
    + hopr_lib::api::node::HasGraphView
    + hopr_lib::api::node::HasTransportApi
    + hopr_lib::api::node::HasTicketManagement
    + hopr_lib::api::node::HoprSessionClientOperations
    + Send
    + Sync
    + 'static
{
}

impl<T> HoprNode for T where
    T: hopr_lib::api::node::HoprNodeOperations
        + hopr_lib::api::node::HasChainApi<ChainError = hopr_lib::errors::HoprLibError>
        + hopr_lib::api::node::HasNetworkView
        + hopr_lib::api::node::HasGraphView
        + hopr_lib::api::node::HasTransportApi
        + hopr_lib::api::node::HasTicketManagement
        + hopr_lib::api::node::HoprSessionClientOperations
        + Send
        + Sync
        + 'static
{
}

pub(crate) struct AppState<H> {
    pub hopr: Arc<H>,
}

impl<H> Clone for AppState<H> {
    fn clone(&self) -> Self {
        Self {
            hopr: self.hopr.clone(),
        }
    }
}

pub type MessageEncoder = fn(&[u8]) -> Box<[u8]>;

pub(crate) struct InternalState<H> {
    pub hoprd_cfg: serde_json::Value,
    pub auth: Arc<Auth>,
    pub hopr: Arc<H>,
    pub open_listeners: Arc<ListenerJoinHandles>,
    pub default_listen_host: std::net::SocketAddr,
}

impl<H> Clone for InternalState<H> {
    fn clone(&self) -> Self {
        Self {
            hoprd_cfg: self.hoprd_cfg.clone(),
            auth: self.auth.clone(),
            hopr: self.hopr.clone(),
            open_listeners: self.open_listeners.clone(),
            default_listen_host: self.default_listen_host,
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        account::addresses,
        account::balances,
        account::withdraw,
        channels::close_channel,
        channels::fund_channel,
        channels::list_channels,
        channels::open_channel,
        channels::show_channel,
        checks::eligiblez,
        checks::healthyz,
        checks::readyz,
        checks::startedz,
        network::price,
        network::probability,
        network::connected,
        network::announced,
        network::graph,
        node::configuration,
        node::info,
        node::status,
        node::version,
        peers::ping_peer,
        peers::show_peer_info,
        root::metrics,
        session::create_client,
        session::create_client_explicit_path,
        session::list_clients,
        session::adjust_session,
        session::session_config,
        session::close_client,
        tickets::redeem_tickets,
        tickets::show_ticket_statistics,
    ),
    components(
        schemas(
            ApiError,
            account::AccountAddressesResponse, account::AccountBalancesResponse, account::WithdrawBodyRequest, account::WithdrawResponse,
            channels::ChannelsQueryRequest,channels::CloseChannelResponse, channels::OpenChannelBodyRequest, channels::OpenChannelResponse, channels::FundChannelResponse,
            channels::NodeChannel, channels::NodeChannelsResponse, channels::ChannelInfoResponse, channels::FundBodyRequest,
            channels::ChannelDirection, channels::ChannelDirectionQuery,
            network::TicketPriceResponse,
            network::TicketProbabilityResponse,
            network::ConnectedPeerResponse,
            network::AnnouncedPeerResponse,
            network::AnnouncementOriginResponse,
            node::NodeInfoResponse, node::NodeVersionResponse, node::NodeStatusResponse, node::ComponentStatusesResponse, node::ComponentStatusInfo,
            peers::MultiaddressSource, peers::NodePeerInfoResponse, peers::PeerChannelInfo, peers::PeerQosInfo, peers::PingResponse,
            session::SessionClientRequest, session::SessionClientExplicitPathRequest, session::SessionCapability, session::RoutingOptions, session::SessionTargetSpec, session::SessionClientResponse, session::IpProtocol, session::SessionConfig,
            tickets::NodeTicketStatisticsResponse, tickets::ChannelTicket, tickets::RedeemTicketsRequest,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Account", description = "HOPR node account endpoints"),
        (name = "Channels", description = "HOPR node chain channels manipulation endpoints"),
        (name = "Configuration", description = "HOPR node configuration endpoints"),
        (name = "Checks", description = "HOPR node functionality checks"),
        (name = "Network", description = "HOPR node network endpoints"),
        (name = "Node", description = "HOPR node information endpoints"),
        (name = "Peers", description = "HOPR node peer manipulation endpoints"),
        (name = "Session", description = "HOPR node session management endpoints"),
        (name = "Tickets", description = "HOPR node ticket management endpoints"),
        (name = "Metrics", description = "HOPR node metrics endpoints"),
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("components should be registered at this point");

        components.add_security_scheme(
            "bearer_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("token")
                    .description(Some("Bearer token authentication".to_string()))
                    .build(),
            ),
        );
        components.add_security_scheme(
            "api_token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "X-Auth-Token",
                "API Token",
            ))),
        );
    }
}

/// Parameters needed to construct the Rest API via [`serve_api`].
pub struct RestApiParameters<H> {
    pub listener: TcpListener,
    pub hoprd_cfg: serde_json::Value,
    pub cfg: crate::config::Api,
    pub hopr: Arc<H>,
    pub session_listener_sockets: Arc<ListenerJoinHandles>,
    pub default_session_listen_host: std::net::SocketAddr,
}

/// Starts the Rest API listener and router.
pub async fn serve_api<H: HoprNode + hopr_utils_session::SessionFactory>(
    params: RestApiParameters<H>,
) -> Result<(), std::io::Error>
where
    <<H as hopr_lib::api::node::HasTransportApi>::Transport as hopr_lib::api::node::TransportOperations>::Error:
        Into<hopr_lib::errors::HoprTransportError>,
{
    let RestApiParameters {
        listener,
        hoprd_cfg,
        cfg,
        hopr,
        session_listener_sockets,
        default_session_listen_host,
    } = params;

    let router = build_api(
        hoprd_cfg,
        cfg,
        hopr,
        session_listener_sockets,
        default_session_listen_host,
    )
    .await;
    axum::serve(listener, router).await
}

#[allow(clippy::too_many_arguments)]
async fn build_api<H: HoprNode + hopr_utils_session::SessionFactory>(
    hoprd_cfg: serde_json::Value,
    cfg: crate::config::Api,
    hopr: Arc<H>,
    open_listeners: Arc<ListenerJoinHandles>,
    default_listen_host: std::net::SocketAddr,
) -> Router
where
    <<H as hopr_lib::api::node::HasTransportApi>::Transport as hopr_lib::api::node::TransportOperations>::Error:
        Into<hopr_lib::errors::HoprTransportError>,
{
    let state = AppState { hopr };
    let inner_state = InternalState {
        auth: Arc::new(cfg.auth.clone()),
        hoprd_cfg,
        hopr: state.hopr.clone(),
        open_listeners,
        default_listen_host,
    };

    Router::new()
        .merge(
            Router::new()
                .merge(
                    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
                )
                .merge(Scalar::with_url("/scalar", ApiDoc::openapi())),
        )
        .merge(
            Router::new()
                .route("/startedz", get(checks::startedz::<H>))
                .route("/readyz", get(checks::readyz::<H>))
                .route("/healthyz", get(checks::healthyz::<H>))
                .route("/eligiblez", get(checks::eligiblez::<H>))
                .layer(
                    ServiceBuilder::new().layer(
                        CorsLayer::new()
                            .allow_methods([Method::GET])
                            .allow_origin(Any)
                            .allow_headers(Any)
                            .max_age(std::time::Duration::from_secs(86400)),
                    ),
                )
                .with_state(state.into()),
        )
        .merge(
            Router::new()
                .route("/metrics", get(root::metrics))
                .layer(axum::middleware::from_fn_with_state(
                    inner_state.clone(),
                    middleware::auth::authenticate::<H>,
                ))
                .layer(
                    ServiceBuilder::new()
                        .layer(TraceLayer::new_for_http())
                        .layer(
                            CorsLayer::new()
                                .allow_methods([Method::GET])
                                .allow_origin(Any)
                                .allow_headers(Any)
                                .max_age(std::time::Duration::from_secs(86400)),
                        )
                        .layer(axum::middleware::from_fn(middleware::prometheus::record))
                        .layer(CompressionLayer::new())
                        .layer(ValidateRequestHeaderLayer::accept("text/plain"))
                        .layer(SetSensitiveRequestHeadersLayer::new([
                            AUTHORIZATION,
                            HeaderName::from_static("x-auth-token"),
                        ])),
                ),
        )
        .nest(
            BASE_PATH,
            Router::new()
                .route("/account/addresses", get(account::addresses::<H>))
                .route("/account/balances", get(account::balances::<H>))
                .route("/account/withdraw", post(account::withdraw::<H>))
                .route("/peers/{address}", get(peers::show_peer_info::<H>))
                .route("/channels", get(channels::list_channels::<H>))
                .route("/channels", post(channels::open_channel::<H>))
                .route("/channels/{address}", get(channels::show_channel::<H>))
                .route("/channels/{address}", delete(channels::close_channel::<H>))
                .route(
                    "/channels/{address}/fund",
                    post(channels::fund_channel::<H>),
                )
                .route("/tickets/redeem", post(tickets::redeem_tickets::<H>))
                .route(
                    "/tickets/statistics",
                    get(tickets::show_ticket_statistics::<H>),
                )
                .route("/network/price", get(network::price::<H>))
                .route("/network/probability", get(network::probability::<H>))
                .route("/network/connected", get(network::connected::<H>))
                .route("/network/announced", get(network::announced::<H>))
                .route("/network/graph", get(network::graph::<H>))
                .route("/node/version", get(node::version))
                .route("/node/configuration", get(node::configuration::<H>))
                .route("/node/info", get(node::info::<H>))
                .route("/node/status", get(node::status::<H>))
                .route("/peers/{address}/ping", post(peers::ping_peer::<H>))
                .route("/session/config/{id}", get(session::session_config::<H>))
                .route("/session/config/{id}", post(session::adjust_session::<H>))
                .route("/session/{protocol}", post(session::create_client::<H>))
                .route(
                    "/session/{protocol}/explicit-path",
                    post(session::create_client_explicit_path::<H>),
                )
                .route("/session/{protocol}", get(session::list_clients::<H>))
                .route(
                    "/session/{protocol}/{ip}/{port}",
                    delete(session::close_client::<H>),
                )
                .with_state(inner_state.clone().into())
                .layer(axum::middleware::from_fn_with_state(
                    inner_state.clone(),
                    middleware::auth::authenticate::<H>,
                ))
                .layer(
                    ServiceBuilder::new()
                        .layer(TraceLayer::new_for_http())
                        .layer(
                            CorsLayer::new()
                                .allow_methods([
                                    Method::GET,
                                    Method::POST,
                                    Method::OPTIONS,
                                    Method::DELETE,
                                ])
                                .allow_origin(Any)
                                .allow_headers(Any)
                                .max_age(std::time::Duration::from_secs(86400)),
                        )
                        .layer(axum::middleware::from_fn(middleware::prometheus::record))
                        .layer(CompressionLayer::new())
                        .layer(ValidateRequestHeaderLayer::accept("application/json"))
                        .layer(SetSensitiveRequestHeadersLayer::new([
                            AUTHORIZATION,
                            HeaderName::from_static("x-auth-token"),
                        ])),
                ),
        )
}

fn checksum_address_serializer<S: serde::Serializer>(a: &Address, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&a.to_checksum())
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "status": "INVALID_INPUT",
    "error": "Invalid value passed in parameter 'XYZ'"
}))]
/// Standardized error response for the API
pub(crate) struct ApiError {
    #[schema(example = "INVALID_INPUT")]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "Invalid value passed in parameter 'XYZ'")]
    pub error: Option<String>,
}

/// Enumerates all API request errors
/// Note that `ApiError` should not be instantiated directly, but always rather through the `ApiErrorStatus`.
#[allow(unused)] // TODO: some errors can no longer be propagated to the REST API
#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum ApiErrorStatus {
    InvalidInput,
    InvalidChannelId,
    PeerNotFound,
    ChannelNotFound,
    TicketsNotFound,
    Timeout,
    PingError(String),
    Unauthorized,
    InvalidQuality,
    NotReady,
    ListenHostAlreadyUsed,
    SessionNotFound,
    InvalidSessionId,
    #[strum(serialize = "UNKNOWN_FAILURE")]
    UnknownFailure(String),
}

impl From<ApiErrorStatus> for ApiError {
    fn from(value: ApiErrorStatus) -> Self {
        let error = match &value {
            ApiErrorStatus::UnknownFailure(e) | ApiErrorStatus::PingError(e) => Some(e.clone()),
            _ => None,
        };
        Self {
            status: value.to_string(),
            error,
        }
    }
}

impl IntoResponse for ApiErrorStatus {
    fn into_response(self) -> Response {
        let error_detail = match &self {
            Self::UnknownFailure(e) | Self::PingError(e) => Some(e.as_str()),
            _ => None,
        };

        let status_code = match &self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InvalidInput | Self::InvalidChannelId | Self::InvalidSessionId => {
                StatusCode::BAD_REQUEST
            }
            Self::PeerNotFound
            | Self::ChannelNotFound
            | Self::TicketsNotFound
            | Self::SessionNotFound => StatusCode::NOT_FOUND,
            Self::Timeout => StatusCode::REQUEST_TIMEOUT,
            Self::ListenHostAlreadyUsed => StatusCode::CONFLICT,
            Self::NotReady => StatusCode::PRECONDITION_FAILED,
            Self::InvalidQuality | Self::PingError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::UnknownFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        if status_code.is_client_error() {
            tracing::warn!(
                status = %status_code,
                error = %self,
                detail = ?error_detail,
                "REST API error response"
            );
        } else if status_code.is_server_error() {
            tracing::error!(
                status = %status_code,
                error = %self,
                detail = ?error_detail,
                "REST API error response"
            );
        }

        (status_code, Json(ApiError::from(self))).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

// Errors lead to `UnknownFailure` per default
impl<T: Error> From<T> for ApiErrorStatus {
    fn from(value: T) -> Self {
        Self::UnknownFailure(value.to_string())
    }
}

// Errors lead to `UnknownFailure` per default
impl<T> From<T> for ApiError
where
    T: Error + Into<HoprLibError>,
{
    fn from(value: T) -> Self {
        Self {
            status: ApiErrorStatus::UnknownFailure("unknown error".to_string()).to_string(),
            error: Some(value.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{http::StatusCode, response::IntoResponse};
    use rstest::rstest;

    use super::{ApiError, ApiErrorStatus};

    #[test]
    fn api_error_defaults_to_500() {
        let error = ApiError {
            status: "UNKNOWN_FAILURE".into(),
            error: Some("Invalid value passed in parameter 'XYZ'".to_string()),
        };
        assert_eq!(
            error.into_response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[rstest]
    #[case(ApiErrorStatus::Unauthorized, StatusCode::UNAUTHORIZED)]
    #[case(ApiErrorStatus::InvalidInput, StatusCode::BAD_REQUEST)]
    #[case(ApiErrorStatus::InvalidChannelId, StatusCode::BAD_REQUEST)]
    #[case(ApiErrorStatus::InvalidSessionId, StatusCode::BAD_REQUEST)]
    #[case(ApiErrorStatus::PeerNotFound, StatusCode::NOT_FOUND)]
    #[case(ApiErrorStatus::ChannelNotFound, StatusCode::NOT_FOUND)]
    #[case(ApiErrorStatus::TicketsNotFound, StatusCode::NOT_FOUND)]
    #[case(ApiErrorStatus::SessionNotFound, StatusCode::NOT_FOUND)]
    #[case(ApiErrorStatus::Timeout, StatusCode::REQUEST_TIMEOUT)]
    #[case(ApiErrorStatus::ListenHostAlreadyUsed, StatusCode::CONFLICT)]
    #[case(ApiErrorStatus::NotReady, StatusCode::PRECONDITION_FAILED)]
    #[case(ApiErrorStatus::InvalidQuality, StatusCode::UNPROCESSABLE_ENTITY)]
    #[case(ApiErrorStatus::PingError("fail".into()), StatusCode::UNPROCESSABLE_ENTITY)]
    #[case(ApiErrorStatus::UnknownFailure("oops".into()), StatusCode::INTERNAL_SERVER_ERROR)]
    fn api_error_status_maps_correct_http_code(
        #[case] status: ApiErrorStatus,
        #[case] expected: StatusCode,
    ) {
        assert_eq!(status.into_response().status(), expected);
    }

    #[test]
    fn ping_error_message_surfaced_in_body() {
        let api_error = ApiError::from(ApiErrorStatus::PingError("connection refused".into()));
        assert_eq!(api_error.error.as_deref(), Some("connection refused"));
    }
}
