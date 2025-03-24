use haemolacriaa::router::app;
use leptos::prelude::*;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use tokio::net::TcpListener;
use tokio::signal;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // rest of the configuration is in haemolacriaa::router
    #[cfg(test)]
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    #[cfg(not(test))]
    let conf = get_configuration(None).unwrap();

    let addr = conf.leptos_options.site_addr;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        //.with_max_level(Level::INFO)
        .init();

    // build the postgres connection pool
    /*
    let user = std::env::var("PG_USER")?;
    let password = std::env::var("PG_PASSWORD")?;
    let host = std::env::var("PG_HOST")?;
    let port = std::env::var("PG_PORT")?
        .parse::<u16>()?;
    let db = std::env::var("PG_DATABASE")?;

    // setup postgres options
    let db_options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&user)
        .password(&password)
        .database(&db)
        .ssl_mode(PgSslMode::Require);

    // setup postgres connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(db_options)
        .await?;

    tracing::info!("Connected to postgres");

    // run migrations
    sqlx::migrate!()
        .run(&db_pool)
        .await?;
    */

    let app = app().await;

    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Listening on http://{addr}");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .inspect_err(|_| tracing::error!("failed to install ctrl-c handler"))
            .unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .inspect_err(|_| tracing::error!("failed to install signal handler"))
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => { tracing::info!("recieved ctrl-c") },
        () = terminate => {},
    }
}
