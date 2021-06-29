use super::PermissionToken;
use std::fmt;

/// Authorization tokens for accessing Cosmos.
///
/// Learn more about the different types of tokens [here](https://docs.microsoft.com/azure/cosmos-db/secure-access-to-data).
#[derive(PartialEq, Clone)]
pub enum AuthorizationToken {
    /// Used for administrative resources: database accounts, databases, users, and permissions
    Primary(Vec<u8>),
    /// Used for application resources: containers, documents, attachments, stored procedures, triggers, and UDFs
    Resource(String),
}

impl AuthorizationToken {
    /// Create a primary `AuthorizationToken` from base64 encoded data
    ///
    /// The token is *not* verified to be valid.
    pub fn primary_from_base64(
        base64_encoded: &str,
    ) -> Result<AuthorizationToken, AuthorizationTokenParsingError> {
        let key = base64::decode(base64_encoded)
            .map_err(AuthorizationTokenParsingError::InvalidBase64Encoding)?;
        Ok(AuthorizationToken::Primary(key))
    }

    /// Create a resource `AuthorizationToken` for the given resource.
    pub fn new_resource(resource: String) -> AuthorizationToken {
        AuthorizationToken::Resource(resource)
    }
}

#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum AuthorizationTokenParsingError {
    #[error("the authorization token was not properly base64 encoded: {0}")]
    InvalidBase64Encoding(#[from] base64::DecodeError),
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
