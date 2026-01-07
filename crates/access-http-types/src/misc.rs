use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct HealthyRequest {
    /// dummy参数
    pub dummy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthyReply {
    /// running status
    pub status: String,
}
