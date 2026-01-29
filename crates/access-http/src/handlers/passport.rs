use axum::{Json, Router, extract::State, routing};

use crate::error::Result;
use access_http_types::passport::{LoginReply, LoginRequest};
use readiness::app_state::AppState;

#[derive(utoipa::OpenApi)]
#[openapi(paths(login), components(schemas(LoginReply)))]
pub(crate) struct PassportApi;

pub fn route_v1() -> impl Into<Router<AppState>> {
    Router::new().nest(
        "/v1",
        Router::new().route("/passport/login", routing::post(login)),
    )
}

/// 账号登陆
#[utoipa::path(
    tags = ["Passport"],
    post,
    operation_id = "v1_login",
    context_path = "v1",
    path = "/passport/login",
    request_body = LoginRequest,
    responses(
        (status = StatusCode::OK, body = inline(LoginReply))
    ),
)]
pub async fn login(State(_state): State<AppState>, _req: Json<LoginRequest>) -> Result<LoginReply> {
    Ok(LoginReply {
        access_token: "token".to_string(),
        refresh_token: "refresh_token".to_string(),
        expires_at: chrono::Utc::now().timestamp(),
    }
    .into())
}
