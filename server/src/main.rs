pub(crate) mod config;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use config::Config;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::env;

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

    // Define an actix application factory closure.
    let app_factory = move || {
        App::new()
            .app_data(config_data.clone())
            .wrap(Logger::default())
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
fn get_config_path() -> String {
    // Get arguments that were passed in when executing the application.
    let args: Vec<String> = env::args().collect();

    // If a configuration file path was specified return that, otherwise return the default path.
    if args.len() > 1 {
        // The first argument will always be the name of the file being executed, so we must skip it.
        return args.iter().skip(1).map(|s| s.chars()).flatten().collect();
    } else {
        return String::from("config.json");
    }
}
