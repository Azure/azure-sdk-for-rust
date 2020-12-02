use crate::prelude::*;
use crate::responses::DeleteDocumentResponse;
use azure_core::modify_conditions::IfMatchCondition;
use azure_core::prelude::*;
use azure_core::IfMatchConditionSupport;
use chrono::{DateTime, Utc};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDocumentBuilder<'a> {
    document_client: &'a DocumentClient,
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_modified_since: Option<&'a DateTime<Utc>>,
    user_agent: Option<azure_core::UserAgent<'a>>,
    activity_id: Option<azure_core::ActivityId<'a>>,
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
}

impl<'a> DeleteDocumentBuilder<'a> {
    pub fn document_client(&self) -> &'a DocumentClient {
        self.document_client
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a> IfModifiedSinceOption<'a> for DeleteDocumentBuilder<'a> {
    fn if_modified_since(&self) -> Option<&'a DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'a>> {
        self.user_agent
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    fn activity_id(&self) -> Option<azure_core::ActivityId<'a>> {
        self.activity_id
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    fn allow_tentative_writes(&self) -> TenativeWritesAllowance {
        self.allow_tentative_writes
    }
}

impl<'a> IfMatchConditionSupport<'a> for DeleteDocumentBuilder<'a> {
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a> IfModifiedSinceSupport<'a> for DeleteDocumentBuilder<'a> {
    type O = Self;

    fn with_if_modified_since(self, if_modified_since: &'a DateTime<Utc>) -> Self::O {
        Self {
            if_modified_since: Some(if_modified_since),
            ..self
        }
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    pub fn with_user_agent(self, user_agent: &'a str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    pub fn with_activity_id(self, activity_id: &'a str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a> DeleteDocumentBuilder<'a> {
    pub fn with_allow_tentative_writes(
        self,
        allow_tentative_writes: TenativeWritesAllowance,
    ) -> Self {
        Self {
            allow_tentative_writes,
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> DeleteDocumentBuilder<'a> {
    pub async fn execute(&self) -> Result<DeleteDocumentResponse, CosmosError> {
        trace!("DeleteDocumentBuilder::execute called");

        let mut req = self
            .document_client
            .prepare_request_with_document_name(http::Method::DELETE);

        // add trait headers
        req = crate::headers::add_header(self.if_match_condition(), req);
        req = IfModifiedSinceOption::add_header(self, req);
        req = crate::headers::add_header(self.user_agent(), req);
        req = crate::headers::add_header(self.activity_id(), req);
        req = crate::headers::add_header(self.consistency_level(), req);
        req = crate::headers::add_header(Some(self.allow_tentative_writes()), req);

        req = crate::headers::add_partition_keys_header(self.document_client.partition_keys(), req);

        let req = req.body(EMPTY_BODY.as_ref())?;
        debug!("{:?}", req);

        Ok(self
            .document_client
            .http_client()
            .execute_request_check_status(req, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
