//! 🔐 OAUTH TOKEN VERIFICATION MODULE
//!
//! This module provides functionality to verify Google OAuth tokens by querying
//! Google's tokeninfo endpoint. It deserializes and validates the token data to
//! confirm authenticity and extract user information.

use reqwest::Method;
use serde::Deserialize;
use serde_json::Value;
use crate::infrastructure::{observability::logs::logger, web::http_client::send_request};

/// Google OAuth2 token info endpoint for verifying ID tokens.
const GOOGLE_ENDPOINT: &str = "https://oauth2.googleapis.com/tokeninfo?id_token=";

/// Represents the structure of Google's OAuth token info response.
#[derive(Deserialize)]
pub struct GoogleTokenInfo {
    pub iss: String,
    pub sub: String,
    pub azp: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,

    pub email: String,
    pub email_verified: Option<bool>,

    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

/// Verifies the provided Google OAuth ID token by querying Google's tokeninfo endpoint.
/// Returns detailed token information on success or an error string on failure.
///
/// # Arguments
///
/// * `token` - The Google ID token to verify.
///
/// # Errors
///
/// Returns an error string if the token is invalid, the request fails,
/// or required user information is missing.
///
/// # Examples
///
/// ```ignore
/// let token_info = check_google_token("some_id_token").await?;
/// println!("User email: {}", token_info.email);
/// ```
pub async fn check_google_token(token: &str) -> Result<GoogleTokenInfo, String> {
    let url = format!("{}{}", GOOGLE_ENDPOINT, token);

    let user_google: Value = match send_request(&url, Method::GET, None, None, 5).await {
        Ok(Some(data)) => data,
        Ok(None) => {
            logger(tracing::Level::ERROR, "check_google_token", "No data returned from Google");
            return Err("No data returned from Google".into());
        }
        Err(e) => {
            logger(tracing::Level::ERROR, "check_google_token", &format!("Error checking Google token: {}", e));
            return Err(e);
        }
    };

    let info: GoogleTokenInfo = serde_json::from_value(user_google.clone())
        .map_err(|e| {
            logger(tracing::Level::ERROR, "check_google_token", &format!("Failed to parse token info: {}", e));
            format!("Failed to parse token info: {}", e)
        })?;

    if info.email.is_empty() || info.email_verified.is_none() {
        logger(tracing::Level::ERROR, "check_google_token", "Email not found in Google token response");
        return Err("Email not found in Google token response".into());
    }

    Ok(info)
}
