use std::collections::HashMap;
use std::path::Path;

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

    pub fn parse(self) -> anyhow::Result<()> {
        let path = Path::new(&self.file_path);

        let string = std::fs::read_to_string(path)?;

        let manifest: ConfigurationManifest = toml::from_str(&string)?;

        let mut config = toml::map::Map::new();

        expand_env_vars(&mut config, manifest);

        println!("{config:#?}");

        Ok(())
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
                                continue;
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

    #[test]
    fn it_works() {
        let _ = Konfiguration::from_file("test_files/configs.toml").parse();
    }
}
