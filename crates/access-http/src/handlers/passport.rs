use axum::{Json, Router, response::IntoResponse, routing};

use access_http_types::passport::{LoginReply, LoginRequest};

#[derive(utoipa::OpenApi)]
#[openapi(paths(login), components(schemas(LoginReply)))]
pub(crate) struct PassportApi;

pub fn route_v1() -> Router {
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
pub async fn login(_req: Json<LoginRequest>) -> impl IntoResponse {
    Json(LoginReply {
        access_token: "token".to_string(),
        refresh_token: "refresh_token".to_string(),
        expires_at: chrono::Utc::now().timestamp(),
    })
}
