use crate::{common::enumeration::QueryContext, feature::file::model::File};
use async_trait::async_trait;
use shaku::Interface;
use sqlx::Error;

/// A file repository trait.
#[async_trait]
pub(crate) trait FileRepository: Interface {
    /// # Description
    ///
    /// Insert a file into the file repository.
    ///
    /// # Arguments
    ///
    /// `file` - The file to insert into the file repository.
    ///
    /// `context` - The query context the insertion will be executed in.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the insertion was successful, the Ok variant will be returned with the id of the file
    /// that was inserted.
    /// - If the insertion was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn insert(&self, file: &File, context: &mut QueryContext) -> Result<u64, Error>;

    /// # Description
    ///
    /// Get a file from the file repository.
    ///
    /// # Arguments
    ///
    /// `id` - The id of the file to get from the file repository.
    ///
    /// `context` - The query context the query will be executed in.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the query was successful, the Ok variant will be returned with an optional file.
    /// - If the query was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn get(&self, id: &u64, context: &mut QueryContext) -> Result<Option<File>, Error>;

    /// # Description
    ///
    /// Update a file in the file repository.
    ///
    /// # Arguments
    ///
    /// `file` - The file to update in the file repository.
    ///
    /// `context` - The query context the update will be executed in.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the update was successful, the Ok variant will be returned with the amount of records
    /// modified.
    /// - If the update was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn update(&self, file: &File, context: &mut QueryContext) -> Result<u64, Error>;

    /// # Description
    ///
    /// Delete a file from the file repository.
    ///
    /// # Arguments
    ///
    /// `id` - The id of the file to delete from the file repository.
    ///
    /// `context` - The query context the deletion will be executed in.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the deletion was successful, the Ok variant will be returned with the amount of records
    /// deleted.
    /// - If the deletion was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn delete(&self, id: &u64, context: &mut QueryContext) -> Result<u64, Error>;
}
