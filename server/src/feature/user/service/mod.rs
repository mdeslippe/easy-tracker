use crate::{
    common::enumeration::{
        DeletionResult, InsertionResult, QueryContext, QueryResult, UpdateResult,
    },
    database::DatabaseConnectionFactory,
    feature::user::{model::User, repository::UserRepository},
};
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::{error::Error, sync::Arc};
use validator::ValidationErrors;

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
    #[shaku(inject)]
    connection_factory: Arc<dyn DatabaseConnectionFactory>,

    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

/// A UserService implementation for the UserServiceImpl struct.
#[async_trait(?Send)]
impl UserService for UserServiceImpl {
    async fn insert(&self, user: &User) -> InsertionResult<User, ValidationErrors, Box<dyn Error>> {
        todo!();
    }

    async fn insert_with_context(
        &self,
        user: &User,
        context: &mut QueryContext,
    ) -> InsertionResult<User, ValidationErrors, Box<dyn Error>> {
        todo!();
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
        todo!();
    }

    async fn delete_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> DeletionResult<Box<dyn Error>> {
        todo!();
    }
}
