//! 📦 AUTHENTICATION DTOs
//! =====================
//!
//! This module defines the Data Transfer Objects (DTOs) used in the authentication flow.
//! These structs represent the input and output data structures for various auth operations.
//!
//! ## Structs
//! - `RegisterInput`: Data required for user registration.
//! - `LoginInput`: Credentials for logging in.
//! - `LoginOutput`: Response containing the authentication token.
//! - `GoogleLoginInput`: Input for Google OAuth login using a Google token.
//! - `ForgotPasswordInput`: Input to initiate password reset via email.
//! - `ResetPasswordInput`: Input for resetting the password with a token.
//! - `VerifyEmailInput`: Input for email verification with a token.

use serde::{Deserialize, Serialize};

/// Input data required for registering a new user
#[derive(Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

/// Input data for user login
#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

/// Output data returned after successful login
#[derive(Serialize)]
pub struct LoginOutput {
    pub token: String,
}

/// Input data for logging in via Google OAuth
#[derive(Deserialize)]
pub struct GoogleLoginInput {
    pub google_token: String,
}

/// Input data for initiating a forgot password request
#[derive(Deserialize)]
pub struct ForgotPasswordInput {
    pub email: String,
}

/// Input data for resetting password
#[derive(Deserialize)]
pub struct ResetPasswordInput {
    pub email: String,
    pub token: String,
    pub new_password: String,
}

/// Input data for verifying user email address
#[derive(Deserialize)]
pub struct VerifyEmailInput {
    pub email: String,
    pub token: String,
}
