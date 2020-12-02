//! Utilities for interacting with Cosmos DB's permissions system.
//!
//! You can learn more about how the system works [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/permissions).
mod authorization_token;
mod permission;
mod permission_token;

pub use authorization_token::AuthorizationToken;
pub use permission::{Permission, PermissionMode};
pub use permission_token::PermissionToken;

use crate::headers;
use azure_core::AddAsHeader;
use http::request::Builder;

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
