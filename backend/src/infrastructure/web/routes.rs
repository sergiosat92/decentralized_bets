//! 🚀 WEB ROUTES SETUP FOR THE APPLICATION 🌐
//!
//! This module defines the HTTP routes for the web server, organizing
//! public endpoints such as authentication and metrics, as well as
//! protected routes behind authentication middleware.
//!
//! Includes CORS support and HTTP metrics tracking middleware.

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::{
    domain::sports::services::get_leagues, infrastructure::web::authorization::cors_layer,
};

/// Basic health check or welcome endpoint returning a JSON message.
async fn index() -> impl IntoResponse {
    Json(json!({"message": "Hello, World!"}))
}

/// Handles preflight OPTIONS requests with appropriate CORS headers.
async fn handle_options() -> impl IntoResponse {
    (
        StatusCode::NO_CONTENT,
        [("Access-Control-Max-Age", "86400")],
    )
}

/// Publicly accessible routes that do not require authentication.
/// Includes registration, login, password reset, email verification, and metrics.
fn public_routes() -> Router {
    Router::new()
        .route("/", get(index).options(handle_options))
        .route("/get_leagues", get(get_leagues))
}

/// Aggregates all routes into a single router, applying
/// middleware layers for metrics tracking and CORS globally.
pub fn routes() -> Router {
    Router::new().merge(public_routes()).layer(cors_layer())
}
