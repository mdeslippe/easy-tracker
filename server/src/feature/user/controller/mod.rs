mod data;

use self::data::{CreateUserRequestBody, UpdateUserRequestBody};
use crate::{
    common::enumeration::{
        AuthenticationResult, DeletionResult, InsertionResult, QueryResult, UpdateResult,
    },
    config::Config,
    feature::{
        auth::service::AuthService,
        user::{controller::data::GetUserResponseBody, model::User, service::UserService},
    },
    injector::DependencyInjector,
};
use actix_web::{
    delete, get, patch, post,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse,
};
use shaku_actix::Inject;

/// # Description
///
/// Add the user controller configuration to a service config.
///
/// # Arguments
///
/// `config` - The service config that the user controller configuration will be added to.
pub(crate) fn configure(config: &mut ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(create_user)
            .service(get_user_by_id)
            .service(get_user_by_username)
            .service(get_user_by_email)
            .service(update_user)
            .service(delete_user),
    );
}

/// # Description
///
/// An api endpoint to create a user account.
///
/// # Arguments
///
/// `body` - The request body which contains information about the account that is being created.
///
/// `config` - The server's configuration data.
///
/// `user_service` - The user service that will be used to create the user.
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

    // Give the user the default profile picture.
    user.profile_picture_url = config.default.user_profile_picture.clone();

    // Create the user.
    return match user_service.insert(&user).await {
        InsertionResult::Ok(user) => HttpResponse::Created().json(user),
        InsertionResult::Invalid(details) => HttpResponse::BadRequest().json(details),
        InsertionResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

/// # Description
///
/// An api endpoint to get a user account by their id.
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
/// `user_service` - The user service that will be used to get the user.
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
/// An api endpoint to get a user account by their username.
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
/// `user_service` - The user service that will be used to get the user.
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

/// # Description
///
/// An api endpoint to get a user account by their email address.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `email` - The email address of the user that is being queried.
///
/// `auth_service` - The authentication service that will be used to authenticate the user sending
/// the request.
///
/// `user_service` - The user service that will be used to get the user.
///
/// # Returns
///
/// An http response.
#[get("/email/{email}")]
async fn get_user_by_email(
    request: HttpRequest,
    email: web::Path<String>,
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
    let user: GetUserResponseBody = match user_service.get_by_email(&email).await {
        QueryResult::Ok(user) => user.into(),
        QueryResult::NotFound => return HttpResponse::NotFound().finish(),
        QueryResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Return the user.
    return HttpResponse::Ok().json(user);
}

/// # Description
///
/// An api endpoint to update a user account.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `update` - The update request data.
///
/// `auth_service` - The authentication service that will be used to authenticate the user sending
/// the request.
///
/// `user_service` - The user service that will be used to update the user.
///
/// # Returns
///
/// An http response.
#[patch("")]
async fn update_user(
    request: HttpRequest,
    update: web::Json<UpdateUserRequestBody>,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    user_service: Inject<DependencyInjector, dyn UserService>,
) -> HttpResponse {
    // Authenticate the user.
    let mut user: User = match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Apply the update to the user.
    update.apply(&mut user);

    // Update the user.
    return match user_service.update(&user).await {
        UpdateResult::Ok(user) => HttpResponse::Ok().json(user),
        UpdateResult::NotFound => HttpResponse::NotFound().finish(),
        UpdateResult::Invalid(details) => HttpResponse::BadRequest().json(details),
        UpdateResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

/// # Description
///
/// An api endpoint to delete a user account.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `auth_service` - The authentication service that will be used to authenticate the user sending
/// the request.
///
/// `user_service` - The user service that will be used to delete the user.
///
/// # Returns
///
/// An http response.
#[delete("")]
async fn delete_user(
    request: HttpRequest,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    user_service: Inject<DependencyInjector, dyn UserService>,
) -> HttpResponse {
    // Authenticate the user.
    let user: User = match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Delete the user.
    return match user_service.delete(&user.id).await {
        DeletionResult::Ok => HttpResponse::Ok().finish(),
        DeletionResult::NotFound => HttpResponse::NotFound().finish(),
        DeletionResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}
