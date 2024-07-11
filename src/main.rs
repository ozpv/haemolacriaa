#[cfg(feature = "ssr")] 
#[tokio::main]
async fn main() {
    use axum::{middleware, routing::{get, post, patch}, Router};
    use tower_http::compression::CompressionLayer;
    use haemolacriaa::app::*;
    use haemolacriaa::fileserv::file_and_error_handler;
    use haemolacriaa::app_state::AppState;
    use haemolacriaa::{song_db, jwt};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::postgres::{PgSslMode, PgConnectOptions, PgPoolOptions};
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

    // to build the postgres connection
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

    // app state
    let state = AppState {
        db_pool,
        leptos_options,
    };
    
    // protected routes requiring jwt auth
    let protected = Router::new()
        .route("/api/song", post(song_db::add_song))
        .route("/api/song/:name", 
               patch(song_db::update_song_entry)
               .delete(song_db::delete_song_by_name)
        ).layer(middleware::from_fn(jwt::auth_mw));
    
    // build our application with a route
    let app = Router::new()
        .route("/api/song", get(song_db::get_latest_song_album))
        .route("/api/song/:name", get(song_db::get_song_by_name))
        .merge(protected)
        .leptos_routes(&state, routes, App)
        .fallback(file_and_error_handler)
        .with_state(state)
        .layer(CompressionLayer::new());

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
