//! 🏗️ INFRASTRUCTURE LAYER - Adapters & External Interfaces
//! ========================================================
//!
//! This module provides the technical implementations that support the domain logic,
//! including database setup, observability (logging + metrics), secret management,
//! and web routing.
//!
//! ## Modules
//! - `database`: PostgreSQL connection pooling and migrations (SeaORM).
//! - `observability`: Tracing logs and Prometheus metrics setup.
//! - `secrets`: Access to secret keys and environment configuration.
//! - `web`: HTTP routing with Axum (including middleware, CORS, and controllers).
//!
//! ## Responsibilities
//! - Implements concrete details such as DB access, HTTP handling, or logging.
//! - May depend on external libraries, frameworks, or OS functionality.
//!
//! ## Note
//! This layer **should never** contain business logic. It only supports it.

pub mod database;
pub mod observability;
pub mod secrets;
pub mod web;
