use core::panic;
use std::sync::Arc;

use shaku::HasComponent;
use sqlx::{pool::PoolConnection, Connection, MySql};
use time::OffsetDateTime;

use crate::{
    common::{
        enumeration::{DeletionResult, InsertionResult, QueryContext},
        utility::generate_random_string,
    },
    config::Config,
    database::DatabaseConnectionFactory,
    feature::{
        file::model::File,
        user::{model::User, service::UserService},
    },
    injector::DependencyInjector,
};

/// # Description
///
/// Create a user that can be used for testing.
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
/// Create a file that can be used for testing.
///
/// # Returns
///
/// The file that was created.
fn create_test_file() -> File {
    return File {
        id: 0,
        user_id: 0,
        file_created_at: OffsetDateTime::now_utc(),
        mime_type: String::from("text/plain"),
        name: format!("{}.txt", generate_random_string(8)),
        data: generate_random_string(1024).into_bytes(),
    };
}

/// # Description
///
/// Insert a test user with the user service.
///
/// # Arguments
///
/// `injector` - The dependency injector that will be used to acquire a user service instance.
///
/// `context` - The query context the user will be inserted in.
///
/// # Panics
///
/// This function will panic if an error occurs while attempting to insert the user with the user
/// service.
///
/// # Returns
///
/// The user that was inserted.
async fn insert_test_user(injector: &DependencyInjector, context: &mut QueryContext<'_>) -> User {
    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Perform the insertion.
    let user: User = match user_service
        .insert_with_context(&create_test_user(), context)
        .await
    {
        InsertionResult::Ok(user) => user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {}", error),
    };

    // Return the user.
    return user;
}

/// # Description
///
/// Delete a test user with the user service.
///
/// # Arguments
///
/// `user` - The user to delete.
///
/// `injector` - The dependency injector that will be used to acquire a user service instance.
///
/// `context` - The query context the user will be inserted in.
///
/// # Panics
///
/// This function will panic if an error occurs while attempting to delete the user with the user
/// service.
async fn delete_test_user(
    user: User,
    injector: &DependencyInjector,
    context: &mut QueryContext<'_>,
) {
    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Perform the deletion.
    match user_service.delete_with_context(&user.id, context).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete test user: User not found"),
        DeletionResult::Err(error) => panic!("Failed to delete test user: {}", error),
    }
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
    // Load the config.
    let config: Config =
        Config::load_config(String::from("config.json")).expect("Failed to load config");

    // Return the config.
    return config;
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

    // Create the dependency injector.
    let injector: DependencyInjector = DependencyInjector::create_from_config(&config)
        .await
        .expect("Failed to load dependency injector");

    // Return the dependency injector.
    return injector;
}

/// # Description
///
/// Acquire a database connection.
///
/// # Arguments
///
/// `injector` - The dependency injector that will be used to the database connection factory
/// instance.
///
/// # Panics
///
/// This function will panic if a database connection could not be created.
///
/// # Returns
///
/// The database connection that was created.
async fn get_database_connection(injector: &DependencyInjector) -> PoolConnection<MySql> {
    // Get the database connection factory.
    let connection_factory: Arc<dyn DatabaseConnectionFactory> = injector.resolve();

    // Acquire a database connection.
    let connection = connection_factory
        .get_connection()
        .await
        .expect("Failed to acquire a database connection");

    // Return the connection.
    return connection;
}
