use crate::prelude::*;
use crate::responses::GetDocumentResponse;
use azure_core::modify_conditions::IfMatchCondition;
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
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
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

    pub fn document_client(&self) -> &'a DocumentClient {
        self.document_client
    }

    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    pub fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }

    pub fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self {
        Self {
            if_modified_since: Some(IfModifiedSince::new(if_modified_since)),
            ..self
        }
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub async fn execute<T>(&self) -> Result<GetDocumentResponse<T>, CosmosError>
    where
        T: DeserializeOwned,
    {
        let mut req = self
            .document_client
            .prepare_request_with_document_name(http::Method::GET);

        // add trait headers
        req = crate::headers::add_header(self.if_match_condition(), req);
        req = crate::headers::add_header(self.if_modified_since(), req);
        req = crate::headers::add_header(self.user_agent(), req);
        req = crate::headers::add_header(self.activity_id(), req);
        req = crate::headers::add_header(self.consistency_level(), req);

        req = crate::headers::add_partition_keys_header(self.document_client.partition_keys(), req);

        let req = req.body(EMPTY_BODY.as_ref())?;

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

impl<'a, 'b> GetDocumentBuilder<'a, 'b> {
    fn if_modified_since(&self) -> Option<IfModifiedSince> {
        self.if_modified_since.clone()
    }
}
