
//! 🧪 AUTHENTICATION UNIT TESTING
//! =============================
//!
//! These tests cover password hashing and verification, login attempt tracking and lockout,
//! as well as generation and verification of email verification tokens.
//!
//! They ensure the user entity behaves correctly for common authentication flows.

#[cfg(test)]
mod user_auth_unit_tests {
    use crate::domain::users::traits::create_new_user;
    use sea_orm::Set;

    const TEST_EMAIL: &str = "test@gmail.com";
    const TEST_USERNAME: &str = "test_user";
    const TEST_PASSWORD: &str = "test_password";

    /// Test password hashing, verification, and update functionality.
    #[test]
    fn test_verify_and_update_password() {
        let mut user = create_new_user(
            TEST_EMAIL.into(),
            TEST_USERNAME.into(),
            TEST_PASSWORD.into(),
            None,
            None,
        )
        .expect("Failed to create user");

        // Check that email and username are set correctly.
        assert_eq!(user.email, Set(TEST_EMAIL.into()));
        assert_eq!(user.username, Set(TEST_USERNAME.into()));

        // Password hash should be present.
        assert!(user.password_hash.is_set());

        // First and last name fields explicitly set to None.
        assert!(user.first_name.is_set());
        assert_eq!(user.first_name.as_ref(), &None);
        assert!(user.last_name.is_set());
        assert_eq!(user.last_name.as_ref(), &None);

        // Password verification: correct password passes, wrong password fails.
        assert!(user.verify_password(TEST_PASSWORD));
        assert!(!user.verify_password("wrong_password"));

        // Updating password updates the hash and verification works with new password.
        user.update_password("newpass");
        assert!(user.verify_password("newpass"));
    }

    /// Test recording failed login attempts and account lockout logic.
    #[test]
    fn test_record_failed_login_and_lockout() {
        let mut user = create_new_user(
            TEST_EMAIL.into(),
            TEST_USERNAME.into(),
            TEST_PASSWORD.into(),
            Some("Test".into()),
            Some("User".into()),
        )
        .expect("Failed to create user");

        // Simulate 5 failed login attempts.
        for _ in 0..5 {
            user.record_failed_login();
        }

        // Account should be locked after threshold exceeded.
        assert!(user.is_account_locked());
    }

    /// Test generation and verification of email verification tokens.
    #[test]
    fn test_generate_and_verify_token() {
        let mut user = create_new_user(
            TEST_EMAIL.into(),
            TEST_USERNAME.into(),
            TEST_PASSWORD.into(),
            Some("Test".into()),
            Some("User".into()),
        )
        .expect("Failed to create user");

        let token = user.generate_verification_token();

        // Token verification should succeed.
        assert!(user.verify_account(&token));

        // Verification should clear the token.
        assert!(user.verification_token.as_ref().is_none());
    }
}
