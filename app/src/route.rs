use axum::Router;
use handlers::{misc, passport, swagger};

pub fn route() -> Router {
    Router::new().merge(swagger::route()).nest(
        "/api",
        Router::new()
            .merge(misc::route_v1())
            .merge(passport::route_v1()),
    )
}
