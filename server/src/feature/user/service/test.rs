use crate::{
    common::{
        enumeration::{DeletionResult, InsertionResult, QueryResult, UpdateResult},
        utility::generate_random_string,
    },
    config::Config,
    feature::user::model::User,
    feature::user::service::UserService,
    injector::DependencyInjector,
};
use core::panic;
use shaku::HasComponent;
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
/// Test inserting a valid user.
#[actix_web::test]
async fn inserting_a_user_returns_the_user_after_insertion() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let mut user: User = create_test_user();

    // Perform the insertion.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Make sure the user returned is correct.
    user.id = inserted_user.id;
    user.password = inserted_user.password.clone();
    assert_eq!(user, inserted_user);

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Test inserting a user whose username is already in use.
#[actix_web::test]
async fn inserting_a_user_with_username_that_is_already_in_use_returns_a_validation_error() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let mut user: User = create_test_user();

    // Perform the insertion.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Make sure the user returned is correct.
    user.id = inserted_user.id;
    user.password = inserted_user.password.clone();
    assert_eq!(user, inserted_user);

    // Create a new user that has the same username as the previous user.
    let mut bad_username_user: User = create_test_user();
    bad_username_user.username = user.username.clone();

    // Perform the insertion.
    match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => {
            panic!(
                "Inserted user whose username is already in use: {:?}",
                inserted_user
            )
        }
        InsertionResult::Invalid(_) => {}
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Test inserting a user whose email address is already in use.
#[actix_web::test]
async fn inserting_a_user_with_email_that_is_already_in_use_returns_a_validation_error() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let mut user: User = create_test_user();

    // Perform the insertion.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Make sure the user returned is correct.
    user.id = inserted_user.id;
    user.password = inserted_user.password.clone();
    assert_eq!(user, inserted_user);

    // Create a new user that has the same username as the previous user.
    let mut bad_username_user: User = create_test_user();
    bad_username_user.email = user.email.clone();

    // Perform the insertion.
    match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => {
            panic!(
                "Inserted user whose email is already in use: {:?}",
                inserted_user
            )
        }
        InsertionResult::Invalid(_) => {}
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Test querying a user by id that does not exist.
#[actix_web::test]
async fn querying_a_user_by_id_that_does_not_exist_should_not_return_a_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    match user_service.get_by_id(&0).await {
        QueryResult::Ok(user) => panic!(
            "Returned a user when it should have returned NotFound: {:?}",
            user
        ),
        QueryResult::NotFound => {}
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };
}

/// # Description
///
/// Test querying a user by id that does exist.
#[actix_web::test]
async fn querying_a_user_by_id_that_does_exist_returns_the_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let mut user: User = create_test_user();

    // Perform the insertion.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Make sure the user returned is correct.
    user.id = inserted_user.id;
    user.password = inserted_user.password.clone();
    assert_eq!(user, inserted_user);

    // Query the user that was inserted.
    let queried_user: User = match user_service.get_by_id(&user.id).await {
        QueryResult::Ok(queried_user) => queried_user,
        QueryResult::NotFound => panic!("User not found"),
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };

    // Make sure the queried user has the correct values.
    assert_eq!(queried_user, user);
    assert_eq!(queried_user, inserted_user);

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Test querying a user by username that does not exist.
#[actix_web::test]
async fn querying_a_user_by_username_that_does_not_exist_should_not_return_a_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    match user_service
        .get_by_username(&String::from("My_Invalid_Username"))
        .await
    {
        QueryResult::Ok(user) => panic!(
            "Returned a user when it should have returned NotFound: {:?}",
            user
        ),
        QueryResult::NotFound => {}
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };
}

/// # Description
///
/// Test querying a user by username that does exist.
#[actix_web::test]
async fn querying_a_user_by_username_that_does_exist_returns_the_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let mut user: User = create_test_user();

    // Perform the insertion.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Make sure the user returned is correct.
    user.id = inserted_user.id;
    user.password = inserted_user.password.clone();
    assert_eq!(user, inserted_user);

    // Query the user that was inserted.
    let queried_user: User = match user_service.get_by_username(&user.username).await {
        QueryResult::Ok(queried_user) => queried_user,
        QueryResult::NotFound => panic!("User not found"),
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };

    // Make sure the queried user has the correct values.
    assert_eq!(queried_user, user);
    assert_eq!(queried_user, inserted_user);

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Test querying a user by email address that does not exist.
#[actix_web::test]
async fn querying_a_user_by_email_that_does_not_exist_should_not_return_a_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    match user_service
        .get_by_email(&String::from("not@areal.email"))
        .await
    {
        QueryResult::Ok(user) => panic!(
            "Returned a user when it should have returned NotFound: {:?}",
            user
        ),
        QueryResult::NotFound => {}
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };
}

/// # Description
///
/// Test querying a user by email address that does exist.
#[actix_web::test]
async fn querying_a_user_by_email_that_does_exist_returns_the_user() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let mut user: User = create_test_user();

    // Perform the insertion.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Make sure the user returned is correct.
    user.id = inserted_user.id;
    user.password = inserted_user.password.clone();
    assert_eq!(user, inserted_user);

    // Query the user that was inserted.
    let queried_user: User = match user_service.get_by_email(&user.email).await {
        QueryResult::Ok(queried_user) => queried_user,
        QueryResult::NotFound => panic!("User not found"),
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };

    // Make sure the queried user has the correct values.
    assert_eq!(queried_user, user);
    assert_eq!(queried_user, inserted_user);

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
/// Updating a user that does not exist returns not found.
#[actix_web::test]
async fn updating_a_user_that_does_not_exist_returns_not_found() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let user: User = create_test_user();

    match user_service.update(&user).await {
        UpdateResult::Ok(user) => panic!(
            "User was updated when NotFound should have been returned: {:?}",
            user
        ),
        UpdateResult::NotFound => {}
        UpdateResult::Invalid(errors) => panic!("Data validation detected an error: {:?}", errors),
        UpdateResult::Err(error) => panic!(
            "An error occurred while attempting to update user: {:?}",
            error
        ),
    }
}

/// # Description
///
/// Test updating a users username to a username that is already in use.
#[actix_web::test]
async fn updating_a_users_username_to_a_username_that_is_already_in_use_returns_a_validation_error()
{
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create test users.
    let mut user_one: User = create_test_user();
    let mut user_two: User = create_test_user();

    // Perform the insertions.
    let inserted_user_one: User = match user_service.insert(&user_one).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };
    let inserted_user_two: User = match user_service.insert(&user_two).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Update the users' id and password.
    user_one.id = inserted_user_one.id;
    user_one.password = inserted_user_one.password.clone();
    user_two.id = inserted_user_two.id;
    user_two.password = inserted_user_two.password.clone();

    // Make sure the inserted user has the correct information.
    assert_eq!(user_one, inserted_user_one);
    assert_eq!(user_two, inserted_user_two);

    // Make sure both users are queryable.
    let queried_user_one: User = match user_service.get_by_id(&user_one.id).await {
        QueryResult::Ok(queried_user) => queried_user,
        QueryResult::NotFound => panic!("User not found"),
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };
    let queried_user_two: User = match user_service.get_by_id(&user_two.id).await {
        QueryResult::Ok(queried_user) => queried_user,
        QueryResult::NotFound => panic!("User not found"),
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };

    assert_eq!(user_one, queried_user_one);
    assert_eq!(user_two, queried_user_two);

    // Attempt to update user two's username to be the same as user 1.
    user_two.username = user_one.username.clone();

    match user_service.update(&user_two).await {
        UpdateResult::Ok(user) => panic!(
            "Update succeeded when it should have returned a validation error: {:?}",
            user
        ),
        UpdateResult::NotFound => panic!("Updated failed: the user could not be found"),
        UpdateResult::Invalid(_) => {}
        UpdateResult::Err(error) => {
            panic!("Failed to perform update, an error occurred: {:?}", error)
        }
    }

    // Delete the test users.
    match user_service.delete(&user_one.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
    match user_service.delete(&user_two.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Test updating a users email address to a email address that is already in use.
#[actix_web::test]
async fn updating_a_users_email_to_an_email_that_is_already_in_use_returns_a_validation_error() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create test users.
    let mut user_one: User = create_test_user();
    let mut user_two: User = create_test_user();

    // Perform the insertions.
    let inserted_user_one: User = match user_service.insert(&user_one).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };
    let inserted_user_two: User = match user_service.insert(&user_two).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Update the users' id and password.
    user_one.id = inserted_user_one.id;
    user_one.password = inserted_user_one.password.clone();
    user_two.id = inserted_user_two.id;
    user_two.password = inserted_user_two.password.clone();

    // Make sure the inserted users have the correct information.
    assert_eq!(user_one, inserted_user_one);
    assert_eq!(user_two, inserted_user_two);

    // Make sure both users are queryable.
    let queried_user_one: User = match user_service.get_by_id(&user_one.id).await {
        QueryResult::Ok(queried_user) => queried_user,
        QueryResult::NotFound => panic!("User not found"),
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };
    let queried_user_two: User = match user_service.get_by_id(&user_two.id).await {
        QueryResult::Ok(queried_user) => queried_user,
        QueryResult::NotFound => panic!("User not found"),
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };

    assert_eq!(user_one, queried_user_one);
    assert_eq!(user_two, queried_user_two);

    // Attempt to update user two's username to be the same as user 1.
    user_two.email = user_one.email.clone();

    match user_service.update(&user_two).await {
        UpdateResult::Ok(user) => panic!(
            "Update succeeded when it should have returned a validation error: {:?}",
            user
        ),
        UpdateResult::NotFound => panic!("Updated failed: the user could not be found"),
        UpdateResult::Invalid(_) => {}
        UpdateResult::Err(error) => {
            panic!("Failed to perform update, an error occurred: {:?}", error)
        }
    }

    // Delete the test users.
    match user_service.delete(&user_one.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
    match user_service.delete(&user_two.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Update a user with valid updated information should succeed.
#[actix_web::test]
async fn updating_a_user_with_valid_information_should_succeed() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let mut user: User = create_test_user();

    // Perform the insertion.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Update the user's id and password.
    user.id = inserted_user.id;
    user.password = inserted_user.password.clone();

    // Make sure the inserted user has the correct information.
    assert_eq!(user, inserted_user);

    // Create an updated user.
    let mut updated_user: User = create_test_user();
    updated_user.id = user.id;
    updated_user.email_is_verified = !user.email_is_verified;
    updated_user.password_reset_is_required = !user.password_reset_is_required;
    updated_user.account_is_locked = !user.account_is_locked;
    updated_user.account_is_banned = !user.account_is_banned;

    // Perform the update.
    let returned_updated_user: User = match user_service.update(&updated_user).await {
        UpdateResult::Ok(returned_updated_user) => returned_updated_user,
        UpdateResult::NotFound => panic!("Failed to update user: user was not found"),
        UpdateResult::Invalid(errors) => panic!(
            "Failed to update user, validation errors occurred: {:?}",
            errors
        ),
        UpdateResult::Err(error) => {
            panic!("Failed to update user, an error occurred: {:?}", error)
        }
    };

    // Make sure the updated user has the new hashed password.
    assert_ne!(&updated_user.password, &returned_updated_user.password);
    updated_user.password = returned_updated_user.password.clone();

    // Make sure the returned update user has the correct information.
    assert_eq!(updated_user, returned_updated_user);

    // Query the user again to make sure the information is correct.
    let queried_updated_user: User = match user_service.get_by_id(&user.id).await {
        QueryResult::Ok(queried_user) => queried_user,
        QueryResult::NotFound => panic!("User not found"),
        QueryResult::Err(error) => panic!("Failed to execute query: {:?}", error),
    };

    assert_eq!(queried_updated_user, returned_updated_user);

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Deleting a user that does not exist returns not found.
#[actix_web::test]
async fn deleting_a_user_that_does_not_exist_returns_not_found() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Delete the test user.
    match user_service.delete(&0).await {
        DeletionResult::Ok => panic!("Deleted a user when it should have returned NotFound"),
        DeletionResult::NotFound => {}
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };
}

/// # Description
///
/// Test that deleted users cannot be queried after they are deleted.
#[actix_web::test]
async fn deleted_users_cannot_be_queried_after_being_deleted() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Create a test user.
    let mut user: User = create_test_user();

    // Perform the insertion.
    let inserted_user: User = match user_service.insert(&user).await {
        InsertionResult::Ok(inserted_user) => inserted_user,
        InsertionResult::Invalid(details) => panic!("Failed to insert user: {:?}", details),
        InsertionResult::Err(error) => panic!("Failed to insert user: {:?}", error),
    };

    // Make sure the user returned is correct.
    user.id = inserted_user.id;
    user.password = inserted_user.password.clone();
    assert_eq!(user, inserted_user);

    // Delete the test user.
    match user_service.delete(&user.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => panic!("Failed to delete user"),
        DeletionResult::Err(error) => panic!("Failed to delete user: {:?}", error),
    };

    // Query the deleted user.
    match user_service.get_by_id(&user.id).await {
        QueryResult::Ok(queried_user) => panic!(
            "Query returned a user that should be deleted: {:?}",
            queried_user
        ),
        QueryResult::NotFound => {}
        QueryResult::Err(error) => panic!("Failed to execute deletion: {:?}", error),
    };
}
