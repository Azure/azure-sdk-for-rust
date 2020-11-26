use crate::from_headers::*;
use crate::resources::document::DocumentAttributes;
use crate::{CosmosError, ResourceQuota};
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use chrono::{DateTime, Utc};
use http::response::Response;
use http::StatusCode;

#[derive(Debug, Clone)]
pub struct CreateDocumentResponse {
    pub document_attributes: DocumentAttributes,
    pub is_update: bool,
    pub last_state_change: DateTime<Utc>,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
    pub session_token: String,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl std::convert::TryFrom<Response<Vec<u8>>> for CreateDocumentResponse {
    type Error = CosmosError;

    fn try_from(response: Response<Vec<u8>>) -> Result<Self, Self::Error> {
        let status_code = response.status();
        let headers = response.headers();
        let body = response.body();

        debug!("status_code == {:#?}", status_code);
        debug!("headers == {:#?}", headers);
        debug!("body == {:#?}", std::str::from_utf8(body));

        Ok(CreateDocumentResponse {
            is_update: status_code == StatusCode::OK,

            last_state_change: last_state_change_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers(headers)?,
            current_write_quorum: current_write_quorum_from_headers(headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(headers)?,
            role: role_from_headers(headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,

            document_attributes: DocumentAttributes::try_from(response)?,
        })
    }
}
