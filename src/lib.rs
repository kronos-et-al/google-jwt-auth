use crate::error::{Result, TokenGenerationError};
use crate::json_structs::{Claims, GoogleResponse, ServiceAccountInfoJson};

use crate::error::TokenGenerationError::InvalidLifetime;
use jsonwebtoken::{Algorithm, EncodingKey, Header};

pub mod error;
pub mod json_structs;

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
    /// See [`TokenGenerationError`] for a more detailed answer.
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
    /// See [`TokenGenerationError`] for a more detailed answer.
    /// # Returns
    /// The above mentioned auth_token as String.
    pub async fn generate_auth_token(&self, lifetime: i64) -> Result<String> {
        if !(29..3601).contains(&lifetime) {
            return Err(InvalidLifetime(lifetime));
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
            GoogleResponse::ValidResponse { access_token, .. } => {
                Ok(access_token)
            }
            GoogleResponse::ErrorResponse { error, error_description, .. } => {
                Err(TokenGenerationError::AuthenticationError(error, error_description))
            }
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
        let config = get_valid_jwt();
        let token = config.generate_auth_token(3600).await;
        println!("{}", token.unwrap_err().to_string());
        assert!(token.is_ok());
        println!("{}", token.unwrap());
    }

    #[tokio::test]
    async fn test_generate_auth_token_wrong_json() {
        let contents = fs::read_to_string("tests/invalid-client.json").unwrap();
        let config = AuthConfig::build(
            contents,
            String::from("https://www.googleapis.com/auth/cloud-vision"),
        );
        assert!(config.is_err());
    }

    #[tokio::test]
    async fn test_invalid_usage() {
        todo!()
    }

    #[tokio::test]
    async fn test_invalid_lifetime() {
        todo!()
    }

    #[tokio::test]
    async fn test_invalid_client_info() {
        todo!()
    }

    #[tokio::test]
    async fn test_invalid_client_key() {
        todo!()
    }

    fn get_valid_jwt() -> AuthConfig {
        AuthConfig::build(
            fs::read_to_string("tests/test-client.json").unwrap(),
            String::from("https://www.googleapis.com/auth/cloud-vision"),
        )
        .unwrap()
    }

    fn get_valid_jwt_with_invalid_values() -> AuthConfig {
        AuthConfig::build(
            fs::read_to_string("tests/invalid-value-client.json").unwrap(),
            String::from("https://www.googleapis.com/auth/cloud-vision"),
        )
        .unwrap()
    }
}
