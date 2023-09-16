use serde::de;
use serde_untagged::de::{Map, Seq};
use serde_untagged::UntaggedEnumVisitor;

use crate::value::{ConfigurationEntry, TomlValue};

/// Enables deserialization of a configuration entry from a Toml file.
///
/// We need a custom implementation because we want to support both simple and
/// detailed configuration entries. A simple configuration entry is just a
/// TOML value, while a detailed configuration entry is a TOML that has special keys
/// and must not be deserialized as a Table.
impl<'de> de::Deserialize<'de> for ConfigurationEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        UntaggedEnumVisitor::new()
            .expecting(
                "a configuration like \"http://localhost:8080\" or a \
                     detailed configuration like { env = \"REPLACE_WITH_THIS_ENV\", version = \"0.9.8\" }",
            )
            .i8(ConfigurationEntry::try_from)
            .i16(ConfigurationEntry::try_from)
            .i32(ConfigurationEntry::try_from)
            .i64(ConfigurationEntry::try_from)
            .u8(ConfigurationEntry::try_from)
            .u16(ConfigurationEntry::try_from)
            .u32(ConfigurationEntry::try_from)
            .f32(ConfigurationEntry::try_from)
            .f64(ConfigurationEntry::try_from)
            .bool(ConfigurationEntry::try_from)
            .string(|s| {
                ConfigurationEntry::try_from(s)
            })
            .seq(|seq| {
                ConfigurationEntry::try_from(seq)
            })
            .map(|map| {
                ConfigurationEntry::try_from(map)
            })
            .deserialize(deserializer)
    }
}

impl TryFrom<bool> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Boolean(value)))
    }
}

impl TryFrom<i8> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Integer(value as i64)))
    }
}

impl TryFrom<i16> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Integer(value as i64)))
    }
}

impl TryFrom<i32> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Integer(value as i64)))
    }
}

impl TryFrom<i64> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Integer(value)))
    }
}

impl TryFrom<u8> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Integer(value as i64)))
    }
}

impl TryFrom<u16> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Integer(value as i64)))
    }
}

impl TryFrom<u32> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Integer(value as i64)))
    }
}

impl TryFrom<f32> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Float(value as f64)))
    }
}

impl TryFrom<f64> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::Float(value)))
    }
}

impl TryFrom<&str> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Simple(TomlValue::String(
            value.to_string(),
        )))
    }
}

impl TryFrom<Seq<'_, '_>> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: Seq) -> Result<Self, Self::Error> {
        let array: TomlValue = value.deserialize()?;

        Ok(ConfigurationEntry::Simple(array))
    }
}

impl TryFrom<Map<'_, '_>> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: Map) -> Result<Self, Self::Error> {
        let toml_map: toml::map::Map<String, TomlValue> = value.deserialize()?;

        if let Some(env) = toml_map.get("env") {
            let env_name = env
                .as_str()
                .ok_or_else(|| de::Error::custom("env name must be a string"))?;

            let env_val = std::env::var(env_name).ok();
            let default = toml_map.get("default").cloned();

            match (env_val, default) {
                (Some(env_val), _) => Ok(ConfigurationEntry::Env(env_val)),
                (None, Some(default)) => Ok(ConfigurationEntry::Simple(default)),
                (None, None) => Ok(ConfigurationEntry::UnsetEnv),
            }
        } else {
            let str = toml::to_string(&toml_map).map_err(|e| de::Error::custom(e.to_string()))?;

            Ok(ConfigurationEntry::Table(
                toml::from_str(&str).map_err(|e| de::Error::custom(e.to_string()))?,
            ))
        }
    }
}
