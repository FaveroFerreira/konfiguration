use std::collections::HashMap;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::{de, Deserialize, Serialize};
use serde_untagged::UntaggedEnumVisitor;
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationManifest(HashMap<String, ConfigurationEntry>);

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ConfigurationEntry {
    Simple(Value),
    Detailed(DetailedConfigurationEntry),
    Table(HashMap<String, ConfigurationEntry>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DetailedConfigurationEntry {
    env: Option<String>,
    default: Value,
}

impl<'de> de::Deserialize<'de> for ConfigurationEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        UntaggedEnumVisitor::new()
            .expecting(
                "a configuration like \"0.9.8\" or a \
                     detailed configuration like { env = \"REPLACE_WITH_THIS_ENV\", version = \"0.9.8\" }",
            )
            .i8(|value| {
                Ok(ConfigurationEntry::Simple(Value::Integer(
                    value.into(),
                )))
            })
            .i16(|value| {
                Ok(ConfigurationEntry::Simple(Value::Integer(
                    value.into(),
                )))
            })
            .i32(|value| {
                Ok(ConfigurationEntry::Simple(Value::Integer(
                    value.into(),
                )))
            })
            .i64(|value| {
                Ok(ConfigurationEntry::Simple(Value::Integer(
                    value.into(),
                )))
            })
            .u8(|value| {
                Ok(ConfigurationEntry::Simple(Value::Integer(
                    value.into(),
                )))
            })
            .u16(|value| {
                Ok(ConfigurationEntry::Simple(Value::Integer(
                    value.into(),
                )))
            })
            .f32(|value| Ok(ConfigurationEntry::Simple(Value::Float(value.into()))))
            .f64(|value| Ok(ConfigurationEntry::Simple(Value::Float(value.into()))))
            .char(|value| {
                Ok(ConfigurationEntry::Simple(Value::String(
                    value.to_string(),
                )))
            })
            .string(|value| {
                Ok(ConfigurationEntry::Simple(Value::String(
                    value.to_string(),
                )))
            })
            .seq(|seq| {
                let array: Value = seq.deserialize().unwrap();



                Ok(ConfigurationEntry::Simple(Value::Array(array.as_array().unwrap().to_owned())))
            })
            .map(|value| {
                let map: Value = value.deserialize()?;
                let mut map = map.as_table().unwrap().clone();

                if map.contains_key("env") || map.contains_key("default") {
                    Ok(ConfigurationEntry::Detailed(DetailedConfigurationEntry {
                        env: map
                            .remove("env")
                            .and_then(|v| v.as_str().map(|s| s.to_string())),
                        default: map.remove("default").unwrap(),
                    }))
                } else {
                    let str = toml::to_string(&map).unwrap();

                    Ok(ConfigurationEntry::Table(toml::from_str(&str).unwrap()))
                }
            })
            .deserialize(deserializer)
    }
}

pub struct Konfiguration {
    file_path: String,
}

impl Konfiguration {
    pub fn from_file(path: impl Into<String>) -> Self {
        Konfiguration {
            file_path: path.into(),
        }
    }

    pub fn parse<T: DeserializeOwned>(self) -> anyhow::Result<T> {
        let path = Path::new(&self.file_path);

        let string = std::fs::read_to_string(path)?;

        let manifest: ConfigurationManifest = toml::from_str(&string)?;

        let mut config = toml::map::Map::new();

        expand_env_vars(&mut config, manifest);

        let t = T::deserialize(config).unwrap();

        Ok(t)
    }
}

fn expand_env_vars(configs: &mut toml::map::Map<String, Value>, manifest: ConfigurationManifest) {
    for (name, entry) in manifest.0 {
        match entry {
            ConfigurationEntry::Simple(v) => {
                configs.insert(name, v);
            }
            ConfigurationEntry::Detailed(DetailedConfigurationEntry { env, default }) => {
                if let Some(env) = env {
                    let env_val = std::env::var(env).ok();

                    match env_val {
                        None => configs.insert(name, default),
                        Some(var) => match default {
                            Value::String(_) => configs.insert(name, Value::String(var)),
                            Value::Integer(_) => {
                                configs.insert(name, Value::Integer(var.parse().unwrap()))
                            }
                            Value::Float(_) => {
                                configs.insert(name, Value::Float(var.parse().unwrap()))
                            }
                            Value::Boolean(_) => {
                                configs.insert(name, Value::Boolean(var.parse().unwrap()))
                            }
                            Value::Datetime(_) => {
                                configs.insert(name, Value::Datetime(var.parse().unwrap()))
                            }
                            Value::Array(_) => {
                                let values = var.split(',').collect::<Vec<_>>();

                                let mut arr = Vec::new();

                                for val in values {
                                    arr.push(toml::from_str(val).unwrap());
                                }

                                configs.insert(name, Value::Array(arr))
                            }
                            Value::Table(_) => unreachable!(
                                "this should be handled by the table stuff, how did it get here?"
                            ),
                        },
                    };
                } else {
                    configs.insert(name, default);
                }
            }
            ConfigurationEntry::Table(t) => {
                let mut nested = toml::map::Map::new();

                expand_env_vars(&mut nested, ConfigurationManifest(t));

                configs.insert(name, Value::Table(nested));
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub profile: String,
        pub rust_log: String,
        pub cors_origin: String,
        pub server_port: u16,
        pub mail: MailConfig,
        pub google: GoogleConfig,
        pub security: SecurityConfig,
        pub postgres: PostgresConfig,
        pub redis: RedisConfig,
        pub meilisearch: MeilisearchConfig,
    }

    #[derive(Debug, Deserialize)]
    pub struct MeilisearchConfig {
        pub url: String,
        pub key: String,
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
        pub min_connections: u32,
        pub max_connections: u32,
        pub connection_acquire_timeout_secs: u64,
        pub enable_migration: bool,
        pub migrations_dir: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct SecurityConfig {
        pub password_salt: String,
        pub jwt_secret: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct GoogleConfig {
        pub audience: String,
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
    fn it_works() {
        std::env::set_var("PROFILE", "prod");

        let t = Konfiguration::from_file("test_files/configs.toml").parse::<Config>();

        println!("{t:?}");
    }
}
