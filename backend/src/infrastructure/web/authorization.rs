//! 🔐 AUTHORIZATION MODULE WITH JWT AND CORS SETUP
//!
//! This module handles JWT token creation, validation, and extraction of user credentials.
//! It also provides a CORS layer configuration for HTTP request handling.

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, HeaderMap, Method, header},
};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header as JwtHeader, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, AllowOrigin, AllowMethods, AllowHeaders};
use chrono::{Duration, Utc};
use crate::infrastructure::database::cache::{ALLOWED_ORIGINS, CONFIG};

/// JWT Claims struct storing encoded user info and expiration time.
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    content: String,
    exp: i64,
}

/// User credentials extracted from JWT token.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Credentials {
    pub id: String,
    pub email: String,
    pub username: String,
}

/// Encoding key for JWT creation, loaded once from config secret.
static ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    EncodingKey::from_secret(CONFIG.secrets.jwt_secret().as_bytes())
});

/// Decoding key for JWT validation, loaded once from config secret.
static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_secret(CONFIG.secrets.jwt_secret().as_bytes())
});

/// Validation rules for JWT decoding.
static VALIDATION: Lazy<Validation> = Lazy::new(|| {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation
});

/// Creates a CORS layer configured with allowed origins, methods, headers, and credentials.
pub fn cors_layer() -> CorsLayer {
    let allowed_origins = if ALLOWED_ORIGINS.contains(&"*".to_string()) {
        AllowOrigin::any()
    } else {
        AllowOrigin::list(
            ALLOWED_ORIGINS
                .iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        )
    };

    CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            "X-Requested-With".parse().unwrap(),
            "X-Forwarded-For".parse().unwrap(),
            "X-Real-IP".parse().unwrap()
        ]))
        .allow_credentials(true)
        .expose_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            "X-Total-Count".parse().unwrap(),
        ])
        .max_age(std::time::Duration::from_secs(86400))
}

#[async_trait]
impl<S> FromRequestParts<S> for Credentials
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    /// Extracts and validates the JWT token from HTTP request headers,
    /// then parses and returns the Credentials.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = extract_token(&parts.headers)?;
        let content = decode_token(&token)?;
        parse_credentials(&content)
    }
}

/// Parses the credentials from the decoded JWT content string.
fn parse_credentials(content: &str) -> Result<Credentials, (StatusCode, String)> {
    let parts: Vec<&str> = content.split("::::").collect();
    if parts.len() == 3 {
        Ok(Credentials {
            id: parts[0].to_string(),
            email: parts[1].to_string(),
            username: parts[2].to_string(),
        })
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()))
    }
}

/// Extracts the Bearer token from the Authorization header.
fn extract_token(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))
}

/// Creates a JWT token encoding the user credentials with an expiration of 1 week.
pub fn create_token(credentials: Credentials) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::weeks(1))
        .ok_or("Invalid timestamp")?
        .timestamp();
    let claims = Claims {
        content: format!(
            "{}::::{}::::{}",
            credentials.id, credentials.email, credentials.username
        ),
        exp: expiration,
    };
    encode(&JwtHeader::default(), &claims, &ENCODING_KEY)
        .map_err(|e| e.to_string())
}

/// Decodes and validates the JWT token, returning the contained user info string.
fn decode_token(token: &str) -> Result<String, (StatusCode, String)> {
    decode::<Claims>(token, &DECODING_KEY, &VALIDATION)
        .map(|token| token.claims.content)
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))
}
