use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error: {status_code} - {message}")]
    Api { status_code: u16, message: String },

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Signature verification error: {0}")]
    SignatureVerification(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Other error: {0}")]
    Other(String),
}
