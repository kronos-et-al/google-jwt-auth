use serde::{Deserialize, Serialize};

/// Example for a valid `Claims` json:
/// ```json
/// {
///     "iss": "service_account@??????.gserviceaccount.com",
///     "scope": "https://www.googleapis.com/auth/devstorage.read_only",
///     "aud": "https://oauth2.googleapis.com/token",
///     "exp": 1697281452,
///     "iat": 1697285052
/// }
/// ```
#[derive(Serialize, Debug, PartialEq)]
pub struct Claims {
    pub iss: String,
    pub scope: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(iss: String, scope: String, aud: String, lifetime: i64) -> Claims {
        let now = chrono::offset::Utc::now().timestamp();
        let expire = now + lifetime;
        Self {
            iss,
            scope,
            aud,
            exp: expire,
            iat: now,
        }
    }
}

/// Example for a valid `ServiceAccountInfoJson`:
/// ```json
/// {
///     "type": "service_account",
///     "project_id": "PROJECT_ID",
///     "private_key_id": "0000000000000000000000000000000000000000",
///     "private_key": "PRIVATE_KEY_IN_BASE64_WITH_PEM_HEADER_AND_FOOTER",
///     "client_email": "service_account@??????.gserviceaccount.com",
///     "client_id": "000000000000000000000",
///     "auth_uri": "https://accounts.google.com/o/oauth2/auth",
///     "token_uri": "https://oauth2.googleapis.com/token",
///     "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
///     "client_x509_cert_url": "URL_TO_PUBLIC_KEY",
///     "universe_domain": "googleapis.com"
/// }
/// ```
/// This JSON can be downloaded in the google console service account section during the key generation.
/// This JSON cannot be downloaded twice! A new key must be generated, if the file gets lost!
#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceAccountInfoJson {
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub client_x509_cert_url: String,
}

/// Contains all possible response structures for the google authentication service.
/// See [`ValidResponse`] and [`ErrorResponse`] for more details.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GoogleResponse {
    ValidResponse {access_token: String, expires_in: i16, token_type: String},
    ErrorResponse {error: String, error_description: String}
}

/// Example for a valid `ValidResponse`:
/// ```json
///{
///   "access_token": "VERY_LONG_ACCESS_TOKEN",
///   "expires_in": 3599,
///   "token_type": "Bearer"
///}
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ValidResponse {
    access_token: String,
    expires_in: i16,
    token_type: String
}

/// Example for a valid `ErrorResponse`:
/// ```json
///{
///   "error": "invalid_grant",
///   "error_description": "Invalid JWT Signature."
///}
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    error: String,
    error_description: String
}
