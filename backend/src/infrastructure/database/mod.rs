//! 🗄️ DATABASE MODULE - Persistence Layer
//! =====================================
//!
//! This module manages access to the PostgreSQL database and Redis cache,
//! handling connection pooling, schema migrations, and runtime storage access.
//!
//! ## Modules
//! - `connection`: SeaORM-compatible PostgreSQL connection pooling setup.
//! - `migrations`: Declarative schema migrations with SeaORM Migrator.
//! - `cache`: Redis-based caching and pub-sub integration.
//!
//! ## Responsibilities
//! - Provide database connections for repositories and use cases.
//! - Apply database schema migrations at startup.
//! - Manage Redis cache access for performance optimization.
//!
//! ## Note
//! This module **should not contain** business logic. It only enables data persistence.

pub mod cache;
pub mod connection;
pub mod migrations;
