//! Utilities for interacting with Cosmos DB's permissions system.
//!
//! You can learn more about how the system works [here](https://docs.microsoft.com/rest/api/cosmos-db/permissions).
mod authorization_token;
mod permission;
mod permission_response;
mod permission_token;

pub use authorization_token::AuthorizationToken;
pub use permission::{Permission, PermissionMode};
pub(crate) use permission_response::PermissionResponse;
pub use permission_token::PermissionToken;
pub use permission_token::PermissionTokenParseError;

use crate::headers;
use azure_core::Header;

/// The amount of time before authorization expires
#[derive(Debug, Clone, Copy)]
pub struct ExpirySeconds(u64);

impl ExpirySeconds {
    /// Create an `ExpirySeconds` from a `u64`
    pub fn new(secs: u64) -> Self {
        Self(secs)
    }
}

impl Header for ExpirySeconds {
    fn name(&self) -> azure_core::headers::HeaderName {
        headers::HEADER_DOCUMENTDB_EXPIRY_SECONDS
    }

    fn value(&self) -> azure_core::headers::HeaderValue {
        self.0.to_string().into()
    }
}
