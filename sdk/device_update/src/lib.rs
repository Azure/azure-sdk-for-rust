mod client;
pub mod device_update;

use crate::device_update::UpdateOperation;
pub use client::DeviceUpdateClient;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] azure_core::Error),

    #[error("Base64 Decode Error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Could not get device update domain")]
    DomainParse,

    #[error("Successful import (202 status) but no operation-location header found")]
    NoOperationLocation,

    #[error("Invalid characters in operation-location path")]
    InvalidOperationPath,

    #[error("Import unsuccessful, status: {0}")]
    ImportError(reqwest::StatusCode),

    #[error("Import unsuccessful with status Failed, error: {0:?}")]
    ImportFailed(UpdateOperation),

    #[error("Import unsuccessful with status Undefined, error: {0:?}")]
    ImportUndefined(UpdateOperation),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use azure_core::auth::{TokenCredential, TokenResponse};
    use azure_identity::AutoRefreshingTokenCredential;
    use chrono::{Duration, Utc};
    use oauth2::AccessToken;
    use std::sync::Arc;

    pub(crate) fn mock_client() -> crate::client::DeviceUpdateClient {
        crate::client::DeviceUpdateClient {
            device_update_url: url::Url::parse(&mockito::server_url()).unwrap(),
            endpoint: "".to_string(),
            token_credential: AutoRefreshingTokenCredential::new(Arc::new(MockCredential)),
        }
    }

    pub(crate) struct MockCredential;

    #[async_trait::async_trait]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            _resource: &str,
        ) -> Result<TokenResponse, azure_core::error::Error> {
            Ok(TokenResponse::new(
                AccessToken::new("TOKEN".to_owned()),
                Utc::now() + Duration::days(14),
            ))
        }
    }
}
