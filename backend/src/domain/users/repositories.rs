//! 📦 USER REPOSITORY
//! ==================
//!
//! This module provides database access functions for the User entity using SeaORM.
//! It implements query operations such as finding users by email or by ID.
//!
//! ## Implementation Details
//! - Implements the default behavior for `ActiveModel`.
//! - Provides async functions to fetch user records from the database.
//! - Uses SeaORM's query builder for filtering and finding entities.
//!
//! ## Functions
//! - `find_user_by_email`: Searches the database for a user by their email address.
//! - `find_user_by_id`: Searches the database for a user by their unique identifier (ID).
//!
//! ## Errors
//! Both functions return `DbErr::RecordNotFound` if the user does not exist.

use sea_orm::entity::prelude::*;

use crate::domain::users::user::{ActiveModel, Column, Entity, Model};

/// Default behavior implementation for ActiveModel (no customization)
impl ActiveModelBehavior for ActiveModel {}

/// Finds a user in the database by their email address
///
/// # Arguments
/// * `db` - Database connection handle
/// * `email` - The email address to search for
///
/// # Returns
/// * `Ok(Model)` with the user model if found
/// * `Err(DbErr)` if no user found or on query failure
pub async fn find_user_by_email(db: &DatabaseConnection, email: &str) -> Result<Model, DbErr> {
    Entity::find()
        .filter(Column::Email.eq(email))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("User not found".to_string()))
}

/// Finds a user in the database by their unique ID
///
/// # Arguments
/// * `db` - Database connection handle
/// * `id` - The user ID to search for
///
/// # Returns
/// * `Ok(Model)` with the user model if found
/// * `Err(DbErr)` if no user found or on query failure
pub async fn find_user_by_id(db: &DatabaseConnection, id: &str) -> Result<Model, DbErr> {
    Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("User not found".to_string()))
}
