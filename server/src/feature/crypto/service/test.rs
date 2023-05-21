use crate::{
    common::utility::generate_random_string,
    config::Config,
    feature::{
        crypto::{model::UserClaims, service::CryptoService},
        user::model::User,
    },
    injector::DependencyInjector,
};
use jsonwebtoken::TokenData;
use regex::Regex;
use shaku::HasComponent;
use std::sync::Arc;
use time::OffsetDateTime;

/// # Description
///
/// Check if a string is in the format that json web tokens should be.
///
/// # Arguments
///
/// `token` - The token to check.
///
/// # Panics
///
/// This function will panic if there is an internal error parsing the regex.
///
/// # Returns
///
/// If the token is a valid jwt or not.
fn is_valid_jwt(token: &String) -> bool {
    let jwt_regex = Regex::new(r"(^[\w-]+\.[\w-]+\.[\w-]+$)").expect("Failed to validate token");

    return jwt_regex.is_match(token.as_str());
}

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
/// This function will panic if an error occurs while attempting
/// to load or parse the configuration file.
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
/// Test creating an authentication token for a user.
#[actix_web::test]
async fn creating_a_token_returns_a_valid_jwt() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Create a test user.
    let user: User = create_test_user();

    // Generate the token.
    let token = crypto_service
        .create_token(&user)
        .expect("Failed to create token");

    assert!(is_valid_jwt(&token));
}

/// # Description
///
/// Make sure an error is returned when you attempt to decode an invalid token.
#[actix_web::test]
async fn decoding_an_invalid_token_returns_an_error() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Create a bad token.
    let bad_token = String::from("My.Bad.Token");

    // Attempt to decode the bad token.
    match crypto_service.decode_token(&bad_token) {
        Ok(data) => panic!(
            "Returned token data, when Err should have been returned: {:?}",
            data
        ),
        Err(_) => {}
    };
}

/// # Description
///
/// Test decoding a valid token, and make sure it contains the correct data.
#[actix_web::test]
async fn decoding_a_valid_token_contains_the_correct_information() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Create a test user.
    let user: User = create_test_user();

    // Generate the token.
    let token = crypto_service
        .create_token(&user)
        .expect("Failed to create token");

    assert!(is_valid_jwt(&token));

    // Decode the token.
    let token_data: TokenData<UserClaims> = match crypto_service.decode_token(&token) {
        Ok(data) => data,
        Err(error) => panic!("Failed to decode token, an error occurred: {:?}", error),
    };

    // Make sure the returned token contains correct data.
    assert_eq!(user.id, token_data.claims.id);
    assert_eq!(
        user.password_reset_at.unix_timestamp(),
        token_data.claims.password_last_reset
    );
}

/// # Description
///
/// Test hashing a password, and make sure the hash does not match the original password.
#[actix_web::test]
async fn hashing_a_password_returns_a_hash() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Create a password.
    let password: String = String::from("MyPassword");

    // Hash the password.
    let hash = crypto_service
        .hash_password(&password)
        .expect("Failed to hash password");

    // Make sure the hash returned is not the same as the password.
    assert_ne!(password, hash);
}

/// # Description
///
/// Test verifying an incorrect password, and make sure it fails.
#[actix_web::test]
async fn verifying_an_incorrect_password_fails() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Create a password.
    let password: String = String::from("MyPassword");

    // Create an incorrect password.
    let incorrect_password: String = String::from("NotMyPassword");

    // Hash the password.
    let hash = crypto_service
        .hash_password(&password)
        .expect("Failed to hash password");

    // Make sure the hash returned is not the same as the password.
    assert_ne!(password, hash);

    // Attempt to verify the incorrect password.
    crypto_service
        .verify_password(&incorrect_password, &hash)
        .expect_err("Password matched when it should not have");
}

/// # Description
///
/// Test verifying a correct password, and make sure it succeeds.
#[actix_web::test]
async fn verifying_a_correct_password_succeeds() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a crypto service instance.
    let crypto_service: Arc<dyn CryptoService> = injector.resolve();

    // Create a password.
    let password: String = String::from("MyPassword");

    // Hash the password.
    let hash = crypto_service
        .hash_password(&password)
        .expect("Failed to hash password");

    // Make sure the hash returned is not the same as the password.
    assert_ne!(password, hash);

    // Attempt to verify the correct password.
    crypto_service
        .verify_password(&password, &hash)
        .expect("Password failed when it should have been correct");
}
