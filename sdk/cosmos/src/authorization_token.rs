use crate::PermissionToken;
use std::fmt::{Debug, Error, Formatter};

#[derive(PartialEq, Clone)]
pub enum AuthorizationToken {
    Master(Vec<u8>),
    Resource(String),
}

impl AuthorizationToken {
    pub fn new_master(base64_encoded: &str) -> Result<AuthorizationToken, base64::DecodeError> {
        let key = base64::decode(&base64_encoded)?;
        Ok(AuthorizationToken::Master(key))
    }

    pub fn new_resource(resource: String) -> AuthorizationToken {
        AuthorizationToken::Resource(resource)
    }
}

impl Debug for AuthorizationToken {
    //! We provide a custom implementation to hide the key value.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                AuthorizationToken::Master(_) => "AuthorizationToken::Master(***hidden***)",
                AuthorizationToken::Resource(_) => "AuthorizationToken::Resource(***hidden***)",
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
        Self::new_resource(permission_token.signature)
    }
}
