use azure_core::{
    headers::{self, continuation_token_from_headers_optional},
    prelude::Continuation,
};
use serde_json::Value;

/// The response for a query invocation
pub struct QueryResponse {
    /// The result of the query
    pub result: Value,
    /// The continuation token for the next result of the query
    pub continuation_token: Option<Continuation>,
    /// The type of the item in the result
    pub item_type: String,
}

impl QueryResponse {
    pub(crate) async fn try_from(response: azure_core::Response) -> azure_core::Result<Self> {
        let collected = azure_core::CollectedResponse::from_response(response).await?;
        let result = collected.json()?;

        Ok(QueryResponse {
            result,
            continuation_token: continuation_token_from_headers_optional(collected.headers())?,
            item_type: collected.headers().get_as(&headers::ITEM_TYPE)?,
        })
    }
}
