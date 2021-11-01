use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;
use azure_core::headers::session_token_from_headers;
use azure_core::Request as HttpRequest;
use azure_core::Response as HttpResponse;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl DeleteDatabaseOptions {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub fn decorate_request(&self, request: &mut HttpRequest) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDatabaseResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
}

#[async_trait::async_trait]
impl azure_core::util::AsyncTryFrom<HttpResponse> for DeleteDatabaseResponse {
    type Error = crate::Error;

    async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let headers = response.headers();

        let charge = request_charge_from_headers(headers)?;
        let activity_id = activity_id_from_headers(headers)?;

        Ok(Self {
            charge,
            activity_id,
            session_token: session_token_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
        })
    }
}
