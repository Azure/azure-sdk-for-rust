use azure_core::errors::AzureError;
use http::HeaderMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationToken {
    pub(crate) query_path: String,
    pub(crate) partition_key: String,
    pub(crate) row_key: String,
}

impl ContinuationToken {
    pub(crate) fn parse_from_headers_optional(
        query_path: &str,
        headers: &HeaderMap,
    ) -> Result<Option<Self>, AzureError> {
        const HEADER_NEXTPARTITIONKEY: &str = "x-ms-continuation-NextPartitionKey";
        const HEADER_NEXTROWKEY: &str = "x-ms-continuation-NextRowKey";

        let result = if let (Some(partition_key), Some(row_key)) = (
            headers.get(HEADER_NEXTPARTITIONKEY),
            headers.get(HEADER_NEXTROWKEY),
        ) {
            Some(Self {
                query_path: query_path.to_owned(),
                partition_key: partition_key.to_str()?.to_owned(),
                row_key: row_key.to_str()?.to_owned(),
            })
        } else {
            None
        };

        Ok(result)
    }
}
