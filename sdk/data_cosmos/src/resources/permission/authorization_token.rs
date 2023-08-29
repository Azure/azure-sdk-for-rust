use super::PermissionToken;
use azure_core::auth::TokenCredential;
use azure_core::{
    base64,
    error::{Error, ErrorKind},
};
use std::fmt;
use std::sync::Arc;

/// Authorization tokens for accessing Cosmos.
///
/// Learn more about the different types of tokens [here](https://docs.microsoft.com/azure/cosmos-db/secure-access-to-data).
#[derive(Clone)]
pub enum AuthorizationToken {
    /// Used for administrative resources: database accounts, databases, users, and permissions
    Primary(Vec<u8>),
    /// Used for application resources: containers, documents, attachments, stored procedures, triggers, and UDFs
    Resource(String),
    /// AAD token credential
    TokenCredential(Arc<dyn TokenCredential>),
}

impl AuthorizationToken {
    /// Create a primary `AuthorizationToken` from base64 encoded data
    ///
    /// The token is *not* verified to be valid.
    pub fn primary_from_base64(base64_encoded: &str) -> azure_core::Result<AuthorizationToken> {
        let key = base64::decode(base64_encoded).map_err(|e|
            Error::full(ErrorKind::Credential, e, "failed to base64 decode the primary credential - ensure that the credential is properly base64 encoded")
        )?;

        Ok(AuthorizationToken::Primary(key))
    }

    /// Create a resource `AuthorizationToken` for the given resource.
    pub fn new_resource(resource: String) -> AuthorizationToken {
        AuthorizationToken::Resource(resource)
    }

    /// Create an `AuthorizationToken` from a `TokenCredential`.
    pub fn from_token_credential(token_credential: Arc<dyn TokenCredential>) -> AuthorizationToken {
        AuthorizationToken::TokenCredential(token_credential)
    }
}

impl fmt::Debug for AuthorizationToken {
    // We provide a custom implementation to hide the key value.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AuthorizationToken::{}(***hidden***)",
            match self {
                AuthorizationToken::Primary(_) => "Master",
                AuthorizationToken::Resource(_) => "Resource",
                AuthorizationToken::TokenCredential(_) => "TokenCredential",
            }
        )
    }
}

impl std::convert::From<PermissionToken> for AuthorizationToken {
    fn from(permission_token: PermissionToken) -> Self {
        trace!(
            "Converting permission_token into AuthorizationToken: {:#?}",
            permission_token
        );
        permission_token.token
    }
}
