use crate::headers;
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

// Here we do not implement add_as_header because the trait does not support errors and serializing
// with serde_json returns a Result. I am not sure why a serialization could fail (memory
// allocation)? In case we are confident that no errors should arise we can implement the trait and just
// unwrap the result of serde_json::to_string.
pub(crate) fn add_as_partition_key_header<'a, P: CosmosEntity<'a>>(
    pk: &'a P,
    builder: Builder,
) -> Result<Builder, serde_json::Error> {
    Ok(builder.header(
        headers::HEADER_DOCUMENTDB_PARTITIONKEY,
        &serialize_partition_key(&pk.partition_key())?,
    ))
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
