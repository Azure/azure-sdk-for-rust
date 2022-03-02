//! Utilities for interacting with [`Document`]s.

mod document_attributes;
mod indexing_directive;
mod query;

pub use document_attributes::DocumentAttributes;
pub use indexing_directive::IndexingDirective;
pub use query::{Param, Query};

use super::Resource;
use crate::headers;

use azure_core::AddAsHeader;
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

impl AddAsHeader for QueryCrossPartition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION,
            self.as_bool_str(),
        )
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            headers::HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION,
            http::header::HeaderValue::from_str(self.as_bool_str())?,
        );

        Ok(())
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

impl AddAsHeader for ParallelizeCrossPartition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY,
            self.as_bool_str(),
        )
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            headers::HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY,
            http::header::HeaderValue::from_str(self.as_bool_str())?,
        );

        Ok(())
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

impl AddAsHeader for IsUpsert {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_DOCUMENTDB_IS_UPSERT, self.as_bool_str())
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            headers::HEADER_DOCUMENTDB_IS_UPSERT,
            http::header::HeaderValue::from_str(self.as_bool_str())?,
        );

        Ok(())
    }
}

/// Whether to use an incremental change feed
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum ChangeFeed {
    Incremental,
    None,
}

impl AddAsHeader for ChangeFeed {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            Self::Incremental => builder.header(headers::HEADER_A_IM, "Incremental feed"),
            Self::None => builder,
        }
    }

    fn add_as_header2(
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

impl AddAsHeader for TentativeWritesAllowance {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_ALLOW_MULTIPLE_WRITES, self.as_bool_str())
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            headers::HEADER_ALLOW_MULTIPLE_WRITES,
            http::header::HeaderValue::from_str(self.as_bool_str())?,
        );

        Ok(())
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

impl AddAsHeader for PartitionRangeId {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_DOCUMENTDB_PARTITIONRANGEID, &self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            headers::HEADER_DOCUMENTDB_PARTITIONRANGEID,
            http::header::HeaderValue::from_str(&self.0)?,
        );

        Ok(())
    }
}
