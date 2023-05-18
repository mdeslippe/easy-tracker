use crate::feature::user::model::User;
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::Error;

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
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the insertion is successful, the Ok variant will be returned with the id of the user
    /// that was inserted.
    /// - If the insertion is not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn insert(&self, user: &User) -> Result<u64, Error>;

    /// # Description
    ///
    /// Query a user from the user repository by their id.
    ///
    /// # Arguments
    ///
    /// `id` - The id of the user to query from the user repository.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the query was successful, the Ok variant will be returned with an optional user.
    /// - If the query was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn get_by_id(&self, id: &u64) -> Result<Option<User>, Error>;

    /// # Description
    ///
    /// Query a user from the user repository by their username.
    ///
    /// # Arguments
    ///
    /// `username` - The username of the user to query from the user repository.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the query was successful, the Ok variant will be returned with an optional user.
    /// - If the query was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn get_by_username(&self, username: &String) -> Result<Option<User>, Error>;

    /// # Description
    ///
    /// Query a user from the user repository by their email address.
    ///
    /// # Arguments
    ///
    /// `email` - The email address of the user to query from the user repository.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the query was successful, the Ok variant will be returned with an optional user.
    /// - If the query was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn get_by_email(&self, email: &String) -> Result<Option<User>, Error>;

    /// # Description
    ///
    /// Update a user in the user repository.
    ///
    /// # Arguments
    ///
    /// `user` - The user to update in the user repository.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the update was successful, the Ok variant will be returned with the amount of records
    /// modified.
    /// - If the update was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn update(&self, user: &User) -> Result<u64, Error>;

    /// # Description
    ///
    /// Delete a user from the user repository.
    ///
    /// # Arguments
    ///
    /// `id` - The id of the user to delete from the user repository.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the deletion was successful, the Ok variant will be returned with the amount of records
    /// deleted.
    /// - If the deletion was not successful, the Err variant will be returned with the error that
    /// occurred.
    async fn delete(&self, id: &u64) -> Result<u64, Error>;
}

/// A UserRepositoryImpl struct.
#[derive(Component)]
#[shaku(interface = UserRepository)]
pub(crate) struct UserRepositoryImpl;

/// A UserRepository implementation for the UserRepositoryImpl struct.
#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn insert(&self, user: &User) -> Result<u64, Error> {
        todo!();
    }

    async fn get_by_id(&self, id: &u64) -> Result<Option<User>, Error> {
        todo!();
    }

    async fn get_by_username(&self, username: &String) -> Result<Option<User>, Error> {
        todo!();
    }

    async fn get_by_email(&self, email: &String) -> Result<Option<User>, Error> {
        todo!();
    }

    async fn update(&self, user: &User) -> Result<u64, Error> {
        todo!();
    }

    async fn delete(&self, id: &u64) -> Result<u64, Error> {
        todo!();
    }
}
