use axum::{Json, Router, response::IntoResponse, routing};
use serde::{Deserialize, Serialize};

#[derive(utoipa::OpenApi)]
#[openapi(paths(login), components(schemas(LoginReply)))]
pub(crate) struct PassportApi;

pub fn route_v1() -> Router {
    Router::new().nest(
        "/v1",
        Router::new().route("/passport/login", routing::post(login)),
    )
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    /// 会话id
    session_id: String,
    /// 验证码id
    captcha_id: String,
    /// 验证码
    captcha: String,
    /// 用户名
    username: String,
    /// 密码
    password: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoginReply {
    /// 访问令牌
    access_token: String,
    /// 刷新令牌
    refresh_token: String,
    /// 过期时间(时间戳)
    expires_at: i64,
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
