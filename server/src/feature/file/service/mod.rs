use crate::{
    common::enumeration::{
        DeletionResult, InsertionResult, QueryContext, QueryResult, UpdateResult,
    },
    database::DatabaseConnectionFactory,
    feature::file::{model::File, repository::FileRepository},
};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::Connection;
use std::{error::Error, io, sync::Arc};
use validator::{Validate, ValidationErrors};

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

/// A FileService implementation for the FileServiceImpl struct.
#[async_trait(?Send)]
impl FileService for FileServiceImpl {
    async fn insert(&self, file: &File) -> InsertionResult<File, ValidationErrors, Box<dyn Error>> {
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
        let insertion_result = self.insert_with_context(file, &mut context).await;

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
        file: &File,
        context: &mut QueryContext,
    ) -> InsertionResult<File, ValidationErrors, Box<dyn Error>> {
        // Validate the file.
        match file.validate() {
            Ok(()) => {}
            Err(errors) => return InsertionResult::Invalid(errors),
        };

        // Perform the insertion.
        let file_id = match __self.file_repository.insert(file, context).await {
            Ok(file_id) => file_id,
            Err(error) => return InsertionResult::Err(Box::new(error)),
        };

        // Query the file was was inserted.
        let inserted_file_option = match __self.file_repository.get(&file_id, context).await {
            Ok(inserted_file_option) => inserted_file_option,
            Err(error) => return InsertionResult::Err(Box::new(error)),
        };

        // If the file was found, return the file, otherwise return an error.
        return match inserted_file_option {
            Some(inserted_file) => InsertionResult::Ok(inserted_file),
            None => InsertionResult::Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "File could not be found after insertion",
            ))),
        };
    }

    async fn get(&self, id: &u64) -> QueryResult<File, Box<dyn Error>> {
        // Acquire a database connection.
        let connection = match __self.connection_factory.get_connection().await {
            Ok(connection) => connection,
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // Create the query context.
        let mut context = QueryContext::Connection(connection);

        // Perform the query.
        return self.get_with_context(id, &mut context).await;
    }

    async fn get_with_context(
        &self,
        id: &u64,
        context: &mut QueryContext,
    ) -> QueryResult<File, Box<dyn Error>> {
        // Perform the query.
        let result = match __self.file_repository.get(id, context).await {
            Ok(file_option) => file_option,
            Err(error) => return QueryResult::Err(Box::new(error)),
        };

        // If the file was found, return the file, otherwise return not found.
        return match result {
            Some(file) => QueryResult::Ok(file),
            None => QueryResult::NotFound,
        };
    }

    async fn update(&self, file: &File) -> UpdateResult<File, ValidationErrors, Box<dyn Error>> {
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
        let update_result = self.update_with_context(file, &mut context).await;

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
        file: &File,
        context: &mut QueryContext,
    ) -> UpdateResult<File, ValidationErrors, Box<dyn Error>> {
        // Validate the file.
        match file.validate() {
            Ok(()) => {}
            Err(errors) => return UpdateResult::Invalid(errors),
        };

        // Perform the update.
        let records_updated = match __self.file_repository.update(&file, context).await {
            Ok(records_updated) => records_updated,
            Err(error) => return UpdateResult::Err(Box::new(error)),
        };

        // If no records were updated.
        if records_updated == 0 {
            return UpdateResult::NotFound;
        }

        // Query the updated file.
        let updated_file_option = match __self.file_repository.get(&file.id, context).await {
            Ok(updated_file_option) => updated_file_option,
            Err(error) => return UpdateResult::Err(Box::new(error)),
        };

        // Make sure the file was found, and return the updated file.
        return match updated_file_option {
            Some(updated_file) => UpdateResult::Ok(updated_file),
            None => UpdateResult::Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "File could not be found after update",
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
        let records_deleted = match __self.file_repository.delete(id, context).await {
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
