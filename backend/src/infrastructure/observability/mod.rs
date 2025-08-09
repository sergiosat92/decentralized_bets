//! 📊 OBSERVABILITY MODULE - Logging & Metrics
//! ==========================================
//!
//! This module provides observability features to monitor and debug the system.
//! It centralizes logging and metrics configurations, ensuring consistency across the application.
//!
//! ## Modules
//! - `logs`: Structured logging using `tracing`.
//! - `metrics`: Prometheus-compatible metrics exported via HTTP.
//!
//! ## Responsibilities
//! - Configure and initialize logging with context propagation.
//! - Define and expose custom metrics for performance and behavior analysis.
//!
//! ## Note
//! Observability tools do not alter business logic — they simply make it visible.

pub mod logs;
pub mod metrics;
