use std::fs;

use serde::Deserialize;
use toml::de::ValueDeserializer;

use crate::error::KonfigurationResult;
use crate::value::{ConfigurationEntry, ConfigurationManifest, TomlMap, TomlValue};

mod de;
pub mod error;
mod value;

/// A configuration loader.
///
/// # Examples
///
/// ```no_run
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
    /// Creates a new configuration loader with the given file path.
    pub fn from_file(path: impl Into<String>) -> Self {
        Konfiguration {
            file_path: path.into(),
        }
    }

    /// Parses the configuration file into the given type.
    pub fn parse<T: serde::de::DeserializeOwned>(self) -> KonfigurationResult<T> {
        let text = fs::read_to_string(self.file_path)?;
        let manifest = toml::from_str::<ConfigurationManifest>(&text)?;
        let simple_toml = simplify(manifest)?;

        Ok(T::deserialize(simple_toml)?)
    }
}

/// Takes a configuration manifest and simplifies it into a TOML value.
fn simplify(manifest: ConfigurationManifest) -> KonfigurationResult<TomlMap> {
    let mut map = TomlMap::new();

    for (key, config_entry) in manifest {
        let value = match config_entry {
            ConfigurationEntry::Simple(value) => value,
            ConfigurationEntry::Env(env_value) => {
                env_sanity_check(&env_value);
                expand_with_retry(env_value)?
            }
            ConfigurationEntry::UnsetEnv => continue,
            ConfigurationEntry::Table(table) => {
                let simplified = simplify(table)?;

                TomlValue::Table(simplified)
            }
        };

        map.insert(key, value);
    }

    Ok(map)
}

/// Not much to do here at this point, but we might want to add more checks in the future.
fn env_sanity_check(env: &str) {
    if env.is_empty() {
        panic!("env cannot be empty");
    }
}

/// Expands an env var value into into a TOML value.
///
/// This is ugly because toml sometimes fails to deserialize a simple string
/// I will be looking into this later.
fn expand_with_retry(value: String) -> KonfigurationResult<TomlValue> {
    match TomlValue::deserialize(ValueDeserializer::new(&value)) {
        Ok(v) => Ok(v),
        Err(_) => Ok(TomlValue::String(value)),
    }
}
