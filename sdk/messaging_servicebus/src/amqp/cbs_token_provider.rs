use azure_core::auth::TokenResponse;
use fe2o3_amqp_cbs::{token::CbsToken, AsyncCbsTokenProvider};
use fe2o3_amqp_types::primitives::Timestamp;
use std::{future::Future, pin::Pin, sync::Arc};
use time::Duration as TimeSpan;

use crate::authorization::service_bus_token_credential::ServiceBusTokenCredential;

use super::token_type::TokenType;

#[derive(Debug)]
pub(crate) struct CbsTokenProvider {
    /// The token type
    token_type: TokenType,

    /// The amount of buffer to when evaluating token expiration; the token's expiration date will
    /// be adjusted earlier by this amount.
    token_expiration_buffer: TimeSpan,
}

impl CbsTokenProvider {
    /// Initializes a new instance of the [`CbsTokenProvider`] class.
    pub fn new(
        credential: Arc<ServiceBusTokenCredential>,
        token_expiration_buffer: TimeSpan,
    ) -> Self {
        let token_type = if credential.is_shared_access_credential() {
            TokenType::SharedAccessToken { credential }
        } else {
            TokenType::JsonWebToken {
                credential,
                cached_token: None,
            }
        };

        Self {
            token_type,
            token_expiration_buffer,
        }
    }
}

fn is_nearing_expiration(token: &TokenResponse, token_expiration_buffer: TimeSpan) -> bool {
    token.expires_on - token_expiration_buffer <= crate::util::time::now_utc()
}

impl AsyncCbsTokenProvider for CbsTokenProvider {
    type Fut<'a> =
        Pin<Box<dyn Future<Output = Result<CbsToken<'a>, azure_core::error::Error>> + Send + 'a>>;
    type Error = azure_core::error::Error;

    fn get_token_async(
        &mut self,
        _container_id: impl AsRef<str>,
        _resource_id: impl AsRef<str>,
        _claims: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self::Fut<'_> {
        // CbsTokenFut { provider: self }
        Box::pin(async {
            let expiration_buffer = self.token_expiration_buffer;
            let entity_type = self.token_type.entity_type().to_string();

            let result = match &mut self.token_type {
                TokenType::SharedAccessToken { credential } => {
                    let token = credential.get_token_using_default_resource().await?;
                    Ok(token)
                }
                TokenType::JsonWebToken {
                    credential,
                    cached_token,
                } => match cached_token {
                    Some(cached) => {
                        if is_nearing_expiration(cached, expiration_buffer) {
                            let token = credential.get_token_using_default_resource().await?;
                            *cached = token.clone();
                        }
                        Ok(cached.clone())
                    }
                    None => {
                        let token = credential.get_token_using_default_resource().await?;
                        *cached_token = Some(token.clone());
                        Ok(token)
                    }
                },
            };
            result.map(|token| {
                CbsToken::new(
                    token.token.secret().to_owned(),
                    entity_type,
                    Some(Timestamp::from(token.expires_on)),
                )
            })
        })
    }
}

#[cfg(test)]
mod tests {
    cfg_not_wasm32! {
        use fe2o3_amqp_cbs::AsyncCbsTokenProvider;
        use std::sync::Arc;
        use time::Duration as TimeSpan;

        use azure_core::auth::{AccessToken, TokenResponse};
        use time::{macros::datetime, OffsetDateTime};

        use crate::{
            authorization::{
                service_bus_token_credential::ServiceBusTokenCredential, tests::MockTokenCredential,
            },
            constants::JSON_WEB_TOKEN_TYPE,
        };

        #[tokio::test]
        async fn get_token() {
            let token_value = "ValuE_oF_tHE_tokEn";
            let expires_on: OffsetDateTime = datetime!(2015-10-27 00:00:00).assume_utc();
            let mut mock_credential = MockTokenCredential::new();

            mock_credential
                .expect_get_token()
                .returning(move |_resource| {
                    Ok(TokenResponse {
                        token: AccessToken::new(token_value),
                        expires_on,
                    })
                });

            let credential = ServiceBusTokenCredential::from(mock_credential);
            let mut provider = super::CbsTokenProvider::new(Arc::new(credential), TimeSpan::seconds(0));

            let token = provider
                .get_token_async("http://www.here.com", "nobody", Vec::<String>::new())
                .await
                .unwrap();

            assert_eq!(token.token_type(), JSON_WEB_TOKEN_TYPE);
            assert_eq!(token.token_value(), token_value);
            assert_eq!(
                token.expires_at_utc().clone().map(OffsetDateTime::from),
                Some(expires_on)
            );
        }

        #[tokio::test]
        async fn get_token_respects_cache_for_jwt_tokens() {
            let token_value = "ValuE_oF_tHE_tokEn";
            let expires_on: OffsetDateTime = crate::util::time::now_utc() + TimeSpan::days(60);
            let mut mock_credential = MockTokenCredential::new();

            mock_credential
                .expect_get_token()
                .times(1)
                .returning(move |_resource| {
                    Ok(TokenResponse {
                        token: AccessToken::new(token_value),
                        expires_on,
                    })
                });

            let credential = ServiceBusTokenCredential::from(mock_credential);
            let mut provider = super::CbsTokenProvider::new(Arc::new(credential), TimeSpan::seconds(0));

            let (first_token_value, first_token_type, first_token_expires_at) = {
                let token = provider
                    .get_token_async("http://www.here.com", "nobody", Vec::<String>::new())
                    .await
                    .unwrap();

                (
                    token.token_value().to_owned(),
                    token.token_type().to_owned(),
                    token.expires_at_utc().clone(),
                )
            };

            let second_token = provider
                .get_token_async("http://www.here.com", "nobody", Vec::<String>::new())
                .await
                .unwrap();

            assert_eq!(first_token_value, second_token.token_value());
            assert_eq!(first_token_type, second_token.token_type());
            assert_eq!(
                first_token_expires_at,
                second_token.expires_at_utc().clone()
            );
        }

        #[tokio::test]
        async fn get_token_respects_expiration_buffer_for_jwt() {
            let token_value = "ValuE_oF_tHE_tokEn";
            let buffer = TimeSpan::minutes(5);
            let expires_on: OffsetDateTime =
                crate::util::time::now_utc() - buffer + TimeSpan::seconds(-10);
            let mut mock_credential = MockTokenCredential::new();

            mock_credential
                .expect_get_token()
                .times(2)
                .returning(move |_resource| {
                    Ok(TokenResponse {
                        token: AccessToken::new(token_value),
                        expires_on,
                    })
                });

            let credential = ServiceBusTokenCredential::from(mock_credential);
            let mut provider = super::CbsTokenProvider::new(Arc::new(credential), buffer);

            let (first_token_value, first_token_type, first_token_expires_at) = {
                let token = provider
                    .get_token_async("http://www.here.com", "nobody", Vec::<String>::new())
                    .await
                    .unwrap();

                (
                    token.token_value().to_owned(),
                    token.token_type().to_owned(),
                    token.expires_at_utc().clone(),
                )
            };
            let second_token = provider
                .get_token_async("http://www.here.com", "nobody", Vec::<String>::new())
                .await
                .unwrap();
            assert_eq!(first_token_value, second_token.token_value());
            assert_eq!(first_token_type, second_token.token_type());
            assert_eq!(
                first_token_expires_at,
                second_token.expires_at_utc().clone()
            );
        }
    }
}
