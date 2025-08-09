pub mod domain;
pub mod infrastructure;

use axum::Router;
use infrastructure::web::routes;

/// Builds the Axum application with the injected database connection and all registered routes.
pub fn build_app() -> Router {
    Router::new().merge(routes::routes())
}

/// Runs the Axum server, initializing logging, metrics, DB and binding to `127.0.0.1:8000`.
pub async fn run_server() {
    use std::net::SocketAddr;
    let app = build_app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            println!("❌ Server failed to start: {}", e);
            std::process::exit(1);
        });

    println!("🚀 Server listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap_or_else(|e| {
        println!("❌ Server failed to start: {}", e);
        std::process::exit(1);
    });
}
