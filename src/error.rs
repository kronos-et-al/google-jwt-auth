use serde_json::Number;
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
    #[error("JsonWebTokenError occurred.")]
    JsonWebTokenError {
        #[from]
        source: jsonwebtoken::errors::Error,
    },
    /// Reqwest library error
    #[error("ReqwestError occurred.")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },
    /// Serde library error
    #[error("SerdeError occurred.")]
    SerdeError {
        #[from]
        source: serde_json::Error,
    },
}
