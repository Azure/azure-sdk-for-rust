//! Utilities for interacting with [`Document`]s.

mod document_attributes;
mod indexing_directive;
mod query;

pub use document_attributes::DocumentAttributes;
pub use indexing_directive::IndexingDirective;
pub use query::{Param, Query};

use super::Resource;
use crate::headers;

use azure_core::Header;
use http::header::HeaderMap;
use http::request::Builder;
use serde::de::DeserializeOwned;

/// User-defined content in JSON format.
///
/// You can learn more about Documents [here](https://docs.microsoft.com/rest/api/cosmos-db/documents).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Document<T> {
    #[serde(flatten)]
    pub document_attributes: DocumentAttributes,
    #[serde(flatten)]
    pub document: T, // raw, id not included
}

impl<T> Document<T> {
    /// Create a new document
    pub fn new(document: T) -> Self {
        let document_attributes = DocumentAttributes::default();

        Self {
            document_attributes,
            document,
        }
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for Document<T>
where
    T: DeserializeOwned,
{
    type Error = crate::Error;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let _headers = value.0;
        let body = value.1;

        Ok(serde_json::from_slice(body)?)
    }
}

impl<T> Resource for Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}

impl<T> Resource for &Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}

/// Whether to query across partitions
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum QueryCrossPartition {
    Yes,
    No,
}

impl QueryCrossPartition {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Yes => "true",
            Self::No => "false",
        }
    }
}

impl Header for QueryCrossPartition {
    fn name(&self) -> &'static str {
        headers::HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION
    }

    fn value(&self) -> String {
        self.as_bool_str().to_owned()
    }
}

/// Whether to parallelize across partitions
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParallelizeCrossPartition {
    Yes,
    No,
}

impl ParallelizeCrossPartition {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Yes => "true",
            Self::No => "false",
        }
    }
}

impl Header for ParallelizeCrossPartition {
    fn name(&self) -> &'static str {
        headers::HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY
    }

    fn value(&self) -> String {
        self.as_bool_str().to_owned()
    }
}

/// Whether the operation is an upsert
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum IsUpsert {
    Yes,
    No,
}

impl IsUpsert {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Yes => "true",
            Self::No => "false",
        }
    }
}

impl Header for IsUpsert {
    fn name(&self) -> &'static str {
        headers::HEADER_DOCUMENTDB_IS_UPSERT
    }

    fn value(&self) -> String {
        self.as_bool_str().to_owned()
    }
}

/// Whether to use an incremental change feed
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum ChangeFeed {
    Incremental,
    None,
}

impl Header for ChangeFeed {
    fn add_to_builder(&self, builder: Builder) -> Builder {
        match self {
            Self::Incremental => builder.header(headers::HEADER_A_IM, "Incremental feed"),
            Self::None => builder,
        }
    }

    fn add_to_request(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        match self {
            Self::Incremental => {
                request.headers_mut().append(
                    headers::HEADER_A_IM,
                    http::header::HeaderValue::from_str("Incremental feed")?,
                );
            }
            Self::None => {}
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        todo!()
    }

    fn value(&self) -> String {
        todo!()
    }
}

/// Whether to allow tenative writes allowance
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum TentativeWritesAllowance {
    Allow,
    Deny,
}

impl TentativeWritesAllowance {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Allow => "true",
            Self::Deny => "false",
        }
    }
}

impl Header for TentativeWritesAllowance {
    fn name(&self) -> &'static str {
        headers::HEADER_ALLOW_MULTIPLE_WRITES
    }

    fn value(&self) -> String {
        self.as_bool_str().to_owned()
    }
}

/// Collections of partition keys grouped by physical partitions
#[derive(Debug, Clone)]
pub struct PartitionRangeId(String);

impl PartitionRangeId {
    /// A new partition range id from a string
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Header for PartitionRangeId {
    fn name(&self) -> &'static str {
        headers::HEADER_DOCUMENTDB_PARTITIONRANGEID
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}
