use crate::prelude::*;
use crate::responses::DeleteTriggerResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteTriggerBuilder<'a, 'b> {
    trigger_client: &'a TriggerClient,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
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
}

impl<'a, 'b> TriggerClientRequired<'a> for DeleteTriggerBuilder<'a, 'b> {
    fn trigger_client(&self) -> &'a TriggerClient {
        self.trigger_client
    }
}

impl<'a, 'b> UserAgentOption<'b> for DeleteTriggerBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for DeleteTriggerBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for DeleteTriggerBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> UserAgentSupport<'b> for DeleteTriggerBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for DeleteTriggerBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for DeleteTriggerBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> DeleteTriggerBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<DeleteTriggerResponse, AzureError> {
        trace!("DeleteTriggerBuilder::execute called");

        let req = self
            .trigger_client
            .prepare_request_with_trigger_name(hyper::Method::DELETE);

        // add trait headers
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let request = req.body(hyper::Body::empty())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.trigger_client().hyper_client().request(request),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
