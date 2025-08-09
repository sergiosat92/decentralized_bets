//! 📄 LOGGING SYSTEM (Observability Layer)
//! =======================================
//!
//! This module sets up and manages the application's logging system using
//! `tracing` and `tracing-subscriber`, with optional support for Grafana Loki.
//!
//! ## Features
//! - Human-friendly output in development
//! - JSON structured logs in production
//! - Optional Loki integration for centralized logging
//! - Unified structured log output via `logger` helper
//!
//! ## Functions
//! - `init_logger`: Initializes the logger based on the environment and secrets.
//! - `create_loki_layer`: Configures and returns a `tracing-loki` layer.
//! - `logger`: Centralized helper for structured logs at all levels.
//!
//! ## Types
//! - `ErrorOutput`: A serializable structure used to return error messages from HTTP APIs.
//!
//! ## Dependencies
//! - `tracing`, `tracing-subscriber` for layered logging
//! - `tracing-loki` for external log shipping (optional)
//!
//! ## Example
//! ```rust
//! logger(Level::INFO, "startup", "Application booted successfully");
//! ```

use serde::Serialize;
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};
use crate::infrastructure::{
    database::cache::CONFIG,
    secrets::{self, Secrets},
};

/// Structure used for serializing error messages in API responses.
#[derive(Serialize)]
pub struct ErrorOutput {
    pub message: String,
}

/// Initializes the logger according to the current environment settings.
///
/// - In `development`, it outputs human-readable, colorized logs to the console.
/// - In `production`, it outputs structured JSON logs, and optionally integrates with Loki.
///
/// This function reads the log level and mode from the loaded secrets.
pub fn init_logger() {
    let secrets = &CONFIG.secrets;

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(secrets.log_level()))
        .unwrap();

    match secrets.mode() {
        secrets::Mode::Development => {
            // Development: Pretty, colored console logs.
            let fmt_layer = fmt::layer()
                .pretty()
                .with_ansi(true)
                .with_level(true)
                .with_target(true);

            Registry::default()
                .with(filter_layer)
                .with(fmt_layer)
                .init();
        }
        secrets::Mode::Production => {
            // Production: Structured JSON logs for machine parsing.
            let fmt_layer = fmt::layer()
                .json()
                .flatten_event(true)
                .with_current_span(false)
                .with_span_list(false);

            if secrets.loki_enabled() {
                let loki_layer = create_loki_layer(secrets);
                Registry::default()
                    .with(filter_layer)
                    .with(fmt_layer)
                    .with(loki_layer)
                    .init();
            } else {
                Registry::default()
                    .with(filter_layer)
                    .with(fmt_layer)
                    .init();
            }
        }
    }

    logger(
        Level::INFO,
        "main",
        &format!("✅ Logger initialized with mode: {:?}", CONFIG.secrets.mode()),
    );
}

/// Builds a `tracing-loki` layer for structured log shipping.
///
/// This function is only called if Loki is enabled in the secrets.
/// It adds standard metadata like service name and environment.
fn create_loki_layer(secrets: &Secrets) -> tracing_loki::Layer {
    let url = secrets
        .loki_service_url()
        .parse()
        .expect("Invalid Loki URL");

    let mut labels = HashMap::new();
    labels.insert("service".to_string(), secrets.loki_service_name().to_string());
    labels.insert("env".to_string(), secrets.loki_service_environment().to_string());

    let mut extra_labels = HashMap::new();
    extra_labels.insert("module".to_string(), "".to_string());
    extra_labels.insert("feature".to_string(), "".to_string());
    extra_labels.insert("function".to_string(), "".to_string());

    let (loki_layer, task) =
        tracing_loki::layer(url, labels, extra_labels).expect("Failed to create Loki layer");

    // Background task that periodically flushes logs to Loki.
    tokio::spawn(task);

    loki_layer
}

/// Centralized logging helper for all log levels.
///
/// # Parameters
/// - `level`: One of the `tracing::Level` variants (ERROR, WARN, INFO, etc).
/// - `function`: Name of the function or context calling the logger.
/// - `message`: Log message.
///
/// # Example
/// ```rust
/// logger(Level::ERROR, "db::connect", "Database connection failed");
/// ```
pub fn logger(level: Level, function: &str, message: &str) {
    let mut context = HashMap::new();
    context.insert("message", message);

    match level {
        Level::ERROR => tracing::error!(function = function, ?context),
        Level::WARN  => tracing::warn!(function = function, ?context),
        Level::INFO  => tracing::info!(function = function, ?context),
        Level::DEBUG => tracing::debug!(function = function, ?context),
        Level::TRACE => tracing::trace!(function = function, ?context),
    }
}
