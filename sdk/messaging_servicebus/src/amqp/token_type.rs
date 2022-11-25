use azure_core::auth::{TokenCredential, TokenResponse};
use tokio::sync::Semaphore;

use crate::{authorization::service_bus_token_credential::ServiceBusTokenCredential, constants::{SAS_TOKEN_TYPE, JSON_WEB_TOKEN_TYPE}};

#[derive(Debug)]
pub(crate) enum TokenType<TC: TokenCredential> {
    /// The type to consider a token if it is based on an Service Bus shared access signature.
    SharedAccessToken {
        credential: ServiceBusTokenCredential<TC>,
    },
    /// The type to consider a token if not based on a shared access signature.
    JsonWebToken {
        credential: ServiceBusTokenCredential<TC>,

        /// Tokens are only cached for JWT-based credentials; no need
        /// to instantiate the semaphore if no caching is taking place.
        semaphore: Semaphore,

        /// The JWT-based <see cref="CbsToken" /> that is currently cached for authorization.
        cached_token: Option<TokenResponse>,
    },
}

impl<TC: TokenCredential> TokenType<TC> {
    pub(crate) fn entity_type(&self) -> &str {
        match self {
            TokenType::SharedAccessToken { .. } => SAS_TOKEN_TYPE,
            TokenType::JsonWebToken { .. } => JSON_WEB_TOKEN_TYPE,
        }
    }
}

impl<TC: TokenCredential> ToString for TokenType<TC> {
    fn to_string(&self) -> String {
        match self {
            TokenType::SharedAccessToken { .. } => SAS_TOKEN_TYPE.to_string(),
            TokenType::JsonWebToken { .. } => JSON_WEB_TOKEN_TYPE.to_string(),
        }
    }
}
