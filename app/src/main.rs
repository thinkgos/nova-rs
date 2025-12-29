use std::io;

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

use app::telemetry;
use configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_subscriber(telemetry::get_subscriber("nova", "debug", io::stdout));

    let c = Configuration::load()?;

    let app = Router::new()
        // .merge(swagger::config_router())
        .nest("/api", Router::new().route("/health", get(health_check)));

    let addr = c.app.addr();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server is running on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
           "message": "Server is running"
    }))
}
