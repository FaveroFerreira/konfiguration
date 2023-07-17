use regex::Regex;
use serde::de::DeserializeOwned;
use std::path::Path;

use error::Error;
use error::KonfigurationResult;

mod error;
mod json;
mod yaml;

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
            Some("json") => crate::json::parse(self.file_path),
            Some("yml") | Some("yaml") => crate::yaml::parse(self.file_path),
            Some(format) => Err(Error::UnsupportedFormat {
                format: format.to_string(),
            }),
            None => Err(Error::UnsupportedFormat {
                format: self.file_path,
            }),
        }
    }
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(serde::Deserialize, Debug)]
    pub(crate) struct DatabaseConfig {
        pub database_url: String,
        pub database_username: String,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct WithEnvAndDefault {
        pub with_default: String,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct DefaultOnly {
        pub comes_from_env: String,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct DbConfig {
        pub url: String,
        pub user: String,
        pub password: String,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct AppConfig {
        pub server_port: f64,
        pub database: DbConfig,
    }
}
