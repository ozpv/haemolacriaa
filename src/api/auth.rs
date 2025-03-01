use leptos::prelude::*;
use server_fn::codec::PostUrl;

#[cfg(feature = "ssr")]
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
#[cfg(feature = "ssr")]
use axum_thiserror_tracing::IntoResponse;
#[cfg(feature = "ssr")]
use base64::{engine::general_purpose, Engine};
#[cfg(feature = "ssr")]
use chrono::{DateTime, Duration, Utc};
#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Debug, Error, IntoResponse)]
pub enum AuthError {
    #[error("you're not allowed to access this resource")]
    #[status(StatusCode::UNAUTHORIZED)]
    Unauthorized,
    #[error("session_id is invalid or corrupt")]
    #[status(StatusCode::BAD_REQUEST)]
    Invalid,
    #[error("{0}")]
    Internal(&'static str),
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct Session {
    id: String,
    exp: DateTime<Utc>,
}

#[cfg(feature = "ssr")]
#[allow(unused)]
impl Session {
    fn new() -> Self {
        Self {
            id: gen_rand_string::<32>(),
            exp: Utc::now() + Duration::days(1),
        }
    }

    fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    fn exp_in(mut self, duration: Duration) -> Self {
        self.exp = Utc::now() + duration; 
        self
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_exp(&self) -> &DateTime<Utc> {
        &self.exp
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // TODO: actually extract and check store to validate
        Ok(Session::new())
    }
}

#[server(endpoint = "login", input = PostUrl)]
pub async fn login() -> Result<(), ServerFnError> {
    let session = leptos_axum::extract::<Session>().await?;
    tracing::debug!("got session {session:?}");

    if session.exp > Utc::now() {
        return Ok(());
    }

    let session = Session::new();
    // TODO: store this

    Err(AuthError::Internal("Not implemented yet").into())
}

#[cfg(feature = "ssr")]
#[inline]
pub fn gen_rand_string<const N: usize>() -> String {
    let mut bytes = [0u8; N];

    rand::fill(&mut bytes[..]);

    general_purpose::STANDARD.encode(bytes)
}
