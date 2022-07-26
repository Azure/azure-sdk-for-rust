//! Clients for interacting with Cosmos resources.
//!
//! Each resource has its own client, meaning if you want to interact with Attachments, for example,
//! you need to use the [`AttachmentClient`].
//!
//! The clients form a tree with the `CosmosClient` at the top. The `CosmosClient` can create
//! `DatabaseClients` which can in turn create `CollectionClients`.
//!
//! # Example
//!
//! ```no_run
//! use azure_data_cosmos::prelude::*;
//!
//! let account: String = todo!("Get Cosmos account name from the Azure Portal");
//! let primary_key: String = todo!("Get Cosmos primary key from the Azure Portal");
//! let authorization_token = AuthorizationToken::primary_from_base64(&primary_key).unwrap();
//! let database_name: String = todo!("Think of some database name");
//!
//! // Create an http client, then a `CosmosClient`, and then a `DatabaseClient`
//! let client = CosmosClient::new(account, authorization_token);
//! let client = client.database_client(database_name);
//! ```

mod attachment;
mod collection;
mod cosmos;
mod database;
mod document;
mod permission;
mod stored_procedure;
mod trigger;
mod user;
mod user_defined_function;

pub use attachment::AttachmentClient;
pub use collection::CollectionClient;
pub use cosmos::{CosmosClient, CosmosClientBuilder};
pub use database::DatabaseClient;
pub use document::DocumentClient;
pub use permission::PermissionClient;
pub use stored_procedure::StoredProcedureClient;
pub use trigger::TriggerClient;
pub use user::UserClient;
pub use user_defined_function::UserDefinedFunctionClient;
