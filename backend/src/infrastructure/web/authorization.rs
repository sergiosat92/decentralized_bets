//! 🔐 AUTHORIZATION MODULE WITH JWT AND CORS SETUP
//!
//! This module handles JWT token creation, validation, and extraction of user credentials.
//! It also provides a CORS layer configuration for HTTP request handling.

use axum::http::{header, Method};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

/// Creates a CORS layer configured with allowed origins, methods, headers, and credentials.
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::any())
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
            "X-Real-IP".parse().unwrap(),
        ]))
        .expose_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            "X-Total-Count".parse().unwrap(),
        ])
        .max_age(std::time::Duration::from_secs(86400))
}
