use base64;
use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    Master,
    Resource,
}

#[derive(Clone)]
pub struct AuthorizationToken {
    account: String,
    token_type: TokenType,
    key: Vec<u8>,
}

impl AuthorizationToken {
    pub fn new(
        account: String,
        token_type: TokenType,
        base64_encoded: &str,
    ) -> Result<AuthorizationToken, base64::DecodeError> {
        let key = base64::decode(&base64_encoded)?;
        Ok(AuthorizationToken {
            account,
            token_type,
            key,
        })
    }

    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn key(&self) -> &[u8] {
        &self.key
    }
}

impl Debug for AuthorizationToken {
    //! We provide a custom implementation to hide the key value.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "AuthorizationToken(account == {}, token_type == {:?})",
            self.account, self.token_type
        )
    }
}
