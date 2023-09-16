use serde::de;
use serde::de::Visitor;
use serde_untagged::de::{Map, Seq};
use serde_untagged::UntaggedEnumVisitor;

use crate::value::{ConfigurationEntry, DetailedConfigurationEntry, TomlValue};

/// Custom Deserializer for ConfigurationEntry.
/// Enables deserialization of any type that derives Deserialize.
impl<'de> de::Deserializer<'de> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

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
            .expecting(r#"
                a simple configuration like "http://localhost:8080
                or a detailed configuration like { env = "REPLACE_WITH_THIS_ENV", version = "0.9.8" }"#)
            .i8(ConfigurationEntry::try_from)
            .i16(ConfigurationEntry::try_from)
            .i32(ConfigurationEntry::try_from)
            .i64(ConfigurationEntry::try_from)
            .u8(ConfigurationEntry::try_from)
            .u16(ConfigurationEntry::try_from)
            .u32(ConfigurationEntry::try_from)
            .f32(ConfigurationEntry::try_from)
            .f64(ConfigurationEntry::try_from)
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

impl TryFrom<i8> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Integer(value as i64))
    }
}

impl TryFrom<i16> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Integer(value as i64))
    }
}

impl TryFrom<i32> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Integer(value as i64))
    }
}

impl TryFrom<i64> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Integer(value as i64))
    }
}

impl TryFrom<u8> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Integer(value as i64))
    }
}

impl TryFrom<u16> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Integer(value as i64))
    }
}

impl TryFrom<u32> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Integer(value as i64))
    }
}

impl TryFrom<f32> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Float(value as f64))
    }
}

impl TryFrom<f64> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::Float(value))
    }
}

impl TryFrom<&str> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ConfigurationEntry::String(value.to_string()))
    }
}

impl TryFrom<Seq<'_, '_>> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: Seq) -> Result<Self, Self::Error> {
        let array: TomlValue = value.deserialize()?;

        let array = array
            .as_array()
            .ok_or(de::Error::custom("array expected"))?;

        let array = array
            .iter()
            .cloned()
            .map(|value| ConfigurationEntry::try_from(value))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| de::Error::custom(format!("{e:?}")))?;

        Ok(ConfigurationEntry::Array(array))
    }
}

impl TryFrom<Map<'_, '_>> for ConfigurationEntry {
    type Error = serde_untagged::de::Error;

    fn try_from(value: Map) -> Result<Self, Self::Error> {
        let toml_map: TomlValue = value.deserialize()?;

        if let Some(env) = toml_map.get("env") {
            let default = toml_map
                .get("default")
                .cloned()
                .map(|value| ConfigurationEntry::try_from(value))
                .transpose()
                .map_err(|_| de::Error::custom("failed to parse default value"))?;

            Ok(ConfigurationEntry::Detailed(Box::new(
                DetailedConfigurationEntry {
                    env: env
                        .as_str()
                        .ok_or_else(|| de::Error::custom("env must be a string"))?
                        .to_string(),
                    default: Box::new(default),
                },
            )))
        } else {
            Ok(ConfigurationEntry::try_from(toml_map)
                .map_err(|e| de::Error::custom(format!("{e:?}")))?)
        }
    }
}
