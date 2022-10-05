use azure_core::auth::{TokenCredential, TokenResponse};
use time::{Duration as TimeSpan, OffsetDateTime};
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

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

    /// The cancellation token to consider when making requests.
    cancellation_token: CancellationToken,
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
        cancellation_token: CancellationToken,
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
            cancellation_token,
        }
    }

    /// <summary>
    ///   Asynchronously requests a CBS token to be used for authorization within an AMQP
    ///   scope.
    /// </summary>
    ///
    /// <param name="namespaceAddress">The address of the namespace to be authorized.</param>
    /// <param name="appliesTo">The resource to which the token should apply.</param>
    /// <param name="requiredClaims">The set of claims that are required for authorization.</param>
    ///
    /// <returns>The token to use for authorization.</returns>
    ///
    pub async fn get_token(&mut self) -> azure_core::error::Result<TokenResponse> {
        match &mut self.token_type {
            TokenType::SharedAccessToken { credential } => {
                // GetTokenUsingDefaultScopeAsync
                credential.get_token("").await
            }
            TokenType::JsonWebToken {
                credential,
                semaphore,
                cached_token,
            } => match cached_token {
                Some(cached) => {
                    let _permit = semaphore.acquire().await.map_err(|e| {
                        azure_core::error::Error::new(azure_core::error::ErrorKind::Credential, e)
                    })?;

                    if is_nearing_expiration(cached, self.token_expiration_buffer) {
                        *cached = credential.get_token("").await?;
                    }

                    Ok(cached.clone())
                }
                None => {
                    let _permit = semaphore.acquire().await.map_err(|e| {
                        azure_core::error::Error::new(azure_core::error::ErrorKind::Credential, e)
                    })?;

                    // GetTokenUsingDefaultScopeAsync
                    let token = credential.get_token("").await?;
                    *cached_token = Some(token.clone());
                    Ok(token)
                }
            },
        }
    }
}

fn is_nearing_expiration(token: &TokenResponse, token_expiration_buffer: TimeSpan) -> bool {
    token.expires_on - token_expiration_buffer <= OffsetDateTime::now_utc()
}
