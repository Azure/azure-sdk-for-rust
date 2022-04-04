mod client;
pub mod device_update;

pub use client::DeviceUpdateClient;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Azure Active Directory authorization error")]
    Authorization,

    #[error(
        "Received an error accessing the Device Update, which could not be parsed as expected."
    )]
    UnparsableError,

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Device Update Error: {0}")]
    General(String),

    #[error("Base64 Decode Error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Failed to parse response from Device Update: {0}")]
    SerdeParse(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Could not get device update domain")]
    DomainParse,

    #[error("Successful import (202 status) but no operation-location header found")]
    NoOperationLocation,

    #[error("Invalid characters in operation-location path")]
    InvalidOperationPath(),

    #[error("Import unsuccessful, status: {0}")]
    ImportError(reqwest::StatusCode),

    #[error("Import unsuccessful with status Failed, error: {0}")]
    ImportFailed(String),

    #[error("Import unsuccessful with status Undefined, error: {0}")]
    ImportUndefined(String),
}

#[cfg(test)]
mod tests {
    use azure_core::auth::{TokenCredential, TokenResponse};
    use chrono::{Duration, Utc};
    use oauth2::AccessToken;

    #[macro_export]
    macro_rules! mock_key_client {
        ($device_update_name:expr, $creds:expr, ) => {{
            crate::client::DeviceUpdateClient {
                device_update_url: url::Url::parse(&mockito::server_url()).unwrap(),
                endpoint: "".to_string(),
                token_credential: $creds,
                token: None,
            }
        }};
    }

    pub(crate) struct MockCredential;

    #[async_trait::async_trait]
    impl TokenCredential for MockCredential {
        async fn get_token(&self, _resource: &str) -> Result<TokenResponse, azure_core::Error> {
            Ok(TokenResponse::new(
                AccessToken::new("TOKEN".to_owned()),
                Utc::now() + Duration::days(14),
            ))
        }
    }
}
