use crate::ContinuationNextPartitionAndRowKey;
use azure_core::{error::Error, CollectedResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use serde::de::DeserializeOwned;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct QueryEntityResponse<E>
where
    E: DeserializeOwned,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub entities: Vec<E>,
    pub continuation_next_partition_and_row_key: Option<ContinuationNextPartitionAndRowKey>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct QueryEntityResponseInternal<E> {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(default = "Vec::new")]
    pub value: Vec<E>,
}

impl<E: DeserializeOwned> TryFrom<CollectedResponse> for QueryEntityResponse<E> {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let query_entity_response_internal: QueryEntityResponseInternal<E> =
            serde_json::from_slice(response.body())?;

        Ok(QueryEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            metadata: query_entity_response_internal.metadata,
            entities: query_entity_response_internal.value,
            continuation_next_partition_and_row_key:
                ContinuationNextPartitionAndRowKey::from_header_optional(response.headers())?,
        })
    }
}
