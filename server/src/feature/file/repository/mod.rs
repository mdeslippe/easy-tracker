#[cfg(test)]
mod test;

use crate::{common::enumeration::QueryContext, feature::file::model::File};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{Error, Row};
use time::OffsetDateTime;

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

/// A FileRepositoryImpl struct.
#[derive(Component)]
#[shaku(interface = FileRepository)]
pub(crate) struct FileRepositoryImpl;

/// A FileRepository implementation for the FileRepositoryImpl struct.
#[async_trait]
impl FileRepository for FileRepositoryImpl {
    async fn insert(&self, file: &File, context: &mut QueryContext) -> Result<u64, Error> {
        // Prepare the query.
        let query = sqlx::query!(
            r#"
                INSERT INTO `files` (
                    `id`,
                    `user_id`,
                    `file_created_at`,
                    `mime_type`,
                    `name`,
                    `data`
                ) VALUES (
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?
                ) RETURNING `id`;
            "#,
            file.id,
            file.user_id,
            file.file_created_at,
            file.mime_type,
            file.name,
            file.data
        );

        // Execute the query.
        let result = match context {
            QueryContext::Connection(connection) => query.fetch_one(connection).await,
            QueryContext::Transaction(transaction) => query.fetch_one(transaction).await,
        }?;

        return result.try_get(0);
    }

    async fn get(&self, id: &u64, context: &mut QueryContext) -> Result<Option<File>, Error> {
        // Prepare the query.
        let query = sqlx::query_as!(
            File,
            r#"
                SELECT
                    `id` AS `id: u64`,
                    `user_id` AS `user_id: u64`,
                    `file_created_at` AS `file_created_at: OffsetDateTime`,
                    `mime_type` AS `mime_type: String`,
                    `name` AS `name: String`,
                    `data` AS `data: Vec<u8>`
                FROM
                    `files`
                WHERE
                    `id` = ?;
            "#,
            id
        );

        // Execute the query.
        return match context {
            QueryContext::Connection(connection) => query.fetch_optional(connection).await,
            QueryContext::Transaction(transaction) => query.fetch_optional(transaction).await,
        };
    }

    async fn update(&self, file: &File, context: &mut QueryContext) -> Result<u64, Error> {
        // Prepare the query.
        let query = sqlx::query!(
            r#"
                UPDATE
                    `files`
                SET
                    `user_id` = ?,
                    `file_created_at` = ?,
                    `mime_type` = ?,
                    `name` = ?,
                    `data` = ?
                WHERE
                    `id` = ?;
            "#,
            file.user_id,
            file.file_created_at,
            file.mime_type,
            file.name,
            file.data,
            file.id
        );

        // Execute the query.
        let result = match context {
            QueryContext::Connection(connection) => query.execute(connection).await,
            QueryContext::Transaction(transaction) => query.execute(transaction).await,
        }?;

        return Ok(result.rows_affected());
    }

    async fn delete(&self, id: &u64, context: &mut QueryContext) -> Result<u64, Error> {
        // Prepare the query.
        let query = sqlx::query!(
            r#"
                DELETE FROM
                    `files`
                WHERE
                    `id` = ?;
            "#,
            id
        );

        // Execute the query.
        let result = match context {
            QueryContext::Connection(connection) => query.execute(connection).await,
            QueryContext::Transaction(transaction) => query.execute(transaction).await,
        }?;

        return Ok(result.rows_affected());
    }
}
