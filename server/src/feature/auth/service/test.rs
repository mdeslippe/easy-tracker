use crate::{
    common::{
        enumeration::{
            AuthenticationResult, DeletionResult, InsertionResult, QueryResult, UpdateResult,
        },
        utility::generate_random_string,
    },
    config::Config,
    feature::{
        auth::service::AuthService,
        crypto::service::CryptoService,
        user::{model::User, service::UserService},
    },
    injector::DependencyInjector,
};
use shaku::HasComponent;
use std::{sync::Arc, thread, time::Duration};
use time::OffsetDateTime;

/// # Description
///
/// A function to create a user that can be used for testing.
///
/// # Returns
///
/// The user that was created.
fn create_test_user() -> User {
    return User {
        id: 0,
        account_created_at: OffsetDateTime::now_utc(),
        password_reset_at: OffsetDateTime::now_utc(),
        profile_picture_url: format!(
            "https://{}.com/{}.png",
            generate_random_string(8),
            generate_random_string(8)
        ),
        username: generate_random_string(8),
        password: generate_random_string(8),
        email: format!(
            "{}@{}.com",
            generate_random_string(8),
            generate_random_string(8)
        ),
        email_is_verified: false,
        password_reset_is_required: false,
        account_is_locked: false,
        account_is_banned: false,
    };
}

/// # Description
///
/// A utility function to load the server's configuration file.
///
/// # Panics
///
/// This function will panic if an error occurs while attempting to load or parse the configuration
/// file.
///
/// # Returns
///
/// The server's configuration.
fn load_config() -> Config {
    return Config::load_config(String::from("config.json")).expect("Failed to load config");
}

/// # Description
///
/// Create a dependency injector.
///
/// # Panics
///
/// This panics if there was an error creating the dependency injector.
///
/// # Returns
///
/// The dependency injector that was created.
async fn create_dependency_injector() -> DependencyInjector {
    // Load the config.
    let config: Config = load_config();

    return DependencyInjector::create_from_config(&config)
        .await
        .expect("Failed to load dependency injector");
}

/// # Description
///
/// Test authenticating a user via login credentials that are invalid, and ensure authentication is
/// not successful.
#[actix_web::test]
async fn authenticating_a_user_that_does_not_exist_via_credentials_fails() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Get an auth service instance.
    let auth_service: Arc<dyn AuthService> = injector.resolve();

    // Create a test user.
    let user: User = create_test_user();

    // Make sure the user does not exist.
    match user_service.get_by_username(&user.username).await {
        QueryResult::Ok(user) => panic!(
            "Query result returned a user when it should have returned NotFound: {:?}",
            user
        ),
        QueryResult::NotFound => {}
        QueryResult::Err(error) => panic!(
            "An error occurred while attempting to query a user that should not exist: {:?}",
            error
        ),
    }

    // Attempt to authenticate the user.
    match auth_service
        .authenticate_credentials(&user.username, &user.password)
        .await
    {
        AuthenticationResult::Ok(user) => panic!(
            "Authentication successfully authenticated a user that should not exist: {:?}",
            user
        ),
        AuthenticationResult::NotAuthenticated => {}
        AuthenticationResult::Err(error) => panic!(
            "An error occurred while attempting to perform authentication: {:?}",
            error
        ),
    };
}

/// # Description
///
/// Test authenticating a user via login credentials that are valid, and ensure authentication is
/// successful.
#[actix_web::test]
async fn authenticating_a_user_that_exists_via_credentials_succeeds() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Get an auth service instance.
    let auth_service: Arc<dyn AuthService> = injector.resolve();

    // Create a test user.
    let user: User = create_test_user();

    // Insert the user.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(user) => user,
        InsertionResult::Invalid(details) => panic!(
            "Inserting a test user failed, generated fields failed validation: {:?}",
            details
        ),
        InsertionResult::Err(error) => {
            panic!("Failed to insert test user, an error occurred: {:?}", error)
        }
    };

    // Make sure the user exists.
    match user_service.get_by_username(&user.username).await {
        QueryResult::Ok(_) => {}
        QueryResult::NotFound => panic!("A user that should exist could not be found."),
        QueryResult::Err(error) => panic!(
            "An error occurred while attempting to query a user that should exist: {:?}",
            error
        ),
    }

    // Attempt to authenticate the user.
    let authenticated_user: User = match auth_service
        .authenticate_credentials(&user.username, &user.password)
        .await
    {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => panic!("Failed to authenticate the user."),
        AuthenticationResult::Err(error) => panic!(
            "An error occurred while attempting to perform authentication: {:?}",
            error
        ),
    };

    // Make sure the authenticated user has the correct information.
    assert_eq!(inserted_user, authenticated_user);

    // Delete the test user.
    match user_service.delete(&inserted_user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => {
            panic!("Failed to delete test user: The user could not be found.")
        }
        DeletionResult::Err(error) => {
            panic!("Failed to delete test user, an error occurred: {:?}", error)
        }
    }
}

/// # Description
///
/// Test authenticating a user via token that is invalid, and ensure authentication is not successful.
#[actix_web::test]
async fn authenticating_a_user_that_does_not_exit_via_token_fails() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Get an auth service instance.
    let auth_service: Arc<dyn AuthService> = injector.resolve();

    // Create a test user.
    let user: User = create_test_user();

    // Make sure the user does not exist.
    match user_service.get_by_id(&user.id).await {
        QueryResult::Ok(user) => panic!(
            "Query result returned a user when it should have returned NotFound: {:?}",
            user
        ),
        QueryResult::NotFound => {}
        QueryResult::Err(error) => panic!(
            "An error occurred while attempting to query a user that should not exist: {:?}",
            error
        ),
    }

    // Generate a token for the user.
    let token: String = crypto_service
        .create_token(&user)
        .expect("Failed to create token for user");

    // Attempt to authenticate the user.
    match auth_service.authenticate_token(&token).await {
        AuthenticationResult::Ok(user) => panic!(
            "Authentication successfully authenticated a user that should not exist: {:?}",
            user
        ),
        AuthenticationResult::NotAuthenticated => {}
        AuthenticationResult::Err(error) => panic!(
            "An error occurred while attempting to authenticate a user token: {:?}",
            error
        ),
    };
}

/// # Description
///
/// Test authenticating a user via token that is valid and ensure authentication is successful.
#[actix_web::test]
async fn authenticating_a_user_that_exists_via_token_succeeds() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Get an auth service instance.
    let auth_service: Arc<dyn AuthService> = injector.resolve();

    // Create a test user.
    let user: User = create_test_user();

    // Insert the user.
    let user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(user) => user,
        InsertionResult::Invalid(details) => panic!(
            "Inserting a test user failed, generated fields failed validation: {:?}",
            details
        ),
        InsertionResult::Err(error) => {
            panic!("Failed to insert test user, an error occurred: {:?}", error)
        }
    };

    // Make sure the user exists.
    match user_service.get_by_username(&user.username).await {
        QueryResult::Ok(_) => {}
        QueryResult::NotFound => panic!("A user that should exist could not be found."),
        QueryResult::Err(error) => panic!(
            "An error occurred while attempting to query a user that should exist: {:?}",
            error
        ),
    }

    // Generate a token for the user.
    let token: String = crypto_service
        .create_token(&user)
        .expect("Failed to create token for user");

    // Attempt to authenticate the user.
    let user: User = match auth_service.authenticate_token(&token).await {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => panic!("Failed to authenticate the user"),
        AuthenticationResult::Err(error) => panic!(
            "An error occurred while attempting to authenticate a user token: {:?}",
            error
        ),
    };

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => {
            panic!("Failed to delete test user: The user could not be found.")
        }
        DeletionResult::Err(error) => {
            panic!("Failed to delete test user, an error occurred: {:?}", error)
        }
    }
}

/// # Description
///
/// Make sure existing tokens are expired after a user resets their password.
#[actix_web::test]
async fn updating_a_users_password_expires_existing_tokens() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Get an auth service instance.
    let auth_service: Arc<dyn AuthService> = injector.resolve();

    // Create a test user.
    let user: User = create_test_user();

    // Insert the user.
    let mut user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(user) => user,
        InsertionResult::Invalid(details) => panic!(
            "Inserting a test user failed, generated fields failed validation: {:?}",
            details
        ),
        InsertionResult::Err(error) => {
            panic!("Failed to insert test user, an error occurred: {:?}", error)
        }
    };

    // Make sure the user exists.
    match user_service.get_by_username(&user.username).await {
        QueryResult::Ok(_) => {}
        QueryResult::NotFound => panic!("A user that should exist could not be found."),
        QueryResult::Err(error) => panic!(
            "An error occurred while attempting to query a user that should exist: {:?}",
            error
        ),
    }

    // Generate a token for the user.
    let token: String = crypto_service
        .create_token(&user)
        .expect("Failed to create token for user");

    // Attempt to authenticate the user.
    match auth_service.authenticate_token(&token).await {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => panic!("Failed to authenticate the user"),
        AuthenticationResult::Err(error) => panic!(
            "An error occurred while attempting to authenticate a user token: {:?}",
            error
        ),
    };

    // Sleep for one second.
    thread::sleep(Duration::from_secs(1));

    // Update the user's password.
    user.password = String::from("NewUpdatedPassword!");

    let user = match user_service.update(&user).await {
        UpdateResult::Ok(user) => user,
        UpdateResult::NotFound => {
            panic!("Failed to update the user's password: The user could not be found.")
        }
        UpdateResult::Invalid(details) => panic!(
            "Failed to update the user's password, validation errors were found: {:?}",
            details
        ),
        UpdateResult::Err(error) => panic!(
            "Failed to update the user's password, an unexpected error occurred: {:?}",
            error
        ),
    };

    // Attempt to authenticate the user.
    match auth_service.authenticate_token(&token).await {
        AuthenticationResult::Ok(user) => panic!(
            "Authentication successfully authenticated a user after password reset, when it should have failed: {:?}",
            user
        ),
        AuthenticationResult::NotAuthenticated => {},
        AuthenticationResult::Err(error) => panic!(
            "An error occurred while attempting to authenticate a user token: {:?}",
            error
        ),
    };

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => {
            panic!("Failed to delete test user: The user could not be found.")
        }
        DeletionResult::Err(error) => {
            panic!("Failed to delete test user, an error occurred: {:?}", error)
        }
    }
}
