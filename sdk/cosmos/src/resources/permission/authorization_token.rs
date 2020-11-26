use super::PermissionToken;
use std::fmt;

/// Authorization tokens for accessing Cosmos.
///
/// Learn more about the different types of tokens [here](https://docs.microsoft.com/en-us/azure/cosmos-db/secure-access-to-data).
#[derive(PartialEq, Clone)]
pub enum AuthorizationToken {
    Primary(Vec<u8>),
    Resource(String),
}

impl AuthorizationToken {
    /// Create a primary `AuthorizationToken` from base64 encoded data
    ///
    /// The token is *not* verified to be valid.
    pub fn primary_from_base64(
        base64_encoded: &str,
    ) -> Result<AuthorizationToken, base64::DecodeError> {
        let key = base64::decode(base64_encoded)?;
        Ok(AuthorizationToken::Primary(key))
    }

    /// Create a resource `AuthorizationToken` for the given resource.
    pub fn new_resource(resource: String) -> AuthorizationToken {
        AuthorizationToken::Resource(resource)
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
