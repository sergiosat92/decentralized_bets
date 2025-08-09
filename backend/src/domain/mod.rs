//! 🧠 DOMAIN LAYER - Business Rules & Core Logic
//! ============================================
//!
//! This module defines the core domain of the application.
//! It contains entities, value objects, traits, services, and use cases.
//!
//! Follows the principles of Clean Architecture and Domain-Driven Design (DDD).
//!
//! ## Modules
//! - `shared`: Common traits, helpers and domain-wide utilities.
//! - `users`: Core business logic related to users (e.g., registration, login, profile).
//!
//! ## Responsibilities
//! - Models business rules and use-case-specific behavior.
//! - Should not depend on `infrastructure/` or framework-specific libraries (like Axum or SeaORM).
//!
//! ## Example Usage
//! Use domain logic in your controllers or services:
//! ```rust
//! use crate::domain::users::services::register_user;
//! ```

pub mod shared;
pub mod users;

