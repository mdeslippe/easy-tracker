mod data;

use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse,
};
use shaku_actix::Inject;

use crate::{
    common::enumeration::InsertionResult, config::Config, feature::user::model::User,
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
    config.service(web::scope("/users").service(create_user));
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
/// This endpoint will return the following:
/// - If the request is successful, an http CREATED response will be returned with the user data
/// that was created.
/// - If validation errors occur, an http BAD REQUEST response will be returned with the validation
/// errors.
/// - If an unexpected error occurs, an http INTERNAL SERVER ERROR response will be returned.
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
