use azure_core::auth::TokenResponse;
use std::sync::Arc;

use crate::{
    authorization::event_hub_token_credential::EventHubTokenCredential,
    constants::{JSON_WEB_TOKEN_TYPE, SAS_TOKEN_TYPE},
};

#[derive(Debug)]
pub(crate) enum TokenType {
    /// The type to consider a token if it is based on an Event Hubs shared access signature.
    SharedAccessToken {
        credential: Arc<EventHubTokenCredential>,
    },
    /// The type to consider a token if not based on a shared access signature.
    JsonWebToken {
        credential: Arc<EventHubTokenCredential>,

        /// The JWT-based token that is currently cached for authorization.
        cached_token: Option<TokenResponse>,
    },
}

impl TokenType {
    pub(crate) fn entity_type(&self) -> &str {
        match self {
            TokenType::SharedAccessToken { .. } => SAS_TOKEN_TYPE,
            TokenType::JsonWebToken { .. } => JSON_WEB_TOKEN_TYPE,
        }
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::SharedAccessToken { .. } => SAS_TOKEN_TYPE.to_string(),
            TokenType::JsonWebToken { .. } => JSON_WEB_TOKEN_TYPE.to_string(),
        }
    }
}
