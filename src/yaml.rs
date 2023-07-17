use std::fs::File;

use serde::de::DeserializeOwned;
use serde_yaml::Value as YamlValue;

use crate::env_vars;
use crate::error::KonfigurationResult;
use crate::utils::is_numeric;

pub fn parse<T: DeserializeOwned>(file: File) -> KonfigurationResult<T> {
    let mut yaml_config: YamlValue = serde_yaml::from_reader(file)?;

    parse_configuration(&mut yaml_config)?;

    let config = serde_yaml::from_value(yaml_config)?;

    Ok(config)
}

fn parse_configuration(config: &mut YamlValue) -> KonfigurationResult<()> {
    match config {
        YamlValue::String(value) => {
            if let Ok(Some(val)) = env_vars::expand_env_var(value) {
                if is_numeric(&val) {
                    *config = YamlValue::Number(val.parse::<serde_yaml::Number>()?);
                } else {
                    *config = YamlValue::String(val);
                }
            }
        }
        YamlValue::Mapping(map) => {
            for value in map.values_mut() {
                parse_configuration(value)?;
            }
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct AppConfig {
        pub server_port: u16,
        pub database: DatabaseConfig,
        pub api: ApiConfig,
    }

    #[derive(Debug, Deserialize)]
    pub struct DatabaseConfig {
        pub host: String,
        pub username: String,
        pub password: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct ApiConfig {
        pub url: String,
        pub token: String,
    }

    #[test]
    fn should_return_error_if_file_not_found() {
        let result = crate::Konfiguration::from_file("non-existent-file.yaml").parse::<()>();

        assert!(result.is_err());
    }

    #[test]
    fn should_parse_configuration_correctly() {
        let config = crate::Konfiguration::from_file("test_files/yaml/config.yaml")
            .parse::<AppConfig>()
            .unwrap();

        assert_eq!(config.server_port, 8080);
        assert_eq!(config.database.host, "postgres://localhost:5432/db");
        assert_eq!(config.database.username, "postgres");
        assert_eq!(config.database.password, "postgres");
        assert_eq!(config.api.url, "https://api.example.com");
        assert_eq!(config.api.token, "1234567890");
    }

    #[test]
    fn should_parse_config_with_env_vars_correctly() {
        std::env::set_var("SERVER_PORT", "8080");
        std::env::set_var("POSTGRES_PASSWORD", "postgres");
        std::env::set_var("API_TOKEN", "token");

        let config = crate::Konfiguration::from_file("test_files/yaml/env-vars-config.yaml")
            .parse::<AppConfig>()
            .unwrap();

        assert_eq!(config.server_port, 8080);
        assert_eq!(config.database.host, "postgres://localhost:5432/db");
        assert_eq!(config.database.username, "postgres");
        assert_eq!(config.database.password, "postgres");
        assert_eq!(config.api.url, "https://api.example.com");
        assert_eq!(config.api.token, "token")
    }

    #[test]
    fn should_use_defaults_if_env_var_is_missing() {
        let config =
            crate::Konfiguration::from_file("test_files/yaml/env-vars-default-config.yaml")
                .parse::<AppConfig>()
                .unwrap();

        assert_eq!(config.server_port, 8080);
        assert_eq!(config.database.host, "postgres://localhost:5432/db");
        assert_eq!(config.database.username, "postgres");
        assert_eq!(config.database.password, "postgres");
        assert_eq!(config.api.url, "https://api.example.com");
        assert_eq!(config.api.token, "token")
    }
}
