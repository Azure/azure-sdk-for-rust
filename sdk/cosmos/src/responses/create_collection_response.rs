use crate::collection::Collection;
use crate::from_headers::*;
use crate::CosmosError;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use chrono::{DateTime, Utc};
use http::response::Response;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateCollectionResponse {
    pub collection: Collection,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub last_state_change: DateTime<Utc>,
    pub schema_version: String,
    pub service_version: String,
    pub gateway_version: String,
    pub alt_content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
}

impl std::convert::TryFrom<Response<Vec<u8>>> for CreateCollectionResponse {
    type Error = CosmosError;

    fn try_from(response: Response<Vec<u8>>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = response.body();

        Ok(Self {
            collection: serde_json::from_slice(body)?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            last_state_change: last_state_change_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            service_version: service_version_from_headers(headers)?.to_owned(),
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers(headers)?,
            current_write_quorum: current_write_quorum_from_headers(headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(headers)?,
        })
    }
}
