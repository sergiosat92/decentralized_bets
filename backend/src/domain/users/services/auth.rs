//! 📦 AUTHENTICATION SERVICES
//! =====================
//!
//! Authentication service handlers for user-related operations.
//!
//! This module contains async handlers to manage user registration, login,
//! password resets, email verification, and OAuth login (Google).
//!
//! Each handler receives HTTP requests, interacts with the database,
//! performs business logic via domain traits, and returns appropriate HTTP responses.
//!
//! Handlers:
//! - `register_handler`: Registers a new user and returns a JWT token.
//! - `login_handler`: Authenticates an existing user with email and password.
//! - `google_login_handler`: Authenticates or registers users via Google OAuth token.
//! - `forgot_password_handler`: Initiates a password reset process by generating a reset token.
//! - `reset_password_handler`: Resets the user password given a valid token.
//! - `verify_email_handler`: Verifies the user email with a token and activates the account.

use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use uuid::Uuid;

use crate::{
    domain::users::{
        dtos::auth::{
            ForgotPasswordInput, GoogleLoginInput, LoginInput, LoginOutput, RegisterInput, ResetPasswordInput,
            VerifyEmailInput,
        },
        repositories::find_user_by_email,
        traits::create_new_user,
    },
    infrastructure::{
        observability::logs::{logger, ErrorOutput},
        web::oauth::check_google_token,
    },
};

/// Handler for user registration.
///
/// Checks if the email is already registered, creates a new user, saves it in the DB,
/// generates a JWT token, and returns it.
///
/// Returns:
/// - `201 Created` with JWT token on success.
/// - `409 Conflict` if email is already registered.
/// - `500 Internal Server Error` on failures.
pub async fn register_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(input): Json<RegisterInput>,
) -> Result<(StatusCode, Json<LoginOutput>), (StatusCode, Json<ErrorOutput>)> {
    let exist = match find_user_by_email(&db, &input.email).await {
        Ok(user) => Some(user),
        Err(_) => None,
    };

    if exist.is_some() {
        let message = "Email already registered";
        logger(tracing::Level::WARN, "register_handler", message);
        return Err((StatusCode::CONFLICT, Json(ErrorOutput { message: message.to_string() })));
    }

    let mut new_user = match create_new_user(
        input.email,
        input.username,
        input.password,
        Some(input.first_name),
        Some(input.last_name),
    ) {
        Ok(user) => user,
        Err(e) => {
            let message = format!("Failed to create user: {}", e);
            logger(tracing::Level::ERROR, "register_handler", &message);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message })));
        }
    };

    new_user.record_login();

    let token = match new_user.generate_jwt() {
        Ok(token) => token,
        Err(e) => {
            logger(tracing::Level::ERROR, "register_handler", &format!("Failed to generate JWT: {}", e));
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e })));
        }
    };

    new_user
        .insert(&db)
        .await
        .map_err(|e| {
            logger(tracing::Level::ERROR, "register_handler", &format!("Failed to save user: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e.to_string() }))
        })?;

    Ok((StatusCode::CREATED, Json(LoginOutput { token })))
}

/// Handler for user login.
///
/// Validates credentials, checks if the account is locked, updates login attempts,
/// and returns a JWT token on success.
///
/// Returns:
/// - `200 OK` with JWT token on success.
/// - `401 Unauthorized` if credentials are invalid.
/// - `403 Forbidden` if account is locked.
/// - `404 Not Found` if user does not exist.
pub async fn login_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(input): Json<LoginInput>,
) -> Result<(StatusCode, Json<LoginOutput>), (StatusCode, Json<ErrorOutput>)> {
    let mut user = match find_user_by_email(&db, &input.email).await {
        Ok(user) => user.into_active_model(),
        Err(e) => return Err((StatusCode::NOT_FOUND, Json(ErrorOutput { message: e.to_string() }))),
    };

    if user.is_account_locked() {
        let message = "Account is locked due to too many failed login attempts";
        logger(tracing::Level::WARN, "login_handler", message);
        return Err((StatusCode::FORBIDDEN, Json(ErrorOutput { message: message.to_string() })));
    }

    if !user.verify_password(&input.password) {
        user.record_failed_login();

        user.update(&db)
            .await
            .map_err(|e| {
                logger(tracing::Level::ERROR, "login_handler", &format!("Failed to update user: {}", e));
                (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e.to_string() }))
            })?;

        let message = "Invalid email or password";
        logger(tracing::Level::WARN, "login_handler", message);
        return Err((StatusCode::UNAUTHORIZED, Json(ErrorOutput { message: message.to_string() })));
    };

    user.record_login();

    let token = match user.generate_jwt() {
        Ok(token) => token,
        Err(e) => {
            logger(tracing::Level::ERROR, "login_handler", &format!("Failed to generate JWT: {}", e));
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e })));
        }
    };

    Ok((StatusCode::OK, Json(LoginOutput { token })))
}

/// Handler for Google OAuth login.
///
/// Validates the Google token, checks if the user exists, creates new user if needed,
/// and returns a JWT token.
///
/// Returns:
/// - `200 OK` with JWT token on success.
/// - `401 Unauthorized` if Google token is invalid.
/// - `500 Internal Server Error` on DB or JWT failures.
pub async fn google_login_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(input): Json<GoogleLoginInput>,
) -> Result<(StatusCode, Json<LoginOutput>), (StatusCode, Json<ErrorOutput>)> {
    let user_google = check_google_token(&input.google_token).await.map_err(|e| {
        logger(tracing::Level::ERROR, "google_login_handler", &format!("Failed to check Google token: {}", e));
        (StatusCode::UNAUTHORIZED, Json(ErrorOutput { message: e }))
    })?;

    let mut user = match find_user_by_email(&db, &user_google.email).await {
        Ok(user) => user.into_active_model(),
        Err(_) => {
            let mut new_user = create_new_user(
                user_google.email.clone(),
                user_google.name.unwrap_or(Uuid::new_v4().to_string()),
                "google_login".to_string(),
                user_google.given_name,
                user_google.family_name,
            )
            .map_err(|e| {
                logger(tracing::Level::ERROR, "google_login_handler", &format!("Failed to create user: {}", e));
                (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e.to_string() }))
            })?;

            new_user.record_login();

            let token = match new_user.generate_jwt() {
                Ok(token) => token,
                Err(e) => {
                    logger(tracing::Level::ERROR, "google_login_handler", &format!("Failed to generate JWT: {}", e));
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e })));
                }
            };

            new_user
                .insert(&db)
                .await
                .map_err(|e| {
                    logger(tracing::Level::ERROR, "google_login_handler", &format!("Failed to save user: {}", e));
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e.to_string() }))
                })?;

            return Ok((StatusCode::CREATED, Json(LoginOutput { token })));
        }
    };

    if user.is_account_locked() {
        let message = "Account is locked due to too many failed login attempts";
        logger(tracing::Level::WARN, "login_handler", message);
        return Err((StatusCode::FORBIDDEN, Json(ErrorOutput { message: message.to_string() })));
    }

    user.record_login();

    let token = match user.generate_jwt() {
        Ok(token) => token,
        Err(e) => {
            logger(tracing::Level::ERROR, "login_handler", &format!("Failed to generate JWT: {}", e));
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e })));
        }
    };

    Ok((StatusCode::OK, Json(LoginOutput { token })))
}

/// Handler to initiate a password reset.
///
/// Generates a password reset token and saves it to the user record.
///
/// Returns:
/// - `200 OK` with reset token.
/// - `403 Forbidden` if account is locked.
/// - `404 Not Found` if user does not exist.
pub async fn forgot_password_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(input): Json<ForgotPasswordInput>,
) -> Result<(StatusCode, Json<LoginOutput>), (StatusCode, Json<ErrorOutput>)> {
    let mut user = match find_user_by_email(&db, &input.email).await {
        Ok(user) => user.into_active_model(),
        Err(e) => return Err((StatusCode::NOT_FOUND, Json(ErrorOutput { message: e.to_string() }))),
    };

    if user.is_account_locked() {
        let message = "Account is locked due to too many failed login attempts";
        logger(tracing::Level::WARN, "login_handler", message);
        return Err((StatusCode::FORBIDDEN, Json(ErrorOutput { message: message.to_string() })));
    }

    let token = user.generate_password_reset_token();

    user.update(&db)
        .await
        .map_err(|e| {
            logger(tracing::Level::ERROR, "forgot_password_handler", &format!("Failed to update user: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e.to_string() }))
        })?;

    Ok((StatusCode::OK, Json(LoginOutput { token })))
}

/// Handler to reset password given a valid reset token.
///
/// Verifies token, updates password, and returns new JWT.
///
/// Returns:
/// - `200 OK` with JWT token.
/// - `400 Bad Request` if token invalid or expired.
/// - `403 Forbidden` if account locked.
/// - `404 Not Found` if user not found.
pub async fn reset_password_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(input): Json<ResetPasswordInput>,
) -> Result<(StatusCode, Json<LoginOutput>), (StatusCode, Json<ErrorOutput>)> {
    let mut user = match find_user_by_email(&db, &input.email).await {
        Ok(user) => user.into_active_model(),
        Err(e) => return Err((StatusCode::NOT_FOUND, Json(ErrorOutput { message: e.to_string() }))),
    };

    if user.is_account_locked() {
        let message = "Account is locked due to too many failed login attempts";
        logger(tracing::Level::WARN, "reset_password_handler", message);
        return Err((StatusCode::FORBIDDEN, Json(ErrorOutput { message: message.to_string() })));
    }

    match user.reset_password(&input.token, &input.new_password) {
        Ok(_) => {}
        Err(e) => {
            logger(tracing::Level::ERROR, "reset_password_handler", &format!("Failed to reset password: {}", e));
            return Err((StatusCode::BAD_REQUEST, Json(ErrorOutput { message: e })));
        }
    }

    let token = match user.generate_jwt() {
        Ok(token) => token,
        Err(e) => {
            logger(tracing::Level::ERROR, "reset_password_handler", &format!("Failed to generate JWT: {}", e));
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e })));
        }
    };

    user.update(&db)
        .await
        .map_err(|e| {
            logger(tracing::Level::ERROR, "reset_password_handler", &format!("Failed to update user: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e.to_string() }))
        })?;

    Ok((StatusCode::OK, Json(LoginOutput { token })))
}

/// Handler to verify email using token.
///
/// Checks if email already verified, validates token, marks user verified,
/// and returns JWT token.
///
/// Returns:
/// - `200 OK` with JWT token.
/// - `400 Bad Request` if token invalid or expired.
/// - `404 Not Found` if user not found.
/// - `409 Conflict` if email already verified.
pub async fn verify_email_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(input): Json<VerifyEmailInput>,
) -> Result<(StatusCode, Json<LoginOutput>), (StatusCode, Json<ErrorOutput>)> {
    let mut user = match find_user_by_email(&db, &input.email).await {
        Ok(user) => user.into_active_model(),
        Err(e) => return Err((StatusCode::NOT_FOUND, Json(ErrorOutput { message: e.to_string() }))),
    };

    if *user.is_verified.as_ref() {
        let message = "Email already verified";
        logger(tracing::Level::WARN, "verify_email_handler", message);
        return Err((StatusCode::CONFLICT, Json(ErrorOutput { message: message.to_string() })));
    }

    if !user.verify_account(&input.token) {
        let message = "Invalid or expired verification token";
        logger(tracing::Level::WARN, "verify_email_handler", message);
        return Err((StatusCode::BAD_REQUEST, Json(ErrorOutput { message: message.to_string() })));
    }

    let token = match user.generate_jwt() {
        Ok(token) => token,
        Err(e) => {
            logger(tracing::Level::ERROR, "verify_email_handler", &format!("Failed to generate JWT: {}", e));
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e })));
        }
    };

    user.update(&db)
        .await
        .map_err(|e| {
            logger(tracing::Level::ERROR, "verify_email_handler", &format!("Failed to update user: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorOutput { message: e.to_string() }))
        })?;

    Ok((StatusCode::OK, Json(LoginOutput { token })))
}
