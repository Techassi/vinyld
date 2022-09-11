use dotenvy::dotenv;
use serde::Deserialize;
use toml;

use std::{env, fs};

use crate::config::error::ConfigError;

pub mod error;

#[derive(Deserialize)]
pub struct Config {
    pub store: StoreOptions,
}

#[derive(Deserialize)]
pub struct StoreOptions {
    pub username: String,
    pub password: String,
    pub database: String,
    pub host: String,
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
        let password = match env::var("STORE_PASSWORD") {
            Ok(password) => password,
            Err(_) => return Err(ConfigError::new("STORE_PASSWORD is required to be set")),
        };

        // ENV vars with fallback values
        let username = env::var("STORE_USERNAME").unwrap_or(String::from("vinyld"));
        let database = env::var("STORE_DATABASE").unwrap_or(String::from("vinyld"));
        let host = env::var("STORE_HOST").unwrap_or(String::from("127.0.0.1"));

        let config = Config {
            store: StoreOptions {
                username,
                password,
                database,
                host,
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
