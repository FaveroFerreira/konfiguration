use std::fs::File;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::{Error, KonfigurationResult};

mod env_vars;
mod error;
mod json;
mod utils;
mod yaml;

/// Serialize and Deserialize configuration files.
///
/// # Example
///
/// In your config.json file:
/// ```json
/// {
///     "server_port": 8080,
///     "database": {
///         "host": "localhost",
///         "username": "root",
///         "password": "password"
///     }
/// }
/// ```
///
/// In your main.rs file:
/// ```rust
/// use konfiguration::Konfiguration;
///
/// #[derive(Debug, serde::Deserialize)]
/// pub struct DatabaseConfig {
///     pub host: String,
///     pub username: String,
///     pub password: String,
/// }
///
/// #[derive(Debug, serde::Deserialize)]
/// pub struct AppConfig {
///     pub server_port: u16,
///     pub database: DatabaseConfig,
/// }
///
/// let config = Konfiguration::from_file("test_files/json/config.json").parse::<AppConfig>().unwrap();
///
/// println!("{:#?}", config);
///
pub struct Konfiguration {
    file_path: String,
}

impl Konfiguration {
    pub fn from_file(path: impl Into<String>) -> Self {
        Konfiguration {
            file_path: path.into(),
        }
    }

    pub fn parse<T: DeserializeOwned>(self) -> KonfigurationResult<T> {
        let path = Path::new(&self.file_path);

        let file_extension =
            path.extension()
                .and_then(|ext| ext.to_str())
                .ok_or(Error::FileNotFound {
                    path: self.file_path.clone(),
                })?;

        let file = File::open(path)?;

        match file_extension {
            "json" => json::parse(file),
            "yml" | "yaml" => yaml::parse(file),
            format => Err(Error::UnsupportedFormat {
                format: format.to_string(),
            }),
        }
    }
}
