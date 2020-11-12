use crate::prelude::*;
use crate::responses::DeleteTriggerResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    trigger_client: &'a dyn TriggerClient<C, D, COLL>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, COLL> DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    pub(crate) fn new(
        trigger_client: &'a dyn TriggerClient<C, D, COLL>,
    ) -> DeleteTriggerBuilder<'a, 'b, C, D, COLL> {
        DeleteTriggerBuilder {
            trigger_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, COLL> TriggerClientRequired<'a, C, D, COLL>
    for DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn trigger_client(&self) -> &'a dyn TriggerClient<C, D, COLL> {
        self.trigger_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D, COLL> UserAgentOption<'b> for DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, COLL> ActivityIdOption<'b> for DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, COLL> ConsistencyLevelOption<'b> for DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, COLL> UserAgentSupport<'b> for DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteTriggerBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        DeleteTriggerBuilder {
            trigger_client: self.trigger_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL> ActivityIdSupport<'b> for DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteTriggerBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        DeleteTriggerBuilder {
            trigger_client: self.trigger_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, COLL> ConsistencyLevelSupport<'b> for DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    type O = DeleteTriggerBuilder<'a, 'b, C, D, COLL>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        DeleteTriggerBuilder {
            trigger_client: self.trigger_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, COLL> DeleteTriggerBuilder<'a, 'b, C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
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
            self.trigger_client().http_client().request(request),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
