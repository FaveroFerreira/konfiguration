use serde::de::{IntoDeserializer, Visitor};
use serde::{de, Deserialize};
use serde_untagged::de::{Map, Seq};
use serde_untagged::UntaggedEnumVisitor;

use crate::error::KonfigurationError;
use crate::value::{
    ConfigurationEntry, ConfigurationManifest, DetailedConfigurationEntry, TomlValue,
};

pub struct ManifestDeserializer {
    manifest: ConfigurationManifest,
}

impl ManifestDeserializer {
    pub fn new(manifest: ConfigurationManifest) -> Self {
        ManifestDeserializer { manifest }
    }
}

impl<'de> de::Deserializer<'de> for ManifestDeserializer {
    type Error = KonfigurationError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing any");
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing bool");
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing i8");
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing i16");
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing i32");
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing i64");
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing u8");
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing u16");
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing u32");
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing u64");
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing f32");
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing f64");
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing char");
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing str");
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing string");
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing bytes");
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing byte buf");
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing option");
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing unit");
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
        println!("deserializing unit struct");
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
        println!("deserializing newtype struct");
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing seq");
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing tuple");
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
        println!("deserializing tuple struct");
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing map");
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
        println!("deserializing struct");
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
        println!("deserializing enum");
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing identifier");
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        println!("deserializing ignored any");
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
            Ok(ConfigurationEntry::Detailed(DetailedConfigurationEntry {
                env: env
                    .as_str()
                    .ok_or_else(|| de::Error::custom("env must be a string"))?
                    .to_string(),
                default: toml_map.get("default").cloned(),
            }))
        } else {
            let str = toml::to_string(&toml_map).map_err(|e| de::Error::custom(e.to_string()))?;

            Ok(ConfigurationEntry::Table(
                toml::from_str(&str).map_err(|e| de::Error::custom(e.to_string()))?,
            ))
        }
    }
}
