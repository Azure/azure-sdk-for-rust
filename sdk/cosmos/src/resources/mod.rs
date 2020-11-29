pub mod attachment;
pub mod collection;
pub mod database;
pub mod document;
pub mod permission;
pub mod stored_procedure;
pub mod trigger;
pub mod user;
pub mod user_defined_function;

pub use attachment::Attachment;
pub use collection::Collection;
pub use database::Database;
pub use document::Document;
pub use permission::Permission;
pub use stored_procedure::StoredProcedure;
pub use trigger::Trigger;
pub use user::User;
pub use user_defined_function::UserDefinedFunction;

use permission::PermissionMode;

/// A Cosmos resource such as databases, documents, collections, users, etc.
pub trait Resource {
    fn uri(&self) -> &str;

    fn read_permission(&self) -> PermissionMode<'_> {
        PermissionMode::read(self)
    }

    fn all_permission(&self) -> PermissionMode<'_> {
        PermissionMode::all(self)
    }
}
