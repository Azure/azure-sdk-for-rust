use crate::headers;
use azure_core::Request as HttpRequest;
use http::request::Builder;
use serde::Serialize;

/// CosmosDB partition key. Every CosmosDB entity must implement it.
pub trait CosmosEntity<'a> {
    /// Returned type.
    type Entity: Serialize + 'a;

    /// Return partition key value as reference.
    fn partition_key(&'a self) -> Self::Entity;
}

/// Serialize the partition key in the format CosmosDB expects.
pub(crate) fn serialize_partition_key<PK: Serialize>(pk: &PK) -> Result<String, serde_json::Error> {
    // this must be serialized as an array even tough CosmosDB supports only a sigle
    // partition key.
    serde_json::to_string(&[pk])
}

pub(crate) fn add_as_partition_key_header_serialized(
    partition_key_serialized: &str,
    builder: Builder,
) -> Builder {
    builder.header(
        headers::HEADER_DOCUMENTDB_PARTITIONKEY,
        partition_key_serialized,
    )
}

pub(crate) fn add_as_partition_key_header_serialized2(
    partition_key_serialized: &str,
    request: &mut HttpRequest,
) {
    request.headers_mut().insert(
        headers::HEADER_DOCUMENTDB_PARTITIONKEY,
        http::header::HeaderValue::from_str(partition_key_serialized).unwrap(),
    );
}
