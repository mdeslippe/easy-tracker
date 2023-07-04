use actix_web::web::{self, ServiceConfig};

/// # Description
///
/// Add the file controller configuration to a service config.
///
/// # Arguments
///
/// `config` - The service config that the file controller configuration will be added to.
pub(crate) fn configure(config: &mut ServiceConfig) {
    config.service(web::scope("/files"));
}
