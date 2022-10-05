use azure_core::auth::TokenCredential;
use tokio::sync::Semaphore;

use crate::authorization::service_bus_token_credential::ServiceBusTokenCredential;

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
    },
}

impl<TC: TokenCredential> ToString for TokenType<TC> {
    fn to_string(&self) -> String {
        match self {
            TokenType::SharedAccessToken { .. } => "servicebus.windows.net:sastoken".to_string(),
            TokenType::JsonWebToken { .. } => "jwt".to_string(),
        }
    }
}
