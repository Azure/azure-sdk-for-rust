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
mod consistency_level;
mod errors;
pub(crate) mod from_headers;
mod headers;
mod indexing_directive;
pub mod offer;
mod partition_key_range;
mod partition_keys;
pub mod prelude;
mod query;
mod requests;
mod resource_quota;
pub mod resources;
pub mod responses;
mod to_json_vector;
mod traits;

pub use consistency_level::ConsistencyLevel;
pub use indexing_directive::IndexingDirective;
pub use offer::Offer;
pub use partition_key_range::PartitionKeyRange;
pub use partition_keys::PartitionKeys;
pub use query::{Param, ParamDef, Query};
pub use requests::*;
pub use resource_quota::ResourceQuota;
pub use traits::*;

type ReadonlyString = std::borrow::Cow<'static, str>;
pub type CosmosError = Box<dyn std::error::Error + Sync + Send>;

#[allow(dead_code)]
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
