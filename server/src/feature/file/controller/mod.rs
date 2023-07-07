mod data;

use self::data::{CreateFileRequestBody, GetFileRequestParams, UpdateFileRequestBody};
use crate::{
    common::enumeration::{
        AuthenticationResult, DeletionResult, InsertionResult, QueryResult, UpdateResult,
    },
    feature::auth::service::AuthService,
    feature::{
        file::{model::File, service::FileService},
        user::model::User,
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
/// Add the file controller configuration to a service config.
///
/// # Arguments
///
/// `config` - The service config that the file controller configuration will be added to.
pub(crate) fn configure(config: &mut ServiceConfig) {
    config.service(
        web::scope("/files")
            .service(create_file)
            .service(get_file)
            .service(update_file)
            .service(delete_file),
    );
}

/// # Description
///
/// An api endpoint to create a file.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `body` - The request body which contains information about the file that is being created.
///
/// `auth_service` - The authentication service that will be used to authenticate the sending user.
///
/// `file_service` - The file service that will be used to create the file.
///
/// # Returns
///
/// An http response.
#[post("")]
async fn create_file(
    request: HttpRequest,
    body: web::Json<CreateFileRequestBody>,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    file_service: Inject<DependencyInjector, dyn FileService>,
) -> HttpResponse {
    // Authenticate the user.
    let user: User = match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Convert the request body into a file.
    let mut file: File = body.into_inner().into();
    file.user_id = user.id;

    // Create the file.
    return match file_service.insert(&file).await {
        InsertionResult::Ok(created_file) => HttpResponse::Ok().json(created_file),
        InsertionResult::Invalid(details) => HttpResponse::BadRequest().json(details),
        InsertionResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

/// # Description
///
/// An api endpoint to get a file.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `id` - The id of the file that is being retrieved.
///
/// `params` - Query parameters sent with the request.
///
/// `auth_service` - The authentication service that will be used to authenticate the sending user.
///
/// `file_service` - The file service that will be used to retrieve the file.
///
/// # Returns
///
/// An http response.
#[get("/{id}")]
async fn get_file(
    request: HttpRequest,
    id: web::Path<u64>,
    params: web::Query<GetFileRequestParams>,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    file_service: Inject<DependencyInjector, dyn FileService>,
) -> HttpResponse {
    // Authenticate the user.
    match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(_) => {}
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Get the file the user is requesting.
    let file: File = match file_service.get(&id).await {
        QueryResult::Ok(file) => file,
        QueryResult::NotFound => return HttpResponse::NotFound().finish(),
        QueryResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // If the client wants the raw file data returned.
    if params.raw == Some(true) {
        return HttpResponse::Ok()
            .content_type(file.mime_type)
            .body(file.data);
    } else {
        return HttpResponse::Ok().json(file);
    }
}

/// # Description
///
/// An api endpoint to update a file.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `id` - The id of the file that is being updated.
///
/// `body` - The request body which contains information about the file that is being updated.
///
/// `auth_service` - The authentication service that will be used to authenticate the sending user.
///
/// `file_service` - The file service that will be used to update the file.
///
/// # Returns
///
/// An http response.
#[patch("/{id}")]
async fn update_file(
    request: HttpRequest,
    id: web::Path<u64>,
    body: web::Json<UpdateFileRequestBody>,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    file_service: Inject<DependencyInjector, dyn FileService>,
) -> HttpResponse {
    // Authenticate the user.
    let user: User = match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Get the file that is being updated.
    let mut file: File = match file_service.get(&id).await {
        QueryResult::Ok(file) => file,
        QueryResult::NotFound => return HttpResponse::NotFound().finish(),
        QueryResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // If the user is not the owner of the file, they are not allowed to updated it.
    if user.id != file.user_id {
        return HttpResponse::Forbidden().finish();
    }

    // Apply the update to the file.
    body.apply(&mut file);

    // Update the file.
    return match file_service.update(&file).await {
        UpdateResult::Ok(file) => HttpResponse::Ok().json(file),
        UpdateResult::NotFound => HttpResponse::NotFound().finish(),
        UpdateResult::Invalid(details) => HttpResponse::BadRequest().json(details),
        UpdateResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

/// # Description
///
/// An api endpoint to delete a file.
///
/// # Arguments
///
/// `request` - The http request.
///
/// `id` - The id of the file that is being deleted.
///
/// `auth_service` - The authentication service that will be used to authenticate the sending user.
///
/// `file_service` - The file service that will be used to delete the file.
///
/// # Returns
///
/// An http response.
#[delete("/{id}")]
async fn delete_file(
    request: HttpRequest,
    id: web::Path<u64>,
    auth_service: Inject<DependencyInjector, dyn AuthService>,
    file_service: Inject<DependencyInjector, dyn FileService>,
) -> HttpResponse {
    // Authenticate the user.
    let user: User = match auth_service.authenticate_request(&request).await {
        AuthenticationResult::Ok(user) => user,
        AuthenticationResult::NotAuthenticated => return HttpResponse::Unauthorized().finish(),
        AuthenticationResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Get the file that is being deleted.
    let file: File = match file_service.get(&id).await {
        QueryResult::Ok(file) => file,
        QueryResult::NotFound => return HttpResponse::NotFound().finish(),
        QueryResult::Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // If the user is not the owner of the file, they are not allowed to delete it.
    if user.id != file.user_id {
        return HttpResponse::Forbidden().finish();
    }

    // Delete the file.
    return match file_service.delete(&id).await {
        DeletionResult::Ok => HttpResponse::Ok().finish(),
        DeletionResult::NotFound => HttpResponse::NotFound().finish(),
        DeletionResult::Err(_) => HttpResponse::InternalServerError().finish(),
    };
}
