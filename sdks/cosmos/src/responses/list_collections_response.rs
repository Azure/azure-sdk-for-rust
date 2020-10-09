use crate::collection::Collection;
use crate::from_headers::*;
use crate::ResourceQuota;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{continuation_token_from_headers_optional, session_token_from_headers};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ListCollectionsResponse {
    pub rid: String,
    pub collections: Vec<Collection>,
    pub count: u32,
    pub last_state_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub gateway_version: String,
    pub continuation_token: Option<String>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for ListCollectionsResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        #[derive(Deserialize, Debug)]
        pub struct Response {
            _rid: String,
            #[serde(rename = "DocumentCollections")]
            pub collections: Vec<Collection>,
            #[serde(rename = "_count")]
            pub count: u32,
        }

        let response: Response = serde_json::from_slice(body)?;

        Ok(Self {
            rid: response._rid,
            collections: response.collections,
            count: response.count,
            last_state_change: last_state_change_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            continuation_token: continuation_token_from_headers_optional(headers)?,
        })
    }
}
