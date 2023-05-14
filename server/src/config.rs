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
}

/// The Config implementation.
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
