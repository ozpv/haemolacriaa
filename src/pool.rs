use leptos::prelude::{use_context, ServerFnError};
use sqlx::{Pool, Postgres};

pub fn pool() -> Result<Pool<Postgres>, ServerFnError> {
    use_context::<Pool<Postgres>>()
        .ok_or_else(|| ServerFnError::new("Failed to get DB Pool, missing from context"))
}
