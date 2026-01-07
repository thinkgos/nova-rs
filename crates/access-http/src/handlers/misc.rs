use axum::{Json, Router, extract::Query, response::IntoResponse, routing};
use tracing::debug;

use crate::app_error::AppError;
use access_http_types::misc::{HealthyReply, HealthyRequest};

#[derive(utoipa::OpenApi)]
#[openapi(paths(healthy), components(schemas(HealthyReply)))]
pub(crate) struct MiscApi;

pub fn route_v1() -> Router {
    Router::new().nest(
        "/v1",
        Router::new().route("/public/healthy", routing::get(healthy)),
    )
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
pub async fn healthy(req: Query<HealthyRequest>) -> Result<impl IntoResponse, AppError> {
    debug!("healthy called");
    if req.dummy.is_some() {
        Err(AppError::AnyhowError(anyhow::anyhow!("哈哈")))
    } else {
        Ok(Json(HealthyReply {
            status: "running".to_string(),
        }))
    }
}
