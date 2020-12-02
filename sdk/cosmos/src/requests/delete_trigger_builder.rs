use crate::prelude::*;
use crate::responses::DeleteTriggerResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteTriggerBuilder<'a, 'b> {
    trigger_client: &'a TriggerClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> DeleteTriggerBuilder<'a, 'b> {
    pub(crate) fn new(trigger_client: &'a TriggerClient) -> Self {
        Self {
            trigger_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
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

    pub async fn execute(&self) -> Result<DeleteTriggerResponse, CosmosError> {
        trace!("DeleteTriggerBuilder::execute called");

        let req = self
            .trigger_client
            .prepare_request_with_trigger_name(http::Method::DELETE);

        // add trait headers
        let req = crate::headers::add_header(self.user_agent(), req);
        let req = crate::headers::add_header(self.activity_id(), req);
        let req = crate::headers::add_header(self.consistency_level(), req);

        let request = req.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .trigger_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }

    fn trigger_client(&self) -> &'a TriggerClient {
        self.trigger_client
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
}
