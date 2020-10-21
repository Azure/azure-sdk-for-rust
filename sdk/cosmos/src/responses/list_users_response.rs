use crate::from_headers::*;
use crate::User;
use azure_core::errors::AzureError;
use azure_core::headers::{continuation_token_from_headers_optional, session_token_from_headers};
use azure_core::SessionToken;
use http::HeaderMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ListUsersResponse {
    #[serde(rename = "Users")]
    pub users: Vec<User>,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,

    #[serde(skip_deserializing)]
    pub charge: f64,
    #[serde(skip_deserializing)]
    pub activity_id: uuid::Uuid,
    #[serde(skip_deserializing)]
    pub session_token: SessionToken,
    #[serde(skip_deserializing)]
    pub continuation_token: Option<String>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for ListUsersResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        let mut list_users_response: ListUsersResponse = serde_json::from_slice(body)?;
        list_users_response.charge = request_charge_from_headers(headers)?;
        list_users_response.activity_id = activity_id_from_headers(headers)?;
        list_users_response.continuation_token = continuation_token_from_headers_optional(headers)?;
        list_users_response.session_token = session_token_from_headers(headers)?;

        Ok(list_users_response)
    }
}
