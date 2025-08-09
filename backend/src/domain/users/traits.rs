//! 👥 USER TRAITS AND HELPERS
//! ===========================
//!
//! This module provides helper functions and custom methods implemented on the ActiveModel
//! of the user entity. It handles user creation, password encryption and verification,
//! token generation for email verification and password reset, JWT creation, and account state management.
//!
//! ## Functions
//! - `create_new_user`: Creates a new user ActiveModel instance with hashed password and default fields.
//! - `hash_password`: Encrypts a plaintext password using a configured encryption key.
//!
//! ## ActiveModel Methods
//! - `verify_password`: Checks if a given password matches the stored hashed password.
//! - `update_password`: Updates the password hash with a new password.
//! - `generate_verification_token`: Generates a unique email verification token valid for 24 hours.
//! - `generate_password_reset_token`: Generates a password reset token valid for 1 hour.
//! - `reset_password`: Resets the password if provided token is valid and not expired.
//! - `verify_account`: Marks the account as verified if provided token matches and is valid.
//! - `generate_jwt`: Creates a JWT token for the user with error handling and logging.
//! - `record_login`: Updates last login timestamp and resets failed login attempts and lockout.
//! - `record_failed_login`: Increments failed login attempts and locks the account after 5 failures for 30 minutes.
//! - `is_account_locked`: Checks if the user account is currently locked.
//! - `soft_delete`: Marks the account as deleted and inactive.
//! - `restore`: Restores a soft-deleted account to active status.
//! - `promote_to_admin`: Changes user role to "admin".
//! - `demote_to_user`: Changes user role back to "user".
//!
//! ## Security
//! - Passwords are encrypted and decrypted using the `magic_crypt` crate with a 256-bit key.
//! - Tokens are UUID strings with expiration times enforced using chrono's `Utc` timestamps.
//! - JWT generation uses domain-level authorization helpers with proper error logging.

use chrono::{Duration, Utc};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use sea_orm::Set;

use uuid::Uuid;

use crate::{
    domain::users::user::ActiveModel,
    infrastructure::{
        database::cache::CONFIG,
        observability::logs::logger,
        web::authorization::{create_token, Credentials},
    },
};

/// Creates a new user ActiveModel with hashed password and default values
pub fn create_new_user(
    email: String,
    username: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
) -> Result<ActiveModel, String> {
    // Hash password securely
    let password_hash = hash_password(&password);

    Ok(ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        email: Set(email),
        username: Set(username),
        password_hash: Set(password_hash),
        first_name: Set(first_name),
        last_name: Set(last_name),
        role: Set("user".to_string()),
        failed_login_attempts: Set(0),
        is_locked: Set(false), 
        lockout_until: Set(None),
        is_verified: Set(false),
        is_active: Set(true), 
        deleted_at: Set(None), 
        ..Default::default()
    })
}

/// Encrypts the given password using the configured encryption key
fn hash_password(password: &str) -> String {
    let mc = new_magic_crypt!(CONFIG.secrets.encryption_key(), 256);
    mc.encrypt_str_to_base64(password)
}

/// Custom methods for ActiveModel providing domain logic around user state and authentication
impl ActiveModel {

    /// Verify if the given password matches the stored encrypted password
    pub fn verify_password(&self, password: &str) -> bool {
        let mc = new_magic_crypt!(CONFIG.secrets.encryption_key(), 256);
        let raw_password = mc
            .decrypt_base64_to_string(self.password_hash.as_ref())
            .expect("ERROR decrypting password hash.");
        password == &raw_password
    }

    /// Update the password hash to a new encrypted password
    pub fn update_password(&mut self, new_password: &str) {
        self.password_hash = Set(hash_password(new_password));
    }

    /// Generate a new email verification token valid for 24 hours
    pub fn generate_verification_token(&mut self) -> String {
        let token = Uuid::new_v4().to_string();
        self.verification_token = Set(Some(token.clone()));
        self.verification_token_expires = Set(Some((Utc::now() + Duration::hours(24)).into()));
        token
    }

    /// Generate a password reset token valid for 1 hour
    pub fn generate_password_reset_token(&mut self) -> String {
        let token = Uuid::new_v4().to_string();
        self.password_reset_token = Set(Some(token.clone()));
        self.password_reset_expires = Set(Some((Utc::now() + Duration::hours(1)).into()));
        token
    }

    /// Reset password if the provided token matches and is not expired
    pub fn reset_password(&mut self, token: &str, new_password: &str) -> Result<(), String> {
        match (&self.password_reset_token, &self.password_reset_expires) {
            (sea_orm::ActiveValue::Set(Some(stored_token)), sea_orm::ActiveValue::Set(Some(expiry))) if Utc::now() < *expiry => {
                if stored_token == token {
                    self.update_password(new_password);
                    self.password_reset_token = Set(None);
                    self.password_reset_expires = Set(None);
                    return Ok(());
                }
            }
            _ => {}
        }
        Err("Invalid or expired token".into())
    }

    /// Verify the user account with the given token if it is valid and not expired
    pub fn verify_account(&mut self, token: &str) -> bool {
        match (&self.verification_token, &self.verification_token_expires) {
            (sea_orm::ActiveValue::Set(Some(stored_token)), sea_orm::ActiveValue::Set(Some(expiry)))
                if Utc::now() < *expiry =>
            {
                if stored_token == token {
                    self.is_verified = Set(true);
                    self.verification_token = Set(None);
                    self.verification_token_expires = Set(None);
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    /// Generate a JWT token for the user, logging errors if token creation fails
    pub fn generate_jwt(&self) -> Result<String, String> {
        let id = self.id.as_ref();
        let email = self.email.as_ref();
        let username = self.username.as_ref();

        let credentials = Credentials {
            id: id.to_string(),
            email: email.to_string(),
            username: username.to_string(),
        };

        create_token(credentials).map_err(|e| {
            logger(
                tracing::Level::ERROR,
                "generate_jwt",
                &format!("Failed to create JWT: {}", e),
            );
            format!("Failed to create JWT: {}", e)
        })
    }

    /// Record a successful login resetting failed attempts and lockout
    pub fn record_login(&mut self) {
        self.last_login = Set(Some(Utc::now().into()));
        self.failed_login_attempts = Set(0);
        self.is_locked = Set(false);
        self.lockout_until = Set(None);
    }

    /// Record a failed login attempt, locking the account after 5 failures for 30 minutes
    pub fn record_failed_login(&mut self) {
        let current_attempts = self.failed_login_attempts.as_ref().to_owned();
        self.failed_login_attempts = Set(current_attempts + 1);

        if current_attempts + 1 >= 5 {
            self.is_locked = Set(true);
            self.lockout_until = Set(Some((Utc::now() + Duration::minutes(30)).into()));
        }
    }

    /// Check whether the user account is currently locked based on lockout expiry
    pub fn is_account_locked(&self) -> bool {
        if let sea_orm::ActiveValue::Set(Some(expiry)) = &self.lockout_until {
            Utc::now() < *expiry
        } else {
            false
        }
    }

    /// Soft delete the user account by setting deletion timestamp and deactivating it
    pub fn soft_delete(&mut self) {
        self.deleted_at = Set(Some(Utc::now().into()));
        self.is_active = Set(false);
    }

    /// Restore a soft-deleted user account by clearing deletion timestamp and activating it
    pub fn restore(&mut self) {
        self.deleted_at = Set(None);
        self.is_active = Set(true);
    }

    /// Promote the user role to admin
    pub fn promote_to_admin(&mut self) {
        self.role = Set("admin".to_string());
    }

    /// Demote the user role back to regular user
    pub fn demote_to_user(&mut self) {
        self.role = Set("user".to_string());
    }
}
