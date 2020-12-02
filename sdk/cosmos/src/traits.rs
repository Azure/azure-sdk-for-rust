use crate::resources::*;
use crate::{headers, PartitionKeys};
use azure_core::AddAsHeader;
use document::IndexingDirective;
use http::request::Builder;

pub trait QueryCrossPartitionSupport {
    type O;
    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O;
}

pub trait QueryCrossPartitionOption {
    fn query_cross_partition(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION,
            self.query_cross_partition().to_string(),
        )
    }
}

pub trait ParallelizeCrossPartitionQuerySupport {
    type O;
    fn with_parallelize_cross_partition_query(
        self,
        parallelize_cross_partition_query: bool,
    ) -> Self::O;
}

pub trait ParallelizeCrossPartitionQueryOption {
    fn parallelize_cross_partition_query(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY,
            self.parallelize_cross_partition_query().to_string(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
}

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
}

#[derive(Debug, Clone, Copy)]
pub enum TenativeWritesAllowance {
    Allow,
    Deny,
}

impl TenativeWritesAllowance {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Allow => "true",
            Self::Deny => "false",
        }
    }
}

impl AddAsHeader for TenativeWritesAllowance {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_ALLOW_MULTIPLE_WRITES, self.as_bool_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PartitionRangeId<'a>(&'a str);

impl<'a> PartitionRangeId<'a> {
    pub fn new(id: &'a str) -> Self {
        Self(id)
    }
}

impl AddAsHeader for PartitionRangeId<'_> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_DOCUMENTDB_PARTITIONRANGEID, self.0)
    }
}

impl azure_core::AddAsHeader for IndexingDirective {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            IndexingDirective::Default => builder,
            IndexingDirective::Exclude => {
                builder.header(headers::HEADER_INDEXING_DIRECTIVE, "Exclude")
            }
            IndexingDirective::Include => {
                builder.header(headers::HEADER_INDEXING_DIRECTIVE, "Include")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaxItemCount(i32);

impl MaxItemCount {
    pub fn new(count: i32) -> Self {
        Self(count)
    }
}

impl AddAsHeader for MaxItemCount {
    fn add_as_header(&self, builder: Builder) -> Builder {
        if self.0 <= 0 {
            builder.header(headers::HEADER_MAX_ITEM_COUNT, -1)
        } else {
            builder.header(headers::HEADER_MAX_ITEM_COUNT, self.0)
        }
    }
}

impl AddAsHeader for &'_ PartitionKeys {
    fn add_as_header(&self, builder: Builder) -> Builder {
        headers::add_partition_keys_header(self, builder)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExpirySeconds(u64);

impl ExpirySeconds {
    pub fn new(secs: u64) -> Self {
        Self(secs)
    }
}

impl AddAsHeader for ExpirySeconds {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_DOCUMENTDB_EXPIRY_SECONDS, self.0)
    }
}
