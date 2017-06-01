use base64;

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    Master,
    Resource,
}

pub struct AuthorizationToken {
    token_type: TokenType,
    base64_encoded: String,
    binary_form: Vec<u8>,
}

impl AuthorizationToken {
    pub fn new(token_type: TokenType,
               base64_encoded: String)
               -> Result<AuthorizationToken, base64::DecodeError> {
        let mut v_hmac_key: Vec<u8> = Vec::new();

        v_hmac_key.extend(base64::decode(&base64_encoded)?);

        Ok(AuthorizationToken {
               token_type: token_type,
               base64_encoded: base64_encoded,
               binary_form: v_hmac_key,
           })
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
