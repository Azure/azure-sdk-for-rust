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
    base64_encoded: String,
    binary_form: Vec<u8>,
}

impl AuthorizationToken {
    pub fn new(
        account: String,
        token_type: TokenType,
        base64_encoded: String,
    ) -> Result<AuthorizationToken, base64::DecodeError> {
        let mut v_hmac_key: Vec<u8> = Vec::new();

        v_hmac_key.extend(base64::decode(&base64_encoded)?);

        Ok(AuthorizationToken {
            account,
            token_type,
            base64_encoded,
            binary_form: v_hmac_key,
        })
    }

    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
    pub fn base64_encoded(&self) -> &str {
        &self.base64_encoded
    }
    pub fn binary_form(&self) -> &[u8] {
        &self.binary_form
    }
}

impl Debug for AuthorizationToken {
    //! We provide a custom implementation to hide some of the chars
    //! since they are security sentive.
    //! We show only the 6 first chars of ```base64_encoded``` form and only
    //! the binary vector length.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut obfuscated = Vec::new();

        for (idx, ch) in self.base64_encoded.chars().enumerate() {
            let ch_obfuscated = if idx < 6 { ch } else { '*' };
            obfuscated.push(ch_obfuscated);
        }

        let so = obfuscated.into_iter().collect::<String>();

        write!(
            f,
            "azure::core::cosmos::AuthorizationToken[\
             account == {}, token_type == {:?}, \
             base64_encoded == {}, binary_form.len() == {}]",
            self.account,
            self.token_type,
            so,
            self.binary_form.len()
        )
    }
}
