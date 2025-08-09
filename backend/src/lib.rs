//! 🧠 BACKEND ENTRY POINT - Modular Axum + SeaORM Server
//! ======================================================================
//!
//! This file sets up the main application structure using the Axum framework
//! with a modular architecture composed of `infrastructure` and `domain` layers.
//!
//! ## Features
//! - Initializes application routes
//! - Injects database connection into Axum context
//! - Handles application startup and graceful failure
//!
//! ## Modules
//! - [`infrastructure`]: handles DB, logging, metrics, HTTP
//! - [`domain`]: application logic and domain modeling
//!
//! ## Exposed Functions
//! - `build_app(db)`: returns the configured Axum `Router`
//! - `run_server()`: initializes metrics, logger, database and starts server
//!
//! ## To Run
//! ```bash
//! cargo run
//! ```


pub mod infrastructure;
pub mod domain;

use axum::{Extension, Router};
use infrastructure::web::routes;
use sea_orm::DatabaseConnection;

/// Builds the Axum application with the injected database connection and all registered routes.
pub fn build_app(db: DatabaseConnection) -> Router {
    Router::new()
        .merge(routes::routes())
        .layer(Extension(db))
}

/// Runs the Axum server, initializing logging, metrics, DB and binding to `127.0.0.1:8000`.
pub async fn run_server() {
    use crate::infrastructure::database::connection::init_database;
    use crate::infrastructure::observability::logs::{init_logger, logger};
    use crate::infrastructure::observability::metrics::init_metrics;
    use std::net::SocketAddr;

    init_logger();
    init_metrics();
    let pool = init_database().await;
    let app = build_app(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await
        .unwrap_or_else(|e| {
            logger(tracing::Level::ERROR, "main", &format!("❌ Failed to bind to address {}: {}", addr, e));
            std::process::exit(1);
        });

    logger(tracing::Level::INFO, "main", &format!("✅ Server running at http://{}", addr));

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap_or_else(|e| {
            logger(
                tracing::Level::ERROR,
                "main",
                &format!("❌ Server failed to start: {}", e),
            );
            std::process::exit(1);
        });
}