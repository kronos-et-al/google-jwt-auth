use crate::error::{Result, TokenGenerationError};
use crate::json_structs::{Claims, GoogleResponse, ServiceAccountInfoJson};

use jsonwebtoken::{Algorithm, EncodingKey, Header};

mod error;
mod json_structs;
pub mod usage;

pub type ErrorKind = TokenGenerationError;
static GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";
static CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

pub struct AuthConfig {
    header: Header,
    iss: String,
    scope: String,
    aud: String,
    private_key: String,
}

impl AuthConfig {
    /// This function generates an auth configuration with the provided information. A this config is used to request auth_tokens.
    /// This function generates only tokens with the RS256 encryption like the google jwt authentication service does.
    /// # Params
    /// **service_account_json_str: String**<br>
    /// Each google service account has a json file that can be downloaded in the google console during the key generation.
    /// This json file cannot be downloaded twice! A new key must be generated, if the file gets lost!
    /// The content of this file needs to be provided by this param as string.
    ///
    /// **usage: String**<br>
    /// Each google api request requires individual permissions to be executed.
    /// Beside the service account permission a usage or a scope should be provided.
    /// See here for more information: URL.
    ///
    /// **lifetime: u16**<br>
    /// An auth_token has a limited lifetime to am maximum of 3600 seconds.
    /// This value should be between 30 and 3600 Seconds.
    /// Inputs out of this ranged will not be accepted.
    /// # Errors
    /// See [`ErrorKind`] for a more detailed answer.
    /// # Returns
    /// The above mentioned jwt as String.
    pub fn build(service_account_json_str: String, usage: String) -> Result<Self> {
        //Todo make usage to Custom(String)
        let account_info: ServiceAccountInfoJson = serde_json::from_str(&service_account_json_str)?;
        Ok(Self {
            header: Header::new(Algorithm::RS256),
            iss: account_info.client_email,
            scope: usage,
            aud: account_info.token_uri,
            private_key: account_info.private_key,
        })
    }

    /// With the provided jwt token, an authentication token (short: auth_token) will be requested from google.
    /// This auth_token will be returned and is used for requesting several google api services.
    /// # Errors
    /// See [`ErrorKind`] for a more detailed answer.
    /// # Returns
    /// The above mentioned auth_token as String.
    pub async fn generate_auth_token(&self, lifetime: i64) -> Result<String> {
        if !(30..3601).contains(&lifetime) {
            return Err(ErrorKind::InvalidLifetime(lifetime));
        }

        // TODO add token buffer with lifetime check to minimize auth_token requests
        // <--

        let claims = Claims::new(
            self.iss.clone(),
            self.scope.clone(),
            self.aud.clone(),
            lifetime,
        );
        let assertion = self.sign(claims)?;

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
            } => Err(ErrorKind::AuthenticationError(
                error,
                error_description,
            )),
        }
    }

    fn sign(&self, claims: Claims) -> Result<String> {
        let key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())?;
        Ok(jsonwebtoken::encode::<Claims>(&self.header, &claims, &key)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_generate_auth_token() {
        let valid_config = get_valid_config_complete();

        // The following config depends on an deleted service account key.
        // It is no longer possible to create tokens with this info.
        let invalid_config = AuthConfig::build(
            fs::read_to_string("tests/test-client-old.json").unwrap(),
            String::from("https://www.googleapis.com/auth/cloud-vision"),
        )
        .unwrap();

        assert!(valid_config.generate_auth_token(3600).await.is_ok());
        assert!(invalid_config.generate_auth_token(3600).await.is_err());
    }

    #[tokio::test]
    async fn test_generate_auth_token_wrong_json() {
        let config = AuthConfig::build(
            fs::read_to_string("tests/invalid-client.json").unwrap(),
            String::from("https://www.googleapis.com/auth/cloud-vision"),
        );
        assert!(config.is_err());
    }

    #[tokio::test]
    async fn test_invalid_usage() {
        let invalid_usage_config = get_valid_config(String::from("invalid usage"));
        let no_usage_config = get_valid_config(String::new());
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
        get_valid_config(String::from("https://www.googleapis.com/auth/cloud-vision"))
    }

    fn get_valid_config(usage: String) -> AuthConfig {
        AuthConfig::build(fs::read_to_string("tests/test-client.json").unwrap(), usage).unwrap()
    }
}
