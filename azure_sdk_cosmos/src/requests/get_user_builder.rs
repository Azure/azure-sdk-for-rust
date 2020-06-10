use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_sdk_core::errors::UnexpectedHTTPResult;
use azure_sdk_core::errors::{extract_status_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    user_client: &'a dyn UserClient<C, D>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D> GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    pub(crate) fn new(user_client: &'a dyn UserClient<C, D>) -> GetUserBuilder<'a, 'b, C, D> {
        GetUserBuilder {
            user_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D> UserClientRequired<'a, C, D> for GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn user_client(&self) -> &'a dyn UserClient<C, D> {
        self.user_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D> UserAgentOption<'b> for GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D> ActivityIdOption<'b> for GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D> ConsistencyLevelOption<'b> for GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D> UserAgentSupport<'b> for GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = GetUserBuilder<'a, 'b, C, D>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        GetUserBuilder {
            user_client: self.user_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D> ActivityIdSupport<'b> for GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = GetUserBuilder<'a, 'b, C, D>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        GetUserBuilder {
            user_client: self.user_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D> ConsistencyLevelSupport<'b> for GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = GetUserBuilder<'a, 'b, C, D>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        GetUserBuilder {
            user_client: self.user_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D> GetUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, AzureError> {
        trace!("GetUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(hyper::Method::GET);

        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:?}", req);

        let (status_code, headers, body) =
            extract_status_headers_and_body(self.user_client.hyper_client().request(req)).await?;

        match status_code {
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::OK => Ok(Some((&headers, &body as &[u8]).try_into()?)),
            _ => Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::OK, StatusCode::NOT_FOUND],
                status_code,
                std::str::from_utf8(&body)?,
            )
            .into()),
        }
    }
}
