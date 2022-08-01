#[macro_use]
extern crate azure_core;

mod account;
mod certificates;
mod clients;
mod keys;
pub mod prelude;
mod secrets;

pub use clients::*;

#[cfg(test)]
mod tests {
    use azure_core::auth::AccessToken;
    use azure_core::auth::{TokenCredential, TokenResponse};
    use time::{Duration, OffsetDateTime};

    #[macro_export]
    macro_rules! mock_client {
        ($keyvault_name:expr, $creds:expr) => {{
            $crate::KeyvaultClient {
                vault_url: url::Url::parse(&mockito::server_url()).unwrap(),
                endpoint: "".to_string(),
                token_credential: $creds,
                token: None,
            }
        }};
    }

    pub(crate) struct MockCredential(());
    impl MockCredential {
        pub(crate) fn new() -> std::sync::Arc<Self> {
            std::sync::Arc::new(Self(()))
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            _resource: &str,
        ) -> Result<TokenResponse, azure_core::error::Error> {
            Ok(TokenResponse::new(
                AccessToken::new("TOKEN".to_owned()),
                OffsetDateTime::now_utc() + Duration::days(14),
            ))
        }
    }
}
