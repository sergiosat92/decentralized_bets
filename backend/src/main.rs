
//! 🚀 ENTRY POINT - Launch Axum Server with Tokio Runtime
//! =====================================================
//!
//! This is the binary entry point of the backend. It calls `run_server()` from the library,
//! which initializes all infrastructure and starts the HTTP server.
//! 

use backend_boilerplate_rust::run_server;

/// Main Tokio asynchronous entrypoint.
/// Launches the backend server by calling `run_server()`.
#[tokio::main]
async fn main() {
    run_server().await;
}
