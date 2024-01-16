use super::PermissionToken;
use azure_core::auth::{Secret, TokenCredential};
use std::{fmt, sync::Arc};
use tracing::trace;

/// Authorization tokens for accessing Cosmos.
///
/// Learn more about the different types of tokens [here](https://docs.microsoft.com/azure/cosmos-db/secure-access-to-data).
#[derive(Clone)]
pub enum AuthorizationToken {
    /// Used for administrative resources: database accounts, databases, users, and permissions
    PrimaryKey(Secret),
    /// Used for application resources: containers, documents, attachments, stored procedures, triggers, and UDFs
    Resource(String),
    /// AAD token credential
    TokenCredential(Arc<dyn TokenCredential>),
}

impl AuthorizationToken {
    /// Create a primary `AuthorizationToken` from base64 encoded data
    ///
    /// The token is *not* verified to be valid.
    pub fn primary_key<S>(key: S) -> azure_core::Result<AuthorizationToken>
    where
        S: Into<String>,
    {
        Ok(AuthorizationToken::PrimaryKey(Secret::new(key.into())))
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
                AuthorizationToken::PrimaryKey(_) => "Master",
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
