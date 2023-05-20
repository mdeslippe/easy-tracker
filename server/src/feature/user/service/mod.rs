use crate::{
    common::{
        enumeration::{DeletionResult, InsertionResult, QueryContext, QueryResult, UpdateResult},
        utility::create_value_validation_error,
    },
    database::DatabaseConnectionFactory,
    feature::{
        crypto::service::CryptoService,
        user::{model::User, repository::UserRepository},
    },
};
use async_trait::async_trait;
use nameof::name_of;
use shaku::{Component, Interface};
use sqlx::Acquire;
use std::{borrow::Cow, error::Error, io, sync::Arc};
use time::{OffsetDateTime, UtcOffset};
use validator::{Validate, ValidationError, ValidationErrors};

/// A user service trait.
#[async_trait(?Send)]
pub(crate) trait UserService: Interface {
    /// # Description
    ///
    /// Insert a user into persistent storage.
    ///
    /// # Arguments
    ///
    /// `user` - The user that will be inserted into persistent storage.
    ///
    /// # Returns
    ///
    /// This functions returns an insertion result:
    /// - If the insertion is successful, the Ok variant will be returned with the user that was inserted.
    /// - If the user being inserted contains validation errors, the Invalid variant will be returned
    /// with the validation errors that were detected.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn insert(&self, user: &User) -> InsertionResult<User, ValidationErrors, Box<dyn Error>>;

    /// # Description
    ///
    /// Insert a user into persistent storage.
    ///
    /// # Arguments
    ///
    /// `user` - The user that will be inserted into persistent storage.
    ///
    /// `context` - The context the insertion will be performed in.
    ///
    /// # Returns
    ///
    /// This functions returns an insertion result:
    /// - If the insertion is successful, the Ok variant will be returned with the user that was inserted.
    /// - If the user being inserted contains validation errors, the Invalid variant will be returned
    /// with the validation errors that were detected.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn insert_with_context(
        &self,
        user: &User,
        context: &mut QueryContext,
    ) -> InsertionResult<User, ValidationErrors, Box<dyn Error>>;

    /// # Description
    ///
    /// Query a user from persistent storage by their unique identifier.
    ///
    /// # Arguments
    ///
    /// `id` - The id of the user that is being queried from persistent storage.
    ///
    /// # Returns
    ///
    /// This functions returns a query result:
    /// - If the query is successful and finds the user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_id(&self, id: &u64) -> QueryResult<User, Box<dyn Error>>;

    /// # Description
    ///
    /// Query a user from persistent storage by their unique identifier.
    ///
    /// # Arguments
    ///
    /// `id` - The id of the user that is being queried from persistent storage.
    ///
    /// `context` - The context the query will be performed in.
    ///
    /// # Returns
    ///
    /// This functions returns a query result:
    /// - If the query is successful and finds the user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_id_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> QueryResult<User, Box<dyn Error>>;

    /// # Description
    ///
    /// Query a user from persistent storage by their username.
    ///
    /// # Arguments
    ///
    /// `username` - The username of the user that is being queried from persistent storage.
    ///
    /// # Returns
    ///
    /// This functions returns a query result:
    /// - If the query is successful and finds the user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_username(&self, username: &String) -> QueryResult<User, Box<dyn Error>>;

    /// # Description
    ///
    /// Query a user from persistent storage by their username.
    ///
    /// # Arguments
    ///
    /// `username` - The username of the user that is being queried from persistent storage.
    ///
    /// `context` - The context the query will be performed in.
    ///
    /// # Returns
    ///
    /// This functions returns a query result:
    /// - If the query is successful and finds the user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_username_with_context(
        &self,
        username: &String,
        context: &mut QueryContext,
    ) -> QueryResult<User, Box<dyn Error>>;

    /// # Description
    ///
    /// Query a user from persistent storage by their email address.
    ///
    /// # Arguments
    ///
    /// `email` - The email address of the user that is being queried from persistent storage.
    ///
    /// # Returns
    ///
    /// This functions returns a query result:
    /// - If the query is successful and finds the user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_email(&self, email: &String) -> QueryResult<User, Box<dyn Error>>;

    /// # Description
    ///
    /// Query a user from persistent storage by their email address.
    ///
    /// # Arguments
    ///
    /// `email` - The email address of the user that is being queried from persistent storage.
    ///
    /// `context` - The context the query will be performed in.
    ///
    /// # Returns
    ///
    /// This functions returns a query result:
    /// - If the query is successful and finds the user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_email_with_context(
        &self,
        email: &String,
        context: &mut QueryContext,
    ) -> QueryResult<User, Box<dyn Error>>;

    /// # Description
    ///
    /// Update a user in persistent storage.
    ///
    /// # Arguments
    ///
    /// `user` - The user that is being updated in persistent storage.
    ///
    /// # Returns
    ///
    /// This functions returns an update result:
    /// - If the update is successful, the Ok variant will be returned with the user updated.
    /// - If the user that is being updated could not be found, the NotFound variant will be returned.
    /// - If the user being updated contains validation errors, the Invalid variant will be returned
    /// with the validation errors that were detected.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn update(&self, user: &User) -> UpdateResult<User, ValidationErrors, Box<dyn Error>>;

    /// # Description
    ///
    /// Update a user in persistent storage.
    ///
    /// # Arguments
    ///
    /// `user` - The user that is being updated in persistent storage.
    ///
    /// `context` - The context the update will be performed in.
    ///
    /// # Returns
    ///
    /// This functions returns an update result:
    /// - If the update is successful, the Ok variant will be returned with the user updated.
    /// - If the user that is being updated could not be found, the NotFound variant will be returned.
    /// - If the user being updated contains validation errors, the Invalid variant will be returned
    /// with the validation errors that were detected.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn update_with_context(
        &self,
        user: &User,
        context: &mut QueryContext,
    ) -> UpdateResult<User, ValidationErrors, Box<dyn Error>>;

    /// # Description
    ///
    /// Remove a user from persistent storage.
    ///
    /// # Arguments
    ///
    /// `id` - The unique identifier of the user that is being removed from persistent storage.
    ///
    /// # Returns
    ///
    /// This functions returns a deletion result:
    /// - If the deletion is successful and a user was successfully deleted, the Ok variant will be
    /// returned.
    /// - If the user being deleted could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn delete(&self, id: &u64) -> DeletionResult<Box<dyn Error>>;

    /// # Description
    ///
    /// Remove a user from persistent storage.
    ///
    /// # Arguments
    ///
    /// `id` - The unique identifier of the user that is being removed from persistent storage.
    ///
    /// `context` - The context the deletion will be performed in.
    ///
    /// # Returns
    ///
    /// This functions returns a deletion result:
    /// - If the deletion is successful and a user was successfully deleted, the Ok variant will be
    /// returned.
    /// - If the user being deleted could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn delete_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> DeletionResult<Box<dyn Error>>;
}

/// A UserServiceImpl struct.
#[derive(Component)]
#[shaku(interface = UserService)]
pub(crate) struct UserServiceImpl {
    /// The crypto service that will be used to manage user passwords.
    #[shaku(inject)]
    crypto_service: Arc<dyn CryptoService>,

    /// The user repository that will be used to manage persistent user data.
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,

    /// The database connection factory that will be used to acquire database connections.
    #[shaku(inject)]
    connection_factory: Arc<dyn DatabaseConnectionFactory>,
}

/// A UserService implementation for the UserServiceImpl struct.
#[async_trait(?Send)]
impl UserService for UserServiceImpl {
    async fn insert(&self, user: &User) -> InsertionResult<User, ValidationErrors, Box<dyn Error>> {
        // Acquire a database connection.
        let mut connection = match __self.connection_factory.get_connection().await {
            Ok(connection) => connection,
            Err(error) => return InsertionResult::Err(Box::new(error)),
        };

        // Start a transaction and put it in a query context.
        let mut context = match connection.begin().await {
            Ok(transaction) => QueryContext::Transaction(transaction),
            Err(error) => return InsertionResult::Err(Box::new(error)),
        };

        // Perform the insertion.
        let insertion_result = self.insert_with_context(user, &mut context).await;

        // If the insertion was successful, commit the transaction, otherwise roll it back.
        let transaction_completion_result = match insertion_result {
            InsertionResult::Ok(_) => context.commit_if_transaction().await,
            InsertionResult::Invalid(_) => context.rollback_if_transaction().await,
            InsertionResult::Err(_) => context.rollback_if_transaction().await,
        };

        // If the transaction completion was successful, return the insertion result, otherwise return
        // the transaction completion error.
        return match transaction_completion_result {
            Ok(()) => insertion_result,
            Err(error) => InsertionResult::Err(Box::new(error)),
        };
    }

    async fn insert_with_context(
        &self,
        user: &User,
        context: &mut QueryContext,
    ) -> InsertionResult<User, ValidationErrors, Box<dyn Error>> {
        // Validate the user.
        let mut validation_errors = match user.validate() {
            Ok(()) => ValidationErrors::new(),
            Err(errors) => errors,
        };

        // Check if the username the user is trying to use is available.
        match __self
            .user_repository
            .get_by_username(&user.username, context)
            .await
        {
            Ok(result) => {
                // If the username is already in use, add a validation error.
                if result.is_some() {
                    validation_errors.add(
                        name_of!(username in User),
                        create_value_validation_error("unique", &user.username),
                    );
                }
            }
            Err(error) => return InsertionResult::Err(Box::new(error)),
        }

        // Check if the email address the usr is trying to use is available.
        match __self
            .user_repository
            .get_by_email(&user.email, context)
            .await
        {
            Ok(result) => {
                if result.is_some() {
                    validation_errors.add(
                        name_of!(email in User),
                        create_value_validation_error("unique", &user.email),
                    );
                }
            }
            Err(error) => return InsertionResult::Err(Box::new(error)),
        };

        // If any validation errors exist, return them.
        if !validation_errors.is_empty() {
            return InsertionResult::Invalid(validation_errors);
        }

        // We need to modify some of the user's fields without mutating the original user,
        // so we need to make a copy of it.
        let mut user = user.clone();

        // Hash the user's password.
        user.password = match __self.crypto_service.hash_password(&user.password) {
            Ok(hash) => hash,
            Err(error) => {
                return InsertionResult::Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    error.to_string(),
                )))
            }
        };

        // Perform the insertion.
        let user_id = match __self.user_repository.insert(&user, context).await {
            Ok(user_id) => user_id,
            Err(error) => return InsertionResult::Err(Box::new(error)),
        };

        // Query and return the user that was inserted.
        let inserted_user_option: Option<User> =
            match __self.user_repository.get_by_id(&user_id, context).await {
                Ok(inserted_user_option) => inserted_user_option,
                Err(error) => return InsertionResult::Err(Box::new(error)),
            };

        // If the user was found, return the user, otherwise return an error.
        return match inserted_user_option {
            Some(inserted_user) => InsertionResult::Ok(inserted_user),
            None => InsertionResult::Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "Internal error: user could not be found after insertion",
            ))),
        };
    }

    async fn get_by_id(&self, id: &u64) -> QueryResult<User, Box<dyn Error>> {
        // Acquire a database connection and put it in a query context.
        let mut context = match __self.connection_factory.get_connection().await {
            Ok(connection) => QueryContext::Connection(connection),
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // Perform the query.
        return self.get_by_id_with_context(id, &mut context).await;
    }

    async fn get_by_id_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> QueryResult<User, Box<dyn Error>> {
        // Perform the query.
        let result = match __self.user_repository.get_by_id(id, context).await {
            Ok(user_option) => user_option,
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // If the user was found, return the user, otherwise return not found.
        return match result {
            Some(user) => QueryResult::Ok(user),
            None => QueryResult::NotFound,
        };
    }

    async fn get_by_username(&self, username: &String) -> QueryResult<User, Box<dyn Error>> {
        // Acquire a database connection and put it in a query context.
        let mut context = match __self.connection_factory.get_connection().await {
            Ok(connection) => QueryContext::Connection(connection),
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // Perform the query.
        return self
            .get_by_username_with_context(username, &mut context)
            .await;
    }

    async fn get_by_username_with_context(
        &self,
        username: &String,
        context: &mut QueryContext,
    ) -> QueryResult<User, Box<dyn Error>> {
        // Perform the query.
        let result = match __self
            .user_repository
            .get_by_username(username, context)
            .await
        {
            Ok(user_option) => user_option,
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // If the user was found, return the user, otherwise return not found.
        return match result {
            Some(user) => QueryResult::Ok(user),
            None => QueryResult::NotFound,
        };
    }

    async fn get_by_email(&self, email: &String) -> QueryResult<User, Box<dyn Error>> {
        // Acquire a database connection and put it in a query context.
        let mut context = match __self.connection_factory.get_connection().await {
            Ok(connection) => QueryContext::Connection(connection),
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // Perform the query.
        return self.get_by_email_with_context(email, &mut context).await;
    }

    async fn get_by_email_with_context(
        &self,
        email: &String,
        context: &mut QueryContext,
    ) -> QueryResult<User, Box<dyn Error>> {
        // Perform the query.
        let result = match __self.user_repository.get_by_email(email, context).await {
            Ok(user_option) => user_option,
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // If the user was found, return the user, otherwise return not found.
        return match result {
            Some(user) => QueryResult::Ok(user),
            None => QueryResult::NotFound,
        };
    }

    async fn update(&self, user: &User) -> UpdateResult<User, ValidationErrors, Box<dyn Error>> {
        todo!();
    }

    async fn update_with_context(
        &self,
        user: &User,
        context: &mut QueryContext,
    ) -> UpdateResult<User, ValidationErrors, Box<dyn Error>> {
        todo!();
    }

    async fn delete(&self, id: &u64) -> DeletionResult<Box<dyn Error>> {
        // Acquire a database connection and put it in a query context.
        let mut context = match __self.connection_factory.get_connection().await {
            Ok(connection) => QueryContext::Connection(connection),
            Err(error) => return DeletionResult::Err(Box::new(error)),
        };

        // Perform the deletion.
        return self.delete_with_context(id, &mut context).await;
    }

    async fn delete_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> DeletionResult<Box<dyn Error>> {
        // Perform the deletion.
        let rows_deleted = match __self.user_repository.delete(id, context).await {
            Ok(rows_deleted) => rows_deleted,
            Err(error) => return DeletionResult::Err(Box::new(error)),
        };

        // Return the result.
        if rows_deleted > 0 {
            return DeletionResult::Ok;
        } else {
            return DeletionResult::NotFound;
        }
    }
}