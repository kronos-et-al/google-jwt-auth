use crate::error::Result;
use crate::json_structs::{Claims, GoogleBearerResponseJson, JsonWebToken};

use crate::error::TokenGenerationError::InvalidLifetime;
use jsonwebtoken::{Algorithm, EncodingKey, Header};

mod error;
mod json_structs;

struct Jwt {
    header: Header,
    claims: Claims,
    private_key: String,
}

impl Jwt {
    /// This function generates an jwt with the provided information. A jwt is used to request auth_tokens.
    /// This function generates only tokens with the RS256 encryption like the google jwt authentication service does.
    /// # Params
    /// **json_jwt_file_str: String**<br>
    /// Each google service account has an jwt json file that can be downloaded in the google console.
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
    pub fn build(json_jwt_file_str: String, usage: String, lifetime: i64) -> Result<Self> {
        //Todo make usage to Custom(String)
        if !(29..3601).contains(&lifetime) {
            return Err(InvalidLifetime(lifetime));
        }
        let jwt_json: JsonWebToken = serde_json::from_str(&json_jwt_file_str)?;
        Ok(Self {
            header: create_header(jwt_json.client_id),
            claims: create_claims(jwt_json.client_email, usage, jwt_json.token_uri, lifetime),
            private_key: jwt_json.private_key,
        })
    }

    /// With the provided jwt token, an authentication token (short: auth_token) will be requested from google.
    /// This auth_token will be returned and is used for requesting several google api services.
    /// # Errors
    /// See [`TokenGenerationError`] for a more detailed answer.
    /// # Returns
    /// The above mentioned auth_token as String.
    pub async fn request_auth_token(&self) -> Result<String> {
        let grant_type = "urn:ietf:params:oauth:grant-type:jwt-bearer"; // lazy static?
        let assertion = self.sign()?;
        let params = format!("grant_type={grant_type}&assertion={assertion}");
        let resp = reqwest::Client::new()
            .post(&self.claims.aud)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(params)
            .send()
            .await?
            .json::<GoogleBearerResponseJson>()
            .await?;
        Ok(resp.access_token)
    }

    fn sign(&self) -> Result<String> {
        let key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())?;
        Ok(jsonwebtoken::encode::<Claims>(
            &self.header,
            &self.claims,
            &key,
        )?)
    }
}

fn create_claims(client_email: String, usage: String, token_uri: String, lifetime: i64) -> Claims {
    let now = chrono::offset::Utc::now().timestamp();
    let expire = now + lifetime;
    Claims {
        iss: client_email,
        scope: usage,
        aud: token_uri,
        exp: expire,
        iat: now,
    }
}

fn create_header(_client_id: String) -> Header {
    Header::new(Algorithm::RS256) // add client_id? (not necessary!)
}

#[cfg(test)]
mod tests {
    #[ignore = "dead_code"]
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_generate_auth_token() {
        let jwt = get_valid_jwt();
        let token = jwt.request_auth_token().await;
        assert!(token.is_ok());
        println!("{}", token.unwrap());
    }

    #[tokio::test]
    async fn test_generate_auth_token_wrong_json() {
        let contents = fs::read_to_string("tests/invalid-client.json").unwrap();
        let jwt = Jwt::build(
            contents,
            String::from("https://www.googleapis.com/auth/cloud-vision"),
            3600,
        );
        assert!(jwt.is_err());
    }

    //#[tokio::test]
    async fn test_invalid_usage() {
        todo!()
    }

    //#[tokio::test]
    async fn test_invalid_lifetime() {
        todo!()
    }

    //#[tokio::test]
    async fn test_invalid_client_info() {
        todo!()
    }

    //#[tokio::test]
    async fn test_invalid_client_key() {
        todo!()
    }

    #[test]
    fn test_header() {
        assert_eq!(create_header(String::new()), Header::new(Algorithm::RS256))
    }

    #[test]
    fn test_claims() {
        let mail = String::from("any");
        let usage = String::from("any");
        let token_uri = String::from("any");
        let lifetime = 20;
        let now = chrono::offset::Utc::now().timestamp();
        let expire = now + lifetime;

        let assumption = Claims {
            iss: mail.clone(),
            scope: usage.clone(),
            aud: token_uri.clone(),
            exp: expire,
            iat: now,
        };
        assert_eq!(create_claims(mail, usage, token_uri, lifetime), assumption)
    }

    fn get_valid_jwt() -> Jwt {
        Jwt::build(
            fs::read_to_string("tests/test-client.json").unwrap(),
            String::from("https://www.googleapis.com/auth/cloud-vision"),
            3600,
        )
        .unwrap()
    }

    fn get_valid_jwt_with_invalid_values() -> Jwt {
        Jwt::build(
            fs::read_to_string("tests/invalid-value-client.json").unwrap(),
            String::from("https://www.googleapis.com/auth/cloud-vision"),
            3600,
        )
        .unwrap()
    }
}
