use axum::extract::FromRef;
use leptos::LeptosOptions;
use sqlx::{Pool, Postgres};

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub leptos_options: LeptosOptions,
}
