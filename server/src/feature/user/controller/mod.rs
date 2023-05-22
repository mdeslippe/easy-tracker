use actix_web::web::{self, ServiceConfig};

/// # Description
///
/// Add the user controller configuration to a service config.
///
/// # Arguments
///
/// `config` - The service config that the user controller configuration will be added
pub(crate) fn configure(config: &mut ServiceConfig) {
    config.service(web::scope("/users"));
}
