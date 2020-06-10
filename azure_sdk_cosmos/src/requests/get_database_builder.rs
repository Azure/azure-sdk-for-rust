use crate::prelude::*;
use crate::responses::GetDatabaseResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    database_client: &'a dyn DatabaseClient<C>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C> GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    pub(crate) fn new(database_client: &'a dyn DatabaseClient<C>) -> GetDatabaseBuilder<'a, 'b, C> {
        GetDatabaseBuilder {
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C> DatabaseClientRequired<'a, C> for GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    fn database_client(&self) -> &'a dyn DatabaseClient<C> {
        self.database_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C> UserAgentOption<'b> for GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C> ActivityIdOption<'b> for GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C> ConsistencyLevelOption<'b> for GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C> UserAgentSupport<'b> for GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    type O = GetDatabaseBuilder<'a, 'b, C>;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        GetDatabaseBuilder {
            database_client: self.database_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C> ActivityIdSupport<'b> for GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    type O = GetDatabaseBuilder<'a, 'b, C>;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        GetDatabaseBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C> ConsistencyLevelSupport<'b> for GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    type O = GetDatabaseBuilder<'a, 'b, C>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        GetDatabaseBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C> GetDatabaseBuilder<'a, 'b, C>
where
    C: CosmosClient,
{
    pub async fn execute(&self) -> Result<GetDatabaseResponse, AzureError> {
        trace!("GetDatabaseResponse::execute called");

        let request = self
            .database_client()
            .prepare_request_with_database_name(hyper::Method::GET);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        trace!("request prepared == {:?}", request);

        let future_response = self.database_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok(((&headers, &body as &[u8])).try_into()?)
    }
}
