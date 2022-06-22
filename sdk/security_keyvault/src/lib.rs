pub mod certificate;
mod client;
pub mod key;
pub mod secret;

pub use client::{CertificateClient, KeyClient};

#[cfg(test)]
mod tests {
    use azure_core::auth::AccessToken;
    use azure_core::auth::{TokenCredential, TokenResponse};
    use chrono::{Duration, Utc};

    #[macro_export]
    macro_rules! mock_key_client {
        ($keyvault_name:expr, $creds:expr, ) => {{
            $crate::client::KeyClient {
                vault_url: url::Url::parse(&mockito::server_url()).unwrap(),
                endpoint: "".to_string(),
                token_credential: $creds,
                token: None,
            }
        }};
    }
    #[macro_export]
    macro_rules! mock_cert_client {
        ($keyvault_name:expr, $creds:expr, ) => {{
            $crate::client::CertificateClient {
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
