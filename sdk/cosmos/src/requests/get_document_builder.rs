use crate::prelude::*;
use crate::responses::GetDocumentResponse;
use azure_core::prelude::*;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetDocumentBuilder<'a, 'b> {
    document_client: &'a DocumentClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> GetDocumentBuilder<'a, 'b> {
    pub(crate) fn new(document_client: &'a DocumentClient) -> Self {
        Self {
            document_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        if_modified_since: &'b DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
    }

    pub async fn execute<T>(&self) -> Result<GetDocumentResponse<T>, CosmosError>
    where
        T: DeserializeOwned,
    {
        let mut req = self
            .document_client
            .prepare_request_with_document_name(http::Method::GET);

        // add trait headers
        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.if_modified_since, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        req = crate::cosmos_entity::add_as_partition_key_header_serialized(
            self.document_client.partition_key_serialized(),
            req,
        );

        let req = req.body(bytes::Bytes::from_static(EMPTY_BODY))?;

        Ok(self
            .document_client
            .http_client()
            .execute_request_check_statuses(
                req,
                &[
                    StatusCode::OK,
                    StatusCode::NOT_MODIFIED,
                    StatusCode::NOT_FOUND,
                ],
            )
            .await?
            .try_into()?)
    }
}
