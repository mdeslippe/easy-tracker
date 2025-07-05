#[cfg(test)]
mod test;

use crate::{common::enumeration::QueryContext, feature::user::model::User};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{Error, Row};
use time::OffsetDateTime;

/// A user repository trait.
#[async_trait]
pub(crate) trait UserRepository: Interface {
    /// # Description
    ///
    /// Insert a user into the user repository.
    ///
    /// # Arguments
    ///
    /// `user` - The user to insert into the user repository.
    ///
    /// `context` - The query context the insertion will be executed in.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the insertion was successful, the Ok variant will be returned with the id of the user
    /// that was inserted.
    /// - If the insertion was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn insert(&self, user: &User, context: &mut QueryContext) -> Result<u64, Error>;

    /// # Description
    ///
    /// Query a user from the user repository by their id.
    ///
    /// # Arguments
    ///
    /// `id` - The id of the user to query from the user repository.
    ///
    /// `context` - The query context the query will be executed in.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the query was successful, the Ok variant will be returned with an optional user.
    /// - If the query was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn get_by_id(&self, id: &u64, context: &mut QueryContext) -> Result<Option<User>, Error>;

    /// # Description
    ///
    /// Query a user from the user repository by their username.
    ///
    /// # Arguments
    ///
    /// `username` - The username of the user to query from the user repository.
    ///
    /// `context` - The query context the query will be executed in.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the query was successful, the Ok variant will be returned with an optional user.
    /// - If the query was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn get_by_username(
        &self,
        username: &String,
        context: &mut QueryContext,
    ) -> Result<Option<User>, Error>;

    /// # Description
    ///
    /// Query a user from the user repository by their email address.
    ///
    /// # Arguments
    ///
    /// `email` - The email address of the user to query from the user repository.
    ///
    /// `context` - The query context the query will be executed in.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the query was successful, the Ok variant will be returned with an optional user.
    /// - If the query was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn get_by_email(
        &self,
        email: &String,
        context: &mut QueryContext,
    ) -> Result<Option<User>, Error>;

    /// # Description
    ///
    /// Update a user in the user repository.
    ///
    /// # Arguments
    ///
    /// `user` - The user to update in the user repository.
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
    async fn update(&self, user: &User, context: &mut QueryContext) -> Result<u64, Error>;

    /// # Description
    ///
    /// Delete a user from the user repository.
    ///
    /// # Arguments
    ///
    /// `id` - The id of the user to delete from the user repository.
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

/// A UserRepositoryImpl struct.
#[derive(Component)]
#[shaku(interface = UserRepository)]
pub(crate) struct UserRepositoryImpl;

/// A UserRepository implementation for the UserRepositoryImpl struct.
#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn insert(&self, user: &User, context: &mut QueryContext) -> Result<u64, Error> {
        // Prepare the query.
        let query = sqlx::query!(
            r#"
                INSERT INTO `users` (
                    `id`,
                    `account_created_at`,
                    `password_reset_at`,
                    `profile_picture_url`,
                    `username`,
                    `password`,
                    `email`,
                    `email_is_verified`,
                    `password_reset_is_required`,
                    `account_is_locked`,
                    `account_is_banned`
                ) VALUES (
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?
                ) RETURNING `id`;
            "#,
            user.id,
            user.account_created_at,
            user.password_reset_at,
            user.profile_picture_url,
            user.username,
            user.password,
            user.email,
            user.email_is_verified,
            user.password_reset_is_required,
            user.account_is_locked,
            user.account_is_banned
        );

        // Execute the query.
        let result = match context {
            QueryContext::Connection(connection) => query.fetch_one(connection.as_mut()).await,
            QueryContext::Transaction(transaction) => query.fetch_one(transaction.as_mut()).await,
        }?;

        return result.try_get(0);
    }

    async fn get_by_id(&self, id: &u64, context: &mut QueryContext) -> Result<Option<User>, Error> {
        // Prepare the query.
        let query = sqlx::query_as!(
            User,
            r#"
                SELECT
                    `id` AS `id: u64`,
                    `account_created_at` AS `account_created_at: OffsetDateTime`,
                    `password_reset_at` AS `password_reset_at: OffsetDateTime`,
                    `profile_picture_url` AS `profile_picture_url: String`,
                    `username` AS `username: String`,
                    `password` AS `password: String`,
                    `email` AS `email: String`,
                    `email_is_verified` AS `email_is_verified: bool`,
                    `password_reset_is_required` AS `password_reset_is_required: bool`,
                    `account_is_locked` AS `account_is_locked: bool`,
                    `account_is_banned` AS `account_is_banned: bool`
                FROM
                    `users`
                WHERE
                    `id` = ?;
            "#,
            id
        );

        // Execute the query.
        return match context {
            QueryContext::Connection(connection) => query.fetch_optional(connection.as_mut()).await,
            QueryContext::Transaction(transaction) => query.fetch_optional(transaction.as_mut()).await,
        };
    }

    async fn get_by_username(
        &self,
        username: &String,
        context: &mut QueryContext,
    ) -> Result<Option<User>, Error> {
        // Prepare the query.
        let query = sqlx::query_as!(
            User,
            r#"
                SELECT
                    `id` AS `id: u64`,
                    `account_created_at` AS `account_created_at: OffsetDateTime`,
                    `password_reset_at` AS `password_reset_at: OffsetDateTime`,
                    `profile_picture_url` AS `profile_picture_url: String`,
                    `username` AS `username: String`,
                    `password` AS `password: String`,
                    `email` AS `email: String`,
                    `email_is_verified` AS `email_is_verified: bool`,
                    `password_reset_is_required` AS `password_reset_is_required: bool`,
                    `account_is_locked` AS `account_is_locked: bool`,
                    `account_is_banned` AS `account_is_banned: bool`
                FROM
                    `users`
                WHERE
                    `username` = ?;
            "#,
            username
        );

        // Execute the query.
        return match context {
            QueryContext::Connection(connection) => query.fetch_optional(connection.as_mut()).await,
            QueryContext::Transaction(transaction) => query.fetch_optional(transaction.as_mut()).await,
        };
    }

    async fn get_by_email(
        &self,
        email: &String,
        context: &mut QueryContext,
    ) -> Result<Option<User>, Error> {
        // Prepare the query.
        let query = sqlx::query_as!(
            User,
            r#"
                SELECT
                    `id` AS `id: u64`,
                    `account_created_at` AS `account_created_at: OffsetDateTime`,
                    `password_reset_at` AS `password_reset_at: OffsetDateTime`,
                    `profile_picture_url` AS `profile_picture_url: String`,
                    `username` AS `username: String`,
                    `password` AS `password: String`,
                    `email` AS `email: String`,
                    `email_is_verified` AS `email_is_verified: bool`,
                    `password_reset_is_required` AS `password_reset_is_required: bool`,
                    `account_is_locked` AS `account_is_locked: bool`,
                    `account_is_banned` AS `account_is_banned: bool`
                FROM
                    `users`
                WHERE
                    `email` = ?;
            "#,
            email
        );

        // Execute the query.
        return match context {
            QueryContext::Connection(connection) => query.fetch_optional(connection.as_mut()).await,
            QueryContext::Transaction(transaction) => query.fetch_optional(transaction.as_mut()).await,
        };
    }

    async fn update(&self, user: &User, context: &mut QueryContext) -> Result<u64, Error> {
        // Prepare the query.
        let query = sqlx::query!(
            r#"
                UPDATE
                    `users`
                SET
                    `account_created_at` = ?,
                    `password_reset_at` = ?,
                    `profile_picture_url` = ?,
                    `username` = ?,
                    `password` = ?,
                    `email` = ?,
                    `email_is_verified` = ?,
                    `password_reset_is_required` = ?,
                    `account_is_locked` = ?,
                    `account_is_banned` = ?
                WHERE
                    `id` = ?;
            "#,
            user.account_created_at,
            user.password_reset_at,
            user.profile_picture_url,
            user.username,
            user.password,
            user.email,
            user.email_is_verified,
            user.password_reset_is_required,
            user.account_is_locked,
            user.account_is_banned,
            user.id
        );

        // Execute the query.
        let result = match context {
            QueryContext::Connection(connection) => query.execute(connection.as_mut()).await,
            QueryContext::Transaction(transaction) => query.execute(transaction.as_mut()).await,
        }?;

        return Ok(result.rows_affected());
    }

    async fn delete(&self, id: &u64, context: &mut QueryContext) -> Result<u64, Error> {
        // Prepare the query.
        let query = sqlx::query!(
            r#"
                DELETE FROM
                    `users`
                WHERE
                    `id` = ?;
            "#,
            id
        );

        // Execute the query.
        let result = match context {
            QueryContext::Connection(connection) => query.execute(connection.as_mut()).await,
            QueryContext::Transaction(transaction) => query.execute(transaction.as_mut()).await,
        }?;

        return Ok(result.rows_affected());
    }
}
