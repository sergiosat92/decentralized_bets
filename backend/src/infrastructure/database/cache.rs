//! 🧠 CONFIG CACHE
//! ==================
//!
//! This module provides a global in-memory cache for critical configuration
//! data such as secrets and allowed CORS origins.
//!
//! It uses `once_cell::sync::Lazy` to ensure thread-safe, one-time initialization
//! of the configuration at runtime.
//!
//! ## Components
//! - [`CONFIG`]: Loads secrets from disk or environment and caches them.
//! - [`ALLOWED_ORIGINS`]: Normalized CORS origins derived from the secrets.
//!
//! ## Behavior
//! - If secrets loading fails, the app logs the error and terminates early.
//! - Origin URLs are normalized to lowercase and trimmed of trailing slashes
//!   for consistent CORS handling across the application.
//!
//! ## Usage
//! - Access via `CONFIG.secrets` or `ALLOWED_ORIGINS` anywhere in the app.
//! - Secrets are loaded once and shared across all threads safely.

use once_cell::sync::Lazy;
use std::collections::HashSet;

use crate::infrastructure::{observability::logs::logger, secrets::Secrets};

/// Holds parsed configuration from secrets file or environment.
#[derive(Debug)]
pub struct ServerConfig {
    pub secrets: Secrets,
}

/// Global, lazy-initialized configuration cache.
/// 
/// - Loads secrets using [`Secrets::load_secrets`] at startup.
/// - Crashes the app if secrets cannot be loaded.
pub static CONFIG: Lazy<ServerConfig> = Lazy::new(|| {
    let secrets = match Secrets::load_secrets() {
        Ok(secrets) => secrets,
        Err(e) => {
            logger(
                tracing::Level::ERROR,
                "config",
                &format!("❌ Failed to load secrets: {}", e),
            );
            std::process::exit(1);
        },
    };
    ServerConfig { secrets }
});

/// Global cache of CORS allowed origins.
///
/// Normalizes all origins (lowercase, removes trailing slash)
/// for use in CORS middleware.
pub static ALLOWED_ORIGINS: Lazy<HashSet<String>> = Lazy::new(|| {
    CONFIG
        .secrets
        .allowed_origins()
        .iter()
        .cloned()
        .map(|origin| origin.to_lowercase().trim_end_matches('/').to_string())
        .collect()
});
