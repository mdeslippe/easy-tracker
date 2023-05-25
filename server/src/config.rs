use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

/// The configuration for the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    /// The server's logging configuration.
    pub(crate) log: LogConfig,

    /// The server's http configuration.
    pub(crate) http: HttpConfig,

    /// The server's database configuration.
    pub(crate) database: DatabaseConfig,

    /// The server's jwt configuration.
    pub(crate) jwt: JwtConfig,

    /// The server's default value configuration.
    pub(crate) default: DefaultValueConfig,
}

/// The logging configuration for the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LogConfig {
    /// The minimum log level that will be logged.
    pub(crate) minimum_log_level: String,
}

/// The http configuration for the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HttpConfig {
    /// The host the http server will listen for connections on.
    pub(crate) host: String,

    /// The port the http server will listen for connections on.
    pub(crate) port: u16,

    /// The path of the certificate the http server will use.
    pub(crate) certificate_path: String,

    /// The path of the certificate key the http server will use.
    pub(crate) certificate_key_path: String,

    /// The http methods that clients are permitted to use.
    pub(crate) methods: Vec<String>,

    /// The origins that are permitted to send http requests.
    pub(crate) origins: Vec<String>,
}

/// The database configuration for the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DatabaseConfig {
    /// The connection string that will be used to acquire database connections.
    pub(crate) connection_string: String,

    /// The minimum amount of database connections that must be maintained.
    pub(crate) minimum_connections: u32,

    /// The maximum amount of database connections that are allowed.
    pub(crate) maximum_connections: u32,
}

/// The jwt configuration for the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JwtConfig {
    /// The private key that will be used to create tokens.
    pub(crate) private_key_path: String,

    /// The public key that will be used to verify tokens.
    pub(crate) public_key_path: String,
}

/// The default value configuration for the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DefaultValueConfig {
    /// The default profile picture that will be assigned to users when their account is created.
    pub(crate) user_profile_picture: String,
}

/// An implementation for the Config struct.
impl Config {
    /// # Description
    ///
    /// Load the server configuration from the file at the file path specified.
    ///
    /// # Arguments
    ///
    /// `path` - The path of the configuration file that will be loaded.
    ///
    /// # Returns
    ///
    /// This function returns a result:
    /// - If loading and parsing the file was successful, the configuration will be returned.
    /// - If an error occurs while loading or parsing the file, the error will be returned.
    pub(crate) fn load_config(path: String) -> Result<Self, Box<dyn Error>> {
        // Load the configuration file into a string.
        let raw_json_config: String = fs::read_to_string(path)?;

        // Convert the string to a Config struct.
        let config: Config = serde_json::from_str(&raw_json_config)?;

        // Return the config.
        return Ok(config);
    }
}
