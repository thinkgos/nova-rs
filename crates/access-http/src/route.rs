use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    extract::MatchedPath,
    http::{self, Method, Request},
};

use tower::ServiceBuilder;
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::{self, AllowOrigin, CorsLayer},
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::{error, info, info_span};
use ulid::Ulid;

use crate::app_error::AppError;
use crate::handlers::{misc, passport};
use crate::openapi;
use types::constant;

const APP_NAME: &str = "nova";

pub fn route() -> Router {
    let set_trace_id_layer = SetRequestIdLayer::new(constant::X_TRACE_ID, MakeRequestUlid);
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &http::Request<_>| {
            let trace_id = request
                .headers()
                .get(constant::X_TRACE_ID_HEADER)
                .and_then(|v| v.to_str().ok())
                .unwrap_or(constant::X_TRACE_ID_DEFAULT);
            info_span!(
                "http",
                app=APP_NAME,
                trace_id=%trace_id,
            )
        })
        .on_request(|request: &http::Request<_>, _span: &tracing::Span| {
            let route = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);
            let path = request.uri().path().to_string();
            info!(
                method=%request.method(),
                route=%route.unwrap_or(&path),
                path=%path,
                version=?request.version(),
                "request",
            );
        })
        .on_response(
            |response: &http::Response<_>, latency: Duration, _span: &tracing::Span| {
                let status = response.status();
                if status.is_success() {
                    info!(
                        ?latency,
                        status=%status.as_u16(),
                        "response",
                    );
                } else {
                    let err = response.extensions().get::<Arc<AppError>>().cloned();
                    if let Some(e) = err {
                        error!(
                            ?latency,
                            status=%status.as_u16(),
                            error=%e,
                            "response",
                        );
                    } else {
                        error!(
                            ?latency,
                            status=%status.as_u16(),
                            "response",
                        );
                    }
                }
            },
        )
        .on_failure(
            |err: ServerErrorsFailureClass, latency: Duration, _span: &tracing::Span| {
                error!(
                    ?latency,
                    error=?err,
                    "server failure",
                );
            },
        );

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
        ])
        .allow_headers(cors::Any)
        // .allow_credentials(true)
        .max_age(Duration::from_hours(12));

    Router::new()
        .merge(openapi::route())
        .nest(
            "/api",
            Router::new()
                .merge(misc::route_v1())
                .merge(passport::route_v1()),
        )
        .layer(
            ServiceBuilder::new()
                .layer(set_trace_id_layer)
                .layer(trace_layer)
                .layer(cors_layer)
                .layer(PropagateRequestIdLayer::new(constant::X_TRACE_ID)),
        )
}

/// A [`MakeRequestId`] that generates `Ulid`s.
#[derive(Clone, Copy, Default)]
pub struct MakeRequestUlid;

impl MakeRequestId for MakeRequestUlid {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let request_id = Ulid::new().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}
