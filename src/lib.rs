use std::fs;

use crate::error::{KonfigurationError, KonfigurationResult};
use crate::value::ConfigurationManifest;

mod de;
pub mod error;
mod value;

/// A configuration loader.
///
/// # Examples
///
/// ```rust
/// use konfiguration::Konfiguration;
///
/// #[derive(Debug, serde::Deserialize)]
/// struct Config {
///     profile: String,
///     postgres: Postgres
/// }
///
/// #[derive(Debug, serde::Deserialize)]
/// struct Postgres {
///     host: String,
///     port: u16,   
/// }
///
/// let config = Konfiguration::from_file("test_files/config.toml")
///     .parse::<Config>()
///     .unwrap();
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

    pub fn parse<T: serde::de::DeserializeOwned>(self) -> KonfigurationResult<T> {
        let text = fs::read_to_string(self.file_path)?;
        let manifest = toml::from_str::<ConfigurationManifest>(&text)?;
        // let simple_toml = simplify(manifest)?;

        println!("{:?}\n", manifest);
        println!("{manifest:?}");

        Err(KonfigurationError::Entry("".to_string()))
    }
}
