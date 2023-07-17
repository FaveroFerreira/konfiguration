use std::fs::File;
use std::path::Path;

use regex::Regex;
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

use error::Error;
use error::KonfigurationResult;

mod error;

thread_local! {
    static REGEX: Regex = Regex::new(r#"\$\{([^:]+)(?::([^}]+))?\}"#).unwrap();
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

    pub fn parse<T: DeserializeOwned>(self) -> KonfigurationResult<T> {
        match Path::new(&self.file_path)
            .extension()
            .and_then(|x| x.to_str())
        {
            Some("json") => parse_json(self.file_path),
            Some("yml") | Some("yaml") => parse_yaml(self.file_path),
            Some(format) => Err(Error::UnsupportedFormat {
                format: format.to_string(),
            }),
            None => Err(Error::UnsupportedFormat {
                format: self.file_path,
            }),
        }
    }
}

fn parse_json<T: DeserializeOwned>(file_path: String) -> KonfigurationResult<T> {
    let file = File::open(file_path).map_err(|err| Error::FileNotFound { source: err })?;

    let mut config: JsonValue = serde_json::from_reader(file)?;

    json_expand_env_vars(&mut config)?;

    let config = serde_json::from_value(config)?;

    Ok(config)
}

fn parse_yaml<T: DeserializeOwned>(file_path: String) -> KonfigurationResult<T> {
    let file = File::open(file_path).map_err(|err| Error::FileNotFound { source: err })?;

    let mut config: YamlValue = serde_yaml::from_reader(file)?;

    yaml_expand_env_vars(&mut config)?;

    let config = serde_yaml::from_value(config)?;

    Ok(config)
}

fn is_numeric(str: &str) -> bool {
    str.parse::<i64>().is_ok() || str.parse::<f64>().is_ok()
}

fn expand_env_vars(value: &mut str) -> KonfigurationResult<Option<String>> {
    match REGEX.with(|re| re.captures(value)) {
        Some(captures) => {
            let env = captures.get(1).unwrap().as_str();
            let default = captures.get(2).map(|m| m.as_str());

            let val = match std::env::var(env) {
                Ok(val) => val,
                Err(_) => default
                    .ok_or(Error::DefaultMissing {
                        env: env.to_string(),
                    })?
                    .to_string(),
            };

            Ok(Some(val))
        }
        _ => Ok(None),
    }
}

fn json_expand_env_vars(config: &mut JsonValue) -> KonfigurationResult<()> {
    match config {
        JsonValue::String(value) => {
            if let Ok(Some(val)) = expand_env_vars(value) {
                if is_numeric(&val) {
                    *config = JsonValue::Number(val.parse::<serde_json::Number>()?);
                } else {
                    *config = JsonValue::String(val);
                }
            }
        }
        JsonValue::Object(o) => {
            for (_, value) in o {
                json_expand_env_vars(value)?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn yaml_expand_env_vars(config: &mut YamlValue) -> KonfigurationResult<()> {
    match config {
        YamlValue::String(value) => {
            if let Ok(Some(val)) = expand_env_vars(value) {
                if let Ok(float) = val.parse::<f64>() {
                    *config = YamlValue::Number(serde_yaml::Number::from(float));
                } else if let Ok(int) = val.parse::<i64>() {
                    *config = YamlValue::Number(serde_yaml::Number::from(int));
                } else {
                    *config = YamlValue::String(val);
                }
            }
        }
        YamlValue::Mapping(map) => {
            for value in map.values_mut() {
                yaml_expand_env_vars(value)?;
            }
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::Konfiguration;

    #[derive(serde::Deserialize, Debug)]
    pub struct DatabaseConfig {
        pub database_url: String,
        pub database_username: String,
    }

    #[test]
    fn json_should_load_default_value_if_env_var_is_missing() {
        std::env::remove_var("DATABASE_PASSWORD");

        let json_config: DatabaseConfig = Konfiguration::from_file("test_files/json/config.json")
            .parse()
            .unwrap();

        assert_eq!(json_config.database_url, "postgres://localhost:5432/db");
        assert_eq!(json_config.database_username, "postgres");
    }

    #[test]
    fn yaml_should_load_default_value_if_env_var_is_missing() {
        let yaml_config: DatabaseConfig = Konfiguration::from_file("test_files/yaml/config.yaml")
            .parse()
            .unwrap();

        assert_eq!(yaml_config.database_url, "postgres://localhost:5432/db");
        assert_eq!(yaml_config.database_username, "postgres");
    }

    #[derive(Debug, Deserialize)]
    pub struct WithEnvAndDefault {
        pub with_default: String,
    }

    #[test]
    fn json_should_prioritize_env_var_if_its_set() {
        std::env::set_var("NOT_MISSING", "from env");

        let json_config: WithEnvAndDefault =
            Konfiguration::from_file("test_files/json/config-with-env-vars.json")
                .parse()
                .unwrap();

        assert_eq!(json_config.with_default, "from env");
    }

    #[test]
    fn yaml_should_prioritize_env_var_if_its_set() {
        let yaml_config: WithEnvAndDefault =
            Konfiguration::from_file("test_files/yaml/config-with-env-vars.yaml")
                .parse()
                .unwrap();

        assert_eq!(yaml_config.with_default, "from env");
    }

    #[derive(Debug, Deserialize)]
    pub struct DefaultOnly {
        pub comes_from_env: String,
    }

    #[test]
    fn json_should_use_default_if_env_var_is_not_set() {
        let json_config: DefaultOnly = Konfiguration::from_file("test_files/json/use_default.json")
            .parse()
            .unwrap();

        assert_eq!(json_config.comes_from_env, "-default");
    }

    #[test]
    fn yaml_should_use_default_if_env_var_is_not_set() {
        let yaml_config: DefaultOnly = Konfiguration::from_file("test_files/yaml/use_default.yaml")
            .parse()
            .unwrap();

        assert_eq!(yaml_config.comes_from_env, "-default");
    }

    #[derive(Debug, Deserialize)]
    pub struct DbConfig {
        url: String,
        user: String,
        password: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct AppConfig {
        server_port: f64,
        database: DbConfig,
    }

    #[test]
    fn json_can_handle_nested_objects() {
        std::env::set_var("SERVER_PORT", "8080");
        std::env::set_var("DB_URL", "postgres://localhost:5432/db");
        std::env::set_var("DB_USER", "postgres");
        std::env::set_var("DB_PASSWORD", "postgres");

        let json_config: AppConfig = Konfiguration::from_file("test_files/json/nested-vars.json")
            .parse()
            .unwrap();

        assert_eq!(json_config.database.url, "postgres://localhost:5432/db");
        assert_eq!(json_config.database.user, "postgres");
        assert_eq!(json_config.database.password, "postgres");
        assert_eq!(json_config.server_port, 8080.0);
    }

    #[test]
    fn yaml_can_handle_nested_objects() {
        let yaml_config: AppConfig = Konfiguration::from_file("test_files/yaml/nested-vars.yaml")
            .parse()
            .unwrap();

        assert_eq!(yaml_config.database.url, "postgres://localhost:5432/db");
        assert_eq!(yaml_config.database.user, "postgres");
        assert_eq!(yaml_config.database.password, "postgres");
        assert_eq!(yaml_config.server_port, 8080.0);
    }
}
