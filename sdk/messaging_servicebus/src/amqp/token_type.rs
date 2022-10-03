use tokio::sync::Semaphore;

#[derive(Debug)]
pub(crate) enum TokenType {
    /// The type to consider a token if it is based on an Service Bus shared access signature.
    SharedAccessToken,
    /// The type to consider a token if not based on a shared access signature.
    JsonWebToken {
        /// Tokens are only cached for JWT-based credentials; no need
        /// to instantiate the semaphore if no caching is taking place.
        semaphore: Semaphore,
    },
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::SharedAccessToken => "servicebus.windows.net:sastoken".to_string(),
            TokenType::JsonWebToken { .. } => "jwt".to_string(),
        }
    }
}
