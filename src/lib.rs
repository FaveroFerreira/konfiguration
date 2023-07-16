use std::error::Error as StdError;
use std::fmt::Display;

use regex::Regex;
use serde::de::DeserializeOwned;
use serde_json::Value;

thread_local! {
    static REGEX: Regex = Regex::new(r#"\$\{([^:]+)(?::([^}]+))?\}"#).unwrap();
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
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

    pub fn parse<T: DeserializeOwned>(self) -> Result<T, Box<dyn StdError>> {
        let file = std::fs::File::open(self.file_path)?;
        let mut json_config: Value = serde_json::from_reader(file)?;

        expand_env_vars(&mut json_config)?;

        let config = serde_json::from_value(json_config)?;

        Ok(config)
    }
}

fn is_numeric(str: &str) -> bool {
    str.parse::<i64>().is_ok() || str.parse::<f64>().is_ok()
}

fn expand_env_vars(config: &mut Value) -> Result<(), Box<dyn StdError>> {
    match config {
        Value::String(value) => {
            if let Some(captures) = REGEX.with(|re| re.captures(value)) {
                let env = captures.get(1).unwrap().as_str();
                let default = captures.get(2).map(|m| m.as_str());

                let val = match std::env::var(env) {
                    Ok(val) => val,
                    Err(_) => default
                        .ok_or(Error {
                            message: format!(
                                "Environment variable {} not set with no default value",
                                env
                            ),
                        })?
                        .to_string(),
                };

                if is_numeric(&val) {
                    *config = Value::Number(val.parse::<serde_json::Number>()?);
                } else {
                    *config = Value::String(val);
                }
            }
        }
        Value::Object(o) => {
            for (_, value) in o {
                expand_env_vars(value)?;
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
    fn should_load_default_value_if_env_var_is_missing() {
        std::env::remove_var("DATABASE_PASSWORD");

        let config: DatabaseConfig = Konfiguration::from_file("test-files/config.json")
            .parse()
            .unwrap();

        assert_eq!(config.database_url, "postgres://localhost:5432/db");
        assert_eq!(config.database_username, "postgres");
    }

    #[derive(Debug, Deserialize)]
    pub struct WithEnvAndDefault {
        pub with_default: String,
    }

    #[test]
    fn should_prioritize_env_var_if_its_set() {
        std::env::set_var("NOT_MISSING", "from env");

        let config: WithEnvAndDefault =
            Konfiguration::from_file("test-files/config-with-env-vars.json")
                .parse()
                .unwrap();

        assert_eq!(config.with_default, "from env");
    }

    #[derive(Debug, Deserialize)]
    pub struct DefaultOnly {
        pub comes_from_env: String,
    }

    #[test]
    fn should_use_default_if_env_var_is_not_set() {
        let config: DefaultOnly = Konfiguration::from_file("test-files/use_default.json")
            .parse()
            .unwrap();

        assert_eq!(config.comes_from_env, "-default");
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
    fn can_handle_nested_objects() {
        std::env::set_var("SERVER_PORT", "8080");
        std::env::set_var("DB_URL", "postgres://localhost:5432/db");
        std::env::set_var("DB_USER", "postgres");
        std::env::set_var("DB_PASSWORD", "postgres");

        let config: AppConfig = Konfiguration::from_file("test-files/nested-vars.json")
            .parse()
            .unwrap();

        assert_eq!(config.database.url, "postgres://localhost:5432/db");
        assert_eq!(config.database.user, "postgres");
        assert_eq!(config.database.password, "postgres");
        assert_eq!(config.server_port, 8080.0);
    }
}
