use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::ResourceQuota;
use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::{Request as HttpRequest, Response as HttpResponse};

#[derive(Debug, Clone)]
pub struct DeleteDatabaseOptions<'a> {
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> DeleteDatabaseOptions<'a> {
    pub fn new() -> Self {
        Self {
            activity_id: None,
            consistency_level: None,
        }
    }

    setters! {
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a> DeleteDatabaseOptions<'a> {
    pub fn decorate_request(&self, request: &mut HttpRequest) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.activity_id, request)?;
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;

        request.set_body(bytes::Bytes::from_static(EMPTY_BODY).into());

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

impl DeleteDatabaseResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        let charge = request_charge_from_headers(&headers)?;
        let activity_id = activity_id_from_headers(&headers)?;

        Ok(Self {
            charge,
            activity_id,
            session_token: session_token_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
        })
    }
}
