use crate::{
    config::Config, database::DatabaseConnectionFactoryImpl,
    feature::user::repository::UserRepositoryImpl,
};
use shaku::module;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::error::Error;

// Create the dependency injector module.
module! {
    pub(crate) DependencyInjector {
        components = [
            // Database
            DatabaseConnectionFactoryImpl,

            // User
            UserRepositoryImpl
        ],
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
    pub(crate) async fn create_from_config(config: &Config) -> Result<Self, Box<dyn Error>> {
        // Create the database connection pool.
        let database_connection_pool: Pool<MySql> = MySqlPoolOptions::new()
            .min_connections(config.database.minimum_connections)
            .max_connections(config.database.maximum_connections)
            .connect(&config.database.connection_string)
            .await?;

        // Create the injector.
        let injector: DependencyInjector = DependencyInjector::builder()
            .with_component_parameters::<DatabaseConnectionFactoryImpl>(
                DatabaseConnectionFactoryImpl::create_parameters(database_connection_pool),
            )
            .build();

        // Return the injector.
        return Ok(injector);
    }
}
