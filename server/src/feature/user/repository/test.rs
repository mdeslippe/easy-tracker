use time::OffsetDateTime;

use crate::{
    common::utility::generate_random_string, config::Config, feature::user::model::User,
    injector::DependencyInjector,
};

/// # Description
///
/// Create a dependency injector.
///
/// # Returns
///
/// The dependency injector that was created.
async fn get_dependency_injector() -> DependencyInjector {
    // Load the server's configuration.
    let config: Config = Config::load_config(String::from("config.json"))
        .expect("Failed to load the server configuration");

    // Create a dependency injector.
    let dependency_injector: DependencyInjector = DependencyInjector::create_from_config(&config)
        .await
        .expect("Failed to create dependency injector");

    // Return the dependency injector.
    return dependency_injector;
}

/// # Description
///
/// Create a user with random user data.
///
/// # Returns
///
/// The user that was created with random user data.
fn create_test_user() -> User {
    return User {
        id: 0,
        account_created_at: OffsetDateTime::now_utc(),
        password_reset_at: OffsetDateTime::now_utc(),
        profile_picture_url: format!(
            "{}.com/{}.png",
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
