//! Utilities for interacting with Cosmos DB's permissions system.
//!
//! You can learn more about how the system works [here](https://docs.microsoft.com/rest/api/cosmos-db/permissions).
mod authorization_token;
mod permission;
mod permission_token;

pub use authorization_token::AuthorizationToken;
pub use authorization_token::AuthorizationTokenParsingError;
pub use permission::{Permission, PermissionMode};
pub use permission_token::PermissionToken;
pub use permission_token::PermissionTokenParsingError;

use crate::headers;
use azure_core::AddAsHeader;
use http::request::Builder;

/// The amount of time before authorization expires
#[derive(Debug, Clone, Copy)]
pub struct ExpirySeconds(u64);

impl ExpirySeconds {
    /// Create an `ExpirySeconds` from a `u64`
    pub fn new(secs: u64) -> Self {
        Self(secs)
    }
}

impl AddAsHeader for ExpirySeconds {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_DOCUMENTDB_EXPIRY_SECONDS, self.0)
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        request.headers_mut().append(
            headers::HEADER_DOCUMENTDB_EXPIRY_SECONDS,
            http::header::HeaderValue::from(self.0),
        );
        Ok(())
    }
}
