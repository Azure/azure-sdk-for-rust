use crate::headers::from_headers::*;
use crate::prelude::*;
use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;

use azure_core::{Request as HttpRequest, Response as HttpResponse};
use chrono::{DateTime, Utc};
use http::response::Response;

#[derive(Debug, Clone)]
pub struct DeleteDocumentOptions<'a> {
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_modified_since: Option<IfModifiedSince<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
}

impl<'a> DeleteDocumentOptions<'a> {
    pub(crate) fn new(document_client: &'a DocumentClient) -> DeleteDocumentOptions<'a> {
        Self {
            if_match_condition: None,
            if_modified_since: None,
            consistency_level: None,
            allow_tentative_writes: TenativeWritesAllowance::Deny,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
        allow_tentative_writes: TenativeWritesAllowance,
        if_modified_since: &'a DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
    }

    pub(crate) fn decorate_request(&self, request: &mut HttpRequest) -> Result<(), crate::Error> {
        trace!("DeleteDocumentOptions::execute called");

        // add trait headers
        azure_core::headers::add_optional_header2(&self.if_match_condition, request);
        azure_core::headers::add_optional_header2(&self.if_modified_since, request);
        azure_core::headers::add_optional_header2(&self.consistency_level, request);
        azure_core::headers::add_mandatory_header2(&self.allow_tentative_writes, request);

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDocumentResponse {
    pub charge: f64,
    pub session_token: String,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for DeleteDocumentResponse {
    type Error = crate::Error;

    fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let headers = response.headers();

        let charge = request_charge_from_headers(headers)?;
        let session_token = session_token_from_headers(headers)?;

        Ok(Self {
            charge,
            session_token,
        })
    }
}
