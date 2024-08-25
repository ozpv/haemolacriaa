use axum::extract::FromRef;
use leptos::{use_context, LeptosOptions, ServerFnError};
use leptos_router::RouteListing;
use sqlx::{Pool, Postgres};

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub leptos_options: LeptosOptions,
    pub leptos_routes: Vec<RouteListing>,
}

impl AppState {
    pub fn pool() -> Result<Pool<Postgres>, ServerFnError> {
        use_context::<Pool<Postgres>>().ok_or_else(|| {
            ServerFnError::ServerError("Failed to get DB Pool, missing from context".to_owned())
        })
    }
}
