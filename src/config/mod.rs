use dotenvy::dotenv;
use serde::Deserialize;
use toml;

use std::{env, fs};

use crate::config::error::ConfigError;

pub mod error;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerOptions,
    pub store: StoreOptions,
}

#[derive(Deserialize, Clone)]
pub struct StoreOptions {
    pub username: String,
    pub password: String,
    pub database: String,
    pub host: String,
}

#[derive(Deserialize, Clone)]
pub struct ServerOptions {
    pub address: String,
}

impl Config {
    /// Constructs a [`Config`] from ENV variables. This will return an error if some required ENV variables are not
    /// set. While in development this function will load ENV vars from the `.env` file.
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file
        #[cfg(debug_assertions)]
        match dotenv() {
            Ok(_) => {}
            Err(_) => return Err(ConfigError::new("Failed to load .env file")),
        }

        // Required ENV vars
        let store_password = match env::var("STORE_PASSWORD") {
            Ok(password) => password,
            Err(_) => return Err(ConfigError::new("STORE_PASSWORD is required to be set")),
        };

        // ENV vars with fallback values
        let store_username = env::var("STORE_USERNAME").unwrap_or(String::from("vinyld"));
        let store_database = env::var("STORE_DATABASE").unwrap_or(String::from("vinyld"));
        let store_host = env::var("STORE_HOST").unwrap_or(String::from("127.0.0.1"));

        let server_address = env::var("SERVER_ADDRESS").unwrap_or(String::from("127.0.0.1:8000"));

        let config = Config {
            store: StoreOptions {
                username: store_username,
                password: store_password,
                database: store_database,
                host: store_host,
            },
            server: ServerOptions {
                address: server_address,
            },
        };

        Ok(config)
    }

    /// Constructs a [`Config`] from a TOML file at `path`. This will return an error if the file does not exist and
    /// required keys are not set.
    pub fn from_file(path: String) -> Result<Self, ConfigError> {
        let file_contents = match fs::read_to_string(path) {
            Ok(b) => b,
            Err(err) => {
                return Err(ConfigError::new(format!(
                    "Failed to read config file: {}",
                    err
                )));
            }
        };

        let cfg: Self = match toml::from_str(&file_contents) {
            Ok(c) => c,
            Err(err) => {
                return Err(ConfigError::new(format!("Failed to parse TOML: {}", err)));
            }
        };

        Ok(cfg)
    }
}
