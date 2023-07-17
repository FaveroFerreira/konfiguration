use serde_json::Value;
use std::fs::File;

use crate::error::Error;
use crate::error::KonfigurationResult;
use serde::de::DeserializeOwned;

pub(crate) fn parse<T: DeserializeOwned>(file_path: String) -> KonfigurationResult<T> {
    let file = File::open(file_path).map_err(|err| Error::FileNotFound { source: err })?;

    let mut config: Value = serde_json::from_reader(file)?;

    expand_env_vars(&mut config)?;

    let config = serde_json::from_value(config)?;

    Ok(config)
}

fn expand_env_vars(config: &mut Value) -> KonfigurationResult<()> {
    match config {
        Value::String(value) => {
            if let Ok(Some(val)) = crate::expand_env_vars(value) {
                if crate::is_numeric(&val) {
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
mod json_tests {
    use crate::tests::*;
    use crate::Konfiguration;

    #[test]
    fn json_should_load_default_value_if_env_var_is_missing() {
        std::env::remove_var("DATABASE_PASSWORD");
        let json_config: DatabaseConfig = Konfiguration::from_file("test-files/config.json")
            .parse()
            .unwrap();

        assert_eq!(json_config.database_url, "postgres://localhost:5432/db");
        assert_eq!(json_config.database_username, "postgres");
    }

    #[test]
    fn json_should_prioritize_env_var_if_its_set() {
        std::env::set_var("NOT_MISSING", "from env");

        let json_config: WithEnvAndDefault =
            Konfiguration::from_file("test-files/config-with-env-vars.json")
                .parse()
                .unwrap();

        assert_eq!(json_config.with_default, "from env");
    }

    #[test]
    fn json_should_use_default_if_env_var_is_not_set() {
        let json_config: DefaultOnly = Konfiguration::from_file("test-files/use_default.json")
            .parse()
            .unwrap();

        assert_eq!(json_config.comes_from_env, "-default");
    }

    #[test]
    fn json_can_handle_nested_objects() {
        std::env::set_var("SERVER_PORT", "8080");
        std::env::set_var("DB_URL", "postgres://localhost:5432/db");
        std::env::set_var("DB_USER", "postgres");
        std::env::set_var("DB_PASSWORD", "postgres");

        let json_config: AppConfig = Konfiguration::from_file("test-files/nested-vars.json")
            .parse()
            .unwrap();

        assert_eq!(json_config.database.url, "postgres://localhost:5432/db");
        assert_eq!(json_config.database.user, "postgres");
        assert_eq!(json_config.database.password, "postgres");
        assert_eq!(json_config.server_port, 8080.0);
    }
}
