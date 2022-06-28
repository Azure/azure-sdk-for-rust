use azure_core::error::Error;
use azure_core::headers::{self, continuation_token_from_headers_optional};
use serde_json::Value;

/// The response for a query invocation
pub struct QueryResponse {
    /// The result of the query
    pub result: Value,
    /// The continuation token for the next result of the query
    pub continuation_token: Option<String>,
    /// The type of the item in the result
    pub item_type: String,
}

impl std::convert::TryFrom<crate::service::CollectedResponse> for QueryResponse {
    type Error = Error;

    fn try_from(response: crate::service::CollectedResponse) -> azure_core::Result<Self> {
        let headers = response.headers();
        let body: &[u8] = response.body();

        Ok(QueryResponse {
            result: serde_json::from_slice(body)?,
            continuation_token: continuation_token_from_headers_optional(headers)?,
            item_type: headers.get_string(&headers::ITEM_TYPE)?,
        })
    }
}
