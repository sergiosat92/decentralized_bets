//! 👤 USER DOMAIN MODULE
//! ======================
//!
//! This module encapsulates all user-related domain logic, including data structures,
//! business rules, data persistence, and testing utilities.
//!
//! ## Submodules
//! - `dtos`: Data Transfer Objects used for communication between layers or external interfaces.
//! - `repositories`: Data access layer abstractions and implementations for users.
//! - `services`: Business logic and use cases related to users.
//! - `tests`: Unit and integration tests related to the user domain.
//! - `traits`: Trait definitions for user-related abstractions and contracts.
//! - `user`: Core user entity model and ActiveModel for database interaction.
//!
//! ## Type Aliases
//! - `User`: Alias for the core user domain entity model (`Model`).
//! - `UserActiveModel`: Alias for the ActiveModel type used in database updates/inserts.
//!
//! ## Usage
//! - Use `User` to represent immutable user data in the domain logic.
//! - Use `UserActiveModel` to construct or modify user data for persistence.
//! - The module exposes `ActiveModel` and `Model` from the `user` submodule for direct use.

pub mod dtos;
pub mod repositories;
pub mod services;
pub mod tests;
pub mod traits;
pub mod user;

/// Alias for the user domain entity model.
pub type User = Model;
/// Alias for the user ActiveModel used for updates and inserts.
pub type UserActiveModel = ActiveModel;

/// Re-export core user types from the `user` submodule.
pub use user::{ActiveModel, Model};
