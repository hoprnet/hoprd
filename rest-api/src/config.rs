use std::str::FromStr;

use hopr_lib::config::HostConfig;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

pub const DEFAULT_API_HOST: &str = "127.0.0.1";
pub const DEFAULT_API_PORT: u16 = 3001;
pub const MINIMAL_API_TOKEN_LENGTH: usize = 8;

fn validate_api_auth(token: &Auth) -> Result<(), ValidationError> {
    match &token {
        Auth::None => Ok(()),
        Auth::Token(token) => {
            if token.len() >= MINIMAL_API_TOKEN_LENGTH {
                Ok(())
            } else {
                Err(ValidationError::new("The API token is too short"))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, smart_default::SmartDefault, Serialize, Deserialize)]
pub enum Auth {
    #[default]
    None,
    Token(String),
}

#[derive(
    Debug, Clone, PartialEq, smart_default::SmartDefault, Validate, Serialize, Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct Api {
    /// Selects whether the REST API is enabled
    #[serde(default)]
    pub enable: bool,
    /// Auth enum holding the API auth configuration
    #[validate(custom(function = "validate_api_auth"))]
    #[serde(default)]
    pub auth: Auth,
    /// Host and port combination where the REST API should be located
    #[validate(nested)]
    #[serde(default = "default_api_host")]
    #[default(default_api_host())]
    pub host: HostConfig,
    /// Enables the deprecated explicit-path session creation endpoint.
    #[serde(default)]
    pub enable_explicit_path_sessions: bool,
    /// Flow-control (AIMD send-window) profile applied to sessions this node *initiates*
    /// as a client via the session API (`off` | `clean` | `robust`).
    ///
    /// Flow control is an entry/sending-side mechanism: relays and exit nodes never run
    /// it, so this setting has no effect on nodes used purely for relaying/exiting.
    /// Default is `robust` — the tail-tolerance profile validated for throttled /
    /// high-latency (multi-hop) paths.
    #[serde(default)]
    pub session_flow_control: SessionFlowControl,
}

/// Flow-control profile for node-initiated sessions. See [`Api::session_flow_control`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    smart_default::SmartDefault,
    Serialize,
    Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum SessionFlowControl {
    /// Flow control disabled — sends are hand-paced by the caller.
    Off,
    /// The verified clean profile (`FlowControlConfig::default`).
    Clean,
    /// The tail-tolerance bundle (persist probe + raised frame-retransmission budget) for
    /// throttled / high-latency multi-hop paths (`FlowControlConfig::robust`).
    #[default]
    Robust,
}

impl SessionFlowControl {
    /// Maps the profile to the protocol-level flow-control configuration.
    pub fn to_config(self) -> Option<hopr_lib::exports::transport::FlowControlConfig> {
        use hopr_lib::exports::transport::FlowControlConfig;
        match self {
            SessionFlowControl::Off => None,
            SessionFlowControl::Clean => Some(FlowControlConfig::default()),
            SessionFlowControl::Robust => Some(FlowControlConfig::robust()),
        }
    }
}

#[inline]
fn default_api_host() -> HostConfig {
    HostConfig::from_str(format!("{DEFAULT_API_HOST}:{DEFAULT_API_PORT}").as_str())
        .expect("default credentials should always work")
}
