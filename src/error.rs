pub type KonfigurationResult<T> = Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration file: {source}")]
    FileNotFound { source: std::io::Error },

    #[error("Environment value {env} is not present and no default value was provided")]
    DefaultMissing { env: String },

    #[error("Error parsing configuration file: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Error parsing configuration file: {0}")]
    YamlParse(#[from] serde_yaml::Error),
}
