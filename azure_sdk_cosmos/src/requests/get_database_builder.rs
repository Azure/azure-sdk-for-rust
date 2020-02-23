use crate::clients::{CosmosUriBuilder, DatabaseClient, ResourceType};
use crate::responses::GetDatabaseResponse;
use crate::DatabaseClientRequired;
use crate::DatabaseTrait;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
}

impl<'a, CUB> GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(database_client: &'a DatabaseClient<'a, CUB>) -> GetDatabaseBuilder<'a, CUB> {
        GetDatabaseBuilder {
            database_client,
            user_agent: None,
            activity_id: None,
        }
    }
}

impl<'a, CUB> DatabaseClientRequired<'a, CUB> for GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_client(&self) -> &'a DatabaseClient<'a, CUB> {
        self.database_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> UserAgentOption<'a> for GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB> ActivityIdOption<'a> for GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB> UserAgentSupport<'a> for GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetDatabaseBuilder<'a, CUB>;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        GetDatabaseBuilder {
            database_client: self.database_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
        }
    }
}

impl<'a, CUB> ActivityIdSupport<'a> for GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetDatabaseBuilder<'a, CUB>;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        GetDatabaseBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
        }
    }
}

// methods callable regardless
impl<'a, CUB> GetDatabaseBuilder<'a, CUB> where CUB: CosmosUriBuilder {}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<GetDatabaseResponse, AzureError> {
        trace!("GetDatabaseResponse::execute called");

        let request = self.database_client().main_client().prepare_request(
            &format!("dbs/{}", self.database_client().database_name().name()),
            hyper::Method::GET,
            ResourceType::Databases,
        );

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        trace!("request prepared == {:?}", request);

        let future_response = self.database_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok(GetDatabaseResponse::try_from((&headers, &body as &[u8]))?)
    }
}
