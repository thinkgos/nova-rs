use axum::http::HeaderName;

pub const X_TRACE_ID_HEADER: &str = "x-trace-id";
pub const X_TRACE_ID: HeaderName = HeaderName::from_static(X_TRACE_ID_HEADER);
pub const X_TRACE_ID_DEFAULT: &str = "SetLayerMakeTraceIdFailure";
