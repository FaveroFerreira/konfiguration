use std::fs;

use serde::Deserialize;
use toml::de::ValueDeserializer;

use crate::error::{KonfigurationError, KonfigurationResult};
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

impl TryFrom<ConfigurationEntry> for Option<TomlValue> {
    type Error = KonfigurationError;

    fn try_from(value: ConfigurationEntry) -> Result<Self, Self::Error> {
        match value {
            ConfigurationEntry::Simple(toml) => Ok(Some(toml)),
            ConfigurationEntry::Env(env) => {
                let toml = expand_with_retry(env)?;
                Ok(Some(toml))
            }
            ConfigurationEntry::UnsetEnv => Ok(None),
            ConfigurationEntry::Vec(vec) => {
                let mut entries = Vec::new();

                for entry in vec {
                    match Self::try_from(entry)? {
                        Some(entry) => entries.push(entry),
                        None => continue,
                    }
                }

                Ok(Some(TomlValue::Array(entries)))
            }
            ConfigurationEntry::Table(table) => {
                let mut map = TomlMap::new();

                for (key, entry) in table {
                    match Self::try_from(entry)? {
                        Some(entry) => {
                            map.insert(key, entry);
                        }
                        None => continue,
                    }
                }

                Ok(Some(TomlValue::Table(map)))
            }
        }
    }
}

/// Takes a configuration manifest and simplifies it into a TOML value.
fn simplify(manifest: ConfigurationManifest) -> KonfigurationResult<TomlMap> {
    let mut map = TomlMap::new();

    for (key, entry) in manifest {
        match Option::<TomlValue>::try_from(entry)? {
            Some(entry) => {
                map.insert(key, entry);
            }
            None => continue,
        }
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
    env_sanity_check(&value);

    match TomlValue::deserialize(ValueDeserializer::new(&value)) {
        Ok(v) => Ok(v),
        Err(_) => Ok(TomlValue::String(value)),
    }
}
