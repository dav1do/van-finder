use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("{0}")]
    Custom(String),
    #[error("Reqwest: {0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO Error: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("Serialization Error: {0:?}")]
    Serde(#[from] serde_json::Error),
}
