use crate::error::TokenGenerationError;
use crate::json_structs::{Claims, GoogleBearerResponseJson, JsonWebToken};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use std::collections::HashMap;

mod error;
mod json_structs;

// TODO unwarp() -> expect / error

/// This function generates an jwt token with the provided information.
/// With this jwt token, an authentication token (short: auth_token) will be requested from google.
/// This auth_token will be returned and is used for requesting several google api services.
/// This function only supports the RS256 encryption like the google jwt authentication service does.
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
/// The above mentioned auth_token as String.
pub async fn generate_auth_token(
    json_jwt_file_str: String,
    usage: String,
    lifetime: u16,
) -> String {
    let jwt_json: JsonWebToken = serde_json::from_str(&json_jwt_file_str).unwrap();
    let token_uri = jwt_json.token_uri.clone();
    let gen_token = create_and_sign_jwt(jwt_json, usage, lifetime).await;
    let grant_type = "urn:ietf:params:oauth:grant-type:jwt-bearer";
    request_auth_token(&token_uri, grant_type, &gen_token)
        .await
        .unwrap()
}

async fn request_auth_token(
    token_uri: &str,
    grant_type: &str,
    assertion: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let params = format!("grant_type={grant_type}&assertion={assertion}");
    let resp = reqwest::Client::new()
        .post(token_uri)
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

async fn create_and_sign_jwt(jwt_json: JsonWebToken, usage: String, lifetime: u16) -> String {
    let header = create_header(jwt_json.client_id);
    let claims = create_claims(jwt_json.client_email, usage, jwt_json.token_uri, lifetime);
    let pem_key = jwt_json.private_key;
    let key = EncodingKey::from_rsa_pem(pem_key.as_bytes()).unwrap();
    jsonwebtoken::encode::<Claims>(&header, &claims, &key).unwrap()
}

fn create_claims(client_email: String, usage: String, token_uri: String, lifetime: u16) -> Claims {
    let now = chrono::offset::Utc::now().timestamp();
    let expire = now + lifetime as i64;
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

/// @Deprecated
/// Public-key ist not needed for RS256-Encryption!!
/// To be deleted
async fn request_public_key(
    url: &str,
    private_key_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(resp.get(private_key_id).unwrap().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    /// This test has no assertion for now, but works.
    async fn test_generate_auth_token() {
        let contents = fs::read_to_string("src/tests/test-client.json").unwrap();
        println!(
            "{}",
            generate_auth_token(
                contents,
                String::from("https://www.googleapis.com/auth/cloud-vision"),
                3600
            )
            .await
        );
    }
}
