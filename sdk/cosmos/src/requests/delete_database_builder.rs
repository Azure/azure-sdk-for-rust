use crate::prelude::*;
use crate::responses::DeleteDatabaseResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    database_client: &'a dyn DatabaseClient<C>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, C> DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    pub(crate) fn new(database_client: &'a dyn DatabaseClient<C>) -> DeleteDatabaseBuilder<'a, C> {
        DeleteDatabaseBuilder {
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, C> DatabaseClientRequired<'a, C> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    fn database_client(&self) -> &'a dyn DatabaseClient<C> {
        self.database_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C> UserAgentOption<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, C> ActivityIdOption<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, C> ConsistencyLevelOption<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, C> UserAgentSupport<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = DeleteDatabaseBuilder<'a, C>;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        DeleteDatabaseBuilder {
            database_client: self.database_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C> ActivityIdSupport<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = DeleteDatabaseBuilder<'a, C>;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        DeleteDatabaseBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C> ConsistencyLevelSupport<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = DeleteDatabaseBuilder<'a, C>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        DeleteDatabaseBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    pub async fn execute(&self) -> Result<DeleteDatabaseResponse, AzureError> {
        trace!("DeleteDatabaseResponse::execute called");

        let request = self
            .database_client()
            .prepare_request_with_database_name(hyper::Method::DELETE);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        trace!("request prepared == {:?}", request);

        let future_response = self.database_client().http_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::NO_CONTENT).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
