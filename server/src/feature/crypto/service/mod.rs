use shaku::Interface;

use crate::feature::{crypto::model::UserClaims, user::model::User};
use jsonwebtoken::TokenData;

/// A crypto service trait.
pub(crate) trait CryptoService: Interface {
    /// # Description
    ///
    /// Create an authentication token for a user.
    ///
    /// # Arguments
    ///
    /// `user` - The user the authentication token is being created for.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the token creation was successful, the Ok variant will be returned with the token that
    /// was created.
    /// - If the token creation was not successful, the Err variant will be returned with the error
    /// that occurred.
    fn create_token(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error>;

    /// # Description
    ///
    /// Decode an authentication token.
    ///
    /// # Arguments
    ///
    /// `token` - The authentication token that will be decoded.
    ///
    /// # Returns
    ///
    /// This functions returns a result:
    /// - If the token was successfully decoded, the Ok variant will be returned with the data that
    /// was encoded in the token.
    /// - If the token was not successfully decoded, the Err variant will be returned with the error
    /// that occurred.
    fn decode_token(
        &self,
        token: &String,
    ) -> Result<TokenData<UserClaims>, jsonwebtoken::errors::Error>;

    /// # Description
    ///
    /// Hash a password.
    ///
    /// # Arguments
    ///
    /// `password` - The plain-text password that will be hashed.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the password was successfully hashed, the Ok variant will be returned with the password
    /// hash.
    /// - If the password was not successfully hashed, the Err variant will be returned with the
    /// error that occurred.
    fn hash_password(&self, password: &String) -> Result<String, argon2::password_hash::Error>;

    /// # Description
    ///
    /// Check to see if a plain-text password corresponds to a password hash.
    ///
    /// # Arguments
    ///
    /// `password` - The plain-text password that is being verified.
    ///
    /// `hash` - The password hash that the plain-text password is being verified against.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If the password was successfully verified and is correct, the Ok variant will be returned.
    /// - If the password was not successfully verified or was not correct, the Err variant will be
    /// returned with the error that occurred. Note that an incorrect password will be treated as an
    /// error.
    fn verify_password(
        &self,
        password: &String,
        hash: &String,
    ) -> Result<(), argon2::password_hash::Error>;
}
