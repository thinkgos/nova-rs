use std::io;
// use std::net::SocketAddr;

use access_http::route;
use configuration::Configuration;
use nova::telemetry;

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
    axum::serve(
        listener, app,
        // app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
