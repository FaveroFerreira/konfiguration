use std::fs;

use serde::{Deserialize, Deserializer};
use toml::de::ValueDeserializer;

use crate::de::ManifestDeserializer;
use crate::error::{KonfigurationError, KonfigurationResult};
use crate::value::{
    ConfigurationEntry, ConfigurationManifest, DetailedConfigurationEntry, TomlMap, TomlValue,
};

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
        println!("simplifying");
        // let simple_toml = simplify(manifest)?;

        Ok(T::deserialize(ManifestDeserializer::new(manifest))?)
    }
}

/// Takes a configuration manifest and simplifies it into a TOML value.
fn simplify(manifest: ConfigurationManifest) -> KonfigurationResult<TomlMap> {
    let mut map = TomlMap::new();

    for (key, config_entry) in manifest {
        let value = match config_entry {
            ConfigurationEntry::Simple(value) => value,
            ConfigurationEntry::Detailed(detailed) => expand_env_var(detailed)?,
            ConfigurationEntry::Table(table) => {
                let simplified = simplify(table)?;

                TomlValue::Table(simplified)
            }
        };

        map.insert(key, value);
    }

    Ok(map)
}

/// Expands an `DetailedConfigurationEntry` into a TOML value.
///
/// If the `env` field is `None`, the `default` field is returned.
fn expand_env_var(entry: DetailedConfigurationEntry) -> KonfigurationResult<TomlValue> {
    let DetailedConfigurationEntry { env, default } = entry;

    let Some(override_value) = std::env::var(&env).ok() else {
        return match default {
            None => Err(KonfigurationError::Entry(format!("{} is not set", env))),
            Some(default) => Ok(default),
        }
    };

    println!("override_value: {}", override_value);

    match ConfigurationEntry::deserialize(ValueDeserializer::new(&override_value))? {
        ConfigurationEntry::Simple(inner) => Ok(inner),
        ConfigurationEntry::Detailed(_) => unreachable!(),
        ConfigurationEntry::Table(_) => unreachable!(),
    }
}
