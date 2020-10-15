use crate::traits::{BearerToken, ExtExpiresIn, RefreshToken};
use oauth2::AccessToken;
use serde::{Deserialize, Deserializer};

use std::convert::TryInto;

/// A refresh token
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenResponse {
    token_type: String,
    #[serde(deserialize_with = "split")]
    scopes: Vec<String>,
    expires_in: u64,
    ext_expires_in: u64,
    access_token: AccessToken,
    refresh_token: AccessToken,
}

fn split<'de, D>(scope: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = serde::Deserialize::deserialize(scope)?;
    Ok(string.split(' ').map(|s| s.to_owned()).collect())
}

impl TryInto<RefreshTokenResponse> for String {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<RefreshTokenResponse, Self::Error> {
        serde_json::from_str::<RefreshTokenResponse>(&self)
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
