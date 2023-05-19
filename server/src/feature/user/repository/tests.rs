use crate::{
    common::{enumeration::QueryContext, utility::generate_random_string},
    config::Config,
    database::DatabaseConnectionFactory,
    feature::user::{model::User, repository::UserRepository},
    injector::DependencyInjector,
};
use shaku::HasComponent;
use std::sync::Arc;
use time::OffsetDateTime;

/// # Description
///
/// Create a dependency injector that can be used to inject services for testing.
///
/// # Returns
///
/// The dependency injector that was created.
async fn create_dependency_injector() -> DependencyInjector {
    // Load the server configuration.
    let config: Config = Config::load_config(String::from("config.json"))
        .expect("Failed to load config for testing");

    // Create a dependency injector.
    let dependency_injector: DependencyInjector = DependencyInjector::create_from_config(&config)
        .await
        .expect("Failed to create dependency injector for testing");

    // Return the dependency injector.
    return dependency_injector;
}

/// # Description
///
/// Create a query context that can be used for testing against the database.
///
/// # Arguments
///
/// `dependency_injector` - The dependency injector that will be used to inject services.
///
/// # Returns
///
/// The query context that was created.
async fn create_query_context(dependency_injector: &DependencyInjector) -> QueryContext {
    // Get the database connection factory.
    let connection_factory: Arc<dyn DatabaseConnectionFactory> = dependency_injector.resolve();

    // Create and return the query context.
    return QueryContext::Connection(
        connection_factory
            .get_connection()
            .await
            .expect("Failed to acquire database connection for testing"),
    );
}

/// # Description
///
/// Create a user for testing purposes.
///
/// # Returns
///
/// A user that was generated and can be used for testing purposes.
fn create_test_user() -> User {
    return User {
        id: 0,
        account_created_at: OffsetDateTime::now_utc(),
        password_reset_at: OffsetDateTime::now_utc(),
        profile_picture_url: format!("pictures.com/{}", generate_random_string(16)),
        username: generate_random_string(16),
        password: generate_random_string(16),
        email: format!("{}@email.com", generate_random_string(16)),
        email_is_verified: false,
        password_reset_is_required: false,
        account_is_locked: false,
        account_is_banned: false,
    };
}

/// # Description
///
/// Test querying a user by id that does not exist, and make sure no user is returned.
#[actix_web::test]
async fn get_by_id_returns_none_for_an_id_that_does_not_exist() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Make sure the query does not return a user, for an id that does not exist.
    assert!(user_repository
        .get_by_id(&0, &mut context)
        .await
        .expect("Failed to query user")
        .is_none());
}

/// # Description
///
/// Test querying a user by username that does not exist, and make sure no user is returned.
#[actix_web::test]
async fn get_by_username_returns_none_for_a_username_that_does_not_exist() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Make sure the query does not return a user, for a username that does not exist.
    assert!(user_repository
        .get_by_username(&generate_random_string(16), &mut context)
        .await
        .expect("Failed to query user")
        .is_none());
}

/// # Description
///
/// Test querying a user by email that does not exist, and make sure no user is returned.
#[actix_web::test]
async fn get_by_email_returns_none_for_an_email_that_does_not_exist() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Make sure the query does not return a user, for an email that does not exist.
    assert!(user_repository
        .get_by_email(
            &format!("{}@email.com", generate_random_string(16)),
            &mut context
        )
        .await
        .expect("Failed to query user")
        .is_none());
}

/// # Description
///
/// Test updating a user that does not exist, and make sure no records were modified.
#[actix_web::test]
async fn updating_a_user_that_does_not_exist_modifies_no_records() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Create a fake user that does not exist, and attempt to perform an update on that user.
    let user: User = create_test_user();

    // Attempt to perform the update on the user that does not exist, and make sure no records were
    // modified.
    assert_eq!(
        user_repository
            .update(&user, &mut context)
            .await
            .expect("Failed to update user"),
        0
    );
}

/// # Description
///
/// Test deleting a user that does not exist, and make sure no records were deleted.
#[actix_web::test]
async fn deleting_a_user_that_does_not_exist_deletes_no_records() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Attempt to perform the deletion on the user that does not exist, and make sure no records
    // were deleted.
    assert_eq!(
        user_repository
            .delete(&0, &mut context)
            .await
            .expect("Failed to update user"),
        0
    );
}

/// # Description
///
/// Test inserting a user into the user repository, and querying the user by the id returned from
/// the insertion.
#[actix_web::test]
async fn users_can_be_queried_by_id_after_inserting() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Create a test user.
    let mut user: User = create_test_user();

    // Insert the user into the repository.
    let returned_user_id: u64 = user_repository
        .insert(&user, &mut context)
        .await
        .expect("Failed to insert user");

    // Update the test user's id.
    user.id = returned_user_id;

    // Attempt to query the user that was inserted.
    let queried_user: User = user_repository
        .get_by_id(&returned_user_id, &mut context)
        .await
        .expect("Failed to query user after inserting")
        .expect("User was not found after inserting");

    // Make sure the return users information is correct.
    assert_eq!(user, queried_user);

    // Delete the test user.
    let records_deleted: u64 = user_repository
        .delete(&returned_user_id, &mut context)
        .await
        .expect("Failed to delete user after inserting");

    // Make sure the user was deleted.
    assert_eq!(records_deleted, 1);
}

/// # Description
///
/// Testing query a user by their username after performing an insertion.
#[actix_web::test]
async fn users_can_be_queried_by_username_after_inserting() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Create a test user.
    let mut user: User = create_test_user();

    // Insert the user into the repository.
    let returned_user_id: u64 = user_repository
        .insert(&user, &mut context)
        .await
        .expect("Failed to insert user");

    // Update the test user's id.
    user.id = returned_user_id;

    // Attempt to query the user that was inserted.
    let queried_user: User = user_repository
        .get_by_username(&user.username, &mut context)
        .await
        .expect("Failed to query user after inserting")
        .expect("User was not found after inserting");

    // Make sure the return users information is correct.
    assert_eq!(user, queried_user);

    // Delete the test user.
    let records_deleted: u64 = user_repository
        .delete(&returned_user_id, &mut context)
        .await
        .expect("Failed to delete user after inserting");

    // Make sure the user was deleted.
    assert_eq!(records_deleted, 1);
}

/// # Description
///
/// Testing query a user by their email address after performing an insertion.
#[actix_web::test]
async fn users_can_be_queried_by_email_after_inserting() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Create a test user.
    let mut user: User = create_test_user();

    // Insert the user into the repository.
    let returned_user_id: u64 = user_repository
        .insert(&user, &mut context)
        .await
        .expect("Failed to insert user");

    // Update the test user's id.
    user.id = returned_user_id;

    // Attempt to query the user that was inserted.
    let queried_user: User = user_repository
        .get_by_email(&user.email, &mut context)
        .await
        .expect("Failed to query user after inserting")
        .expect("User was not found after inserting");

    // Make sure the return users information is correct.
    assert_eq!(user, queried_user);

    // Delete the test user.
    let records_deleted: u64 = user_repository
        .delete(&returned_user_id, &mut context)
        .await
        .expect("Failed to delete user after inserting");

    // Make sure the user was deleted.
    assert_eq!(records_deleted, 1);
}

/// # Description
///
/// Test updating users after performing an insertion.
#[actix_web::test]
async fn users_can_be_updated_after_inserting() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Create a test user.
    let mut user: User = create_test_user();

    // Create a user with updated values.
    let mut updated_user: User = create_test_user();
    updated_user.email_is_verified = !user.email_is_verified;
    updated_user.password_reset_is_required = !user.password_reset_is_required;
    updated_user.account_is_locked = !user.account_is_locked;
    updated_user.account_is_banned = !user.account_is_banned;

    // Insert the user into the repository.
    let returned_user_id: u64 = user_repository
        .insert(&user, &mut context)
        .await
        .expect("Failed to insert user");

    // Update the id of the test users.
    user.id = returned_user_id;
    updated_user.id = returned_user_id;

    // Attempt to query the user that was inserted.
    let queried_user: User = user_repository
        .get_by_id(&returned_user_id, &mut context)
        .await
        .expect("Failed to query user after inserting")
        .expect("User was not found after inserting");

    // Make sure the return users information is correct.
    assert_eq!(queried_user, user);
    assert_ne!(queried_user, updated_user);

    // Perform the update and make sure 1 record was modified.
    assert_eq!(
        user_repository
            .update(&updated_user, &mut context)
            .await
            .expect("Failed to update user"),
        1
    );

    // Attempt to query the updated user.
    let queried_updated_user: User = user_repository
        .get_by_id(&returned_user_id, &mut context)
        .await
        .expect("Failed to query user after inserting")
        .expect("User was not found after inserting");

    // Make sure the users information was successfully updated.
    assert_eq!(queried_updated_user, updated_user);
    assert_ne!(queried_updated_user, user);

    // Delete the test user.
    assert_eq!(
        user_repository
            .delete(&returned_user_id, &mut context)
            .await
            .expect("Failed to delete user after inserting"),
        1
    );
}

/// # Description
///
/// Test deleting users after performing an insertion.
#[actix_web::test]
async fn users_can_be_deleted_after_inserting() {
    // Create a dependency injector.
    let dependency_injector: DependencyInjector = create_dependency_injector().await;

    // Get a user repository instance.
    let user_repository: Arc<dyn UserRepository> = dependency_injector.resolve();

    // Create a query context.
    let mut context: QueryContext = create_query_context(&dependency_injector).await;

    // Create a test user.
    let user: User = create_test_user();

    // Insert the user into the repository.
    let returned_user_id: u64 = user_repository
        .insert(&user, &mut context)
        .await
        .expect("Failed to insert user");

    // Make sure the users exists.
    user_repository
        .get_by_id(&returned_user_id, &mut context)
        .await
        .expect("Failed to query user after inserting")
        .expect("User was not found after inserting");

    // Delete the test user.
    let records_deleted: u64 = user_repository
        .delete(&returned_user_id, &mut context)
        .await
        .expect("Failed to delete user after inserting");

    // Make sure the user was deleted.
    assert_eq!(records_deleted, 1);
}
