use azure_core::auth::{TokenCredential, TokenResponse};
use time::Duration as TimeSpan;
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
    //

    // TODO:
    // /// <summary>The JWT-based <see cref="CbsToken" /> that is currently cached for authorization.</summary>
    // private CbsToken _cachedJwtToken;
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
    pub async fn get_token(&mut self) -> TokenResponse {
        match &self.token_type {
            TokenType::SharedAccessToken { credential } => todo!(),
            TokenType::JsonWebToken {
                credential,
                semaphore,
            } => todo!(),
        }
        todo!()
    }
}
