use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_core::prelude::*;
use http::StatusCode;
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
    consistency_level: Option<ConsistencyLevel>,
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
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
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
    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
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
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, CosmosError> {
        trace!("GetUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(http::Method::GET);

        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.body(EMPTY_BODY.as_ref())?;
        debug!("\nreq == {:?}", req);

        let response = self
            .user_client
            .http_client()
            .execute_request_check_statuses(req, &[StatusCode::NOT_FOUND, StatusCode::OK])
            .await?;

        match response.status() {
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::OK => Ok(Some(response.try_into()?)),
            _ => unreachable!(),
        }
    }
}
