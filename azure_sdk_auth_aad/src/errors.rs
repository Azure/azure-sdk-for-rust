use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerReceiveError {
    #[error("unexpected redirect url: {}", url)]
    UnexpectedRedirectUrl { url: String },
    #[error("query pair not found: {}", query_pair)]
    QueryPairNotFound { query_pair: String },
    #[error(
        "State secret mismatch: expected {}, recieved: {}",
        expected_state_secret,
        received_state_secret
    )]
    StateSecretMismatch {
        expected_state_secret: String,
        received_state_secret: String,
    },
}

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorResponse {
    #[error("Unrecognized Azure error response:\n{}\n", error_description)]
    GenericError { error_description: String },
}
