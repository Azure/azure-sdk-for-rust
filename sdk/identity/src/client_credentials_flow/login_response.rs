use chrono::{DateTime, TimeZone, Utc};
use oauth2::AccessToken;
use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
struct _LoginResponse {
    token_type: String,
    expires_in: u64,
    ext_expires_in: u64,
    expires_on: Option<String>,
    not_before: Option<String>,
    resource: Option<String>,
    access_token: String,
}

#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub token_type: String,
    pub expires_in: u64,
    pub ext_expires_in: u64,
    pub expires_on: Option<DateTime<Utc>>,
    pub not_before: Option<DateTime<Utc>>,
    pub resource: Option<String>,
    pub access_token: AccessToken,
}

impl<'de> Deserialize<'de> for LoginResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let resp = _LoginResponse::deserialize(deserializer)?;
        LoginResponse::from_base_response(resp).map_err(de::Error::custom)
    }
}

impl LoginResponse {
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    fn from_base_response(r: _LoginResponse) -> Result<LoginResponse, std::num::ParseIntError> {
        let expires_on: Option<DateTime<Utc>> = match r.expires_on {
            Some(d) => Some(Utc.timestamp(d.parse()?, 0)),
            None => None,
        };
        let not_before: Option<DateTime<Utc>> = match r.not_before {
            Some(d) => Some(Utc.timestamp(d.parse()?, 0)),
            None => None,
        };

        Ok(LoginResponse {
            token_type: r.token_type,
            expires_in: r.expires_in,
            ext_expires_in: r.ext_expires_in,
            expires_on,
            not_before,
            resource: r.resource,
            access_token: AccessToken::new(r.access_token),
        })
    }
}
