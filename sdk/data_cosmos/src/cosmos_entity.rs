use crate::headers;
use azure_core::{headers::HeaderValue, Request as HttpRequest};
use serde::Serialize;

/// `CosmosDB` partition key. Every `CosmosDB` entity must implement it.
///
/// If you want to treat a Rust type as a document to be added to a Cosmos collection,
/// you just need to implement this trait for your type. This specifies what you use as
/// a partitioning key. You can read more about how partitioning works in Cosmos
/// [here](https://docs.microsoft.com/en-us/azure/cosmos-db/partitioning-overview).
///
/// # Example
///
/// ```no_run
/// struct MySampleStruct {
///     id: String,
///     string: String,
///     number: u64,
/// }
///
/// impl azure_data_cosmos::CosmosEntity for MySampleStruct {
///     type Entity = u64;
///
///     fn partition_key(&self) -> Self::Entity {
///         self.number
///     }
/// }
/// ```
pub trait CosmosEntity {
    /// Returned type.
    type Entity: Serialize;

    /// Return partition key value as reference.
    fn partition_key(&self) -> Self::Entity;
}

impl CosmosEntity for serde_json::Value {
    type Entity = Self;
    fn partition_key(&self) -> Self::Entity {
        self.clone()
    }
}

/// Serialize the partition key in the format `CosmosDB` expects.
pub(crate) fn serialize_partition_key<PK: Serialize>(pk: &PK) -> azure_core::Result<String> {
    use azure_core::error::ResultExt;
    // this must be serialized as an array even tough CosmosDB supports only a single partition key.
    serde_json::to_string(&[pk]).context(
        azure_core::error::ErrorKind::DataConversion,
        "could not convert partition_key into String",
    )
}

pub(crate) fn add_as_partition_key_header_serialized(
    partition_key_serialized: &str,
    request: &mut HttpRequest,
) {
    request.insert_header(
        headers::HEADER_DOCUMENTDB_PARTITIONKEY,
        HeaderValue::from(partition_key_serialized.to_string()),
    );
}
