use crate::lazy::JWT_SECRET;
use crate::util::*;
use chrono::Utc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use leptos::server;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: u64,
}

/// Pass Some(exp) otherwise the token will expire after 7 days
/// Encoded with HS256 algoritm
#[cfg(feature = "encode")]
#[server(EncodeJwt, "/api", "GetJson")]
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
#[server(DecodeJwt, "/api", "GetJson")]
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

/// Pass in a token and get () if valid
#[server(VerifyJwt, "/api", "Url")]
pub async fn verify_jwt(token: String) -> Result<()> {
    let claims = decode_jwt(token).await;

    if (Utc::now().timestamp() as u64) < claims?.exp {
        Ok(())
    } else {
        err!("Failed to verify jwt")
    }
}
