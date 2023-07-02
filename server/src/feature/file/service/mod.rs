use crate::{
    common::enumeration::{
        DeletionResult, InsertionResult, QueryContext, QueryResult, UpdateResult,
    },
    database::DatabaseConnectionFactory,
    feature::file::{model::File, repository::FileRepository},
};
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::{error::Error, sync::Arc};
use validator::ValidationErrors;

/// A file service trait.
#[async_trait(?Send)]
pub(crate) trait FileService: Interface {
    /// # Description
    ///
    /// Insert a file into persistent storage.
    ///
    /// # Arguments
    ///
    /// `file` - The file that will be inserted into persistent storage.
    ///
    /// # Returns
    ///
    /// This function returns an insertion result:
    /// - If the insertion is successful, the Ok variant will be returned with the file that was inserted.
    /// - If the file being inserted contains validation errors, the Invalid variant will be returned
    /// with the validation errors that were detected.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn insert(&self, file: &File) -> InsertionResult<File, ValidationErrors, Box<dyn Error>>;

    /// # Description
    ///
    /// Insert a file into persistent storage.
    ///
    /// # Arguments
    ///
    /// `file` - The file that will be inserted into persistent storage.
    ///
    /// `context` - The context the insertion will be performed in.
    ///
    /// # Returns
    ///
    /// This function returns an insertion result:
    /// - If the insertion is successful, the Ok variant will be returned with the file that was inserted.
    /// - If the file being inserted contains validation errors, the Invalid variant will be returned
    /// with the validation errors that were detected.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn insert_with_context(
        &self,
        file: &File,
        context: &mut QueryContext,
    ) -> InsertionResult<File, ValidationErrors, Box<dyn Error>>;

    /// # Description
    ///
    /// Query a file from persistent storage by it's unique identifier.
    ///
    /// # Arguments
    ///
    /// `id` - The unique identifier of the file that is being queried from persistent storage.
    ///
    /// # Returns
    ///
    /// This function returns a query result:
    /// - If the query is successful and finds the file, the Ok variant will be returned with the file.
    /// - If the file could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get(&self, id: &u64) -> QueryResult<File, Box<dyn Error>>;

    /// # Description
    ///
    /// Query a file from persistent storage by it's unique identifier.
    ///
    /// # Arguments
    ///
    /// `id` - The unique identifier of the file that is being queried from persistent storage.
    ///
    /// `context` - The context the query will be performed in.
    ///
    /// # Returns
    ///
    /// This function returns a query result:
    /// - If the query is successful and finds the file, the Ok variant will be returned with the file.
    /// - If the file could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn get_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> QueryResult<File, Box<dyn Error>>;

    /// # Description
    ///
    /// Update a file in persistent storage.
    ///
    /// # Arguments
    ///
    /// `file` - The file that is being updated in persistent storage.
    ///
    /// # Returns
    ///
    /// This function returns an update result:
    /// - If the update is successful, the Ok variant will be returned with the file updated.
    /// - If the file that is being updated could not be found, the NotFound variant will be returned.
    /// - If the file being updated contains validation errors, the Invalid variant will be returned
    /// with the validation errors that were detected.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn update(&self, file: &File) -> UpdateResult<File, ValidationErrors, Box<dyn Error>>;

    /// # Description
    ///
    /// Update a file in persistent storage.
    ///
    /// # Arguments
    ///
    /// `file` - The file that is being updated in persistent storage.
    ///
    /// `context` - The context the update will be performed in.
    ///
    /// # Returns
    ///
    /// This function returns an update result:
    /// - If the update is successful, the Ok variant will be returned with the file updated.
    /// - If the file that is being updated could not be found, the NotFound variant will be returned.
    /// - If the file being updated contains validation errors, the Invalid variant will be returned
    /// with the validation errors that were detected.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn update_with_context(
        &self,
        file: &File,
        context: &mut QueryContext,
    ) -> UpdateResult<File, ValidationErrors, Box<dyn Error>>;

    /// # Description
    ///
    /// Remove a file from persistent storage.
    ///
    /// # Arguments
    ///
    /// `id` - The unique identifier of the file that is being removed from persistent storage.
    ///
    /// # Returns
    ///
    /// This function returns a deletion result:
    /// - If the deletion is successful and a file was successfully deleted, the Ok variant will be
    /// returned.
    /// - If the file being deleted could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn delete(&self, id: &u64) -> DeletionResult<Box<dyn Error>>;

    /// # Description
    ///
    /// Remove a file from persistent storage.
    ///
    /// # Arguments
    ///
    /// `id` - The unique identifier of the file that is being removed from persistent storage.
    ///
    /// `context` - The context the deletion will be performed in.
    ///
    /// # Returns
    ///
    /// This function returns a deletion result:
    /// - If the deletion is successful and a file was successfully deleted, the Ok variant will be
    /// returned.
    /// - If the file being deleted could not be found, the NotFound variant will be returned.
    /// - If an unexpected error occurs, the Err variant will be returned with the error that occurred.
    async fn delete_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> DeletionResult<Box<dyn Error>>;
}

/// A FileServiceImpl struct.
#[derive(Component)]
#[shaku(interface = FileService)]
pub(crate) struct FileServiceImpl {
    /// The file repository that will be used to manage persistent file data.
    #[shaku(inject)]
    file_repository: Arc<dyn FileRepository>,

    /// The database connection factory that will be used to acquire database connections.
    #[shaku(inject)]
    connection_factory: Arc<dyn DatabaseConnectionFactory>,
}
