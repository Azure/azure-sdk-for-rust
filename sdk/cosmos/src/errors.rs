/// A general error having to do with Cosmos.
#[derive(Debug, thiserror::Error)]
pub enum CosmosError {
    #[error("An error parsing json occured: {}", 0)]
    JsonError(#[from] serde_json::Error),
    #[error("An error in building a request occured: {}", 0)]
    RequestBuilderError(#[from] http::Error),
}
