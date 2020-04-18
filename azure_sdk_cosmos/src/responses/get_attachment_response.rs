use crate::from_headers::*;
use crate::{Attachment, IndexingDirective, ResourceQuota};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{etag_from_headers, session_token_from_headers, SessionToken};
use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetAttachmentResponse {
    pub attachment: Attachment,

    pub content_type: String,
    pub content_location: String,
    pub last_change: DateTime<Utc>,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub alt_content_path: String,
    pub content_path: String,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub item_lsn: u64,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_item_llsn: u64,
    pub session_token: SessionToken,
    pub request_charge: f64,
    pub indexing_directive: IndexingDirective,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for GetAttachmentResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = std::str::from_utf8(value.1)?;

        debug!("headers == {:#?}", headers);
        debug!("body == {}", body);

        Ok(Self {
            attachment: serde_json::from_str(body)?,
            content_type: content_type_from_headers(headers)?.to_owned(),
            content_location: content_location_from_headers(headers)?.to_owned(),
            last_change: last_state_change_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            item_lsn: item_lsn_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_item_llsn: cosmos_item_llsn_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            request_charge: request_charge_from_headers(headers)?,
            indexing_directive: indexing_directive_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
        })
    }
}

#[cfg(test)]
mod tests {}
