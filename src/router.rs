use crate::api::cdn::handle_webp_image;
use crate::app::shell;
use axum::{routing::get, Router};
use http::{header, Method};
use leptos::prelude::*;
use leptos_axum::{file_and_error_handler, generate_route_list_with_ssg, LeptosRoutes};
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

/// # Panics
///
/// Should never panic when using cargo leptos
pub async fn app() -> Router {
    #[cfg(test)]
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    #[cfg(not(test))]
    let conf = get_configuration(None).unwrap();

    let leptos_options = conf.leptos_options;
    let (routes, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    static_routes.generate(&leptos_options).await;

    Router::new()
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
        .route("/assets/{*file_name}", get(handle_webp_image))
        .fallback(file_and_error_handler(shell))
        .layer(CompressionLayer::new().br(true))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(
            CorsLayer::new()
                // leptos server fns are RPC-based
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::COOKIE]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options)
}
