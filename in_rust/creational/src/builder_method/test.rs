

#[cfg(test)]
mod builder_method_tests {
    use crate::builder_method::builder::{UserBuilderError, UserRole, UserBuilder, User};
    use tracing::{Level, info};
    use tracing_subscriber::fmt;

    // Setup inicial para configurar el logging
    fn setup_logging() {
        let _ = fmt()
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();
    }

    #[test]
    fn test_user_builder_success() {
        setup_logging();
        info!("Starting test_user_builder_success");
        let user: Result<User, UserBuilderError> = UserBuilder::new()
            .name("Alice")
            .email("alice@example.com")
            .age(30)
            .activo(true)
            .rolee(UserRole::Admin)
            .build(); 

        assert!(user.is_ok(),"User should be built successfully");
        let user = user.unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.age, Some(30));
        assert_eq!(user.activo, true);
        assert_eq!(matches!(user.rolee, UserRole::Admin), true);
    }


    #[test]
    fn test_user_builder_missing_name() {
        setup_logging();
        info!("Starting test_user_builder_missing_name");
        let user: Result<User, UserBuilderError> = UserBuilder::new()
            .email("alice@example.com")
            .age(30)
            .activo(true)
            .rolee(UserRole::Admin)
            .build();

        assert!(user.is_err(), "User should not be built successfully");
        let error: UserBuilderError = user.unwrap_err();
        assert_eq!(error, UserBuilderError::MissingField("name".into()));
    }

    #[test]
    fn test_user_builder_missing_email() {
        setup_logging();
        info!("Starting test_user_builder_missing_email");
        let user: Result<User, UserBuilderError> = UserBuilder::new()
            .name("Alice")
            .age(30)
            .activo(true)
            .rolee(UserRole::Admin)
            .build();   
        assert!(user.is_err(), "User should not be built successfully");
        let error: UserBuilderError = user.unwrap_err();
        assert_eq!(error, UserBuilderError::MissingField("email".into()));
    }

    #[test]
    fn test_user_builder_email_and_name_only() {
        setup_logging();
        info!("Starting test_user_builder_email_and_name_only");
        let user: Result<User, UserBuilderError> = UserBuilder::new()
            .name("Bob")
            .email("bob@example.com")
            .build();

        assert!(user.is_ok(), "User should be built successfully");
        let user = user.unwrap();
        assert_eq!(user.name, "Bob");
        assert_eq!(user.email, "bob@example.com");
        assert_eq!(user.age, None);
        assert_eq!(user.activo, false);
        assert_eq!(matches!(user.rolee, UserRole::Guest), true);
    }

}