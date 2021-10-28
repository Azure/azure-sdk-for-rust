use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;

use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::{collect_pinned_stream, Request as HttpRequest, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default)]
pub struct GetDatabaseOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl GetDatabaseOptions {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub(crate) fn decorate_request(&self, request: &mut HttpRequest) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        request.set_body(bytes::Bytes::from_static(&[]).into());

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GetDatabaseResponse {
    pub database: Database,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub etag: String,
    pub last_state_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub schema_version: String,
    pub service_version: String,
    pub gateway_version: String,
}

impl GetDatabaseResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(Self {
            database: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            service_version: service_version_from_headers(&headers)?.to_owned(),
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
        })
    }
}
