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
pub(crate) struct Claims {
    pub(crate) iss: String,
    pub(crate) scope: String,
    pub(crate) aud: String,
    pub(crate) exp: i64,
    pub(crate) iat: i64,
}

impl Claims {
    pub(crate) fn new(iss: String, scope: String, aud: String, lifetime: i64) -> Self {
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
pub(crate) struct ServiceAccountInfoJson {
    pub(crate) project_id: String,
    pub(crate) private_key_id: String,
    pub(crate) private_key: String,
    pub(crate) client_email: String,
    pub(crate) client_id: String,
    pub(crate) auth_uri: String,
    pub(crate) token_uri: String,
    pub(crate) client_x509_cert_url: String,
}

/// Contains all possible response structures for the google authentication service.
/// See [`ValidResponse`] and [`ErrorResponse`] for more details.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GoogleResponse {
    ///
    ValidResponse {
        // Todo merge with ValidResponse struct
        ///
        access_token: String,
        ///
        expires_in: i16,
        ///
        token_type: String,
    },
    ///
    ErrorResponse {
        // Todo merge with ErrorResponse struct
        ///
        error: String,
        ///
        error_description: String,
    },
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
    token_type: String,
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
    error_description: String,
}
