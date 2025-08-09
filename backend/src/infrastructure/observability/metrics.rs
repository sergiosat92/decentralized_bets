//! 📊 METRICS MODULE - Prometheus Monitoring
//! ========================================
//!
//! This module sets up runtime metrics collection and exposition for monitoring purposes.
//! It integrates with the `metrics` crate and Prometheus to track application-level
//! and system-level metrics, such as HTTP request count, DB query timings,
//! memory usage, and CPU usage.
//!
//! ## Features
//! - Defines and describes application metrics using `metrics`.
//! - Instruments HTTP and database operations with timing and counters.
//! - Collects system resource usage using `sysinfo`.
//! - Exposes a `/metrics` endpoint for Prometheus scraping.
//!
//! ## Prometheus Integration
//! Uses `metrics-exporter-prometheus` to install a recorder and expose metrics.
//!
//! ## Notes
//! - Metric collection is conditional based on configuration (via `Secrets`).
//! - `tokio::spawn` is used to update system metrics periodically in the background.
//! - System metrics include memory and CPU usage.
//!
//! ## Example Metrics
//! - `http_requests_total`: Total number of HTTP requests.
//! - `http_request_duration_seconds`: Time taken to process HTTP requests.
//! - `db_queries_total`: Number of executed DB queries.
//! - `db_query_duration_seconds`: Time spent on DB queries.
//! - `memory_usage_bytes`: Current memory usage in bytes.
//! - `cpu_usage_percent`: CPU utilization percentage.

use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use metrics::{
    counter, describe_counter, describe_gauge, describe_histogram, histogram, Unit
};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::time::Instant;
use crate::infrastructure::database::cache::CONFIG;
use crate::infrastructure::secrets::Secrets;
use once_cell::sync::OnceCell;

static PROMETHEUS_HANDLE: OnceCell<PrometheusHandle> = OnceCell::new();

// -- Metric definitions
pub const HTTP_REQUESTS_TOTAL: &str = "http_requests_total";
pub const HTTP_REQUEST_DURATION_SECONDS: &str = "http_request_duration_seconds";
pub const DB_QUERIES_TOTAL: &str = "db_queries_total";
pub const DB_QUERY_DURATION_SECONDS: &str = "db_query_duration_seconds";
pub const ACTIVE_CONNECTIONS: &str = "active_connections";
pub const MEMORY_USAGE_BYTES: &str = "memory_usage_bytes";
pub const CPU_USAGE_PERCENT: &str = "cpu_usage_percent";

/// Initialize metric declarations with descriptions and units.
///
/// This should be called once at startup. Metrics are only declared if
/// Prometheus is enabled in the configuration.
fn init_metrics_declarations(secrets: &Secrets) {
    if !secrets.prometheus_enabled() {
        return;
    }

    describe_counter!(
        HTTP_REQUESTS_TOTAL,
        Unit::Count,
        "Total number of HTTP requests"
    );
    
    describe_histogram!(
        HTTP_REQUEST_DURATION_SECONDS,
        Unit::Seconds,
        "HTTP request duration in seconds"
    );
    
    describe_counter!(
        DB_QUERIES_TOTAL,
        Unit::Count,
        "Total number of database queries"
    );
    
    describe_histogram!(
        DB_QUERY_DURATION_SECONDS,
        Unit::Seconds,
        "Database query duration in seconds"
    );
    
    describe_gauge!(
        ACTIVE_CONNECTIONS,
        Unit::Count,
        "Number of active connections"
    );
    
    describe_gauge!(
        MEMORY_USAGE_BYTES,
        Unit::Bytes,
        "System memory usage in bytes"
    );
    
    describe_gauge!(
        CPU_USAGE_PERCENT,
        Unit::Percent,
        "Total system CPU usage percentage"
    );
}

/// Middleware to track HTTP request metrics.
///
/// Records the request method, path, status code, and processing duration.
pub async fn track_http_metrics(request: Request<Body>, next: Next) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let start = Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status().as_u16();

    let labels = [
        ("method", method),
        ("path", path),
        ("status", status.to_string()),
    ];
    
    counter!(HTTP_REQUESTS_TOTAL, &labels).increment(1);
    
    // Uncomment below if histogram instrumentation is needed
    // histogram!(HTTP_REQUEST_DURATION_SECONDS, &labels).record(duration.as_secs_f64());

    response
}

/// Instrumentation for tracking database query metrics.
///
/// Records execution time and query metadata (repo and query type).
pub fn track_db_query<T, F>(repo: &str, query: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    
    let result = f();
    
    let duration = start.elapsed();

    let labels = vec![
        ("repo", repo.to_string()),
        ("query", query.to_string()),
    ];
    
    counter!(DB_QUERIES_TOTAL, &labels).increment(1);
    
    // Uncomment below if histogram instrumentation is needed
    // histogram!(DB_QUERY_DURATION_SECONDS, &labels).record(duration.as_secs_f64());

    result
}

/// Periodically updates system-level metrics (memory and CPU).
async fn update_system_metrics() {
    use sysinfo::System;

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    let mut sys = System::new();

    loop {
        interval.tick().await;
        sys.refresh_all();
        
        let memory_usage = sys.used_memory() as f64;
        metrics::gauge!(MEMORY_USAGE_BYTES).set(memory_usage);
        
        let cpu_usage = sys.global_cpu_info().cpu_usage() as f64;
        metrics::gauge!(CPU_USAGE_PERCENT).set(cpu_usage);
    }
}

/// Initializes the Prometheus recorder and stores the handle.
fn init_prometheus_exporter() {
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder().expect("Failed to install Prometheus recorder");
    let _ = PROMETHEUS_HANDLE.set(handle);
}

/// Axum handler that returns the current Prometheus metrics as plain text.
pub async fn metrics_handler() -> String {
    PROMETHEUS_HANDLE
        .get()
        .expect("Prometheus handle not initialized")
        .render()
}

/// Initializes all metric systems: declarations, exporter, and system metrics.
pub fn init_metrics() {
    init_metrics_declarations(&CONFIG.secrets);
    init_prometheus_exporter();
    tokio::spawn(update_system_metrics());
}
