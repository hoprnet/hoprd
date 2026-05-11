use std::str::FromStr;

use axum::{
    extract::{OriginalUri, Request, State},
    http::{
        HeaderMap,
        header::{AUTHORIZATION, HeaderName},
        status::StatusCode,
    },
    middleware::Next,
    response::IntoResponse,
};

use crate::{ApiErrorStatus, Auth, InternalState};

pub(crate) async fn authenticate<H: Send + Sync + 'static>(
    State(state): State<InternalState<H>>,
    _uri: OriginalUri,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let auth = state.auth.clone();

    let x_auth_header =
        HeaderName::from_str("x-auth-token").expect("Invalid header name: x-auth-token");

    let is_authorized = match auth.as_ref() {
        Auth::Token(expected_token) => headers.iter().any(|(n, v)| {
            let Ok(value) = v.to_str() else { return false };
            if AUTHORIZATION.eq(n) {
                value.split_once(' ').is_some_and(|(scheme, token)| {
                    scheme.eq_ignore_ascii_case("bearer") && token == expected_token
                })
            } else if x_auth_header.eq(n) {
                value == expected_token
            } else {
                false
            }
        }),
        Auth::None => true,
    };

    if !is_authorized {
        return (StatusCode::UNAUTHORIZED, ApiErrorStatus::Unauthorized).into_response();
    }

    // Go forward to the next middleware or request handler
    next.run(request).await
}
