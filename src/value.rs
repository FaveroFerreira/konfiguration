use std::collections::HashMap;

pub type TomlValue = toml::Value;
pub type TomlMap = toml::map::Map<String, TomlValue>;

pub type ConfigurationManifest = HashMap<String, ConfigurationEntry>;

#[derive(Debug)]
pub enum ConfigurationEntry {
    Simple(TomlValue),
    Env(String),
    UnsetEnv,
    Table(HashMap<String, ConfigurationEntry>),
}

#[derive(Debug)]
pub struct DetailedConfigurationEntry {
    pub env_val: String,
    pub default: Option<TomlValue>,
}
