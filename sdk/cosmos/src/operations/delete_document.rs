use crate::prelude::*;
use crate::responses::DeleteDocumentResponse;
use crate::headers::from_headers::*;
use azure_core::prelude::*;
use azure_core::headers::session_token_from_headers;

use chrono::{DateTime, Utc};
use http::StatusCode;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDocumentOptions<'a> {
    document_client: &'a DocumentClient,
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_modified_since: Option<IfModifiedSince<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,

    pub(crate) fn new(document_client: &'a DocumentClient) -> DeleteDocumentOptions<'a> {
        Self {
            document_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: TenativeWritesAllowance::Deny,
        }
    }

    setters! {
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
        allow_tentative_writes: TenativeWritesAllowance,
        if_modified_since: &'a DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
    }

    pub(crate) fn decorate_request(&self, request: &mut HttpRequest) -> Result<DeleteDocumentResponse, crate::Error> {
        trace!("DeleteDocumentOptions::execute called");

        // add trait headers
        azure_core::headers::add_optional_header2(&self.if_match_condition, request);
        azure_core::headers::add_optional_header2(&self.if_modified_since, request);
        azure_core::headers::add_optional_header2(&self.activity_id, request);
        azure_core::headers::add_optional_header2(&self.consistency_level, request);
        azure_core::headers::add_mandatory_header2(&self.allow_tentative_writes, request);

        request = crate::cosmos_entity::add_as_partition_key_header_serialized(
            self.document_client.partition_key_serialized(),
            request,
        );

        let req = request.body(bytes::Bytes::from_static(EMPTY_BODY))?;
        debug!("{:?}", req);

        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct DeleteDocumentResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for DeleteDocumentResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, create::Error> {
        let headers = response.headers();

        let charge = request_charge_from_headers(headers)?;
        let activity_id = activity_id_from_headers(headers)?;
        let session_token = session_token_from_headers(headers)?;

        Ok(Self {
            charge,
            activity_id,
            session_token,
        })
    }
}
