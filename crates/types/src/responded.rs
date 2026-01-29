use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply<T> {
    pub code: usize,
    pub message: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub detail: String,
    pub data: T,
}

impl<T> From<T> for Reply<T>
where
    T: Serialize,
{
    fn from(value: T) -> Self {
        Reply {
            code: 200,
            message: "ok".to_string(),
            data: value,
            detail: String::new(),
        }
    }
}

impl Default for Reply<()> {
    fn default() -> Self {
        Reply {
            code: 200,
            message: "ok".to_string(),
            data: (),
            detail: String::new(),
        }
    }
}

impl<T> IntoResponse for Reply<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
