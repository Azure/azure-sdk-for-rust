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

pub trait IsUpsertSupport {
    type O;
    fn with_is_upsert(self, is_upsert: bool) -> Self::O;
}

pub trait IsUpsertOption {
    fn is_upsert(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_IS_UPSERT,
            self.is_upsert().to_string(),
        )
    }
}

pub trait AIMSupport {
    type O;
    fn with_a_im(self, a_im: bool) -> Self::O;
}

pub trait AIMOption {
    fn a_im(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if self.a_im() {
            builder.header(headers::HEADER_A_IM, "Incremental feed")
        } else {
            builder
        }
    }
}

pub trait AllowTentativeWritesSupport {
    type O;
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O;
}

pub trait AllowTentativeWritesOption {
    fn allow_tentative_writes(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_ALLOW_MULTIPLE_WRITES,
            self.allow_tentative_writes().to_string(),
        )
    }
}

pub trait PartitionRangeIdSupport<'a> {
    type O;
    fn with_partition_range_id(self, partition_range_id: &'a str) -> Self::O;
}

pub trait PartitionRangeIdOption<'a> {
    fn partition_range_id(&self) -> Option<&'a str>;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if let Some(partition_range_id) = self.partition_range_id() {
            builder.header(
                headers::HEADER_DOCUMENTDB_PARTITIONRANGEID,
                partition_range_id,
            )
        } else {
            builder
        }
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
