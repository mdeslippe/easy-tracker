use crate::common::enumeration::AuthenticationResult;
use actix_web::HttpRequest;
use async_trait::async_trait;
use shaku::Interface;

/// An authentication service trait.
#[async_trait(?Send)]
pub(crate) trait AuthService: Interface {
    /// # Description
    ///
    /// Authenticate a user with login credentials.
    ///
    /// # Arguments
    ///
    /// `username` - The username being authenticated.
    ///
    /// `password` - The password being authenticated.
    ///
    /// # Returns
    ///
    /// This function returns an authentication result:
    /// - If authentication is successful, the Ok variant will be returned with the User that was
    /// authenticated.
    /// - If authentication is not successful, the NotAuthenticated variant will be returned.
    /// - If an error occurs during the authentication process, the Err variant will be returned
    /// with the error that occurred.
    async fn authenticate_credentials(
        &self,
        username: &String,
        password: &String,
    ) -> AuthenticationResult;

    /// # Description
    ///
    /// Authenticate a user with a user token.
    ///
    /// # Arguments
    ///
    /// `token` - The token being authenticated.
    ///
    /// # Returns
    ///
    /// This function returns an authentication result:
    /// - If authentication is successful, the Ok variant will be returned with the User that was
    /// authenticated.
    /// - If authentication is not successful, the NotAuthenticated variant will be returned.
    /// - If an error occurs during the authentication process, the Err variant will be returned
    /// with the error that occurred.
    async fn authenticate_token(&self, token: &String) -> AuthenticationResult;

    /// # Description
    ///
    /// Authenticate a user with http request data.
    ///
    /// # Arguments
    ///
    /// `request` - The http request being authenticated.
    ///
    /// # Returns
    ///
    /// This function returns an authentication result:
    /// - If authentication is successful, the Ok variant will be returned with the User that was
    /// authenticated.
    /// - If authentication is not successful, the NotAuthenticated variant will be returned.
    /// - If an error occurs during the authentication process, the Err variant will be returned
    /// with the error that occurred.
    async fn authenticate_request(&self, request: &HttpRequest) -> AuthenticationResult;
}
