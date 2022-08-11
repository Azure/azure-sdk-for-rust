//! Utilities for interacting with [`Document`]s.

mod document_attributes;
mod indexing_directive;
mod query;

pub use document_attributes::DocumentAttributes;
pub use indexing_directive::IndexingDirective;
pub use query::{Param, Query};

use super::Resource;
use crate::headers;

use azure_core::headers::{AsHeaders, HeaderName, HeaderValue, Headers};
use azure_core::Header;
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

impl<T> std::convert::TryFrom<(&Headers, &[u8])> for Document<T>
where
    T: DeserializeOwned,
{
    type Error = azure_core::error::Error;
    fn try_from((_, body): (&Headers, &[u8])) -> Result<Self, Self::Error> {
        use azure_core::error::ResultExt;
        serde_json::from_slice::<Self>(body).with_context(
            azure_core::error::ErrorKind::DataConversion,
            || {
                format!(
                    "could not convert json '{}' into Permission",
                    std::str::from_utf8(body).unwrap_or("<NON-UTF8>")
                )
            },
        )
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum QueryCrossPartition {
    Yes,
    No,
}

impl Default for QueryCrossPartition {
    fn default() -> Self {
        Self::No
    }
}

impl From<bool> for QueryCrossPartition {
    fn from(b: bool) -> Self {
        if b {
            Self::Yes
        } else {
            Self::No
        }
    }
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
    fn name(&self) -> HeaderName {
        headers::HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION
    }

    fn value(&self) -> HeaderValue {
        self.as_bool_str().to_owned().into()
    }
}

/// Whether to parallelize across partitions
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<bool> for ParallelizeCrossPartition {
    fn from(b: bool) -> Self {
        if b {
            Self::Yes
        } else {
            Self::No
        }
    }
}

impl Header for ParallelizeCrossPartition {
    fn name(&self) -> HeaderName {
        headers::HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY
    }

    fn value(&self) -> HeaderValue {
        self.as_bool_str().to_owned().into()
    }
}

/// Whether the operation is an upsert
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn name(&self) -> HeaderName {
        headers::HEADER_DOCUMENTDB_IS_UPSERT
    }

    fn value(&self) -> HeaderValue {
        self.as_bool_str().to_owned().into()
    }
}

/// Whether to use an incremental change feed
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum ChangeFeed {
    Incremental,
    None,
}

impl Default for ChangeFeed {
    fn default() -> Self {
        Self::None
    }
}

impl AsHeaders for ChangeFeed {
    type Iter = std::option::IntoIter<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Self::Iter {
        match self {
            Self::Incremental => {
                Some((headers::HEADER_A_IM, "Incremental feed".into())).into_iter()
            }
            Self::None => None.into_iter(),
        }
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

impl Default for TentativeWritesAllowance {
    fn default() -> Self {
        Self::Deny
    }
}

impl Header for TentativeWritesAllowance {
    fn name(&self) -> HeaderName {
        headers::HEADER_ALLOW_MULTIPLE_WRITES
    }

    fn value(&self) -> HeaderValue {
        self.as_bool_str().to_owned().into()
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
    fn name(&self) -> HeaderName {
        headers::HEADER_DOCUMENTDB_PARTITIONRANGEID
    }

    fn value(&self) -> HeaderValue {
        self.0.clone().into()
    }
}
