use crate::prelude::*;
use oauth2::AccessToken;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct RefreshTokenResponse {
    token_type: String,
    scopes: Vec<String>,
    expires_in: u64,
    ext_expires_in: u64,
    access_token: AccessToken,
    refresh_token: AccessToken,
}

impl TryInto<RefreshTokenResponse> for String {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<RefreshTokenResponse, Self::Error> {
        // we use a temp struct to deserialize the scope into
        // the scopes vec at later time
        #[derive(Debug, Clone, Deserialize)]
        pub struct _RefreshTokenResponse<'a> {
            token_type: String,
            scope: &'a str,
            expires_in: u64,
            ext_expires_in: u64,
            access_token: AccessToken,
            refresh_token: AccessToken,
        }

        serde_json::from_str::<_RefreshTokenResponse>(&self).map(|rtr| RefreshTokenResponse {
            token_type: rtr.token_type,
            scopes: rtr.scope.split(' ').map(|s| s.to_owned()).collect(),
            expires_in: rtr.expires_in,
            ext_expires_in: rtr.ext_expires_in,
            access_token: rtr.access_token,
            refresh_token: rtr.refresh_token,
        })
    }
}

impl BearerToken for RefreshTokenResponse {
    fn token_type(&self) -> &str {
        &self.token_type
    }
    fn scopes(&self) -> &[String] {
        &self.scopes
    }
    fn expires_in(&self) -> u64 {
        self.expires_in
    }
    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
}

impl RefreshToken for RefreshTokenResponse {
    fn refresh_token(&self) -> &AccessToken {
        &self.refresh_token
    }
}

impl ExtExpiresIn for RefreshTokenResponse {
    fn ext_expires_in(&self) -> u64 {
        self.ext_expires_in
    }
}
