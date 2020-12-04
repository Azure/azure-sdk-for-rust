use crate::prelude::*;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetAttachmentBuilder<'a, 'b> {
    attachment_client: &'a AttachmentClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> GetAttachmentBuilder<'a, 'b> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    pub fn attachment_client(&self) -> &'a AttachmentClient {
        self.attachment_client
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

    pub async fn execute(&self) -> Result<crate::responses::GetAttachmentResponse, CosmosError> {
        let mut req = self
            .attachment_client
            .prepare_request_with_attachment_name(http::Method::GET);

        // add trait headers
        req = crate::headers::add_optional_header(self.if_match_condition(), req);
        req = crate::headers::add_optional_header(self.user_agent(), req);
        req = crate::headers::add_optional_header(self.activity_id(), req);
        req = crate::headers::add_optional_header(self.consistency_level(), req);

        req = crate::headers::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        let req = req.body(EMPTY_BODY.as_ref())?;

        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
