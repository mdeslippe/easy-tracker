mod data;

use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse,
};
use shaku_actix::Inject;

use crate::{
    common::enumeration::{AuthenticationResult, InsertionResult, QueryResult},
    config::Config,
    feature::{
        auth::service::AuthService,
        user::{controller::data::GetUserResponseBody, model::User},
    },
    injector::DependencyInjector,
};

use self::data::CreateUserRequestBody;

use super::service::UserService;

/// # Description
///
/// Add the user controller configuration to a service config.
///
/// # Arguments
///
/// `config` - The service config that the user controller configuration will be added
pub(crate) fn configure(config: &mut ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(create_user)
            .service(get_user_by_id)
            .service(get_user_by_username),
    );
}

/// # Description
///
/// An api endpoint for creating user accounts.
///
/// # Arguments
///
/// `body` - The request body which contains information about the account that is being created.
///
/// `config` - The server's configuration data.
///
/// `user_service` - The user service that will be used to process the request.
///
/// # Returns
///
/// An http response.
#[post("")]
async fn create_user(
    body: web::Json<CreateUserRequestBody>,
    config: web::Data<Config>,
    user_service: Inject<DependencyInjector, dyn UserService>,
) -> HttpResponse {
    // Convert the request body into a user.
    let mut user: User = body.into_inner().into();
    user.profile_picture_url = config.default.user_profile_picture.clone();

    // Attempt to create the user, and return the result.
    return match user_service.insert(&user).await {
        InsertionResult::Ok(user) => HttpResponse::Created().json(user),
        InsertionResult::Invalid(details) => HttpResponse::BadRequest().json(details),
        InsertionResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

/// # Description
///
/// An api endpoint to get users by their id.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `id` - The id of the user that is being queried.
///
/// `auth_service` - The authentication service that will be used to authenticate the user sending
/// the request.
///
/// `user_service` - The user service that will be used to get the user's data.
///
/// # Returns
///
/// An http response.
#[get("/id/{id}")]
async fn get_user_by_id(
    request: HttpRequest,
    id: web::Path<u64>,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    user_service: Inject<DependencyInjector, dyn UserService>,
) -> HttpResponse {
    // Authenticate the user.
    match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(_) => {}
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Query the target user, and if found, convert the user into the response body format.
    let user: GetUserResponseBody = match user_service.get_by_id(&id).await {
        QueryResult::Ok(user) => user.into(),
        QueryResult::NotFound => return HttpResponse::NotFound().finish(),
        QueryResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Return the user.
    return HttpResponse::Ok().json(user);
}

/// # Description
///
/// An api endpoint to get users by their username.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `username` - The username of the user that is being queried.
///
/// `auth_service` - The authentication service that will be used to authenticate the user sending
/// the request.
///
/// `user_service` - The user service that will be used to get the user's data.
///
/// # Returns
///
/// An http response.
#[get("/username/{username}")]
async fn get_user_by_username(
    request: HttpRequest,
    username: web::Path<String>,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    user_service: Inject<DependencyInjector, dyn UserService>,
) -> HttpResponse {
    // Authenticate the user.
    match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(_) => {}
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Query the target user, and if found, convert the user into the response body format.
    let user: GetUserResponseBody = match user_service.get_by_username(&username).await {
        QueryResult::Ok(user) => user.into(),
        QueryResult::NotFound => return HttpResponse::NotFound().finish(),
        QueryResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Return the user.
    return HttpResponse::Ok().json(user);
}
