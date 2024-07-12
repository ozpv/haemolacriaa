use crate::lazy::KEYS;
use axum::{
    body::Body, extract::Request, middleware::Next, response::IntoResponse, response::Response,
    Json,
};
use chrono::Utc;
use http::header::AUTHORIZATION;
use http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::json;

pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    MissingCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        use AuthError::*;
        let (status, message) = match self {
            WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub struct Keys {
    decode: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            decode: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Deserialize)]
pub struct Claims {
    exp: usize,
}

pub async fn auth_mw(mut req: Request, next: Next) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers_mut().get(AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError::MissingCredentials)?,
        None => return Err(AuthError::MissingCredentials),
    };

    let mut header = auth_header.split_whitespace();

    let (_, token) = (header.next(), header.next());

    let token_data = decode::<Claims>(
        &token.unwrap().to_string(),
        &KEYS.decode,
        &Validation::default(),
    )
    .map_err(|_| AuthError::WrongCredentials)?;

    if token_data.claims.exp <= Utc::now().timestamp() as usize {
        Err(AuthError::WrongCredentials)
    } else {
        Ok(next.run(req).await)
    }
}
