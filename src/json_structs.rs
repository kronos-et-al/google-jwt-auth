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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Claims {
    pub iss: String,
    pub scope: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
}

/// Example for a valid `JsonWebToken`:
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
/// This JSON can be obtained in the google console service account section.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonWebToken {
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub client_x509_cert_url: String,
}

/// Example for a valid `GoogleBearerResponseJson`:
/// ```json
///{
///   "access_token": "VERY_LONG_ACCESS_TOKEN",
///   "expires_in": 3599,
///   "token_type": "Bearer"
///
///}
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleBearerResponseJson {
    pub access_token: String,
    pub expires_in: i16,
    pub token_type: String,
}
