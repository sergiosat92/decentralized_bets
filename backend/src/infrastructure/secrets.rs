//! 🔐 SECRETS MODULE - Environment & Configuration Loader
//! =====================================================
//!
//! Centralized configuration handler for environment variables and secrets.
//! It encapsulates loading, validating, and accessing runtime environment
//! configuration such as database credentials, JWT keys, logging setup,
//! external service URLs, and observability toggles.
//!
//! ## Structure
//! - `Secrets`: Strongly-typed, immutable configuration object.
//! - `Mode`: Enum to distinguish between `Development` and `Production`.
//! - `Secrets::load_secrets()`: Main entry point for loading all required settings.
//!
//! ## Responsibilities
//! - Load `.env` using `dotenvy` and `std::env`.
//! - Fail early with descriptive errors if required secrets are missing.
//! - Provide typed accessors for all relevant settings.
//! - Support conditional logic for optional services like Prometheus and Loki.
//!
//! ## Example
//! ```rust
//! let secrets = Secrets::load_secrets()?;
//! let db_url = secrets.database_url();
//! if secrets.loki_enabled() {
//!     let loki_url = secrets.loki_service_url();
//!     // connect to Loki...
//! }
//! ```

use std::env;

/// Runtime execution mode.
#[derive(Debug)]
pub enum Mode {
    Development,
    Production,
}

/// Immutable, pre-parsed secret and config store.
#[derive(Debug)]
pub struct Secrets {
    // Execution context
    mode: Mode,

    // Auth & encryption
    database_url: String,
    jwt_secret: String,
    encryption_key: String,
    allowed_origins: Vec<String>,

    // Logging & observability
    log_level: String,

    // External services
    email_service_url: String,
    notification_service_url: String,

    // Prometheus
    prometheus_service_enabled: bool,
    prometheus_service_url: String,
    prometheus_service_metrics_port: u16,
    prometheus_service_metrics_path: String,

    // Loki
    loki_service_enabled: bool,
    loki_service_url: String,
    loki_service_username: String,
    loki_service_password: String,
    loki_service_name: String,
    loki_service_environment: String,
    loki_service_timeout_sec: u64,
    loki_service_batch_size: usize,
}

impl Secrets {
    // === Getters ===

    pub fn mode(&self) -> &Mode { &self.mode }
    pub fn database_url(&self) -> &str { &self.database_url }
    pub fn jwt_secret(&self) -> &str { &self.jwt_secret }
    pub fn encryption_key(&self) -> &str { &self.encryption_key }
    pub fn allowed_origins(&self) -> &[String] { &self.allowed_origins }
    pub fn log_level(&self) -> &str { &self.log_level }
    pub fn email_service_url(&self) -> &str { &self.email_service_url }
    pub fn notification_service_url(&self) -> &str { &self.notification_service_url }

    // Prometheus
    pub fn prometheus_enabled(&self) -> bool { self.prometheus_service_enabled }
    pub fn prometheus_service_url(&self) -> &str { &self.prometheus_service_url }
    pub fn prometheus_metrics_port(&self) -> u16 { self.prometheus_service_metrics_port }
    pub fn prometheus_metrics_path(&self) -> &str { &self.prometheus_service_metrics_path }

    // Loki
    pub fn loki_enabled(&self) -> bool { self.loki_service_enabled }
    pub fn loki_service_url(&self) -> &str { &self.loki_service_url }
    pub fn loki_service_username(&self) -> &str { &self.loki_service_username }
    pub fn loki_service_password(&self) -> &str { &self.loki_service_password }
    pub fn loki_service_name(&self) -> &str { &self.loki_service_name }
    pub fn loki_service_environment(&self) -> &str { &self.loki_service_environment }
    pub fn loki_service_timeout_sec(&self) -> u64 { self.loki_service_timeout_sec }
    pub fn loki_batch_size(&self) -> usize { self.loki_service_batch_size }

    /// Load all environment-based configuration at runtime.
    ///
    /// Fails fast with a descriptive error if any critical value is missing
    /// or if optional services (e.g., Loki) are misconfigured.
    pub fn load_secrets() -> Result<Secrets, String> {
        dotenvy::dotenv().map_err(|e| format!("❌ Error loading .env file: {}", e))?;

        // Detect mode
        let mode_str = env::var("MODE").unwrap_or_else(|_| "development".to_string());
        let mode = match mode_str.to_lowercase().as_str() {
            "production" => Mode::Production,
            _ => Mode::Development,
        };

        // Required base secrets
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "❌ DATABASE_URL environment variable must be set.".to_string())?;
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| "❌ JWT_SECRET environment variable must be set.".to_string())?;
        let encryption_key = env::var("ENCRYPTION_KEY")
            .map_err(|_| "❌ ENCRYPTION_KEY environment variable must be set.".to_string())?;
        let email_service_url = env::var("EMAIL_SERVICE_URL")
            .map_err(|_| "❌ EMAIL_SERVICE_URL environment variable must be set.".to_string())?;
        let notification_service_url = env::var("NOTIFICATION_SERVICE_URL")
            .map_err(|_| "❌ NOTIFICATION_SERVICE_URL environment variable must be set.".to_string())?;

        // Origins
        let origins_str = env::var("ALLOWED_ORIGINS")
            .map_err(|_| "❌ ALLOWED_ORIGINS environment variable must be set.".to_string())?;
        let allowed_origins: Vec<String> = origins_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Logging
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| {
            match mode {
                Mode::Development => "debug".to_string(),
                Mode::Production => "info".to_string(),
            }
        });

        // Prometheus (optional)
        let prometheus_service_enabled = env::var("PROMETHEUS_SERVICE_ENABLED")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(false);
        let prometheus_service_url = if prometheus_service_enabled {
            env::var("PROMETHEUS_SERVICE_URL")
                .map_err(|_| "❌ PROMETHEUS_SERVICE_URL must be set when Prometheus is enabled".to_string())?
        } else {
            "".to_string()
        };
        let prometheus_service_metrics_port = env::var("PROMETHEUS_SERVICE_METRICS_PORT")
            .map(|v| v.parse().unwrap_or(9000))
            .unwrap_or(9000);
        let prometheus_service_metrics_path = env::var("PROMETHEUS_SERVICE_METRICS_PATH")
            .unwrap_or_else(|_| "/metrics".to_string());

        // Loki (optional)
        let loki_service_enabled = env::var("LOKI_SERVICE_ENABLED")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(false);
        let (loki_service_url, loki_service_username, loki_service_password, loki_service_name, loki_service_environment) =
            if loki_service_enabled {
                (
                    env::var("LOKI_SERVICE_URL")
                        .map_err(|_| "❌ LOKI_SERVICE_URL must be set when Loki is enabled".to_string())?,
                    env::var("LOKI_SERVICE_USERNAME")
                        .map_err(|_| "❌ LOKI_SERVICE_USERNAME must be set when Loki is enabled".to_string())?,
                    env::var("LOKI_SERVICE_PASSWORD")
                        .map_err(|_| "❌ LOKI_SERVICE_PASSWORD must be set when Loki is enabled".to_string())?,
                    env::var("LOKI_SERVICE_NAME")
                        .map_err(|_| "❌ LOKI_SERVICE_NAME must be set when Loki is enabled".to_string())?,
                    env::var("LOKI_SERVICE_ENVIRONMENT")
                        .map_err(|_| "❌ LOKI_SERVICE_ENVIRONMENT must be set when Loki is enabled".to_string())?,
                )
            } else {
                ("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string())
            };
        let loki_service_timeout_sec = env::var("LOKI_SERVICE_TIMEOUT_SEC")
            .map(|v| v.parse().unwrap_or(5))
            .unwrap_or(5);
        let loki_service_batch_size = env::var("LOKI_SERVICE_BATCH_SIZE")
            .map(|v| v.parse().unwrap_or(100))
            .unwrap_or(100);

        Ok(Secrets {
            mode,
            database_url,
            jwt_secret,
            encryption_key,
            allowed_origins,
            log_level,
            email_service_url,
            notification_service_url,
            prometheus_service_enabled,
            prometheus_service_url,
            prometheus_service_metrics_port,
            prometheus_service_metrics_path,
            loki_service_enabled,
            loki_service_url,
            loki_service_username,
            loki_service_password,
            loki_service_name,
            loki_service_environment,
            loki_service_timeout_sec,
            loki_service_batch_size,
        })
    }
}
