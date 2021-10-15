//! TODO: Documentation

#![allow(missing_docs)]

mod create_collection;
mod create_database;
mod create_document;
mod create_permission;
mod create_user;
mod delete_permission;
mod delete_user;
mod get_database;
mod get_document;
mod get_permission;
mod get_user;
mod list_databases;
mod list_users;
mod replace_permission;
mod replace_user;

pub use create_collection::*;
pub use create_database::*;
pub use create_document::*;
pub use create_permission::*;
pub use create_user::*;
pub use delete_permission::*;
pub use delete_user::*;
pub use get_database::*;
pub use get_document::*;
pub use get_permission::*;
pub use get_user::*;
pub use list_databases::*;
pub use list_users::*;
pub use replace_permission::*;
pub use replace_user::*;
