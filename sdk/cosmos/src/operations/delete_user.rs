use crate::headers::from_headers::*;
use crate::prelude::*;
use azure_core::{
    headers::session_token_from_headers, Request as HttpRequest, Response as HttpResponse,
};

#[derive(Debug, Clone, Default)]
pub struct DeleteUserOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl DeleteUserOptions {
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
pub struct DeleteUserResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl DeleteUserResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
        })
    }
}
