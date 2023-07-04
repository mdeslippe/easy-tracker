use crate::{
    common::{
        enumeration::{DeletionResult, InsertionResult, QueryResult, UpdateResult},
        utility::generate_random_string,
    },
    config::Config,
    feature::{
        file::{model::File, service::FileService},
        user::{model::User, service::UserService},
    },
    injector::DependencyInjector,
};
use core::panic;
use shaku::HasComponent;
use std::sync::Arc;
use time::OffsetDateTime;

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
/// # Panics
///
/// This function will panic if an error occurs while attempting to insert the user with the user
/// service.
///
/// # Returns
///
/// The user that was inserted.
async fn insert_test_user(injector: &DependencyInjector) -> User {
    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Perform the insertion.
    let user: User = match user_service.insert(&create_test_user()).await {
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
/// # Panics
///
/// This function will panic if an error occurs while attempting to delete the user with the user
/// service.
async fn delete_test_user(user: &User, injector: &DependencyInjector) {
    // Get a user service instance.
    let user_service: Arc<dyn UserService> = injector.resolve();

    // Perform the deletion.
    match user_service.delete(&user.id).await {
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
        .expect("Failed to create dependency injector");

    // Return the dependency injector.
    return injector;
}

/// # Description
///
/// Test inserting a file and make sure the correct data is returned.
#[actix_web::test]
async fn file_is_returned_after_insertion() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a file service instance.
    let file_service: Arc<dyn FileService> = injector.resolve();

    // Insert a test user.
    let user: User = insert_test_user(&injector).await;

    // Create a test file.
    let mut file: File = create_test_file();
    file.user_id = user.id;

    // Insert the file.
    let inserted_file = match file_service.insert(&file).await {
        InsertionResult::Ok(inserted_file) => inserted_file,
        InsertionResult::Invalid(details) => {
            panic!("Failed to insert file, the file was invalid: {}", details)
        }
        InsertionResult::Err(error) => panic!(
            "Failed to insert file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Update the file id with the one that was generated.
    file.id = inserted_file.id;

    // Make sure the correct data was returned.
    assert_eq!(file, inserted_file);

    // Delete the test file.
    match file_service.delete(&file.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => {
            panic!("Failed to delete file: File could not be found")
        }
        DeletionResult::Err(error) => panic!(
            "Failed to delete file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Delete the test user.
    delete_test_user(&user, &injector).await;
}

/// # Description
///
/// Test inserting a file with validation errors and make sure it does not succeed.
#[actix_web::test]
async fn inserting_a_file_with_validation_errors_does_not_succeed() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a file service instance.
    let file_service: Arc<dyn FileService> = injector.resolve();

    // Insert a test user.
    let user: User = insert_test_user(&injector).await;

    // Create a test file.
    let mut file: File = create_test_file();
    file.user_id = user.id;
    file.name = generate_random_string(1048576);

    // Insert the file, and make sure the validation errors are detected.
    match file_service.insert(&file).await {
        InsertionResult::Ok(_) => panic!("Insertion succeeded when it should have failed"),
        InsertionResult::Invalid(_) => {}
        InsertionResult::Err(error) => panic!(
            "Failed to insert file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Delete the test user.
    delete_test_user(&user, &injector).await;
}

/// # Description
///
/// Test querying a file after it has been inserted and make sure the correct data is returned.
#[actix_web::test]
async fn file_is_queryable_after_insertion() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a file service instance.
    let file_service: Arc<dyn FileService> = injector.resolve();

    // Insert a test user.
    let user: User = insert_test_user(&injector).await;

    // Create a test file.
    let mut file: File = create_test_file();
    file.user_id = user.id;

    // Insert the file.
    let inserted_file = match file_service.insert(&file).await {
        InsertionResult::Ok(inserted_file) => inserted_file,
        InsertionResult::Invalid(details) => {
            panic!("Failed to insert file, the file was invalid: {}", details)
        }
        InsertionResult::Err(error) => panic!(
            "Failed to insert file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Update the file id with the one that was generated.
    file.id = inserted_file.id;

    // Make sure the correct data was returned.
    assert_eq!(file, inserted_file);

    // Query the file that was inserted.
    let queried_file = match file_service.get(&file.id).await {
        QueryResult::Ok(queried_file) => queried_file,
        QueryResult::NotFound => panic!("Failed to query file after insertion: File not found"),
        QueryResult::Err(error) => panic!(
            "Failed to query file after insertion: an unexpected error has occurred: {}",
            error
        ),
    };

    // Make sure the file has the correct data.
    assert_eq!(file, queried_file);

    // Delete the test file.
    match file_service.delete(&file.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => {
            panic!("Failed to delete file: File could not be found")
        }
        DeletionResult::Err(error) => panic!(
            "Failed to delete file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Delete the test user.
    delete_test_user(&user, &injector).await;
}

/// # Description
///
/// Test updating a file after insertion and make sure the data is updated correctly.
#[actix_web::test]
async fn file_is_updatable_after_insertion() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a file service instance.
    let file_service: Arc<dyn FileService> = injector.resolve();

    // Insert a test user.
    let user: User = insert_test_user(&injector).await;

    // Create a test file.
    let mut file: File = create_test_file();
    file.user_id = user.id;

    // Insert the file.
    let inserted_file = match file_service.insert(&file).await {
        InsertionResult::Ok(inserted_file) => inserted_file,
        InsertionResult::Invalid(details) => {
            panic!("Failed to insert file, the file was invalid: {}", details)
        }
        InsertionResult::Err(error) => panic!(
            "Failed to insert file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Update the file id with the one that was generated.
    file.id = inserted_file.id;

    // Make sure the correct data was returned.
    assert_eq!(file, inserted_file);

    // Query the file that was inserted.
    let queried_file = match file_service.get(&file.id).await {
        QueryResult::Ok(queried_file) => queried_file,
        QueryResult::NotFound => panic!("Failed to query file after insertion: File not found"),
        QueryResult::Err(error) => panic!(
            "Failed to query file after insertion: an unexpected error has occurred: {}",
            error
        ),
    };

    // Make sure the file has the correct data.
    assert_eq!(file, queried_file);

    // Create an updated file.
    let mut updated_file = create_test_file();
    updated_file.id = file.id;
    updated_file.user_id = file.user_id;

    // Perform the update.
    let returned_updated_file = match file_service.update(&updated_file).await {
        UpdateResult::Ok(returned_updated_file) => returned_updated_file,
        UpdateResult::NotFound => panic!("Failed to update file: File not found"),
        UpdateResult::Invalid(details) => panic!(
            "Failed to update file, invalid values specified: {}",
            details
        ),
        UpdateResult::Err(error) => panic!(
            "Failed to update file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Make sure the returned updated file contains the correct information.
    assert_eq!(updated_file, returned_updated_file);

    // Query the updated file.
    let queried_updated_file = match file_service.get(&file.id).await {
        QueryResult::Ok(queried_updated_file) => queried_updated_file,
        QueryResult::NotFound => panic!("Failed to query file after update: File not found"),
        QueryResult::Err(error) => panic!(
            "Failed to query file after update: an unexpected error has occurred: {}",
            error
        ),
    };

    // Make sure the queried updated file contains the correct information.
    assert_eq!(updated_file, queried_updated_file);

    // Delete the test file.
    match file_service.delete(&file.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => {
            panic!("Failed to delete file: file could not be found")
        }
        DeletionResult::Err(error) => panic!(
            "Failed to delete file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Delete the test user.
    delete_test_user(&user, &injector).await;
}

/// # Description
///
/// Test updating a file with validation errors and make sure it does not succeed.
#[actix_web::test]
async fn updating_a_file_with_validation_errors_does_not_succeed() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a file service instance.
    let file_service: Arc<dyn FileService> = injector.resolve();

    // Insert a test user.
    let user: User = insert_test_user(&injector).await;

    // Create a test file.
    let mut file: File = create_test_file();
    file.user_id = user.id;

    // Insert the file.
    let inserted_file = match file_service.insert(&file).await {
        InsertionResult::Ok(inserted_file) => inserted_file,
        InsertionResult::Invalid(details) => {
            panic!("Failed to insert file, the file was invalid: {}", details)
        }
        InsertionResult::Err(error) => panic!(
            "Failed to insert file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Update the file id with the one that was generated.
    file.id = inserted_file.id;

    // Make sure the correct data was returned.
    assert_eq!(file, inserted_file);

    // Query the file that was inserted.
    let queried_file = match file_service.get(&file.id).await {
        QueryResult::Ok(queried_file) => queried_file,
        QueryResult::NotFound => panic!("Failed to query file after insertion: File not found"),
        QueryResult::Err(error) => panic!(
            "Failed to query file after insertion: an unexpected error has occurred: {}",
            error
        ),
    };

    // Make sure the file has the correct data.
    assert_eq!(file, queried_file);

    // Create an updated file.
    let mut updated_file = create_test_file();
    updated_file.id = file.id;
    updated_file.user_id = file.user_id;
    updated_file.name = generate_random_string(1048576);

    // Perform the update.
    match file_service.update(&updated_file).await {
        UpdateResult::Ok(_) => panic!("File was updated when it contained validation errors"),
        UpdateResult::NotFound => panic!("Failed to update file: File not found"),
        UpdateResult::Invalid(_) => {}
        UpdateResult::Err(error) => panic!(
            "Failed to update file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Delete the test file.
    match file_service.delete(&file.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => {
            panic!("Failed to delete file: file could not be found")
        }
        DeletionResult::Err(error) => panic!(
            "Failed to delete file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Delete the test user.
    delete_test_user(&user, &injector).await;
}

/// # Description
///
/// Test querying a file after it was deleted and make sure it was not returned.
#[actix_web::test]
async fn file_is_not_queryable_after_deletion() {
    // Create a dependency injector.
    let injector: DependencyInjector = create_dependency_injector().await;

    // Get a file service instance.
    let file_service: Arc<dyn FileService> = injector.resolve();

    // Insert a test user.
    let user: User = insert_test_user(&injector).await;

    // Create a test file.
    let mut file: File = create_test_file();
    file.user_id = user.id;

    // Insert the file.
    let inserted_file = match file_service.insert(&file).await {
        InsertionResult::Ok(inserted_file) => inserted_file,
        InsertionResult::Invalid(details) => {
            panic!("Failed to insert file, the file was invalid: {}", details)
        }
        InsertionResult::Err(error) => panic!(
            "Failed to insert file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Update the file id with the one that was generated.
    file.id = inserted_file.id;

    // Make sure the correct data was returned.
    assert_eq!(file, inserted_file);

    // Delete the test file.
    match file_service.delete(&file.id).await {
        DeletionResult::Ok => {}
        DeletionResult::NotFound => {
            panic!("Failed to delete file: File could not be found")
        }
        DeletionResult::Err(error) => panic!(
            "Failed to delete file, an unexpected error has occurred: {}",
            error
        ),
    };

    // Query the file and make sure it was not found.
    match file_service.get(&file.id).await {
        QueryResult::Ok(_) => {
            panic!("Filed was returned when it should have been deleted")
        }
        QueryResult::NotFound => {}
        QueryResult::Err(error) => panic!(
            "Failed to query file after insertion, an unexpected error has occurred: {}",
            error
        ),
    };

    // Delete the test user.
    delete_test_user(&user, &injector).await;
}
