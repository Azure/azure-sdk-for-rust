//! All the resources that can be interacted with in a Cosmos instance.
//!
//! You can learn about the Cosmos DB resource model [here](https://docs.microsoft.com/azure/cosmos-db/account-databases-containers-items).

pub mod collection;
pub mod document;
pub mod permission;
pub mod stored_procedure;
pub mod trigger;

mod attachment;
mod database;
mod user;
mod user_defined_function;

#[doc(inline)]
pub use attachment::Attachment;
#[doc(inline)]
pub use collection::Collection;
#[doc(inline)]
pub use database::Database;
#[doc(inline)]
pub use document::Document;
#[doc(inline)]
pub use permission::Permission;
#[doc(inline)]
pub use stored_procedure::StoredProcedure;
#[doc(inline)]
pub use trigger::Trigger;
#[doc(inline)]
pub use user::User;
#[doc(inline)]
pub use user_defined_function::UserDefinedFunction;

use permission::PermissionMode;

/// A Cosmos resource such as databases, documents, collections, users, etc.
pub trait Resource {
    /// Get the uri for a resource
    fn uri(&self) -> &str;

    /// Get the read permissions for the resource
    fn read_permission(&self) -> PermissionMode<'_> {
        PermissionMode::read(self)
    }

    /// Get all permissions for the resource
    fn all_permission(&self) -> PermissionMode<'_> {
        PermissionMode::all(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResourceType {
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
