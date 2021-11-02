//! Azure Key Vault crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust]().
mod client;
pub mod key;
pub mod secret;

pub use client::KeyClient;
pub use secret::RecoveryLevel;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Key Vault does not exist, or is unreachable at '{keyvault_name:?}.vault.azure.net'")]
    KeyVaultDoesNotExist { keyvault_name: String },

    #[error("Azure Active Directory authorization error")]
    Authorization,

    #[error("Received an error accessing the Key Vault, which could not be parsed as expected.")]
    UnparsableError,

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Key Vault Error: {0}")]
    General(String),

    #[error("Failed to parse response from Key Vault: {0}")]
    SerdeParse(#[from] serde_json::Error),

    #[error("Could not get vault domain")]
    DomainParse,

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Failed to parse response from Key Vault when backing up secret {}, response body: {}, error: {}", secret_name, response_body, error)]
    BackupSecretParseError {
        error: serde_json::Error,
        secret_name: String,
        response_body: String,
    },

    #[error("Encryption algorithm mismatch")]
    EncryptionAlgorithmMismatch,
}

#[cfg(test)]
mod tests {
    use azure_core::{TokenCredential, TokenResponse};
    use chrono::{Duration, Utc};
    use oauth2::AccessToken;

    #[macro_export]
    macro_rules! mock_client {
        ($keyvault_name:expr, $creds:expr, ) => {{
            KeyClient {
                vault_url: url::Url::parse(&mockito::server_url()).unwrap(),
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
