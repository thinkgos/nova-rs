use axum::{
    Router,
    extract::{Query, State},
    routing,
};

use crate::error::{AppError, Result};
use access_http_types::misc::{HealthyReply, HealthyRequest};
use readiness::app_state::AppState;

#[derive(utoipa::OpenApi)]
#[openapi(paths(healthy), components(schemas(HealthyReply)))]
pub(crate) struct MiscApi;

pub fn route_v1() -> impl Into<Router<AppState>> {
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
pub async fn healthy(
    State(state): State<AppState>,
    req: Query<HealthyRequest>,
) -> Result<HealthyReply> {
    if req.dummy.is_some() {
        Err(AppError::AnyhowError(anyhow::anyhow!("哈哈")))
    } else {
        Ok(HealthyReply {
            db: state.db.ping().await.is_ok(),
        }
        .into())
    }
}
