use axum::Router;
use haemolacriaa::app::*;
use leptos::{
    config::LeptosOptions,
    prelude::{get_configuration, provide_context},
};
use leptos_axum::{file_and_error_handler, generate_route_list, LeptosRoutes};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::time::Duration;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
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

    tracing::info!("Connecting to DB");

    // setup postgres connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(db_options)
        .await?;

    tracing::info!("Running migrations");

    // run migrations
    sqlx::migrate!()
        .run(&db_pool)
        .await?;
    */

    let app = Router::new()
        /*
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(db_pool.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        */
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(file_and_error_handler::<LeptosOptions, _>(shell))
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
