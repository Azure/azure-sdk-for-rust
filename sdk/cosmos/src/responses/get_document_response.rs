use crate::headers::from_headers::*;
use crate::resources::Document;
use crate::CosmosError;
use crate::ResourceQuota;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::SessionToken;
use chrono::{DateTime, Utc};
use http::response::Response;
use http::StatusCode;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub enum GetDocumentResponse<T> {
    Found(Box<FoundDocumentResponse<T>>),
    NotFound(Box<NotFoundDocumentResponse>),
}

impl<T> std::convert::TryFrom<Response<bytes::Bytes>> for GetDocumentResponse<T>
where
    T: DeserializeOwned,
{
    type Error = CosmosError;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let status_code = response.status();
        let headers = response.headers();
        let body: &[u8] = response.body();

        debug!("status_code == {:?}", status_code);
        debug!("headers == {:#?}", headers);
        debug!("body == {:#?}", std::str::from_utf8(body)?);

        let has_been_found =
            status_code == StatusCode::OK || status_code == StatusCode::NOT_MODIFIED;

        if has_been_found {
            Ok(GetDocumentResponse::Found(Box::new(
                FoundDocumentResponse::try_from(response)?,
            )))
        } else {
            Ok(GetDocumentResponse::NotFound(Box::new(
                NotFoundDocumentResponse::try_from(response)?,
            )))
        }
    }
}

#[derive(Debug, Clone)]
pub struct FoundDocumentResponse<T> {
    pub document: Document<T>,
    pub content_location: String,
    pub last_state_change: DateTime<Utc>,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub item_lsn: u64,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_item_llsn: u64,
    pub session_token: SessionToken,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl<T> std::convert::TryFrom<Response<bytes::Bytes>> for FoundDocumentResponse<T>
where
    T: DeserializeOwned,
{
    type Error = CosmosError;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body: &[u8] = response.body();

        Ok(Self {
            document: Document::try_from((headers, body))?,

            content_location: content_location_from_headers(headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            role: role_from_headers(headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            item_lsn: item_lsn_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_item_llsn: cosmos_item_llsn_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NotFoundDocumentResponse {
    pub content_location: String,
    pub last_state_change: DateTime<Utc>,
    pub lsn: u64,
    pub schema_version: String,
    pub current_write_quorum: Option<u64>,
    pub current_replica_set_size: Option<u64>,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: Option<u64>,
    pub session_token: SessionToken,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for NotFoundDocumentResponse {
    type Error = CosmosError;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();

        Ok(Self {
            content_location: content_location_from_headers(headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            current_write_quorum: current_write_quorum_from_headers_optional(headers)?,
            current_replica_set_size: current_replica_set_size_from_headers_optional(headers)?,
            role: role_from_headers(headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers_optional(headers)?,
            session_token: session_token_from_headers(headers)?,
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
        })
    }
}
