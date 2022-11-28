mod create_table;
mod delete_entity;
mod delete_table;
mod get_entity;
mod insert_entity;
pub(crate) mod insert_or_replace_or_merge_entity;
mod list_tables;
mod query_entity;
mod transaction;
pub(crate) mod update_or_merge_entity;
pub use create_table::CreateTableBuilder;
pub use delete_entity::DeleteEntityBuilder;
pub use delete_table::DeleteTableBuilder;
pub use get_entity::GetEntityBuilder;
pub use insert_entity::InsertEntityBuilder;
pub use insert_or_replace_or_merge_entity::InsertOrReplaceOrMergeEntityBuilder;
pub use list_tables::ListTablesBuilder;
pub use query_entity::{QueryEntityBuilder, QueryEntityResponse};
pub use transaction::TransactionBuilder;
pub use update_or_merge_entity::UpdateOrMergeEntityBuilder;

use crate::EntityWithMetadata;
use azure_core::{
    error::{Error, ErrorKind},
    headers::{self, etag_from_headers, HeaderName},
    CollectedResponse, Etag,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use serde::de::DeserializeOwned;
use std::convert::{TryFrom, TryInto};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InsertOperation {
    InsertOrReplace,
    InsertOrMerge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UpdateOperation {
    Update,
    Merge,
}

#[derive(Debug, Clone)]
pub struct OperationOnEntityResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
}

impl TryFrom<CollectedResponse> for OperationOnEntityResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        Ok(OperationOnEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            etag: etag_from_headers(response.headers())?.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct InsertEntityResponse<T>
where
    T: DeserializeOwned + Send,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub location: Option<Url>,
    pub entity_with_metadata: Option<EntityWithMetadata<T>>,
}

impl<T> TryFrom<CollectedResponse> for InsertEntityResponse<T>
where
    T: DeserializeOwned + Send,
{
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let headers = response.headers();
        let entity_with_metadata =
            match headers.get_str(&HeaderName::from_static("preference-applied"))? {
                "return-no-content" => None,
                "return-content" => Some(response.clone().try_into()?),
                _ => {
                    return Err(Error::message(
                        ErrorKind::DataConversion,
                        "Unexpected value for preference-applied header",
                    ))
                }
            };

        Ok(InsertEntityResponse {
            common_storage_response_headers: headers.try_into()?,
            etag: etag_from_headers(headers)?.into(),
            location: headers.get_optional_as(&headers::LOCATION)?,
            entity_with_metadata,
        })
    }
}
