use std::sync::Arc;

use axum::{
    extract::{Json, Query, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use hopr_lib::api::{
    node::{
        HasChainApi, HasTicketManagement, IncentiveChannelOperations, IncentiveRedeemOperations,
    },
    tickets::{ChannelStats, TicketManagement},
    types::{
        crypto::types::Hash,
        internal::prelude::ChannelStatus,
        primitive::prelude::{Address, HoprBalance},
    },
};
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{ApiError, ApiErrorStatus, BASE_PATH, InternalState};

#[serde_as]
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
        "amount": "100",
        "channelEpoch": 1,
        "channelId": "0x04efc1481d3f106b88527b3844ba40042b823218a9cd29d1aa11c2c2ef8f538f",
        "index": 0,
        "indexOffset": 1,
        "signature": "0xe445fcf4e90d25fe3c9199ccfaff85e23ecce8773304d85e7120f1f38787f2329822470487a37f1b5408c8c0b73e874ee9f7594a632713b6096e616857999891",
        "winProb": "1"
    }))]
#[serde(rename_all = "camelCase")]
/// Represents a ticket in a channel.
pub(crate) struct ChannelTicket {
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String, example = "0x04efc1481d3f106b88527b3844ba40042b823218a9cd29d1aa11c2c2ef8f538f")]
    channel_id: Hash,
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String, example = "1.0 wxHOPR")]
    amount: HoprBalance,
    #[schema(example = 0)]
    index: u64,
    #[schema(example = "1")]
    win_prob: String,
    #[schema(example = 1)]
    channel_epoch: u32,
    #[schema(
        example = "0xe445fcf4e90d25fe3c9199ccfaff85e23ecce8773304d85e7120f1f38787f2329822470487a37f1b5408c8c0b73e874ee9f7594a632713b6096e616857999891"
    )]
    signature: String,
}

#[serde_as]
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
        "winningCount": 0,
        "neglectedValue": "0 wxHOPR",
        "redeemedValue": "1000 wxHOPR",
        "rejectedValue": "0 wxHOPR",
        "unredeemedValue": "2000 wxHOPR",
    }))]
#[serde(rename_all = "camelCase")]
/// Received tickets statistics.
pub(crate) struct NodeTicketStatisticsResponse {
    #[schema(example = 0)]
    winning_count: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String, example = "20 wxHOPR")]
    unredeemed_value: HoprBalance,
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String,example = "0 wxHOPR")]
    neglected_value: HoprBalance,
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String, example = "0 wHOPR")]
    rejected_value: HoprBalance,
}

impl From<ChannelStats> for NodeTicketStatisticsResponse {
    fn from(value: ChannelStats) -> Self {
        Self {
            winning_count: value.winning_tickets as u64,
            unredeemed_value: value.unredeemed_value,
            neglected_value: value.neglected_value,
            rejected_value: value.rejected_value,
        }
    }
}

#[serde_as]
#[derive(Debug, Default, Clone, Deserialize, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
/// Query parameters for scoping ticket statistics.
pub(crate) struct TicketStatisticsQuery {
    /// On-chain address of the counterparty whose incoming channel to report on.
    /// If omitted, statistics are aggregated across every channel.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[param(value_type = Option<String>, example = "0x188c4462b75e46f0c7262d7f48d182447b93a93c")]
    address: Option<Address>,
}

/// Returns current complete statistics on tickets.
///
/// The counterparty scope mirrors `POST /tickets/redeem`: an `address` names the incoming
/// channel from that counterparty, which for a relay is what separates one direction of
/// traffic from the other. Aggregated statistics hide that, because a relay's two directions
/// are two different channels earning independently.
#[utoipa::path(
        get,
        path = const_format::formatcp!("{BASE_PATH}/tickets/statistics"),
        description = "Returns current complete statistics on tickets. When a counterparty address is given, only the incoming channel from that counterparty is reported.",
        params(TicketStatisticsQuery),
        responses(
            (status = 200, description = "Tickets statistics fetched successfully. Check schema for description of every field in the statistics.", body = NodeTicketStatisticsResponse),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 404, description = "Channel with the given counterparty not found.", body = ApiError),
            (status = 422, description = "Unknown failure", body = ApiError)
        ),
        security(
            ("api_token" = []),
            ("bearer_token" = [])
        ),
        tag = "Tickets"
    )]
pub(super) async fn show_ticket_statistics<
    H: HasChainApi<ChainError = hopr_lib::errors::HoprLibError>
        + HasTicketManagement
        + Send
        + Sync
        + 'static,
>(
    State(state): State<Arc<InternalState<H>>>,
    Query(query): Query<TicketStatisticsQuery>,
) -> impl IntoResponse {
    let hopr = state.hopr.clone();

    // Both arms end in `ticket_stats`; `ticket_statistics()` is its `None` case. They are kept
    // apart because the compound error type of the unscoped call does not match the ticket
    // manager's own, so there is no single `Result` to match on.
    let stats = match query.address {
        Some(address) => {
            // Resolve the incoming channel from the counterparty (counterparty → me), the same
            // way `redeem_tickets` does.
            let me = hopr.identity().node_address;
            let channel_id = match hopr.channel(address, me) {
                Ok(Some(ch)) if ch.status != ChannelStatus::Closed => *ch.get_id(),
                Ok(_) => {
                    return (StatusCode::NOT_FOUND, ApiErrorStatus::ChannelNotFound)
                        .into_response();
                }
                Err(e) => {
                    return (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        ApiErrorStatus::UnknownFailure(e.to_string()),
                    )
                        .into_response();
                }
            };
            hopr.ticket_management()
                .ticket_stats(Some(&channel_id))
                .map_err(|e| e.to_string())
        }
        None => hopr.ticket_statistics().map_err(|e| e.to_string()),
    };

    match stats {
        Ok(stats) => (
            StatusCode::OK,
            Json(NodeTicketStatisticsResponse::from(stats)),
        )
            .into_response(),
        Err(e) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ApiErrorStatus::UnknownFailure(e),
        )
            .into_response(),
    }
}

#[serde_as]
#[derive(Debug, Default, Clone, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({
    "address": "0x188c4462b75e46f0c7262d7f48d182447b93a93c"
}))]
#[serde(rename_all = "camelCase")]
/// Request body for ticket redemption with optional fields.
pub(crate) struct RedeemTicketsRequest {
    /// On-chain address of the counterparty whose incoming channel tickets to redeem.
    /// If omitted, tickets in all channels are redeemed.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[schema(value_type = Option<String>, example = "0x188c4462b75e46f0c7262d7f48d182447b93a93c")]
    address: Option<Address>,
}

/// Starts redeeming tickets.
///
/// When an `address` is specified, only tickets in the incoming channel from that
/// counterparty are redeemed. When omitted, tickets in all channels are redeemed.
///
/// **WARNING:** Redeeming many tickets can incur significant transaction costs.
#[utoipa::path(
        post,
        path = const_format::formatcp!("{BASE_PATH}/tickets/redeem"),
        description = "Starts redeeming tickets. When a counterparty address is specified, only tickets from that counterparty are redeemed.",
        request_body(
            content = RedeemTicketsRequest,
            description = "Optional counterparty address to scope ticket redemption.",
            content_type = "application/json",
        ),
        responses(
            (status = 202, description = "Ticket redemption started successfully."),
            (status = 400, description = "Invalid request body or malformed JSON.", body = ApiError),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 404, description = "Channel with counterparty not found.", body = ApiError),
            (status = 422, description = "Unknown failure", body = ApiError),
        ),
        security(
            ("api_token" = []),
            ("bearer_token" = [])
        ),
        tag = "Tickets"
    )]
pub(super) async fn redeem_tickets<
    H: HasChainApi<ChainError = hopr_lib::errors::HoprLibError>
        + HasTicketManagement
        + Send
        + Sync
        + 'static,
>(
    State(state): State<Arc<InternalState<H>>>,
    Json(req): Json<RedeemTicketsRequest>,
) -> impl IntoResponse {
    let hopr = state.hopr.clone();

    match req.address {
        Some(address) => {
            // Resolve the incoming channel from the counterparty (counterparty → me).
            let me = hopr.identity().node_address;
            let channel_id = match hopr.channel(address, me) {
                Ok(Some(ch)) if ch.status != ChannelStatus::Closed => *ch.get_id(),
                Ok(_) => {
                    return (StatusCode::NOT_FOUND, ApiErrorStatus::ChannelNotFound)
                        .into_response();
                }
                Err(e) => {
                    return (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        ApiErrorStatus::UnknownFailure(e.to_string()),
                    )
                        .into_response();
                }
            };

            tokio::spawn(async move {
                match hopr.redeem_tickets_with_counterparty(address, 0).await {
                    Ok(_) => {
                        tracing::info!(%channel_id, "tickets in channel redeemed on API request");
                    }
                    Err(error) => {
                        tracing::error!(%error, %channel_id, "failed to redeem tickets in channel on API request");
                    }
                }
            });

            (StatusCode::ACCEPTED, "").into_response()
        }
        None => {
            tokio::spawn(async move {
                match hopr.redeem_all_tickets(0).await {
                    Ok(_) => {
                        tracing::info!("all tickets redeemed on API request");
                    }
                    Err(error) => {
                        tracing::error!(%error, "failed to redeem all tickets on API request");
                    }
                }
            });

            (StatusCode::ACCEPTED, "").into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticket_statistics_response_should_serialize_correctly() {
        let stats = NodeTicketStatisticsResponse {
            winning_count: 5,
            unredeemed_value: "20 wxHOPR".parse().unwrap(),
            neglected_value: "0 wxHOPR".parse().unwrap(),
            rejected_value: "0 wxHOPR".parse().unwrap(),
        };

        let json = serde_json::to_value(&stats).unwrap();
        assert_eq!(json["winningCount"], 5);
        assert_eq!(json["unredeemedValue"], "20 wxHOPR");
        assert_eq!(json["neglectedValue"], "0 wxHOPR");
        assert_eq!(json["rejectedValue"], "0 wxHOPR");
    }

    #[test]
    fn redeem_tickets_request_should_deserialize_with_address() {
        let json = serde_json::json!({
            "address": "0x188c4462b75e46f0c7262d7f48d182447b93a93c"
        });

        let req: RedeemTicketsRequest = serde_json::from_value(json).unwrap();
        assert!(req.address.is_some());
    }

    #[test]
    fn redeem_tickets_request_should_deserialize_without_address() {
        let json = serde_json::json!({});
        let req: RedeemTicketsRequest = serde_json::from_value(json).unwrap();
        assert!(req.address.is_none());
    }

    #[test]
    fn channel_stats_should_convert_to_response() {
        let stats = ChannelStats {
            winning_tickets: 10,
            unredeemed_value: "100 wxHOPR".parse().unwrap(),
            neglected_value: "5 wxHOPR".parse().unwrap(),
            rejected_value: "1 wxHOPR".parse().unwrap(),
        };

        let response = NodeTicketStatisticsResponse::from(stats);
        assert_eq!(response.winning_count, 10);
        assert_eq!(response.unredeemed_value, "100 wxHOPR".parse().unwrap());
        assert_eq!(response.neglected_value, "5 wxHOPR".parse().unwrap());
        assert_eq!(response.rejected_value, "1 wxHOPR".parse().unwrap());
    }

    #[test]
    fn redeem_tickets_request_default_should_have_no_address() {
        let req = RedeemTicketsRequest::default();
        assert!(req.address.is_none());
    }

    #[test]
    fn redeem_tickets_request_should_reject_invalid_address() {
        let json = serde_json::json!({ "address": "not-an-address" });
        assert!(serde_json::from_value::<RedeemTicketsRequest>(json).is_err());
    }

    #[test]
    fn channel_ticket_should_serialize_correctly() {
        let ticket = ChannelTicket {
            channel_id: Hash::default(),
            amount: "1.0 wxHOPR".parse().unwrap(),
            index: 7,
            win_prob: "1".to_string(),
            channel_epoch: 2,
            signature: "0xdeadbeef".to_string(),
        };
        let json = serde_json::to_value(&ticket).unwrap();
        assert_eq!(json["amount"], "1 wxHOPR");
        assert_eq!(json["index"], 7);
        assert_eq!(json["winProb"], "1");
        assert_eq!(json["channelEpoch"], 2);
        assert_eq!(json["signature"], "0xdeadbeef");
        assert!(json.get("channelId").is_some());
    }

    // ── Endpoint-level tests ───────────────────────────────────────────────

    use std::sync::Arc;

    use anyhow::Context;
    use axum::{Router, body::Body, http::Request, routing::get};
    use hopr_lib::api::types::internal::prelude::{ChannelEntry, generate_channel_id};
    use tower::ServiceExt;

    use crate::testing::MockChainNode;

    const COUNTERPARTY: &str = "0x188c4462b75e46f0c7262d7f48d182447b93a93c";

    /// A channel from `src` to `dst`.
    ///
    /// `ChannelEntry` derives its id from the pair, which is both what the stub keys on and
    /// what the handler's counterparty→me lookup recomputes — so the direction here is what
    /// decides whether a scoped request finds this channel.
    fn channel(src: Address, dst: Address, status: ChannelStatus) -> anyhow::Result<ChannelEntry> {
        Ok(ChannelEntry::builder()
            .between(src, dst)
            .balance("10 wxHOPR".parse()?)
            .status(status)
            .build()?)
    }

    async fn get_statistics(
        node: MockChainNode,
        uri: &str,
    ) -> anyhow::Result<(StatusCode, serde_json::Value)> {
        let resp = tickets_router(node)
            .oneshot(Request::get(uri).body(Body::empty())?)
            .await?;
        let status = resp.status();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
        Ok((
            status,
            serde_json::from_slice(&body).context("response body is not JSON")?,
        ))
    }

    fn tickets_router(node: MockChainNode) -> Router {
        let state = Arc::new(crate::InternalState {
            version: "test-version".to_string(),
            hoprd_cfg: serde_json::Value::Null,
            auth: Arc::new(crate::config::Auth::Token("test".into())),
            hopr: Arc::new(node),
            open_listeners: Arc::new(hopr_utils_session::ListenerJoinHandles::default()),
            default_listen_host: "127.0.0.1:0".parse().unwrap(),
            session_flow_control: Default::default(),
        });

        Router::new()
            .route(
                "/tickets/statistics",
                get(show_ticket_statistics::<MockChainNode>),
            )
            .with_state(state)
    }

    #[tokio::test]
    async fn ticket_statistics_unscoped_should_report_the_aggregate() -> anyhow::Result<()> {
        let node = MockChainNode::random().with_aggregate_ticket_stats(ChannelStats {
            winning_tickets: 99,
            unredeemed_value: "99 wxHOPR".parse()?,
            ..Default::default()
        });
        let calls = node.ticket_stats_calls();

        let (status, json) = get_statistics(node, "/tickets/statistics").await?;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["winningCount"], 99);
        assert_eq!(json["unredeemedValue"], "99 wxHOPR");
        // Symmetric to the scoped case below: an unscoped request has to ask for the
        // aggregate, not for some channel.
        assert_eq!(calls.snapshot(), vec![None]);

        Ok(())
    }

    #[tokio::test]
    async fn ticket_statistics_scoped_to_a_counterparty_should_report_only_that_channel()
    -> anyhow::Result<()> {
        let node = MockChainNode::random();
        let me = node.identity.node_address;
        let counterparty: Address = COUNTERPARTY.parse()?;
        let incoming = channel(counterparty, me, ChannelStatus::Open)?;

        let node = node
            .with_channel(incoming)
            .with_aggregate_ticket_stats(ChannelStats {
                winning_tickets: 99,
                unredeemed_value: "99 wxHOPR".parse()?,
                ..Default::default()
            })
            .with_channel_ticket_stats(
                *incoming.get_id(),
                ChannelStats {
                    winning_tickets: 7,
                    unredeemed_value: "3 wxHOPR".parse()?,
                    ..Default::default()
                },
            );

        let (status, json) =
            get_statistics(node, &format!("/tickets/statistics?address={COUNTERPARTY}")).await?;

        assert_eq!(status, StatusCode::OK);
        // The channel's own numbers, not the aggregate. Dropping the scope at the
        // `ticket_stats` call would report 99 here and still return 200.
        assert_eq!(json["winningCount"], 7);
        assert_eq!(json["unredeemedValue"], "3 wxHOPR");

        Ok(())
    }

    #[tokio::test]
    async fn ticket_statistics_scoped_to_a_counterparty_should_use_the_incoming_channel()
    -> anyhow::Result<()> {
        let node = MockChainNode::random();
        let me = node.identity.node_address;
        let counterparty: Address = COUNTERPARTY.parse()?;
        let incoming = channel(counterparty, me, ChannelStatus::Open)?;
        let outgoing = channel(me, counterparty, ChannelStatus::Open)?;

        let node = node
            .with_channel(incoming)
            .with_channel(outgoing)
            .with_channel_ticket_stats(
                *incoming.get_id(),
                ChannelStats {
                    winning_tickets: 7,
                    ..Default::default()
                },
            )
            .with_channel_ticket_stats(
                *outgoing.get_id(),
                ChannelStats {
                    winning_tickets: 42,
                    ..Default::default()
                },
            );
        let calls = node.ticket_stats_calls();

        let (status, json) =
            get_statistics(node, &format!("/tickets/statistics?address={COUNTERPARTY}")).await?;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["winningCount"], 7);
        // Both directions exist, so a reversed lookup would also return 200 — only the
        // forwarded id says which of the two channels was actually read. For a relay these
        // are two independently earning channels, which is the whole point of the scope.
        assert_eq!(
            calls.snapshot(),
            vec![Some(generate_channel_id(&counterparty, &me))]
        );

        Ok(())
    }

    #[tokio::test]
    async fn ticket_statistics_scoped_to_a_pending_to_close_channel_should_report_it()
    -> anyhow::Result<()> {
        // Tickets stay redeemable while a channel is closing, so its statistics have to stay
        // reachable. This is the other half of the handler's `!= Closed` guard.
        let node = MockChainNode::random();
        let me = node.identity.node_address;
        let counterparty: Address = COUNTERPARTY.parse()?;
        let pending = channel(
            counterparty,
            me,
            ChannelStatus::PendingToClose(std::time::SystemTime::now()),
        )?;

        let node = node.with_channel(pending).with_channel_ticket_stats(
            *pending.get_id(),
            ChannelStats {
                winning_tickets: 7,
                ..Default::default()
            },
        );

        let (status, json) =
            get_statistics(node, &format!("/tickets/statistics?address={COUNTERPARTY}")).await?;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["winningCount"], 7);

        Ok(())
    }

    #[tokio::test]
    async fn ticket_statistics_scoped_to_a_closed_channel_should_404() -> anyhow::Result<()> {
        let node = MockChainNode::random();
        let me = node.identity.node_address;
        let counterparty: Address = COUNTERPARTY.parse()?;
        let closed = channel(counterparty, me, ChannelStatus::Closed)?;

        let node = node.with_channel(closed).with_channel_ticket_stats(
            *closed.get_id(),
            ChannelStats {
                winning_tickets: 7,
                ..Default::default()
            },
        );
        let calls = node.ticket_stats_calls();

        let (status, json) =
            get_statistics(node, &format!("/tickets/statistics?address={COUNTERPARTY}")).await?;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(json["status"], "CHANNEL_NOT_FOUND");
        // The channel does have statistics in the stub; a handler that dropped the `!= Closed`
        // guard would happily report them instead of 404-ing.
        assert!(calls.snapshot().is_empty());

        Ok(())
    }

    #[test]
    fn ticket_statistics_query_default_should_have_no_address() {
        let query = TicketStatisticsQuery::default();
        assert!(query.address.is_none());
    }

    #[tokio::test]
    async fn ticket_statistics_scoped_to_a_counterparty_without_a_channel_should_404()
    -> anyhow::Result<()> {
        // No channel is planted, so this exercises the whole scoped path — the address
        // parses, the counterparty→me lookup runs, and finding nothing is reported as such
        // rather than silently falling back to the aggregate.
        let node = MockChainNode::random().with_aggregate_ticket_stats(ChannelStats {
            winning_tickets: 99,
            ..Default::default()
        });
        let calls = node.ticket_stats_calls();

        let (status, json) =
            get_statistics(node, &format!("/tickets/statistics?address={COUNTERPARTY}")).await?;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(json["status"], "CHANNEL_NOT_FOUND");
        assert!(calls.snapshot().is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn ticket_statistics_should_reject_a_malformed_counterparty() -> anyhow::Result<()> {
        let node = MockChainNode::random();

        let resp = tickets_router(node)
            .oneshot(
                Request::get("/tickets/statistics?address=not-an-address").body(Body::empty())?,
            )
            .await?;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }
}
