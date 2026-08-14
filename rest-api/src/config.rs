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

/// Flow-control profile for node-initiated sessions. See the `api.session_flow_control` config option.
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use anyhow::Context;
    use hopr_lib::exports::transport::FlowControlConfig;

    use super::*;

    #[test]
    fn session_flow_control_deserializes_from_lowercase_strings() -> anyhow::Result<()> {
        for (input, expected) in [
            ("\"off\"", SessionFlowControl::Off),
            ("\"clean\"", SessionFlowControl::Clean),
            ("\"robust\"", SessionFlowControl::Robust),
        ] {
            let parsed: SessionFlowControl = serde_json::from_str(input)
                .with_context(|| format!("should deserialize {input}"))?;
            assert_eq!(parsed, expected, "{input} should map to {expected:?}");
        }
        Ok(())
    }

    #[test]
    fn session_flow_control_default_is_robust() {
        assert_eq!(SessionFlowControl::default(), SessionFlowControl::Robust);
    }

    #[test]
    fn api_omitting_session_flow_control_defaults_to_robust() -> anyhow::Result<()> {
        let api: Api = serde_json::from_str("{}")
            .context("empty object should deserialize into Api defaults")?;
        assert_eq!(api.session_flow_control, SessionFlowControl::Robust);
        Ok(())
    }

    #[test]
    fn to_config_maps_each_profile() {
        assert_eq!(SessionFlowControl::Off.to_config(), None);
        assert_eq!(
            SessionFlowControl::Clean.to_config(),
            Some(FlowControlConfig::default())
        );
        assert_eq!(
            SessionFlowControl::Robust.to_config(),
            Some(FlowControlConfig::robust())
        );
    }

    /// The bound comes from `FlowControlConfig::robust` upstream; this pins the value hoprd
    /// actually ships with, so a change there does not silently alter node behaviour.
    #[test]
    fn robust_profile_should_bound_frame_age_at_two_seconds() -> anyhow::Result<()> {
        let config = SessionFlowControl::Robust
            .to_config()
            .context("robust profile should carry a flow-control config")?;
        assert_eq!(config.max_frame_age, Some(Duration::from_secs(2)));
        Ok(())
    }

    /// The production default path: `api.session_flow_control` is `#[serde(default)]`, so a node
    /// that never states it uses this profile, and it is what
    /// `SessionClientRequest::into_protocol_session_config` receives as its fallback. Asserting
    /// on `Robust` in isolation would not catch the default being changed out from under it.
    #[test]
    fn node_default_profile_should_be_robust_and_carry_its_bound() -> anyhow::Result<()> {
        assert_eq!(SessionFlowControl::Robust, SessionFlowControl::default());

        let config = SessionFlowControl::default()
            .to_config()
            .context("the node default must carry a flow-control config")?;
        assert_eq!(config.max_frame_age, Some(Duration::from_secs(2)));
        Ok(())
    }

    /// Precedence, mirroring the expression in `into_protocol_session_config`:
    ///
    /// ```ignore
    /// self.flow_control.map(SessionFlowControl::to_config).unwrap_or(node_default)
    /// ```
    ///
    /// The `off` case is the one worth pinning: it nests to `Some(None)`, so an explicit
    /// per-request `off` disables flow control even though the node default enables it. A naive
    /// flattening would silently re-enable it.
    #[test]
    fn per_request_profile_should_override_the_node_default() {
        let node_default = SessionFlowControl::Robust.to_config();

        let resolve = |per_request: Option<SessionFlowControl>| {
            per_request
                .map(SessionFlowControl::to_config)
                .unwrap_or(node_default)
        };

        assert_eq!(
            node_default,
            resolve(None),
            "no per-request profile falls back to the node default"
        );
        assert_eq!(
            Some(FlowControlConfig::default()),
            resolve(Some(SessionFlowControl::Clean)),
            "an explicit profile wins over the node default"
        );
        assert_eq!(
            None,
            resolve(Some(SessionFlowControl::Off)),
            "an explicit `off` disables flow control despite a node default that enables it"
        );
    }
}
