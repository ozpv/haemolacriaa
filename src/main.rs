cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            body::Body,
            extract::{Request, State},
            middleware,
            response::{IntoResponse, Response},
            routing::get,
            Router,
        };
        use haemolacriaa::app::*;
        use haemolacriaa::fileserv::file_and_error_handler;
        use haemolacriaa::jwt;
        use haemolacriaa::state::AppState;
        use leptos::*;
        use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
        use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
        use std::time::Duration;

        async fn server_fn_handler(
            State(app_state): State<AppState>,
            request: Request<Body>,
        ) -> impl IntoResponse {
            handle_server_fns_with_context(
                move || {
                    provide_context(app_state.db_pool.clone());
                },
                request,
            )
            .await
        }

        async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<Body>) -> Response {
            let handler = leptos_axum::render_route_with_context(
                app_state.leptos_options.clone(),
                app_state.leptos_routes.clone(),
                move || {
                    provide_context(app_state.db_pool.clone());
                },
                App,
            );

            handler(req).await.into_response()
        }

        #[tokio::main]
        async fn main() {
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
                leptos_routes: routes.clone(),
            };

            // server functions
            let server = Router::new().route(
                "/api/*fn_name",
                get(server_fn_handler).post(server_fn_handler),
            );
            //    .layer(middleware::from_fn(jwt::auth_mw));

            // build our application with a route
            let app = Router::new()
                .merge(server)
                .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                .fallback(file_and_error_handler)
                .with_state(state);

            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            logging::log!("listening on http://{}", &addr);
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }
    }
}
