pub type KonfigurationResult<T> = Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration file at {path}")]
    FileNotFound { path: String },

    #[error("Error trying to read data from file: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("Unsupported configuration file format: {format}")]
    UnsupportedFormat { format: String },

    #[error("Environment value {env} is not present and no default value was provided")]
    DefaultMissing { env: String },

    #[error("Error parsing configuration file: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Error parsing configuration file: {0}")]
    YamlParse(#[from] serde_yaml::Error),
}
