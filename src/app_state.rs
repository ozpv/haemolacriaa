use sqlx::{Pool, Postgres};
use leptos::LeptosOptions;
use axum::extract::FromRef;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub leptos_options: LeptosOptions,
}
