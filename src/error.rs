use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum KonfigurationError {
    #[error("failed to load configuration file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse configuration file: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("failed to parse configuration entry: {0}")]
    Entry(String),

    #[error("failed to deserialize configuration: {0}")]
    Deserialization(#[from] serde_untagged::de::Error),
}

impl serde::de::Error for KonfigurationError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        KonfigurationError::Entry(msg.to_string())
    }
}

pub type KonfigurationResult<T> = Result<T, KonfigurationError>;
