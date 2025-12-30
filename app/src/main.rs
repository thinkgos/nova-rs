use std::io;

use app::route;
use app::telemetry;
use configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_subscriber("debug", io::stdout)?;

    let c = Configuration::load()?;
    let app = route::route();
    let addr = c.app.addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(
        addr = %format_args!("http://{}", addr),
        "Server is running"
    );
    axum::serve(listener, app).await?;
    Ok(())
}
