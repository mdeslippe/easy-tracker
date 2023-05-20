use std::error::Error;

use async_trait::async_trait;
use shaku::Interface;
use validator::ValidationErrors;

use crate::common::enumeration::{
    DeletionResult, InsertionResult, QueryContext, QueryResult, UpdateResult,
};

use super::model::User;

/// A user service trait.
#[async_trait]
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
    /// - If the insertion is successful, the Ok variant will be returned with the user's information.
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
    /// - If the insertion is successful, the Ok variant will be returned with the user's information.
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
    /// - If the query is successful and finds a user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_id(&self, id: u64) -> QueryResult<User, Box<dyn Error>>;

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
    /// - If the query is successful and finds a user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_id_with_context(
        &self,
        id: u64,
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
    /// - If the query is successful and finds a user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_username(&self, username: String) -> QueryResult<User, Box<dyn Error>>;

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
    /// - If the query is successful and finds a user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_username_with_context(
        &self,
        username: String,
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
    /// - If the query is successful and finds a user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_email(&self, email: String) -> QueryResult<User, Box<dyn Error>>;

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
    /// - If the query is successful and finds a user, the Ok variant will be returned with the user's
    /// information.
    /// - If the user could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_by_email_with_context(
        &self,
        email: String,
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
    async fn delete(&self, id: u64) -> DeletionResult<Box<dyn Error>>;

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
        id: u64,
        context: &mut QueryContext,
    ) -> DeletionResult<Box<dyn Error>>;
}
