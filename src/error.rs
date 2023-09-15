#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to load configuration file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse configuration file: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("failed to parse configuration entry: {0}")]
    Entry(String),
}

pub type KonfigurationResult<T> = Result<T, Error>;
