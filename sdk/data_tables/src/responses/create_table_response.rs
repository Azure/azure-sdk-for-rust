use crate::prelude::*;
use azure_core::{error::Error, CollectedResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct CreateTableResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub table: Table,
}

impl TryFrom<CollectedResponse> for CreateTableResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        Ok(CreateTableResponse {
            common_storage_response_headers: response.headers().try_into()?,
            table: serde_json::from_slice(response.body())?,
        })
    }
}
