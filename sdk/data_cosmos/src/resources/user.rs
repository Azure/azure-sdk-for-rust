//! Utilities for interacting with [`User`]s.

use super::Resource;
use crate::headers::from_headers::*;
use azure_core::{
    headers::{etag_from_headers, session_token_from_headers},
    Response as HttpResponse,
};
/// A logical namespace for scoping permissions on resources.
///
/// You can learn more about users [here](https://docs.microsoft.com/rest/api/cosmos-db/users).
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct User {
    /// The user's id
    pub id: String,
    /// The resource id for a user
    #[serde(skip_serializing)]
    #[serde(rename = "_rid")]
    pub rid: String,
    /// The last updated timestamp
    #[serde(skip_serializing)]
    #[serde(rename = "_ts")]
    pub ts: u64,
    /// The url for this user resource
    #[serde(skip_serializing)]
    #[serde(rename = "_self")]
    pub _self: String,
    /// The user's etag used for concurrency control
    #[serde(skip_serializing)]
    #[serde(rename = "_etag")]
    pub etag: String,
    /// The user's permissions
    #[serde(skip_serializing)]
    #[serde(rename = "_permissions")]
    pub permissions: String,
}

impl std::convert::TryFrom<&[u8]> for User {
    type Error = serde_json::Error;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(body)
    }
}

impl Resource for User {
    fn uri(&self) -> &str {
        &self._self
    }
}

/// The Cosmos user response
#[derive(Debug, Clone)]
pub struct UserResponse {
    /// The Cosmos user
    pub user: User,
    /// The charge for this request from the Cosmos service
    pub charge: f64,
    /// Represents a unique identifier for the operation
    pub activity_id: uuid::Uuid,
    /// The etag for the resource retrieved
    pub etag: String,
    /// The session token for the request
    pub session_token: String,
}

impl UserResponse {
    /// Creates a UserResponse from an HttpResponse
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            user: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
        })
    }
}
