//! Utilities for interacting with Cosmos DB's permissions system.
//!
//! You can learn more about how the system works [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/permissions).
mod authorization_token;
mod permission;
mod permission_token;

pub use authorization_token::AuthorizationToken;
pub use permission::{Permission, PermissionMode};
pub use permission_token::PermissionToken;
