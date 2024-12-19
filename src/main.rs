use axum::Router;
use haemolacriaa::app::*;
use haemolacriaa::jwt;
use leptos::{config::LeptosOptions, prelude::provide_context, IntoView};
use leptos::{logging::log, prelude::get_configuration};
use leptos_axum::{file_and_error_handler, generate_route_list, LeptosRoutes};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // to build the postgres connection
    /*
    let user = std::env::var("PG_USER").expect("Failed to get postgres user!");
    let password = std::env::var("PG_PASSWORD").expect("Failed to get postgres password!");
    let host = std::env::var("PG_HOST").expect("Failed to get postgres host!");
    let port = std::env::var("PG_PORT")
        .expect("Failed to get postgres port!")
        .parse::<u16>()
        .ok()
        .expect("Failed to parse port as a u16!");
    let db = std::env::var("PG_DATABASE").expect("Failed to get postgres database!");

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
        .await
        .expect("Failed to connect to database");

    // run migrations
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Failed to run SQLx migrations!");
    */
    // build our application with a route
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
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
