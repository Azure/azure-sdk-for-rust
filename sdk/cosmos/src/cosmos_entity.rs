use crate::headers;
use http::request::Builder;
use serde::Serialize;

/// CosmosDB partition key. Every CosmosDB entity must implement it.
pub trait CosmosEntity<'a, T: Serialize + 'a> {
    /// This function returns the partition key value as reference. Implement it by returning
    /// a reference of your partition key.
    fn partition_key(&'a self) -> T;
}

/// This function serializes the partition key in the format CosmosDB expects. It's a single line
/// but since it must be wrapped in an array (and it could change) we create a function to avoid
/// having to replicate this logic in the rest of the code.
pub(crate) fn serialize_partition_key_to_string<PK: Serialize>(
    pk: &PK,
) -> Result<String, serde_json::Error> {
    // this must be serialized as an array even tough CosmosDB supports only a sigle
    // partition key.
    serde_json::to_string(&[pk])
}

// Here we do not implement add_as_header because the trait does not support errors and serializing
// with serde_json returns a Result. I am not sure why a serialization could fail (memory
// allocation)? In case we are confident that no errors should arise we can implement the trait and just
// unwrap the result of serde_json::to_string.
pub(crate) fn add_as_partition_key_header<'a, T: Serialize + 'a, P: CosmosEntity<'a, T> + 'a>(
    pk: &'a P,
    builder: Builder,
) -> Result<Builder, serde_json::Error> {
    Ok(builder.header(
        headers::HEADER_DOCUMENTDB_PARTITIONKEY,
        &serialize_partition_key_to_string(&pk.partition_key())?,
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
