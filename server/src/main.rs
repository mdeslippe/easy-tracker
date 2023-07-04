pub(crate) mod common;
pub(crate) mod config;
pub(crate) mod database;
pub(crate) mod feature;
pub(crate) mod injector;

use crate::{common::utility::get_config_path, config::Config, injector::DependencyInjector};
use actix_cors::Cors;
use actix_web::{http::Method, middleware::Logger, web::Data, App, HttpServer};
use common::utility::map_to_owned;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::sync::Arc;

/// # Description
///
/// The entry point of the application.
///
/// # Arguments
///
/// If a path is passed in while executing the application, the server will attempt to load the
/// configuration from that path.
/// If no path is passed in while executing the application, the server will attempt to load the
/// configuration from the present working directory.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the server's configuration.
    let config: Config = Config::load_config(get_config_path())
        .expect("Failed to load the server's configuration file");

    // Initialize the logger.
    env_logger::init_from_env(env_logger::Env::default().filter_or(
        env_logger::DEFAULT_FILTER_ENV,
        &config.log.minimum_log_level,
    ));

    // Log that the server is initializing.
    log::info!("Initializing the server");
    log::debug!("Using the following server configuration:\n {:#?}", config);

    // Create an ssl acceptor builder.
    let mut ssl_builder: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Failed to create a SSL/TLS socket");

    // Load the certificate.
    ssl_builder
        .set_certificate_chain_file(&config.http.certificate_path)
        .expect("Failed to load the SSL/TLS certificate");

    // Load the certificate's private key.
    ssl_builder
        .set_private_key_file(&config.http.certificate_key_path, SslFiletype::PEM)
        .expect("Failed to load the SSL/TLS certificate key");

    // Put the configuration in a reference counted Data struct, so we can use it as application data.
    let config_data: Data<Config> = Data::new(config.clone());

    // Create the dependency injector.
    let dependency_injector: Arc<DependencyInjector> = Arc::new(
        DependencyInjector::create_from_config(&config)
            .await
            .expect("Failed to create the dependency injector"),
    );

    // Define an actix application factory closure.
    let app_factory = move || {
        App::new()
            .app_data(config_data.clone())
            .app_data(Arc::clone(&dependency_injector))
            .wrap(create_cors_configuration(&config_data.clone()))
            .wrap(Logger::default())
            .configure(crate::feature::user::controller::configure)
            .configure(crate::feature::auth::controller::configure)
            .configure(crate::feature::file::controller::configure)
    };

    // Log the address the server will be bound to.
    log::info!(
        "Starting the server on {}:{}",
        &config.http.host,
        &config.http.port
    );

    // Build and run the server.
    return HttpServer::new(app_factory)
        .bind_openssl(
            ((&config.http.host).as_str(), config.http.port),
            ssl_builder,
        )
        .expect("Failed to bind the server to the host and port configured")
        .run()
        .await;
}

/// # Description
///
/// Create a cors configuration based on the server's configuration.
///
/// # Arguments
///
/// `config` - The server's configuration.
///
/// # Panics
///
/// This function will panic if the configuration contains an invalid cors configuration.
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

    // Add the allowed methods.
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
