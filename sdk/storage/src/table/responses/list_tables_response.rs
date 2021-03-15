use crate::table::prelude::*;
use azure_core::{errors::AzureError, headers::CommonStorageResponseHeaders, prelude::NextMarker};
use bytes::Bytes;
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct ListTablesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub tables: Vec<Table>,
    pub next_marker: Option<NextMarker>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ListTablesResponseInternal {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(default = "Vec::new")]
    pub value: Vec<Table>,
}

impl TryFrom<&Response<Bytes>> for ListTablesResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("{}", std::str::from_utf8(response.body())?);
        debug!("headers == {:#?}", response.headers());

        let list_tables_response_internal: ListTablesResponseInternal =
            serde_json::from_slice(response.body())?;

        Ok(ListTablesResponse {
            common_storage_response_headers: response.headers().try_into()?,
            metadata: list_tables_response_internal.metadata,
            tables: list_tables_response_internal.value,
            next_marker: NextMarker::from_table_header_optional(response.headers())?,
        })
    }
}
