#![warn(unused_extern_crates)]
#![recursion_limit = "128"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate azure_core;

pub mod attachment;
mod authorization_token;
pub mod clients;
pub mod collection;
mod consistency_level;
mod database;
mod document;
mod document_attributes;
mod errors;
pub(crate) mod from_headers;
mod headers;
mod indexing_directive;
pub mod offer;
mod partition_key_range;
mod partition_keys;
mod permission;
mod permission_resource;
mod permission_token;
pub mod prelude;
mod query;
mod requests;
mod resource;
mod resource_quota;
pub mod responses;
pub mod stored_procedure;
mod to_json_vector;
mod traits;
pub mod trigger;
mod user;
mod user_defined_function;

pub use attachment::Attachment;
pub use authorization_token::*;
pub use consistency_level::ConsistencyLevel;
pub use database::{Database, DatabaseName};
pub use document::{Document, DocumentName};
pub use document_attributes::DocumentAttributes;
pub use indexing_directive::IndexingDirective;
pub use offer::Offer;
pub use partition_key_range::PartitionKeyRange;
pub use partition_keys::PartitionKeys;
pub use permission::{Permission, PermissionMode, PermissionName};
pub use permission_resource::PermissionResource;
pub use permission_token::PermissionToken;
pub use query::{Param, ParamDef, Query};
pub use requests::*;
pub use resource::Resource;
pub use resource_quota::ResourceQuota;
pub use traits::*;
pub use trigger::{Trigger, TriggerName};
pub use user::{User, UserName};
pub use user_defined_function::UserDefinedFunctionName;

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
