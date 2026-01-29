use std::io;
// use std::net::SocketAddr;

use access_http::route;
use nova::telemetry;
use readiness::app_state;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::try_init_subscriber("debug", io::stdout)?;
    configuration::try_init()?;

    let c = configuration::use_config();

    tracing::info!("{:?}", c);

    let state = app_state::AppState {
        db: app_state::new_database_connection(&c.database).await?,
    };

    let app = route::route(state);
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
