#[cfg(test)]
mod test;

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
use std::{error::Error, io, sync::Arc};
use time::OffsetDateTime;
use validator::{Validate, ValidationErrors};

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
    /// This function returns an insertion result:
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
    /// This function returns an insertion result:
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
    /// This function returns a query result:
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
    /// This function returns a query result:
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
    /// This function returns a query result:
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
    /// This function returns a query result:
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
    /// This function returns a query result:
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
    /// This function returns a query result:
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
    /// This function returns an update result:
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
    /// This function returns an update result:
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
    /// This function returns a deletion result:
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
    /// This function returns a deletion result:
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

        // Start a transaction.
        let transaction = match connection.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return InsertionResult::Err(Box::new(error)),
        };

        // Create the query context.
        let mut context = QueryContext::Transaction(transaction);

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

        // Check if the email address the user is trying to use is available.
        match __self
            .user_repository
            .get_by_email(&user.email, context)
            .await
        {
            Ok(result) => {
                // If the email is already in use, add a validation error.
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
                "User could not be found after insertion",
            ))),
        };
    }

    async fn get_by_id(&self, id: &u64) -> QueryResult<User, Box<dyn Error>> {
        // Acquire a database connection.
        let connection = match __self.connection_factory.get_connection().await {
            Ok(connection) => connection,
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // Create the query context.
        let mut context = QueryContext::Connection(connection);

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
        // Acquire a database connection.
        let connection = match __self.connection_factory.get_connection().await {
            Ok(connection) => connection,
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // Create the query context.
        let mut context = QueryContext::Connection(connection);

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
        // Acquire a database connection.
        let connection = match __self.connection_factory.get_connection().await {
            Ok(connection) => connection,
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // Create the query context.
        let mut context = QueryContext::Connection(connection);

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
        // Acquire a database connection.
        let mut connection = match __self.connection_factory.get_connection().await {
            Ok(connection) => connection,
            Err(error) => return UpdateResult::Err(Box::new(error)),
        };

        // Start a transaction.
        let transaction = match connection.begin().await {
            Ok(transaction) => transaction,
            Err(error) => return UpdateResult::Err(Box::new(error)),
        };

        // Create the query context.
        let mut context = QueryContext::Transaction(transaction);

        // Perform the update.
        let update_result = self.update_with_context(user, &mut context).await;

        // If the update was successful, commit the transaction, otherwise roll it back.
        let transaction_completion_result = match update_result {
            UpdateResult::Ok(_) => context.commit_if_transaction().await,
            UpdateResult::NotFound => context.rollback_if_transaction().await,
            UpdateResult::Invalid(_) => context.rollback_if_transaction().await,
            UpdateResult::Err(_) => context.rollback_if_transaction().await,
        };

        // If the transaction completion was successful, return the update result, otherwise return
        // the transaction completion error.
        return match transaction_completion_result {
            Ok(()) => update_result,
            Err(error) => UpdateResult::Err(Box::new(error)),
        };
    }

    async fn update_with_context(
        &self,
        user: &User,
        context: &mut QueryContext,
    ) -> UpdateResult<User, ValidationErrors, Box<dyn Error>> {
        // Query the existing user.
        let existing_user_option = match __self.user_repository.get_by_id(&user.id, context).await {
            Ok(existing_user_option) => existing_user_option,
            Err(error) => return UpdateResult::Err(Box::new(error)),
        };

        // Make sure the user exists.
        let existing_user = match existing_user_option {
            Some(existing_user) => existing_user,
            None => return UpdateResult::NotFound,
        };

        // Validate the user.
        let mut validation_errors = match user.validate() {
            Ok(()) => ValidationErrors::new(),
            Err(errors) => errors,
        };

        // If the user is changing their username, make sure the new username is available.
        if &user.username != &existing_user.username {
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
                Err(error) => return UpdateResult::Err(Box::new(error)),
            }
        }

        // If the user is changing the email address, make sure the new email is available.
        if &user.email != &existing_user.email {
            match __self
                .user_repository
                .get_by_email(&user.email, context)
                .await
            {
                Ok(result) => {
                    // If the email is already in use, add a validation error.
                    if result.is_some() {
                        validation_errors.add(
                            name_of!(email in User),
                            create_value_validation_error("unique", &user.email),
                        );
                    }
                }
                Err(error) => return UpdateResult::Err(Box::new(error)),
            };
        }

        // If any validation errors exist, return them.
        if !validation_errors.is_empty() {
            return UpdateResult::Invalid(validation_errors);
        }

        // We may need to modify some of the user's fields without mutating the original user
        // so we must make a copy of it.
        let mut user = user.clone();

        // If the user is updating their password, hash it.
        if &user.password != &existing_user.password {
            user.password_reset_at = OffsetDateTime::now_utc();
            user.password = match __self.crypto_service.hash_password(&user.password) {
                Ok(hash) => hash,
                Err(error) => {
                    return UpdateResult::Err(Box::new(io::Error::new(
                        io::ErrorKind::InvalidData,
                        error.to_string(),
                    )))
                }
            };
        }

        // Perform the update.
        let records_updated = match __self.user_repository.update(&user, context).await {
            Ok(records_updated) => records_updated,
            Err(error) => return UpdateResult::Err(Box::new(error)),
        };

        // If no records were updated.
        if records_updated == 0 {
            return UpdateResult::Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "No records were modified when updating a user that should exist",
            )));
        }

        // Query the updated user.
        let updated_user_option = match __self.user_repository.get_by_id(&user.id, context).await {
            Ok(updated_user_option) => updated_user_option,
            Err(error) => return UpdateResult::Err(Box::new(error)),
        };

        // Make sure the user was found, and return the updated user.
        return match updated_user_option {
            Some(updated_user) => UpdateResult::Ok(updated_user),
            None => UpdateResult::Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "User could not be found after update",
            ))),
        };
    }

    async fn delete(&self, id: &u64) -> DeletionResult<Box<dyn Error>> {
        // Acquire a database connection.
        let connection = match __self.connection_factory.get_connection().await {
            Ok(connection) => connection,
            Err(error) => return DeletionResult::Err(Box::new(error)),
        };

        // Create the query context.
        let mut context = QueryContext::Connection(connection);

        // Perform the deletion.
        return self.delete_with_context(id, &mut context).await;
    }

    async fn delete_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> DeletionResult<Box<dyn Error>> {
        // Perform the deletion.
        let records_deleted = match __self.user_repository.delete(id, context).await {
            Ok(records_deleted) => records_deleted,
            Err(error) => return DeletionResult::Err(Box::new(error)),
        };

        // Return the result.
        if records_deleted > 0 {
            return DeletionResult::Ok;
        } else {
            return DeletionResult::NotFound;
        }
    }
}
