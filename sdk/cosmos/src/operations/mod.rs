//! TODO: Documentation

#![allow(missing_docs)]

mod create_collection;
mod create_database;
mod create_document;
mod create_permission;
mod create_user;
mod delete_collection;
mod delete_database;
mod delete_document;
mod delete_permission;
mod delete_user;
mod get_collection;
mod get_database;
mod get_document;
mod get_permission;
mod get_user;
mod list_collections;
mod list_databases;
mod list_users;
mod replace_collection;
mod replace_document;
mod replace_permission;
mod replace_user;

pub use create_collection::*;
pub use create_database::*;
pub use create_document::*;
pub use create_permission::*;
pub use create_user::*;
pub use delete_collection::*;
pub use delete_database::*;
pub use delete_document::*;
pub use delete_permission::*;
pub use delete_user::*;
pub use get_collection::*;
pub use get_database::*;
pub use get_document::*;
pub use get_permission::*;
pub use get_user::*;
pub use list_collections::*;
pub use list_databases::*;
pub use list_users::*;
pub use replace_collection::*;
pub use replace_document::*;
pub use replace_permission::*;
pub use replace_user::*;
