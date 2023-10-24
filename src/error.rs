use thiserror::Error;

pub type Result<T> = std::result::Result<T, TokenGenerationError>;

/// This error contains all predictable types of failures
/// that can occur during the token generation/signing and requesting.
#[derive(Debug, Error)]
pub enum TokenGenerationError {
    /// Invalid lifetime
    #[error("The provided lifetime '{0}' is out of range 30..3600.")]
    InvalidLifetime(i64),
    /// JsonWebToken library error
    #[error("JsonWebTokenError occurred. {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    /// Reqwest library error
    #[error("ReqwestError occurred. {0}")]
    ReqwestError(#[from] reqwest::Error),
    /// Serde library error
    #[error("SerdeError occurred. {0}")]
    SerdeError(#[from] serde_json::Error),
}
