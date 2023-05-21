use crate::{
    common::{enumeration::QueryContext, utility::generate_random_string},
    config::Config,
    feature::user::{model::User, repository::UserRepository},
    injector::DependencyInjector,
};
use shaku::HasComponent;
use sqlx::Connection;
use sqlx::{mysql::MySqlPoolOptions, pool::PoolConnection, MySql, Pool};
use std::sync::Arc;
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

/// # Description
///
/// Test inserting a user into the repository, and validate the id returned.
#[actix_web::test]
async fn users_id_is_returned_after_insertion() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Create a test user.
    let test_user: User = create_test_user();

    // Insert the user.
    let id = user_repository
        .insert(&test_user, &mut context)
        .await
        .expect("Failed to insert user");

    // Make sure the user's id was returned.
    assert!(id > 0);

    // Delete the test user.
    let rows_deleted = user_repository
        .delete(&id, &mut context)
        .await
        .expect("Failed to delete test user");

    assert!(rows_deleted > 0);

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description
///
/// Test querying a user with an id that should not exist.
#[actix_web::test]
async fn querying_a_user_by_id_that_does_not_exist_should_return_none() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Execute the query.
    let result = user_repository
        .get_by_id(&0, &mut context)
        .await
        .expect("Failed to execute user query");

    // Make sure no user was returned.
    assert!(result.is_none());

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description.
///
/// Test querying a user with by id that should exist.
#[actix_web::test]
async fn querying_a_user_by_id_that_exists_should_return_the_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Create a test user.
    let mut test_user: User = create_test_user();

    // Insert the user.
    let id = user_repository
        .insert(&test_user, &mut context)
        .await
        .expect("Failed to insert user");

    // Set the user's id.
    test_user.id = id;

    // Make sure the user's id was returned.
    assert!(id > 0);

    // Query the user.
    let query_result = user_repository
        .get_by_id(&id, &mut context)
        .await
        .expect("Failed to query user");

    // Extract the user.
    let queried_user = query_result.expect("Query did not return user");

    // Make sure the user's data is correct.
    assert!(test_user == queried_user);

    // Delete the test user.
    let rows_deleted = user_repository
        .delete(&id, &mut context)
        .await
        .expect("Failed to delete test user");

    assert!(rows_deleted > 0);

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description
///
/// Test querying a user with a username that should not exist.
#[actix_web::test]
async fn querying_a_user_by_username_that_does_not_exist_should_return_none() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Execute the query.
    let result = user_repository
        .get_by_username(&generate_random_string(32), &mut context)
        .await
        .expect("Failed to execute user query");

    // Make sure no user was returned.
    assert!(result.is_none());

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description.
///
/// Test querying a user with username that should exist.
#[actix_web::test]
async fn querying_a_user_by_username_that_exists_should_return_the_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Create a test user.
    let mut test_user: User = create_test_user();

    // Insert the user.
    let id = user_repository
        .insert(&test_user, &mut context)
        .await
        .expect("Failed to insert user");

    // Set the user's id.
    test_user.id = id;

    // Make sure the user's id was returned.
    assert!(id > 0);

    // Query the user.
    let query_result = user_repository
        .get_by_username(&test_user.username, &mut context)
        .await
        .expect("Failed to query user");

    // Extract the user.
    let queried_user = query_result.expect("Query did not return user");

    // Make sure the user's data is correct.
    assert!(test_user == queried_user);

    // Delete the test user.
    let rows_deleted = user_repository
        .delete(&id, &mut context)
        .await
        .expect("Failed to delete test user");

    assert!(rows_deleted > 0);

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description
///
/// Test querying a user with a email address that should not exist.
#[actix_web::test]
async fn querying_a_user_by_email_that_does_not_exist_should_return_none() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Execute the query.
    let result = user_repository
        .get_by_email(
            &format!(
                "{}@{}.{}",
                generate_random_string(10),
                generate_random_string(10),
                generate_random_string(10)
            ),
            &mut context,
        )
        .await
        .expect("Failed to execute user query");

    // Make sure no user was returned.
    assert!(result.is_none());

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description.
///
/// Test querying a user with email address that should exist.
#[actix_web::test]
async fn querying_a_user_by_email_that_exists_should_return_the_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Create a test user.
    let mut test_user: User = create_test_user();

    // Insert the user.
    let id = user_repository
        .insert(&test_user, &mut context)
        .await
        .expect("Failed to insert user");

    // Set the user's id.
    test_user.id = id;

    // Make sure the user's id was returned.
    assert!(id > 0);

    // Query the user.
    let query_result = user_repository
        .get_by_email(&test_user.email, &mut context)
        .await
        .expect("Failed to query user");

    // Extract the user.
    let queried_user = query_result.expect("Query did not return user");

    // Make sure the user's data is correct.
    assert!(test_user == queried_user);

    // Delete the test user.
    let rows_deleted = user_repository
        .delete(&id, &mut context)
        .await
        .expect("Failed to delete test user");

    assert!(rows_deleted > 0);

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description
///
/// Test updating a user that does not exist, and make sure no records were modified.
#[actix_web::test]
async fn updating_a_user_that_does_not_exist_does_not_modify_any_records() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Query the user id to make sure no user exists.
    let query_result = user_repository
        .get_by_id(&0, &mut context)
        .await
        .expect("Failed to query user");

    assert!(query_result.is_none());

    // Create an "updated" user.
    let mut user: User = User::default();
    user.id = 0;

    // Attempt to update the user that does not exist.
    let rows_updated = user_repository
        .update(&user, &mut context)
        .await
        .expect("Failed to update user");

    assert_eq!(rows_updated, 0);

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description
///
/// Test updating a user that does exist, and verify the values were
/// successfully updated.
#[actix_web::test]
async fn updating_a_user_that_does_exist_modifies_their_record() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Create a test user.
    let mut test_user: User = create_test_user();

    // Insert the user.
    let id = user_repository
        .insert(&test_user, &mut context)
        .await
        .expect("Failed to insert user");

    // Set the user's id.
    test_user.id = id;

    // Make sure the user's id was returned.
    assert!(id > 0);

    // Query the user.
    let query_result = user_repository
        .get_by_id(&id, &mut context)
        .await
        .expect("Failed to query user");

    // Extract the user.
    let queried_user = query_result.expect("Query did not return user");

    // Make sure the user's data is correct.
    assert!(test_user == queried_user);

    // Create an update version of the user.
    let mut user_with_updated_information: User = create_test_user();
    user_with_updated_information.id = id;
    user_with_updated_information.email_is_verified = !test_user.email_is_verified;
    user_with_updated_information.password_reset_is_required =
        !test_user.password_reset_is_required;
    user_with_updated_information.account_is_locked = !test_user.account_is_locked;
    user_with_updated_information.account_is_banned = !test_user.account_is_banned;

    // Make sure the updated information is different from the original.
    assert!(queried_user != user_with_updated_information);

    // Perform the update.
    let rows_updated = user_repository
        .update(&user_with_updated_information, &mut context)
        .await
        .expect("Failed to perform update");

    // Make sure one row was modified.
    assert_eq!(rows_updated, 1);

    // Query the updated user.
    let updated_query_result = user_repository
        .get_by_id(&id, &mut context)
        .await
        .expect("Failed to query updated user");

    // Extract the user.
    let updated_queried_user = updated_query_result.expect("Query did not return updated user");

    // Make sure the user's data is correct.
    assert!(queried_user != updated_queried_user);
    assert!(user_with_updated_information == updated_queried_user);

    // Delete the test user.
    let rows_deleted = user_repository
        .delete(&id, &mut context)
        .await
        .expect("Failed to delete test user");

    assert!(rows_deleted > 0);

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description
///
/// Test deleting a user that does not exist, and make sure no records are deleted.
#[actix_web::test]
async fn deleting_a_user_that_does_not_exist_does_not_modify_any_records() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Query the user id to make sure no user exists.
    let query_result = user_repository
        .get_by_id(&0, &mut context)
        .await
        .expect("Failed to query user");

    assert!(query_result.is_none());

    // Test deleting the user that does not exist.
    let rows_deleted = user_repository
        .delete(&0, &mut context)
        .await
        .expect("Failed to delete test user");

    assert_eq!(rows_deleted, 0);

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}

/// # Description
///
/// Test deleting a user from the repository and make sure they cannot be queried again.
#[actix_web::test]
async fn users_cannot_be_queried_after_being_deleted() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = injector.resolve();

    // Acquire a database connection.
    let mut connection = get_database_connection().await;

    // Start a transaction.
    let transaction = connection
        .begin()
        .await
        .expect("Failed to start transaction");

    // Create a query context.
    let mut context: QueryContext = QueryContext::Transaction(transaction);

    // Create a test user.
    let mut test_user: User = create_test_user();

    // Insert the user.
    let id = user_repository
        .insert(&test_user, &mut context)
        .await
        .expect("Failed to insert user");

    // Set the user's id.
    test_user.id = id;

    // Make sure the user's id was returned.
    assert!(id > 0);

    // Query the user.
    let query_result = user_repository
        .get_by_id(&id, &mut context)
        .await
        .expect("Failed to query user");

    // Extract the user.
    let queried_user = query_result.expect("Query did not return user");

    // Make sure the user's data is correct.
    assert!(test_user == queried_user);

    // Delete the test user.
    let rows_deleted = user_repository
        .delete(&id, &mut context)
        .await
        .expect("Failed to delete test user");

    assert!(rows_deleted > 0);

    // Query the deleted user.
    let deleted_query_result = user_repository
        .get_by_id(&id, &mut context)
        .await
        .expect("Failed to query user");

    // Extract the user.
    assert!(deleted_query_result.is_none());

    // Rollback the transaction.
    match context {
        QueryContext::Transaction(transaction) => transaction.rollback().await,
        QueryContext::Connection(_) => unreachable!(),
    }
    .expect("Failed to roll transaction back");
}
