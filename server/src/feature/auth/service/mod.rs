use crate::{
    common::{
        enumeration::{AuthenticationResult, QueryResult},
        utility::{get_token_from_cookie, get_token_from_header},
    },
    feature::{
        crypto::{model::UserClaims, service::CryptoService},
        user::{model::User, service::UserService},
    },
};
use actix_web::HttpRequest;
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

/// The prefix for Bearer tokens.
const BEARER: &str = "Bearer ";

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

/// An AuthServiceImpl struct.
#[derive(Component)]
#[shaku(interface = AuthService)]
pub(crate) struct AuthServiceImpl {
    /// The crypto service that will be used for authentication.
    #[shaku(inject)]
    crypto_service: Arc<dyn CryptoService>,

    /// The user service that will be used for authentication.
    #[shaku(inject)]
    user_service: Arc<dyn UserService>,
}

/// An AuthService implementation for the AuthServiceImpl struct.
#[async_trait(?Send)]
impl AuthService for AuthServiceImpl {
    async fn authenticate_credentials(
        &self,
        username: &String,
        password: &String,
    ) -> AuthenticationResult {
        // Query the user from persistent storage.
        let user: User = match __self.user_service.get_by_username(username).await {
            QueryResult::Ok(user) => user,
            QueryResult::NotFound => return AuthenticationResult::NotAuthenticated,
            QueryResult::Err(error) => return AuthenticationResult::Err(error),
        };

        // Make sure the password is correct.
        return match __self
            .crypto_service
            .verify_password(password, &user.password)
        {
            Ok(()) => AuthenticationResult::Ok(user),
            Err(_) => AuthenticationResult::NotAuthenticated,
        };
    }

    async fn authenticate_token(&self, token: &String) -> AuthenticationResult {
        // Decode the token.
        let claims: UserClaims = match __self.crypto_service.decode_token(token) {
            Ok(token_data) => token_data.claims,
            Err(_) => return AuthenticationResult::NotAuthenticated,
        };

        // Query the user from persistent storage.
        let user: User = match __self.user_service.get_by_id(&claims.id).await {
            QueryResult::Ok(user) => user,
            QueryResult::NotFound => return AuthenticationResult::NotAuthenticated,
            QueryResult::Err(error) => return AuthenticationResult::Err(error),
        };

        // Make sure the token has not expired (this happens when the user resets their password).
        if claims.password_last_reset == user.password_reset_at.unix_timestamp() {
            return AuthenticationResult::Ok(user);
        } else {
            return AuthenticationResult::NotAuthenticated;
        }
    }

    async fn authenticate_request(&self, request: &HttpRequest) -> AuthenticationResult {
        // Attempt to locate the user's authentication token from the request.
        // This will attempt to locate the token in the request headers before checking cookies
        // to allow overriding cookies when sending requests.
        let token: String =
            match get_token_from_header(request).or_else(|| get_token_from_cookie(request)) {
                Some(token) => token,
                None => return AuthenticationResult::NotAuthenticated,
            };

        // Make sure the token is in the correct format.
        if !token.starts_with(BEARER) {
            return AuthenticationResult::NotAuthenticated;
        }

        // Now that we have the token, we can authenticate the user.
        return __self
            .authenticate_token(&token.trim_start_matches(BEARER).to_owned())
            .await;
    }
}
