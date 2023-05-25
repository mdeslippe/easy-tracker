use actix_cors::Cors;
use actix_web::{
    http::{
        header::{HeaderValue, AUTHORIZATION},
        Method,
    },
    HttpRequest,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use std::{borrow::Cow, env};
use validator::ValidationError;

use crate::config::Config;

/// # Description
///
/// Get the file path of the server's configuration file.
///
/// If arguments are passed in when executing the application, that will be used as the file path.
/// If arguments are not passed in when executing the application, this this will attempt to load
/// it from the present working directory.
///
/// Note that the returned path could be malformed or could be a path to a file that does not exist.
///
/// # Returns
///
/// The file path of the server's configuration file.
pub(crate) fn get_config_path() -> String {
    // Get arguments that were passed in when executing the application.
    // The first argument will always be the name of the program being executed, so we must skip it.
    let args: Vec<String> = env::args().skip(1).collect();

    // If a configuration file path was specified return that, otherwise return the default path.
    if args.len() > 0 {
        return args.join(" ");
    } else {
        return String::from("config.json");
    }
}

/// # Description
///
/// Create a randomly generated string.
///
/// # Arguments
///
/// `size` - The amount of characters to include in the randomly generated string.
///
/// # Returns
///
/// The string that was randomly generated.
pub(crate) fn generate_random_string(size: usize) -> String {
    let mut rng = thread_rng();

    return (&mut rng)
        .sample_iter(Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
}

/// # Description
///
/// Create a validation error.
///
/// # Arguments
///
/// `code` - A short text code that identifies the category of error (for example "unique").
///
/// `value` - The value that is invalid.
///
/// # Returns
///
/// The validation error that was created.
pub(crate) fn create_value_validation_error<T: Serialize>(
    code: &'static str,
    value: &T,
) -> ValidationError {
    // Create the error.
    let mut error = ValidationError::new(code);
    error.add_param(Cow::from("value"), value);

    // Return the error.
    return error;
}

/// # Description
///
/// Get the user's authentication token from an http request's headers.
///
/// # Arguments
///
/// `request` - The http request.
///
/// # Returns
///
/// An option that will contain the token if it was found.
pub(crate) fn get_token_from_header(request: &HttpRequest) -> Option<String> {
    // Get the authorization header.
    let auth_header: &HeaderValue = match request.headers().get(AUTHORIZATION) {
        Some(auth_header) => auth_header,
        None => return None,
    };

    // Get the authorization header value.
    return match auth_header.to_str() {
        Ok(value) => Some(String::from(value)),
        Err(_) => None,
    };
}

/// # Description
///
/// Get the user's authentication token from an http request's cookies.
///
/// # Arguments
///
/// `request` - The http request.
///
/// # Returns
///
/// An option that will contain the token if it was found.
pub(crate) fn get_token_from_cookie(request: &HttpRequest) -> Option<String> {
    // Get the authorization cookie.
    return match request.cookie("authorization") {
        Some(cookie) => Some(String::from(cookie.value())),
        None => None,
    };
}

/// # Description
///
/// Map a borrowed vector of type I to an owned vector of type O.
///
/// # Arguments
///
/// `vec` - The vector to map.
///
/// `map_fn` - The map function to apply on each element in the vector.
///
/// # Returns
///
/// A vector containing the mapped elements.
pub(crate) fn map_to_owned<I, O>(vec: &Vec<I>, map_fn: fn(&I) -> O) -> Vec<O> {
    vec.into_iter().map(map_fn).collect::<Vec<O>>()
}

/// # Description
///
/// Create a cors configuration based on the server's configuration.
///
/// # Arguments
///
/// `config` - The server's configuration.
///
/// # Returns
///
/// The cors configuration.
pub(crate) fn create_cors_configuration(config: &Config) -> Cors {
    // Create a default cors configuration.
    let mut cors: Cors = Cors::default();

    // Add the allowed request origins.
    for origin in (&config.http.origins).into_iter() {
        cors = cors.allowed_origin(origin);
    }

    let cors: Cors = cors.allowed_methods(map_to_owned::<String, Method>(
        &config.http.methods,
        |method| {
            Method::from_bytes(method.as_bytes())
                .expect("Invalid request methods configured in the config")
        },
    ));

    // Allow headers.
    let cors: Cors = cors.allow_any_header();
    let cors: Cors = cors.expose_any_header();

    // Allow credentials.
    let cors: Cors = cors.supports_credentials();

    // Return the cors configuration.
    return cors;
}
