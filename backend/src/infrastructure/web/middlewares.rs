//! 🔐 AUTHORIZATION MIDDLEWARE FOR AXUM ROUTES
//!
//! This middleware extracts and verifies JWT credentials from incoming requests.
//! On success, it inserts the authenticated credentials into the request extensions for downstream handlers.
//! On failure, it returns an authorization error with appropriate HTTP status code.

use axum::{
    extract::FromRequestParts,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use crate::infrastructure::web::authorization::Credentials;

/// Authentication middleware for Axum handlers.
/// Extracts `Credentials` from request parts and, if valid, attaches them to the request extensions.
/// Returns an error tuple with status and message if authorization fails.
pub async fn auth_middleware(request: Request<Body>, next: Next) -> Result<Response, (StatusCode, String)> {
    let (mut parts, body) = request.into_parts();
    
    match Credentials::from_request_parts(&mut parts, &()).await {
        Ok(credentials) => {
            parts.extensions.insert(credentials);
            
            let request = Request::from_parts(parts, body);
            Ok(next.run(request).await)
        }
        Err(e) => Err(e),
    }
}
