mod data;

use crate::{
    common::enumeration::AuthenticationResult,
    feature::{
        auth::{controller::data::LoginRequestBody, service::AuthService},
        crypto::service::CryptoService,
        user::model::User,
    },
    injector::DependencyInjector,
};
use actix_web::{
    cookie::{Cookie, SameSite},
    get, post,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse,
};
use shaku_actix::Inject;

/// # Description
///
/// Add the auth controller configuration to a service config.
///
/// # Arguments
///
/// `config` - The service config that the auth controller configuration will be added to.
pub(crate) fn configure(config: &mut ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(login)
            .service(logout)
            .service(status)
            .service(current_user),
    );
}

/// # Description
///
/// An api endpoint to authenticate a user.
///
/// # Arguments
///
/// `body` - The request body that will be used to attempt to authenticate the user.
///
/// `auth_service` - The authentication service that will be used to authenticate the user.
///
/// `crypto_service` - The crypto service that will be used to create a token for the user.
///
/// # Returns
///
/// An http response.
#[post("/login")]
async fn login(
    body: web::Json<LoginRequestBody>,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    crypto_service: Inject<DependencyInjector, dyn CryptoService>,
) -> HttpResponse {
    // Authenticate the user.
    let user: User = match auth_service
        .authenticate_credentials(&body.username, &body.password)
        .await
    {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Create an authentication token for the user.
    let token: String = match crypto_service.create_token(&user) {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Create an authentication cookie for the user.
    let auth_cookie: Cookie = Cookie::build("authorization", format!("Bearer {}", token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .expires(None)
        .finish();

    // Send the response.
    return HttpResponse::Ok().cookie(auth_cookie).json(user);
}

/// # Description
///
/// An api endpoint to unauthenticate a user.
///
/// # Returns
///
/// An http response.
#[post("/logout")]
async fn logout() -> HttpResponse {
    // Clear the user's authentication token.
    let auth_cookie: Cookie = Cookie::build("authorization", "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .expires(None)
        .finish();

    // Send the response.
    return HttpResponse::Ok().cookie(auth_cookie).finish();
}

/// # Description
///
/// An api endpoint to check if a user is authenticated.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `auth_service` - The auth service that will be used to check if the client is authenticated.
///
/// # Returns
///
/// An http response.
#[get("/status")]
async fn status(
    request: HttpRequest,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
) -> HttpResponse {
    // Check if the user is authenticated.
    return match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(_) => HttpResponse::Ok().json(true),
        AuthenticationResult::NotAuthenticated => HttpResponse::Ok().json(false),
        AuthenticationResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

/// # Description
///
/// An api endpoint to get information about the user that sent the request.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `auth_service` - The auth service that will be used to get information about the user that sent
/// the request.
///
/// # Returns
///
/// An http response.
#[get("/user")]
async fn current_user(
    request: HttpRequest,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
) -> HttpResponse {
    // Attempt to get the user that sent the request.
    return match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(user) => HttpResponse::Ok().json(user),
        AuthenticationResult::NotAuthenticated => HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}
