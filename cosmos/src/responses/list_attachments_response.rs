use crate::from_headers::*;
use crate::{Attachment, ResourceQuota};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    continuation_token_from_headers_optional, session_token_from_headers, SessionToken,
};
use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;

#[derive(Debug, Clone, Deserialize)]
struct JsonListAttachmentResponse {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,
    #[serde(rename = "Attachments")]
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListAttachmentsResponse {
    pub rid: String,
    pub count: u64,
    pub attachments: Vec<Attachment>,

    pub max_media_storage_usage_mb: u64,
    pub media_storage_usage_mb: u64,
    pub last_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub item_count: u32,
    pub alt_content_path: String,
    pub content_path: String,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub session_token: SessionToken,
    pub request_charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
    pub continuation_token: Option<String>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for ListAttachmentsResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = std::str::from_utf8(value.1)?;

        debug!("headers == {:#?}", headers);
        debug!("body == {:#?}", body);

        let json: JsonListAttachmentResponse = serde_json::from_str(&body)?;

        Ok(Self {
            rid: json.rid,
            count: json.count,
            attachments: json.attachments,

            max_media_storage_usage_mb: max_media_storage_usage_mb_from_headers(headers)?,
            media_storage_usage_mb: media_storage_usage_mb_from_headers(headers)?,
            last_change: last_state_change_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            item_count: item_count_from_headers(headers)?,
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            request_charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
            continuation_token: continuation_token_from_headers_optional(headers)?,
        })
    }
}

#[cfg(test)]
mod tests {}
