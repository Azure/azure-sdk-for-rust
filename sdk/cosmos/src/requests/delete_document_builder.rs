use crate::prelude::*;
use crate::responses::DeleteDocumentResponse;
use azure_core::prelude::*;

use chrono::{DateTime, Utc};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDocumentBuilder<'a> {
    document_client: &'a DocumentClient,
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_modified_since: Option<IfModifiedSince<'a>>,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
}

impl<'a> DeleteDocumentBuilder<'a> {
    pub(crate) fn new(document_client: &'a DocumentClient) -> DeleteDocumentBuilder<'a> {
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
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
        allow_tentative_writes: TenativeWritesAllowance,
        if_modified_since: &'a DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
    }

    pub async fn execute(&self) -> Result<DeleteDocumentResponse, CosmosError> {
        trace!("DeleteDocumentBuilder::execute called");

        let mut req = self
            .document_client
            .prepare_request_with_document_name(http::Method::DELETE);

        // add trait headers
        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.if_modified_since, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);
        req = azure_core::headers::add_mandatory_header(&self.allow_tentative_writes, req);

        req = crate::cosmos_entity::add_as_partition_key_header_serialized(
            self.document_client.partition_key_serialized(),
            req,
        );

        let req = req.body(bytes::Bytes::from_static(EMPTY_BODY))?;
        debug!("{:?}", req);

        Ok(self
            .document_client
            .http_client()
            .execute_request_check_status(req, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
