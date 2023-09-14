use std::fs;

use serde::Deserialize;
use toml::de::ValueDeserializer;

use crate::error::{Error, KonfigurationResult};
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

    let Some(env) = env else {
        return Ok(default);
    };

    let Some(override_value) = std::env::var(env).ok() else {
        return Ok(default)
    };

    // Ugly stuff to make sure we can parse the value into the correct type.
    match default {
        TomlValue::String(_) => to_toml_string(&override_value),
        TomlValue::Integer(_) => to_toml_integer(&override_value),
        TomlValue::Float(_) => to_toml_float(&override_value),
        TomlValue::Boolean(_) => to_toml_boolean(&override_value),
        TomlValue::Datetime(_) => to_toml_date_time(&override_value),
        TomlValue::Array(_) => to_toml_array(&override_value),
        TomlValue::Table(_) => {
            unreachable!("this should be handled by the table stuff, how did it get here?");
        }
    }
}

/// Converts a string into a TOML array.
fn to_toml_array(value: &str) -> KonfigurationResult<TomlValue> {
    Ok(TomlValue::deserialize(ValueDeserializer::new(value))?)
}

/// Converts a string into a TOML boolean.
fn to_toml_boolean(value: &str) -> KonfigurationResult<TomlValue> {
    value
        .parse()
        .map(TomlValue::Boolean)
        .map_err(|e| Error::Entry(e.to_string()))
}

/// Converts a string into a TOML date time.
fn to_toml_date_time(value: &str) -> KonfigurationResult<TomlValue> {
    value
        .parse()
        .map(TomlValue::Datetime)
        .map_err(|e| Error::Entry(e.to_string()))
}

/// Converts a string into a TOML string.
fn to_toml_string(value: &str) -> KonfigurationResult<TomlValue> {
    Ok(TomlValue::String(value.to_string()))
}

/// Converts a string into a TOML integer.
fn to_toml_integer(value: &str) -> KonfigurationResult<TomlValue> {
    value
        .parse()
        .map(TomlValue::Integer)
        .map_err(|e| Error::Entry(e.to_string()))
}

/// Converts a string into a TOML float.
fn to_toml_float(value: &str) -> KonfigurationResult<TomlValue> {
    value
        .parse()
        .map(TomlValue::Float)
        .map_err(|e| Error::Entry(e.to_string()))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub profile: String,
        pub rust_log: String,
        pub cors_origin: String,
        pub server_port: u16,
        pub exponential_backoff: Vec<u16>,
        pub mail: MailConfig,
        pub postgres: PostgresConfig,
        pub redis: RedisConfig,
    }

    #[derive(Debug, Deserialize)]
    pub struct RedisConfig {
        pub url: String,
        pub max_connections: u32,
        pub min_connections: u32,
        pub connection_acquire_timeout_secs: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct PostgresConfig {
        pub host: String,
        pub username: String,
        pub password: String,
        pub database: String,
        pub port: u16,
        pub min_connections: u32,
        pub max_connections: u32,
        pub connection_acquire_timeout_secs: u64,
        pub enable_migration: bool,
        pub migrations_dir: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct MailConfig {
        pub from: String,
        pub templates_dir: String,
        pub smtp: SmtpConfig,
    }

    #[derive(Debug, Deserialize)]
    pub struct SmtpConfig {
        pub host: String,
        pub username: String,
        pub password: String,
    }

    #[test]
    fn can_parse_configs() {
        std::env::set_var("EXPONENTIAL_BACKOFF", "[3, 4, 5]");
        std::env::set_var("PROFILE", "prod");
        std::env::set_var("SMTP_PASSWORD", "password");
        std::env::set_var("DATABASE_PORT", "1111");

        let config = Konfiguration::from_file("test_files/config.toml")
            .parse::<Config>()
            .unwrap();

        assert_eq!(config.profile, "prod");
        assert_eq!(config.rust_log, "info");
        assert_eq!(config.cors_origin, "*");
        assert_eq!(config.server_port, 8080);
        assert_eq!(config.exponential_backoff, vec![3, 4, 5]);
        assert_eq!(config.mail.templates_dir, "templates/**/*.html");
        assert_eq!(config.postgres.port, 1111);
        assert_eq!(config.mail.smtp.password, "password");
    }
}
