use axum::{Json, Router, extract::Query, response::IntoResponse, routing};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(utoipa::OpenApi)]
#[openapi(paths(healthy), components(schemas(HealthyReply)))]
pub(crate) struct MiscApi;

pub fn route_v1() -> Router {
    Router::new().nest(
        "/v1",
        Router::new().route("/public/healthy", routing::get(healthy)),
    )
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct HealthyRequest {
    /// dummy参数
    dummy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthyReply {
    /// running status
    status: String,
}

/// 健康检查
#[utoipa::path(
    tags = ["Misc"],
    get,
    operation_id = "v1_healthy",
    context_path = "v1",
    path = "/public/healthy",
    params(HealthyRequest),
    responses(
        (status = StatusCode::OK, body = inline(HealthyReply))
    ),
)]
pub async fn healthy(_query: Query<HealthyRequest>) -> impl IntoResponse {
    debug!("healthy called");
    Json(HealthyReply {
        status: "running".to_string(),
    })
}
