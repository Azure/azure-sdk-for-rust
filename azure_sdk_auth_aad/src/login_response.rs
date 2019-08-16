use azure_sdk_core::errors::AzureError;
use chrono::{DateTime, TimeZone, Utc};
use oauth2::AccessToken;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
struct _LoginResponse {
    token_type: String,
    expires_in: String,
    ext_expires_in: String,
    expires_on: String,
    not_before: String,
    resource: String,
    access_token: String,
}

#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub token_type: String,
    pub expires_in: u64,
    pub ext_expires_in: u64,
    pub expires_on: DateTime<Utc>,
    pub not_before: DateTime<Utc>,
    pub resource: String,
    pub access_token: AccessToken,
}

impl FromStr for LoginResponse {
    type Err = AzureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: _LoginResponse = serde_json::from_str(s)?;

        let expires_on: i64 = r.expires_on.parse()?;
        let expires_on: DateTime<Utc> = Utc.timestamp(expires_on, 0);

        let not_before: i64 = r.not_before.parse()?;
        let not_before: DateTime<Utc> = Utc.timestamp(not_before, 0);

        Ok(LoginResponse {
            token_type: r.token_type,
            expires_in: r.expires_in.parse()?,
            ext_expires_in: r.ext_expires_in.parse()?,
            expires_on,
            not_before,
            resource: r.resource,
            access_token: AccessToken::new(r.access_token),
        })
    }
}

impl LoginResponse {
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
}
