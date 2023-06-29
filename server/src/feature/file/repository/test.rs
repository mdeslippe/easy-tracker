use sqlx::{mysql::MySqlPoolOptions, pool::PoolConnection, MySql, Pool};
use time::OffsetDateTime;

use crate::{
    common::utility::generate_random_string, config::Config, feature::user::model::User,
    injector::DependencyInjector,
};

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
/// Acquire a database connection.
///
/// # Panics
///
/// This function will panic if an error occurs while attempting
/// to load or parse the configuration file, or if an error occurs
/// while attempting to acquire a database connection.
///
/// # Returns
///
/// The database connection that was acquired.
async fn get_database_connection() -> PoolConnection<MySql> {
    // Load the config.
    let config: Config = load_config();

    // Create a connection pool.
    let database_connection_pool: Pool<MySql> = MySqlPoolOptions::new()
        .min_connections(config.database.minimum_connections)
        .max_connections(config.database.maximum_connections)
        .connect(&config.database.connection_string)
        .await
        .expect("Failed to connect to the database");

    // Acquire and return a connection.
    return database_connection_pool
        .acquire()
        .await
        .expect("Failed to acquire database connection");
}
