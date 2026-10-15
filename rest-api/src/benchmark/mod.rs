//! Relay throughput benchmark endpoints.

mod relay;
mod synthetic;

use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{Json, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use hopr_crypto_packet::prelude::HoprPacket;
use hopr_lib::utils::parallelize::cpu::rayon;
use serde::{Deserialize, Serialize};

use self::relay::{
    RelayBenchmarkConfig, RelayBenchmarkError, RelayBenchmarkPhase, RelayBenchmarkReport,
    run_relay_benchmark,
};
use crate::{ApiError, ApiErrorStatus, BASE_PATH, InternalState};

const DEFAULT_PACKETS: u32 = 20_000;
/// Prepared traffic is held in memory, about 3.7 kB per packet.
const MAX_PACKETS: u32 = 100_000;
const MAX_CONCURRENCY: u32 = 4_096;

#[derive(Debug, Clone, Default, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[schema(example = json!({ "packets": 20000 }))]
/// Parameters of a relay benchmark run.
pub(crate) struct RelayBenchmarkRequest {
    /// Number of packets to relay. Defaults to 20000, at most 100000. Prepared traffic is held in
    /// memory, about 3.7 kB per packet.
    #[serde(default)]
    #[schema(minimum = 1, maximum = 100000, example = 20000)]
    packets: Option<u32>,
    /// Number of packets decoded concurrently. Defaults to the packet pipeline's own setting.
    #[serde(default)]
    #[schema(minimum = 1, maximum = 4096, example = 64)]
    concurrency: Option<u32>,
}

impl RelayBenchmarkRequest {
    fn into_config(self) -> Result<RelayBenchmarkConfig, ApiErrorStatus> {
        let packets = self.packets.unwrap_or(DEFAULT_PACKETS);
        if !(1..=MAX_PACKETS).contains(&packets) {
            return Err(ApiErrorStatus::InvalidInputDetail(format!(
                "packets must be between 1 and {MAX_PACKETS}"
            )));
        }

        let concurrency = match self.concurrency {
            Some(c) if (1..=MAX_CONCURRENCY).contains(&c) => c as usize,
            Some(_) => {
                return Err(ApiErrorStatus::InvalidInputDetail(format!(
                    "concurrency must be between 1 and {MAX_CONCURRENCY}"
                )));
            }
            None => RelayBenchmarkConfig::default_concurrency(),
        };

        Ok(RelayBenchmarkConfig {
            packets: packets as usize,
            concurrency,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
/// Stage of a relay benchmark run.
pub(crate) enum RelayBenchmarkState {
    /// No benchmark has run since the node started.
    #[default]
    Idle,
    /// Generating the traffic to relay; not measured.
    Preparing,
    /// Relaying the prepared traffic; measured.
    Relaying,
    Completed,
    Failed,
}

impl RelayBenchmarkState {
    fn is_running(self) -> bool {
        matches!(self, Self::Preparing | Self::Relaying)
    }
}

#[derive(Debug, Clone, Default, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "state": "completed",
    "packets": 20000,
    "concurrency": 64,
    "startedAt": 1760529600,
    "finishedAt": 1760529609,
    "result": {
        "relayedPackets": 20000,
        "rejectedPackets": 0,
        "acknowledgementsSent": 20000,
        "acknowledgementPacketsSent": 607,
        "ticketsAcknowledged": 20000,
        "preparationMs": 5258,
        "durationMs": 3226,
        "packetsPerSecond": 6200.5,
        "bytesPerSecond": 22749634.5,
        "megabitsPerSecond": 182.0,
        "packetSize": 3669,
        "cpuThreads": 8
    }
}))]
/// Latest relay benchmark run.
pub(crate) struct RelayBenchmarkResponse {
    state: RelayBenchmarkState,
    /// Number of packets the run relays.
    #[serde(skip_serializing_if = "Option::is_none")]
    packets: Option<u32>,
    /// Number of packets the run decodes concurrently.
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency: Option<u32>,
    /// Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    started_at: Option<u64>,
    /// Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    finished_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<RelayBenchmarkResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
/// Measured relay throughput.
pub(crate) struct RelayBenchmarkResult {
    /// Packets forwarded to the next hop.
    relayed_packets: u64,
    /// Packets the relay refused to forward.
    rejected_packets: u64,
    /// Acknowledgements signed and sent back to the previous hop.
    acknowledgements_sent: u64,
    /// Packets carrying those acknowledgements.
    acknowledgement_packets_sent: u64,
    /// Tickets the next hop's acknowledgements turned into winning tickets.
    tickets_acknowledged: u64,
    /// Time spent generating the traffic; not part of the measurement.
    preparation_ms: u64,
    /// Time spent relaying.
    duration_ms: u64,
    packets_per_second: f64,
    /// Relayed bytes per second, in each direction.
    bytes_per_second: f64,
    /// Relayed megabits per second, in each direction.
    megabits_per_second: f64,
    /// Size of a HOPR packet on the wire.
    packet_size: u64,
    /// Threads of the CPU pool packet processing runs on.
    cpu_threads: u64,
    /// First failure seen while relaying, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    first_error: Option<String>,
}

impl From<RelayBenchmarkReport> for RelayBenchmarkResult {
    fn from(report: RelayBenchmarkReport) -> Self {
        let bytes_per_second = report.bytes_per_second();
        Self {
            relayed_packets: report.relayed as u64,
            rejected_packets: report.rejected as u64,
            acknowledgements_sent: report.acks_sent as u64,
            acknowledgement_packets_sent: report.ack_packets_sent as u64,
            tickets_acknowledged: report.tickets_acknowledged as u64,
            preparation_ms: report.preparation.as_millis() as u64,
            duration_ms: report.duration.as_millis() as u64,
            packets_per_second: report.packets_per_second(),
            bytes_per_second,
            megabits_per_second: bytes_per_second * 8.0 / 1_000_000.0,
            packet_size: HoprPacket::SIZE as u64,
            cpu_threads: rayon::current_num_threads() as u64,
            first_error: report.first_error,
        }
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Latest relay benchmark run; at most one runs at a time.
#[derive(Debug, Default)]
pub(crate) struct RelayBenchmark(parking_lot::Mutex<RelayBenchmarkResponse>);

impl RelayBenchmark {
    fn status(&self) -> RelayBenchmarkResponse {
        self.0.lock().clone()
    }

    /// Returns `None` if a run is already in progress.
    fn try_start(&self, cfg: RelayBenchmarkConfig) -> Option<RelayBenchmarkResponse> {
        let mut status = self.0.lock();
        if status.state.is_running() {
            return None;
        }
        *status = RelayBenchmarkResponse {
            state: RelayBenchmarkState::Preparing,
            packets: Some(cfg.packets as u32),
            concurrency: Some(cfg.concurrency as u32),
            started_at: Some(unix_now()),
            ..Default::default()
        };
        Some(status.clone())
    }

    fn enter(&self, phase: RelayBenchmarkPhase) {
        self.0.lock().state = match phase {
            RelayBenchmarkPhase::Preparing => RelayBenchmarkState::Preparing,
            RelayBenchmarkPhase::Relaying => RelayBenchmarkState::Relaying,
        };
    }

    fn finish(&self, outcome: Result<RelayBenchmarkReport, RelayBenchmarkError>) {
        let mut status = self.0.lock();
        status.finished_at = Some(unix_now());
        match outcome {
            Ok(report) if report.relayed > 0 => {
                status.state = RelayBenchmarkState::Completed;
                status.result = Some(report.into());
            }
            Ok(report) => {
                status.state = RelayBenchmarkState::Failed;
                status.error = Some(
                    report
                        .first_error
                        .unwrap_or_else(|| "no packet was relayed".into()),
                );
            }
            Err(error) => {
                status.state = RelayBenchmarkState::Failed;
                status.error = Some(error.to_string());
            }
        }
    }

    async fn run(self: Arc<Self>, cfg: RelayBenchmarkConfig) {
        tracing::info!(
            packets = cfg.packets,
            concurrency = cfg.concurrency,
            "starting relay benchmark"
        );
        let outcome = run_relay_benchmark(cfg, |phase| self.enter(phase)).await;
        match &outcome {
            Ok(report) => tracing::info!(
                relayed = report.relayed,
                rejected = report.rejected,
                duration_ms = report.duration.as_millis() as u64,
                packets_per_second = report.packets_per_second(),
                first_error = ?report.first_error,
                "relay benchmark finished"
            ),
            Err(error) => tracing::error!(%error, "relay benchmark failed"),
        }
        self.finish(outcome);
    }
}

/// Start a relay throughput benchmark.
#[utoipa::path(
    post,
    path = const_format::formatcp!("{BASE_PATH}/node/benchmark/relay"),
    description = "Start a relay throughput benchmark. Synthetic traffic routed through this node is prepared first, then relayed as fast as the node's CPU allows: SPHINX processing, ticket validation and re-issue, and acknowledgements in both directions. Network I/O is not measured. The benchmark shares the CPU pool with live traffic and slows it down while running. Poll the GET endpoint for the result.",
    request_body(
        content = RelayBenchmarkRequest,
        description = "Benchmark parameters, all optional. The body may be omitted.",
        content_type = "application/json",
    ),
    responses(
        (status = 202, description = "Benchmark started.", body = RelayBenchmarkResponse),
        (status = 400, description = "Invalid benchmark parameters.", body = ApiError),
        (status = 401, description = "Invalid authorization token.", body = ApiError),
        (status = 409, description = "A benchmark is already running.", body = ApiError),
    ),
    security(
        ("api_token" = []),
        ("bearer_token" = [])
    ),
    tag = "Node"
)]
pub(crate) async fn start_relay_benchmark<H: Send + Sync + 'static>(
    State(state): State<Arc<InternalState<H>>>,
    req: Option<Json<RelayBenchmarkRequest>>,
) -> impl IntoResponse {
    let req = req.map(|Json(req)| req).unwrap_or_default();
    let cfg = match req.into_config() {
        Ok(cfg) => cfg,
        Err(error) => return error.into_response(),
    };

    match state.relay_benchmark.try_start(cfg) {
        Some(status) => {
            tokio::spawn(state.relay_benchmark.clone().run(cfg));
            (StatusCode::ACCEPTED, Json(status)).into_response()
        }
        None => ApiErrorStatus::BenchmarkInProgress.into_response(),
    }
}

/// Get the state and result of the latest relay throughput benchmark.
#[utoipa::path(
    get,
    path = const_format::formatcp!("{BASE_PATH}/node/benchmark/relay"),
    description = "Get the state and result of the latest relay throughput benchmark",
    responses(
        (status = 200, description = "Latest relay benchmark.", body = RelayBenchmarkResponse),
        (status = 401, description = "Invalid authorization token.", body = ApiError),
    ),
    security(
        ("api_token" = []),
        ("bearer_token" = [])
    ),
    tag = "Node"
)]
pub(crate) async fn relay_benchmark<H: Send + Sync + 'static>(
    State(state): State<Arc<InternalState<H>>>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(state.relay_benchmark.status())).into_response()
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::Body,
        http::{Request, header::CONTENT_TYPE},
        routing::get,
    };
    use tower::ServiceExt;

    use super::*;
    use crate::testing::NoopNode;

    fn router(benchmark: Arc<RelayBenchmark>) -> Router {
        let state: Arc<InternalState<NoopNode>> = Arc::new(InternalState {
            version: "test-version".to_string(),
            hoprd_cfg: serde_json::json!({}),
            auth: Arc::new(crate::config::Auth::None),
            hopr: Arc::new(NoopNode),
            open_listeners: Arc::new(hopr_utils_session::ListenerJoinHandles::default()),
            default_listen_host: "127.0.0.1:0".parse().unwrap(),
            session_flow_control: Default::default(),
            relay_benchmark: benchmark,
        });
        Router::new()
            .route(
                &format!("{BASE_PATH}/node/benchmark/relay"),
                get(relay_benchmark::<NoopNode>).post(start_relay_benchmark::<NoopNode>),
            )
            .with_state(state)
    }

    fn post(body: serde_json::Value) -> anyhow::Result<Request<Body>> {
        Ok(Request::post(format!("{BASE_PATH}/node/benchmark/relay"))
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))?)
    }

    async fn status(app: &Router) -> anyhow::Result<serde_json::Value> {
        let resp = app
            .clone()
            .oneshot(Request::get(format!("{BASE_PATH}/node/benchmark/relay")).body(Body::empty())?)
            .await?;
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    #[test]
    fn an_empty_request_should_use_the_defaults() -> anyhow::Result<()> {
        let cfg = RelayBenchmarkRequest::default()
            .into_config()
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        assert_eq!(cfg.packets, DEFAULT_PACKETS as usize);
        assert_eq!(cfg.concurrency, RelayBenchmarkConfig::default_concurrency());
        Ok(())
    }

    #[test]
    fn a_second_run_should_not_start_while_one_is_running() {
        let benchmark = RelayBenchmark::default();
        let cfg = RelayBenchmarkConfig {
            packets: 1,
            concurrency: 1,
        };

        assert!(benchmark.try_start(cfg).is_some());
        assert!(benchmark.try_start(cfg).is_none());

        benchmark.finish(Ok(RelayBenchmarkReport {
            relayed: 1,
            ..Default::default()
        }));
        assert!(benchmark.try_start(cfg).is_some());
    }

    #[test]
    fn a_run_that_relays_nothing_should_fail() {
        let benchmark = RelayBenchmark::default();
        benchmark.finish(Ok(RelayBenchmarkReport {
            packets: 5,
            rejected: 5,
            first_error: Some("bad ticket".into()),
            ..Default::default()
        }));

        let status = benchmark.status();
        assert_eq!(status.state, RelayBenchmarkState::Failed);
        assert_eq!(status.error.as_deref(), Some("bad ticket"));
        assert!(status.result.is_none());
    }

    #[tokio::test]
    async fn status_should_be_idle_before_any_run() -> anyhow::Result<()> {
        let body = status(&router(Arc::default())).await?;
        assert_eq!(body, serde_json::json!({ "state": "idle" }));
        Ok(())
    }

    #[tokio::test]
    async fn out_of_range_parameters_should_be_rejected() -> anyhow::Result<()> {
        let app = router(Arc::default());
        for body in [
            serde_json::json!({ "packets": 0 }),
            serde_json::json!({ "packets": MAX_PACKETS + 1 }),
            serde_json::json!({ "concurrency": 0 }),
        ] {
            let resp = app.clone().oneshot(post(body)?).await?;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }
        assert_eq!(status(&app).await?["state"], "idle");
        Ok(())
    }

    #[tokio::test]
    async fn starting_while_running_should_conflict() -> anyhow::Result<()> {
        let benchmark = Arc::new(RelayBenchmark::default());
        benchmark.enter(RelayBenchmarkPhase::Relaying);
        let app = router(benchmark);

        let with_body = app.clone().oneshot(post(serde_json::json!({}))?).await?;
        assert_eq!(with_body.status(), StatusCode::CONFLICT);

        let without_body = app
            .oneshot(
                Request::post(format!("{BASE_PATH}/node/benchmark/relay")).body(Body::empty())?,
            )
            .await?;
        assert_eq!(without_body.status(), StatusCode::CONFLICT);
        Ok(())
    }

    #[tokio::test]
    async fn a_started_run_should_complete_with_a_result() -> anyhow::Result<()> {
        let app = router(Arc::default());

        let resp = app
            .clone()
            .oneshot(post(
                serde_json::json!({ "packets": 25, "concurrency": 4 }),
            )?)
            .await?;
        assert_eq!(resp.status(), StatusCode::ACCEPTED);

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
        let body = loop {
            let body = status(&app).await?;
            if !matches!(body["state"].as_str(), Some("preparing" | "relaying")) {
                break body;
            }
            anyhow::ensure!(
                std::time::Instant::now() < deadline,
                "benchmark did not finish"
            );
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        };

        assert_eq!(body["state"], "completed", "{body}");
        assert_eq!(body["packets"], 25);
        assert_eq!(body["result"]["relayedPackets"], 25);
        assert_eq!(body["result"]["rejectedPackets"], 0);
        assert_eq!(body["result"]["ticketsAcknowledged"], 25);
        assert!(body["result"]["packetsPerSecond"].as_f64() > Some(0.0));
        Ok(())
    }
}
