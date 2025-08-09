//! 👤 USER ENTITY MODEL
//! ====================
//!
//! This module defines the core user entity model used in the domain and persistence layers.
//! It includes the data structure for the user table and validation rules.
//!
//! ## Structs
//! - `Model`: Represents the user entity with all fields persisted in the database.
//!
//! ## Features
//! - Uses `sea_orm` for ORM mapping to the `users` table.
//! - Implements `serde` serialization and deserialization for JSON handling.
//! - Uses `validator` for input validation rules on fields like email, username, and names.
//!
//! ## Fields
//! - `id`: Primary key, unique identifier for the user (non-auto-incremented).
//! - `email`: Unique and indexed email with validation (must be valid email format).
//! - `username`: Unique and indexed username with length validation.
//! - `password_hash`: Hashed password, omitted from JSON serialization for security.
//! - `first_name`, `last_name`: Optional user names with length limits.
//! - `is_verified`: Indicates if the user's email is verified (default false).
//! - `is_active`: Indicates if the user is active (default true).
//! - `is_locked`: Indicates if the user account is locked (default false).
//! - `failed_login_attempts`: Number of failed login attempts (default 0).
//! - `lockout_until`: Optional timestamp until which the user is locked out.
//! - `verification_token`, `verification_token_expires`: Token and expiry for email verification.
//! - `password_reset_token`, `password_reset_expires`: Token and expiry for password reset.
//! - `last_login`: Timestamp of the last successful login.
//! - `created_at`: Timestamp of user creation (defaults to current timestamp).
//! - `updated_at`: Timestamp of last update (auto-updated on change).
//! - `deleted_at`: Optional timestamp for soft deletion.
//! - `role`: User role (default "user").
//!
//! ## Relations
//! - Empty enum, no relations defined currently.
//!
//! ## Usage
//! - This model is used with SeaORM to perform database operations related to users.
//! - Validation ensures data integrity before persistence or processing.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Validate)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// Primary key: unique user ID (non auto-increment)
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    /// Unique user email, indexed, validated for proper email format and length
    #[sea_orm(unique, indexed)]
    #[validate(length(min = 3, max = 50), email)]
    pub email: String,

    /// Unique username, indexed, validated for length
    #[sea_orm(unique, indexed)]
    #[validate(length(min = 3, max = 30))]
    pub username: String,

    /// Hashed password, excluded from JSON serialization
    #[serde(skip_serializing)]
    pub password_hash: String,

    /// Optional first name with max length 50
    #[validate(length(max = 50))]
    pub first_name: Option<String>,

    /// Optional last name with max length 50
    #[validate(length(max = 50))]
    pub last_name: Option<String>,

    /// Flag indicating if the user's email is verified (default: false)
    #[sea_orm(default_value = "false")]
    pub is_verified: bool,

    /// Flag indicating if the user account is active (default: true)
    #[sea_orm(default_value = "true")]
    pub is_active: bool,

    /// Flag indicating if the user account is locked (default: false)
    #[sea_orm(default_value = "false")]
    pub is_locked: bool,

    /// Number of failed login attempts (default: 0)
    #[sea_orm(default_value = "0")]
    pub failed_login_attempts: u32,

    /// Optional timestamp until which the user is locked out
    pub lockout_until: Option<DateTimeWithTimeZone>,

    /// Optional token for email verification
    pub verification_token: Option<String>,

    /// Optional expiry time for the verification token
    pub verification_token_expires: Option<DateTimeWithTimeZone>,

    /// Optional token for password reset
    pub password_reset_token: Option<String>,

    /// Optional expiry time for the password reset token
    pub password_reset_expires: Option<DateTimeWithTimeZone>,

    /// Timestamp of the last successful login
    pub last_login: Option<DateTimeWithTimeZone>,

    /// Timestamp of when the user was created (default: current timestamp)
    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTimeWithTimeZone,

    /// Timestamp of last update (auto-updated on change)
    #[sea_orm(default_value = "CURRENT_TIMESTAMP", on_update = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTimeWithTimeZone,

    /// Optional timestamp for soft deletion
    pub deleted_at: Option<DateTimeWithTimeZone>,

    /// User role with default "user"
    #[sea_orm(default_value = "'user'")]
    pub role: String,
}

/// Enum defining relationships for the user model (currently empty)
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
