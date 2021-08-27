use crate::headers::from_headers::*;
use crate::prelude::*;
use azure_core::{
    collect_pinned_stream,
    headers::{etag_from_headers, session_token_from_headers},
    Request as HttpRequest, Response as HttpResponse,
};

#[derive(Debug, Clone, Default)]
pub struct GetUserOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl GetUserOptions {
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
        request.set_body(bytes::Bytes::from_static(&[]).into());

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GetUserResponse {
    pub user: User,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
}

impl GetUserResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(Self {
            user: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
        })
    }
}
