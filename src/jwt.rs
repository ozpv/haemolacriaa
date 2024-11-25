#![allow(unexpected_cfgs)]

use crate::lazy::JWT_SECRET;
use crate::util::*;
use axum::{extract::Request, middleware::Next, response::Response};
use chrono::Utc;
use headers::{Cookie, HeaderMapExt};
use http::StatusCode;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
#[allow(unused_imports)]
use leptos::server;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: u64,
}

/// Pass Some(exp) otherwise the token will expire after 7 days
/// Encoded with HS256 algoritm
/// add the feature encode to Cargo.toml to include
#[cfg(feature = "encode")]
// #[server(EncodeJwt, "/api", "GetJson")]
pub async fn encode_jwt(exp: Option<u64>, sub: String) -> Result<String> {
    use chrono::Duration;
    use jsonwebtoken::{encode, EncodingKey, Header};

    let exp = exp.unwrap_or((Utc::now() + Duration::days(7)).timestamp() as u64);

    let claims = Claims { sub, exp };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

/// Pass any token encoded with JWT_SECRET
/// Decodes HS256 algoritm
// #[server(DecodeJwt, "/api", "GetJson")]
pub async fn decode_jwt(token: String) -> Result<Claims> {
    let token_claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(Claims {
        sub: token_claims.claims.sub,
        exp: token_claims.claims.exp,
    })
}

/// Pass in a token and get Ok(()) if valid
// #[server(VerifyJwt, "/api", "Url")]
pub async fn verify_jwt(token: String) -> Result<(), StatusCode> {
    let claims = decode_jwt(token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if (Utc::now().timestamp() as u64) < claims.exp {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn protected_check(req: Request, next: Next) -> Result<Response, StatusCode> {
    let Some(cookie) = req.headers().typed_get::<Cookie>() else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let Some(token) = cookie.get("tok") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if verify_jwt(token.to_string()).await.is_ok() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
