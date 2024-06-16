#[cfg(feature = "ssr")] 
pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub leptos_options: LeptosOptions,
}

#[cfg(feature = "ssr")] 
#[tokio::main]
async fn main() {
    use axum::Router;
    use haemolacriaa::app::*;
    use haemolacriaa::fileserv::file_and_error_handler;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use haemolacriaa::song_db;
    use axum::routing::get;
    use std::time::Duration;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // set up postgres connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to database");

    // app state
    let state = AppState {
        db_pool,
        leptos_options,
    };

    // build our application with a route
    let app = Router::new()
        .route("/api/song", 
               get(song_db::get_latest_song_album)
               .post(song_db::add_song))
        .route("/api/song/:id", 
               get(song_db::get_song_by_id)
               .patch(song_db::update_song_entry)
               .delete(song_db::delete_song_by_id))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
