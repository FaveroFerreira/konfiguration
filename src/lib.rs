use std::fs::File;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::{Error, KonfigurationResult};

mod env_vars;
mod error;
mod json;
mod utils;
mod yaml;

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
        let path = Path::new(&self.file_path);

        let file_extension =
            path.extension()
                .and_then(|ext| ext.to_str())
                .ok_or(Error::FileNotFound {
                    path: self.file_path.clone(),
                })?;

        let file = File::open(path)?;

        match file_extension {
            "json" => json::parse(file),
            "yml" | "yaml" => yaml::parse(file),
            format => Err(Error::UnsupportedFormat {
                format: format.to_string(),
            }),
        }
    }
}
