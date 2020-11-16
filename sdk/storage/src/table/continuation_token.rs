use azure_core::errors::AzureError;
use http::HeaderMap;

#[derive(Debug, Clone)]
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

        Ok(
            if headers.contains_key(HEADER_NEXTPARTITIONKEY)
                && headers.contains_key(HEADER_NEXTROWKEY)
            {
                Some(Self {
                    query_path: query_path.to_owned(),
                    partition_key: headers[HEADER_NEXTPARTITIONKEY].to_str()?.to_owned(),
                    row_key: headers[HEADER_NEXTROWKEY].to_str()?.to_owned(),
                })
            } else {
                None
            },
        )
    }
}
