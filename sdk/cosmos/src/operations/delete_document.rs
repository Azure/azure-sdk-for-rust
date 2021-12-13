use crate::prelude::*;

use azure_core::prelude::*;
use azure_core::{Request as HttpRequest, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeleteDocumentOptions<'a> {
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_modified_since: Option<IfModifiedSince<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TentativeWritesAllowance,
}

impl<'a> DeleteDocumentOptions<'a> {
    pub fn new() -> DeleteDocumentOptions<'a> {
        Self {
            if_match_condition: None,
            if_modified_since: None,
            consistency_level: None,
            allow_tentative_writes: TentativeWritesAllowance::Deny,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
        allow_tentative_writes: TentativeWritesAllowance,
        if_modified_since: &'a DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
    }

    pub fn decorate_request(
        &self,
        request: &mut HttpRequest,
        serialized_partition_key: &str,
    ) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.if_match_condition, request)?;
        azure_core::headers::add_optional_header2(&self.if_modified_since, request)?;
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        azure_core::headers::add_mandatory_header2(&self.allow_tentative_writes, request)?;

        crate::cosmos_entity::add_as_partition_key_header_serialized2(
            serialized_partition_key,
            request,
        );

        Ok(())
    }
}

use crate::headers::from_headers::*;
use azure_core::headers::session_token_from_headers;

#[derive(Debug, Clone)]
pub struct DeleteDocumentResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl DeleteDocumentResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        let charge = request_charge_from_headers(&headers)?;
        let activity_id = activity_id_from_headers(&headers)?;
        let session_token = session_token_from_headers(&headers)?;

        Ok(Self {
            charge,
            activity_id,
            session_token,
        })
    }
}
