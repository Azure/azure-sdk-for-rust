use crate::headers::from_headers::*;
use crate::prelude::*;

use azure_core::headers::session_token_from_headers;
use azure_core::Request as HttpRequest;
use azure_core::Response as HttpResponse;

#[derive(Debug, Clone)]
pub struct DeletePermissionOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl DeletePermissionOptions {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub(crate) fn decorate_request(&self, request: &mut HttpRequest) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeletePermissionResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl DeletePermissionResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            content_path: String::from(content_path_from_headers(&headers)?),
            alt_content_path: String::from(alt_content_path_from_headers(&headers)?),
        })
    }
}
