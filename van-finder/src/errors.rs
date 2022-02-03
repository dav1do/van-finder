use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("{0}")]
    Custom(String),
    #[error("Lettre error: {0:?}")]
    Lettre(#[from] lettre::error::Error),
    #[error("Lettre SMTP error: {0:?}")]
    LettreSmtp(#[from] lettre::transport::smtp::Error),
    #[error("Reqwest: {0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO Error: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("Serialization Error: {0:?}")]
    Serde(#[from] serde_json::Error),
}
