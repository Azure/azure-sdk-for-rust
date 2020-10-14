use crate::from_headers::*;
use crate::stored_procedure::StoredProcedure;
use crate::ResourceQuota;
use azure_core::errors::AzureError;
use azure_core::etag_from_headers;
use azure_core::session_token_from_headers;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateStoredProcedureResponse {
    pub stored_procedure: StoredProcedure,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub last_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for CreateStoredProcedureResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        Ok(Self {
            stored_procedure: serde_json::from_slice(body)?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            last_change: last_state_change_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(headers)?,
            current_write_quorum: current_write_quorum_from_headers(headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(headers)?,
        })
    }
}
