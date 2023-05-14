use std::error::Error;

use shaku::module;

use crate::config::Config;

// Create the dependency injector module.
module! {
    pub(crate) DependencyInjector {
        components = [],
        providers = []
    }
}

// An implementation for the DependencyInjector struct.
impl DependencyInjector {
    /// # Description
    ///
    /// Create a dependency injector from the server's configuration.
    ///
    /// # Arguments
    ///
    /// `config` - The configuration that will be used to create the dependency injector.
    ///
    /// # Returns
    ///
    /// This method returns a result:
    /// - The Ok variant will be returned with the injector, if it was successfully created.
    /// - The Err variant will be returned with an error, if an error occurs while attempting to
    /// create the injector.
    pub(crate) async fn create_from_config(_config: &Config) -> Result<Self, Box<dyn Error>> {
        // Create the injector.
        let injector: DependencyInjector = DependencyInjector::builder().build();

        // Return the injector.
        return Ok(injector);
    }
}
