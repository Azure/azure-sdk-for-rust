use azure_core::{
    headers::{self, Headers},
    SessionToken,
};
use time::OffsetDateTime;

use crate::ResourceQuota;

#[derive(Debug, Clone, PartialEq)]
pub struct CommonHeaders {
    pub content_type: String,
    pub date: OffsetDateTime,
    pub etag: Option<String>, // todo: make azure_core::Etag
    pub activity_id: uuid::Uuid,
    pub request_charge: f64,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub schema_version: String,
    pub service_version: String,
    pub session_token: SessionToken,
    pub gateway_version: Option<String>,
    pub alt_content_path: Option<String>,
    pub last_state_change: Option<OffsetDateTime>,
    pub lsn: u64,
}

impl TryFrom<&Headers> for CommonHeaders {
    type Error = azure_core::error::Error;
    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(Self {
            content_type: headers::content_type_from_headers(headers)?,
            date: headers::date_from_headers(headers)?,
            etag: headers::etag_from_headers(headers).ok(),
            activity_id: super::from_headers::activity_id_from_headers(headers)?,
            request_charge: super::from_headers::request_charge_from_headers(headers)?,
            resource_quota: super::from_headers::resource_quota_from_headers(headers)
                .unwrap_or_default(),
            resource_usage: super::from_headers::resource_usage_from_headers(headers)
                .unwrap_or_default(),
            schema_version: super::from_headers::schema_version_from_headers(headers)?,
            service_version: super::from_headers::service_version_from_headers(headers)?,
            session_token: headers::session_token_from_headers(headers)?,
            gateway_version: super::from_headers::gateway_version_from_headers(headers).ok(),
            alt_content_path: super::from_headers::alt_content_path_from_headers(&headers).ok(),
            last_state_change: super::from_headers::last_state_change_from_headers(&headers).ok(),
            lsn: super::from_headers::lsn_from_headers(headers)?,
        })
    }
}
