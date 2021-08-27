use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Database;
use crate::ResourceQuota;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::{collect_pinned_stream, Request as HttpRequest, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CreateDatabaseOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl CreateDatabaseOptions {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl CreateDatabaseOptions {
    pub(crate) fn decorate_request(
        &self,
        request: &mut HttpRequest,
        database_name: &str,
    ) -> Result<(), crate::Error> {
        #[derive(Serialize)]
        struct CreateDatabaseRequest<'a> {
            pub id: &'a str,
        }
        let req = CreateDatabaseRequest { id: database_name };

        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        request.set_body(bytes::Bytes::from(serde_json::to_string(&req)?).into());
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CreateDatabaseResponse {
    pub database: Database,
    pub charge: f64,
    pub etag: String,
    pub session_token: String,
    pub last_state_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub schema_version: String,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
}

impl CreateDatabaseResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(Self {
            database: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
        })
    }
}
