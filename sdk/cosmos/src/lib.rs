#![warn(unused_extern_crates)]
#![recursion_limit = "128"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate azure_core;

pub mod clients;
pub mod from_headers;
pub mod prelude;
pub mod resources;
pub mod responses;

mod consistency_level;
mod errors;
mod headers;
mod indexing_directive;
mod offer;
mod partition_key_range;
mod partition_keys;
mod query;
mod requests;
mod resource_quota;
mod to_json_vector;
mod traits;

pub use consistency_level::ConsistencyLevel;
pub use indexing_directive::IndexingDirective;
pub use offer::Offer;
pub use partition_key_range::PartitionKeyRange;
pub use partition_keys::PartitionKeys;
pub use query::{Param, ParamDef, Query};
pub use resource_quota::ResourceQuota;

use http::request::Builder;

type ReadonlyString = std::borrow::Cow<'static, str>;
pub type CosmosError = Box<dyn std::error::Error + Sync + Send>;

pub(crate) fn add_partition_keys_header(
    partition_keys: &PartitionKeys,
    builder: Builder,
) -> Builder {
    let serialized = partition_keys.to_json();
    builder.header(headers::HEADER_DOCUMENTDB_PARTITIONKEY, serialized)
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Databases,
    Collections,
    Documents,
    StoredProcedures,
    Users,
    Permissions,
    Attachments,
    PartitionKeyRanges,
    UserDefinedFunctions,
    Triggers,
}
