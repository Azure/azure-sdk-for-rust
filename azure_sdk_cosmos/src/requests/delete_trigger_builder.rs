use crate::clients::CosmosUriBuilder;
use crate::prelude::*;
use crate::responses::DeleteTriggerResponse;
use crate::TriggerBuilderTrait;
use crate::TriggerClient;
use crate::TriggerClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    trigger_client: &'a TriggerClient<'a, CUB>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(trigger_client: &'a TriggerClient<'a, CUB>) -> DeleteTriggerBuilder<'a, CUB> {
        DeleteTriggerBuilder {
            trigger_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB> TriggerClientRequired<'a, CUB> for DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn trigger_client(&self) -> &'a TriggerClient<'a, CUB> {
        self.trigger_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> UserAgentOption<'a> for DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB> ActivityIdOption<'a> for DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB> ConsistencyLevelOption<'a> for DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, CUB> UserAgentSupport<'a> for DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteTriggerBuilder<'a, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        DeleteTriggerBuilder {
            trigger_client: self.trigger_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB> ActivityIdSupport<'a> for DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteTriggerBuilder<'a, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        DeleteTriggerBuilder {
            trigger_client: self.trigger_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB> ConsistencyLevelSupport<'a> for DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteTriggerBuilder<'a, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        DeleteTriggerBuilder {
            trigger_client: self.trigger_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> DeleteTriggerBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<DeleteTriggerResponse, AzureError> {
        trace!("DeleteTriggerBuilder::execute called");

        let req = self
            .trigger_client
            .prepare_request(hyper::Method::DELETE, true);

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
