use crate::from_headers::*;
use crate::stored_procedure::StoredProcedure;
use crate::ResourceQuota;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{continuation_token_from_headers_optional, session_token_from_headers};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ListStoredProceduresResponse {
    pub stored_procedures: Vec<StoredProcedure>,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub last_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub gateway_version: String,
    pub continuation_token: Option<String>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for ListStoredProceduresResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Response {
            pub _rid: String,
            #[serde(rename = "StoredProcedures")]
            pub stored_procedures: Vec<StoredProcedure>,
            pub _count: u64,
        }

        Ok(Self {
            stored_procedures: serde_json::from_slice::<Response>(body)?.stored_procedures,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            last_change: last_state_change_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            continuation_token: continuation_token_from_headers_optional(headers)?,
        })
    }
}
