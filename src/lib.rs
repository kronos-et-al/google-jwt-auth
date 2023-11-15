#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::dbg_macro,
    clippy::todo
)]
#![allow(clippy::module_name_repetitions)]

//! Crate google-jwt-auth
//!
//! This crate provides the functionality to create authentication tokens for google requests.
//!
//! # Two easy steps to obtain a token:
//! 1. Create an [`AuthConfig`] instance with [`AuthConfig::build()`]
//! 2. Generate a token with [`AuthConfig::generate_auth_token()`]
//!
//! # Example
//! ```
//! use google_jwt_auth::AuthConfig;
//! use google_jwt_auth::usage::Usage;
//!
//! // Step one: Create AuthConfig
//! let client_json = std::fs::read_to_string("tests/test-client.json").unwrap();
//! let usage = Usage::CloudVision;
//!
//! let config = AuthConfig::build(&client_json, &usage).unwrap();
//!
//! // Step two: Generate token
//! let lifetime = 3600_i64;
//! let token_response = config.generate_auth_token(lifetime);
//! ```
//! After awaiting the `token_response` the result can be obtained.

use crate::error::{Result, TokenGenerationError};
use crate::json_structs::{Claims, GoogleResponse, ServiceAccountInfoJson};

use crate::usage::Usage;
use jsonwebtoken::{Algorithm, EncodingKey, Header};

/// This module contains all error types and meanings.
pub mod error;
pub(crate) mod json_structs;
/// This module contains all types of usages and their description.
pub mod usage;

/// This typing is used to have easy access to the library errors.
pub type Error = TokenGenerationError;

static GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";
static CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

/// This struct contains all necessary information to request an authentication token from google.
/// This structure is intended to be reused by the client for several token generation requests.
pub struct AuthConfig {
    header: Header,
    iss: String,
    scope: String,
    aud: String,
    private_key: String,
}

impl AuthConfig {
    /// This function generates an auth configuration with the provided information. A this config is used to request `auth_tokens`.
    /// This function generates only tokens with the RS256 encryption like the google jwt authentication service does.
    /// # Params
    /// **`service_account_json_str`: String**<br>
    /// Each google service account has a json file that can be downloaded in the google console during the key generation.
    /// This json file cannot be downloaded twice! A new key must be generated, if the file gets lost!
    /// The content of this file needs to be provided by this param as string.
    ///
    /// **`usage`: String**<br>
    /// Each google api request requires individual permissions to be executed.
    /// Beside the service account permission a usage or a scope should be provided.
    /// See here for more information: [Google Scopes](https://developers.google.com/identity/protocols/oauth2/scopes?hl=en).
    ///
    /// **`lifetime`: u16**<br>
    /// An `auth_token` has a limited lifetime to am maximum of 3600 seconds.
    /// This value should be between 30 and 3600 Seconds.
    /// Inputs out of this ranged will not be accepted.
    /// # Errors
    /// See [`ErrorKind`] for a more detailed answer.
    /// # Returns
    /// The above mentioned jwt as String.
    pub fn build(service_account_json_str: &str, usage: &Usage) -> Result<Self> {
        let account_info: ServiceAccountInfoJson = serde_json::from_str(service_account_json_str)?;
        Ok(Self {
            header: Header::new(Algorithm::RS256),
            iss: account_info.client_email,
            scope: usage.as_string(),
            aud: account_info.token_uri,
            private_key: account_info.private_key,
        })
    }

    /// With the provided jwt token, an authentication token (short: `auth_token`) will be requested from google.
    /// This `auth_token` will be returned and is used for requesting several google api services.
    /// # Errors
    /// See [`ErrorKind`] for a more detailed answer.
    /// # Returns
    /// The above mentioned `auth_token` as String.
    pub async fn generate_auth_token(&self, lifetime: i64) -> Result<String> {
        if !(30..=3600).contains(&lifetime) {
            return Err(Error::InvalidLifetime(lifetime));
        }

        // TODO add token buffer with lifetime check to minimize auth_token requests
        // <--

        let claims = Claims::new(
            self.iss.clone(),
            self.scope.clone(),
            self.aud.clone(),
            lifetime,
        );
        let assertion = self.sign(&claims)?;

        let params = format!("grant_type={GRANT_TYPE}&assertion={assertion}");
        let resp = reqwest::Client::new()
            .post(&self.aud)
            .header(reqwest::header::CONTENT_TYPE, CONTENT_TYPE)
            .body(params)
            .send()
            .await?
            .json::<GoogleResponse>()
            .await?;
        match resp {
            GoogleResponse::ValidResponse { access_token, .. } => Ok(access_token),
            GoogleResponse::ErrorResponse {
                error,
                error_description,
                ..
            } => Err(Error::AuthenticationError(error, error_description)),
        }
    }

    fn sign(&self, claims: &Claims) -> Result<String> {
        let key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())?;
        Ok(jsonwebtoken::encode::<Claims>(&self.header, claims, &key)?)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_generate_auth_token() {
        let valid_config = get_valid_config_complete();

        // The following config depends on an deleted service account key.
        // It is no longer possible to create tokens with this info.
        let invalid_config = AuthConfig::build(
            &fs::read_to_string("tests/test-client-old.json").unwrap(),
            &Usage::CloudVision,
        )
        .unwrap();

        assert!(valid_config.generate_auth_token(3600).await.is_ok());
        assert!(invalid_config.generate_auth_token(3600).await.is_err());
    }

    #[tokio::test]
    async fn test_generate_auth_token_wrong_json() {
        let config = AuthConfig::build(
            &fs::read_to_string("tests/invalid-client.json").unwrap(),
            &Usage::CloudVision,
        );
        assert!(config.is_err());
    }

    #[tokio::test]
    async fn test_invalid_usage() {
        let invalid_usage_config = get_valid_config(&Usage::Custom(String::from("invalid usage")));
        let no_usage_config = get_valid_config(&Usage::Custom(String::new()));
        assert!(invalid_usage_config
            .generate_auth_token(3600)
            .await
            .is_err());
        assert!(no_usage_config.generate_auth_token(3600).await.is_err());
    }

    #[tokio::test]
    async fn test_lifetime() {
        let valid_config = get_valid_config_complete();
        assert!(valid_config.generate_auth_token(3601).await.is_err());
        assert!(valid_config.generate_auth_token(29).await.is_err());
        assert!(valid_config.generate_auth_token(30).await.is_ok());
        assert!(valid_config.generate_auth_token(250).await.is_ok());
        assert!(valid_config.generate_auth_token(0).await.is_err());
        assert!(valid_config.generate_auth_token(-10).await.is_err());
    }

    fn get_valid_config_complete() -> AuthConfig {
        get_valid_config(&Usage::CloudVision)
    }

    fn get_valid_config(usage: &Usage) -> AuthConfig {
        AuthConfig::build(
            &fs::read_to_string("tests/test-client.json").unwrap(),
            usage,
        )
        .unwrap()
    }
}
