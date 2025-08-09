//! 🚀 WEB ROUTES SETUP FOR THE APPLICATION 🌐
//!
//! This module defines the HTTP routes for the web server, organizing
//! public endpoints such as authentication and metrics, as well as
//! protected routes behind authentication middleware.
//!
//! Includes CORS support and HTTP metrics tracking middleware.

use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::{
    domain::users::services::auth::{
        forgot_password_handler, google_login_handler, login_handler, register_handler,
        reset_password_handler, verify_email_handler,
    },
    infrastructure::{database::cache::CONFIG, observability::metrics::{metrics_handler, track_http_metrics}, web::{
        authorization::cors_layer,
        middlewares::auth_middleware,
    }},
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
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/login/google", post(google_login_handler))
        .route("/forgot-password", post(forgot_password_handler))
        .route("/reset-password", post(reset_password_handler))
        .route("/verify-email", post(verify_email_handler))
        .route(CONFIG.secrets.prometheus_metrics_path(), get(metrics_handler))
}

/// Routes that require authentication middleware.
/// Placeholder for protected user endpoints like profile or settings.
fn authenticated_routes() -> Router {
    Router::new()
        /* 
        .route("/profile", routing::get(get_profile))
        .route("/settings", routing::put(update_settings)) 
        */
        .layer(middleware::from_fn(auth_middleware))
}

/// Aggregates all routes into a single router, applying
/// middleware layers for metrics tracking and CORS globally.
pub fn routes() -> Router {
    Router::new()
        .merge(public_routes())
        .merge(authenticated_routes())
        .layer(middleware::from_fn(track_http_metrics))
        .layer(cors_layer())
}
