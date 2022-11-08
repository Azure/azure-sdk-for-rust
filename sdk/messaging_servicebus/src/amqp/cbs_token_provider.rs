use azure_core::auth::{TokenCredential, TokenResponse};
use fe2o3_amqp_cbs::{token::CbsToken, AsyncCbsTokenProvider};
use fe2o3_amqp_types::primitives::Timestamp;
use futures_util::{pin_mut, ready};
use std::{future::Future, task::Poll};
use time::{Duration as TimeSpan, OffsetDateTime};
use tokio::sync::Semaphore;

use crate::authorization::service_bus_token_credential::ServiceBusTokenCredential;

use super::token_type::TokenType;

#[derive(Debug)]
pub(crate) struct CbsTokenProvider<TC: TokenCredential> {
    /// The type to consider a token generated from the associated <see cref="Credential" />
    token_type: TokenType<TC>,

    // /// The credential used to generate access tokens.
    // credential: ServiceBusTokenCredential<TC>,
    /// The amount of buffer to when evaluating token expiration; the token's expiration date will
    /// be adjusted earlier by this amount.
    token_expiration_buffer: TimeSpan,
}

impl<TC> CbsTokenProvider<TC>
where
    TC: TokenCredential,
{
    /// Initializes a new instance of the <see cref="CbsTokenProvider"/> class.
    ///
    /// # Arguments
    ///
    /// `credential` - The credential to use for access token generation.
    /// `token_expiration_buffer` - The amount of time to buffer expiration
    /// `cancellation_token` - The cancellation token to consider when making requests.
    pub fn new(
        credential: ServiceBusTokenCredential<TC>,
        token_expiration_buffer: TimeSpan,
    ) -> Self {
        let token_type = if credential.is_shared_access_credential() {
            TokenType::SharedAccessToken { credential }
        } else {
            TokenType::JsonWebToken {
                credential,
                // Tokens are only cached for JWT-based credentials; no need
                // to instantiate the semaphore if no caching is taking place.
                semaphore: Semaphore::new(1),
                cached_token: None,
            }
        };

        Self {
            token_type,
            // credential,
            token_expiration_buffer,
        }
    }
}

fn is_nearing_expiration(token: &TokenResponse, token_expiration_buffer: TimeSpan) -> bool {
    token.expires_on - token_expiration_buffer <= OffsetDateTime::now_utc()
}

pub struct CbsTokenFut<'a, TC>
where
    TC: TokenCredential,
{
    provider: &'a mut CbsTokenProvider<TC>,
}

impl<'a, TC> Future for CbsTokenFut<'a, TC>
where
    TC: TokenCredential,
{
    type Output = Result<CbsToken<'a>, azure_core::error::Error>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        let expiration_buffer = self.provider.token_expiration_buffer.clone();
        let entity_type = self.provider.token_type.entity_type().to_owned(); // TODO: fix lifetime
        let result = match &mut self.provider.token_type {
            TokenType::SharedAccessToken { credential } => {
                let fut = credential.get_token("");
                pin_mut!(fut);
                ready!(fut.poll(cx))
            }
            TokenType::JsonWebToken {
                credential,
                semaphore,
                cached_token,
            } => match cached_token {
                Some(cached) => {
                    let fut = semaphore.acquire();
                    pin_mut!(fut);
                    let _permit = ready!(fut.poll(cx)).map_err(|e| {
                        azure_core::error::Error::new(azure_core::error::ErrorKind::Credential, e)
                    })?;
                    if is_nearing_expiration(cached, expiration_buffer) {
                        let fut = credential.get_token("");
                        pin_mut!(fut);
                        let token = ready!(fut.poll(cx))?;
                        *cached = token;
                    }
                    Ok(cached.clone())
                }
                None => {
                    let fut = semaphore.acquire();
                    pin_mut!(fut);
                    let _permit = ready!(fut.poll(cx)).map_err(|e| {
                        azure_core::error::Error::new(azure_core::error::ErrorKind::Credential, e)
                    })?;

                    // GetTokenUsingDefaultScopeAsync
                    let fut = credential.get_token("");
                    pin_mut!(fut);
                    let token = ready!(fut.poll(cx))?;
                    *cached_token = Some(token.clone());
                    Ok(token)
                }
            },
        };

        match result {
            Ok(token) => Poll::Ready(Ok(CbsToken::new(
                token.token.secret().to_owned(),
                entity_type,
                Some(Timestamp::from(token.expires_on)),
            ))),
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}

impl<TC> AsyncCbsTokenProvider for CbsTokenProvider<TC>
where
    TC: TokenCredential + 'static,
{
    type Fut<'a> = CbsTokenFut<'a, TC>;
    type Error = azure_core::error::Error;

    fn get_token_async(
        &mut self,
        _container_id: impl AsRef<str>,
        _resource_id: impl AsRef<str>,
        _claims: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self::Fut<'_> {
        CbsTokenFut { provider: self }
    }
}
