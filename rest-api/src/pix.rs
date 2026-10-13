//! The PIX deposit pool's funding, from outside the node.
//!
//! A deposit pool never funds itself: the strategy's configured funding policy does, or an
//! operator does through these two endpoints — put wxHOPR in, and ask what is left.

use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use futures::future::BoxFuture;
use hopr_lib::api::types::primitive::prelude::HoprBalance;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

use crate::{ApiError, ApiErrorStatus, BASE_PATH, InternalState};

/// The deposit pool's funding, as the node's PIX strategy exposes it.
///
/// Implemented by hoprd over the pool it built the strategy with; absent on a node without a
/// PIX pool.
pub trait PixFunding: Send + Sync {
    /// Moves `amount` from the node's Safe into the pool's funding.
    fn fund(&self, amount: HoprBalance) -> BoxFuture<'_, Result<(), String>>;

    /// What the pool can pay deposits from right now.
    fn available_balance(&self) -> BoxFuture<'_, Result<HoprBalance, String>>;
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({ "available": "173.2 wxHOPR" }))]
#[serde(rename_all = "camelCase")]
/// What the PIX deposit pool can pay deposits from right now.
pub(crate) struct PixFundingResponse {
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String, example = "173.2 wxHOPR")]
    available: HoprBalance,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({ "funded": "218 wxHOPR" }))]
#[serde(rename_all = "camelCase")]
/// What a funding moved.
pub(crate) struct PixFundResponse {
    /// The amount moved from the Safe, gross of the pool's own fees.
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String, example = "218 wxHOPR")]
    funded: HoprBalance,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(example = json!({ "amount": "218 wxHOPR" }))]
#[serde(rename_all = "camelCase")]
/// Request body for funding the PIX deposit pool.
pub(crate) struct PixFundBodyRequest {
    #[serde_as(as = "DisplayFromStr")]
    #[schema(value_type = String, example = "218 wxHOPR")]
    amount: HoprBalance,
}

fn no_pool() -> axum::response::Response {
    (StatusCode::NOT_FOUND, ApiErrorStatus::PixPoolNotConfigured).into_response()
}

#[utoipa::path(
        get,
        path = const_format::formatcp!("{BASE_PATH}/pix/funding"),
        description = "Get what the PIX deposit pool can pay deposits from right now",
        responses(
            (status = 200, description = "The pool's available funding", body = PixFundingResponse),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 404, description = "This node runs no PIX deposit pool", body = ApiError),
            (status = 422, description = "Unknown failure", body = ApiError)
        ),
        security(
            ("api_token" = []),
            ("bearer_token" = [])
        ),
        tag = "PIX"
    )]
pub(super) async fn pool_funding<H: Send + Sync + 'static>(
    State(state): State<Arc<InternalState<H>>>,
) -> impl IntoResponse {
    let Some(funding) = state.pix_funding.as_ref() else {
        return no_pool();
    };
    match funding.available_balance().await {
        Ok(available) => (StatusCode::OK, Json(PixFundingResponse { available })).into_response(),
        Err(error) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ApiErrorStatus::UnknownFailure(error),
        )
            .into_response(),
    }
}

#[utoipa::path(
        post,
        path = const_format::formatcp!("{BASE_PATH}/pix/fund"),
        description = "Move wxHOPR from the node's Safe into the PIX deposit pool's funding. The pool never funds itself; this is the operator's lever, beside the strategy's optional funding policy.",
        request_body(content = PixFundBodyRequest, description = "How much to move, gross of the pool's own fees", content_type = "application/json"),
        responses(
            (status = 200, description = "The pool was funded; the amount shows under /pix/funding once the pool has settled it", body = PixFundResponse),
            (status = 400, description = "The amount is not a positive wxHOPR balance", body = ApiError),
            (status = 401, description = "Invalid authorization token.", body = ApiError),
            (status = 404, description = "This node runs no PIX deposit pool", body = ApiError),
            (status = 422, description = "Funding failed; nothing was moved", body = ApiError)
        ),
        security(
            ("api_token" = []),
            ("bearer_token" = [])
        ),
        tag = "PIX"
    )]
pub(super) async fn fund_pool<H: Send + Sync + 'static>(
    State(state): State<Arc<InternalState<H>>>,
    Json(req_data): Json<PixFundBodyRequest>,
) -> impl IntoResponse {
    let Some(funding) = state.pix_funding.as_ref() else {
        return no_pool();
    };
    if req_data.amount.is_zero() {
        return (
            StatusCode::BAD_REQUEST,
            ApiErrorStatus::InvalidInputDetail("amount must be a positive wxHOPR balance".into()),
        )
            .into_response();
    }
    if let Err(error) = funding.fund(req_data.amount).await {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            ApiErrorStatus::UnknownFailure(error),
        )
            .into_response();
    }
    (
        StatusCode::OK,
        Json(PixFundResponse {
            funded: req_data.amount,
        }),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::{get, post},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::testing::MockChainNode;

    /// A funding that answers with what it was told and records what it was asked.
    struct ScriptedFunding {
        available: Result<HoprBalance, String>,
        fund_outcome: Result<(), String>,
        funded: Mutex<Vec<HoprBalance>>,
    }

    impl ScriptedFunding {
        fn new(available: &str) -> anyhow::Result<Arc<Self>> {
            Ok(Arc::new(Self {
                available: Ok(available.parse()?),
                fund_outcome: Ok(()),
                funded: Mutex::new(Vec::new()),
            }))
        }
    }

    impl PixFunding for ScriptedFunding {
        fn fund(&self, amount: HoprBalance) -> BoxFuture<'_, Result<(), String>> {
            Box::pin(async move {
                self.funded.lock().unwrap().push(amount);
                self.fund_outcome.clone()
            })
        }

        fn available_balance(&self) -> BoxFuture<'_, Result<HoprBalance, String>> {
            Box::pin(async move { self.available.clone() })
        }
    }

    fn fund_request(body: &'static str) -> anyhow::Result<Request<Body>> {
        Ok(Request::builder()
            .method("POST")
            .uri("/pix/fund")
            .header("content-type", "application/json")
            .body(Body::from(body))?)
    }

    async fn json_body(response: axum::response::Response) -> anyhow::Result<serde_json::Value> {
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
        Ok(serde_json::from_slice(&body)?)
    }

    fn pix_router(pix_funding: Option<Arc<dyn PixFunding>>) -> Router {
        let state = Arc::new(crate::InternalState {
            version: "test-version".to_string(),
            hoprd_cfg: serde_json::Value::Null,
            auth: Arc::new(crate::config::Auth::Token("test".into())),
            hopr: Arc::new(MockChainNode::random()),
            open_listeners: Default::default(),
            default_listen_host: "127.0.0.1:0".parse().unwrap(),
            session_flow_control: Default::default(),
            pix_funding,
        });
        Router::new()
            .route("/pix/funding", get(pool_funding::<MockChainNode>))
            .route("/pix/fund", post(fund_pool::<MockChainNode>))
            .with_state(state)
    }

    #[tokio::test]
    async fn a_node_without_a_pool_says_so() -> anyhow::Result<()> {
        let response = pix_router(None)
            .oneshot(Request::builder().uri("/pix/funding").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let parsed = json_body(response).await?;
        assert_eq!(parsed["status"], "PIX_POOL_NOT_CONFIGURED");

        let response = pix_router(None)
            .oneshot(fund_request(r#"{"amount":"218 wxHOPR"}"#)?)
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "nor is it funded");
        Ok(())
    }

    #[tokio::test]
    async fn funding_is_read_and_moved_through_the_pool() -> anyhow::Result<()> {
        let scripted = ScriptedFunding::new("173.2 wxHOPR")?;
        let router = pix_router(Some(scripted.clone()));

        let response = router
            .clone()
            .oneshot(Request::builder().uri("/pix/funding").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(json_body(response).await?["available"], "173.2 wxHOPR");

        let response = router
            .oneshot(fund_request(r#"{"amount":"218 wxHOPR"}"#)?)
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(json_body(response).await?["funded"], "218 wxHOPR");
        assert_eq!(
            *scripted.funded.lock().unwrap(),
            vec!["218 wxHOPR".parse::<HoprBalance>()?]
        );
        Ok(())
    }

    /// Nothing reaches the pool for a zero amount.
    #[tokio::test]
    async fn a_zero_amount_is_refused_before_the_pool() -> anyhow::Result<()> {
        let scripted = ScriptedFunding::new("0 wxHOPR")?;
        let router = pix_router(Some(scripted.clone()));

        let response = router
            .clone()
            .oneshot(fund_request(r#"{"amount":"0 wxHOPR"}"#)?)
            .await?;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(json_body(response).await?["status"], "INVALID_INPUT");

        assert!(scripted.funded.lock().unwrap().is_empty());
        Ok(())
    }

    /// A pool that refuses is a failed funding, with the pool's reason.
    #[tokio::test]
    async fn a_refused_funding_fails_with_the_pools_reason() -> anyhow::Result<()> {
        let refusing = Arc::new(ScriptedFunding {
            available: Ok("1 wxHOPR".parse()?),
            fund_outcome: Err("shield in progress".into()),
            funded: Mutex::new(Vec::new()),
        });
        let response = pix_router(Some(refusing.clone()))
            .oneshot(fund_request(r#"{"amount":"218 wxHOPR"}"#)?)
            .await?;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(json_body(response).await?["error"], "shield in progress");
        Ok(())
    }
}
