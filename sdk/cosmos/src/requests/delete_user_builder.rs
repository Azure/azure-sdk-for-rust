use crate::prelude::*;
use crate::responses::DeleteUserResponse;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    user_client: &'a dyn UserClient<C, D>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D> DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    pub(crate) fn new(user_client: &'a dyn UserClient<C, D>) -> DeleteUserBuilder<'a, 'b, C, D> {
        DeleteUserBuilder {
            user_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D> UserClientRequired<'a, C, D> for DeleteUserBuilder<'a, 'b, C, D>
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
impl<'a, 'b, C, D> UserAgentOption<'b> for DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D> ActivityIdOption<'b> for DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D> ConsistencyLevelOption<'b> for DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D> UserAgentSupport<'b> for DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = DeleteUserBuilder<'a, 'b, C, D>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        DeleteUserBuilder {
            user_client: self.user_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D> ActivityIdSupport<'b> for DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = DeleteUserBuilder<'a, 'b, C, D>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        DeleteUserBuilder {
            user_client: self.user_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D> ConsistencyLevelSupport<'b> for DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = DeleteUserBuilder<'a, 'b, C, D>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        DeleteUserBuilder {
            user_client: self.user_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D> DeleteUserBuilder<'a, 'b, C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    pub async fn execute(&self) -> Result<DeleteUserResponse, AzureError> {
        trace!("DeleteUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(hyper::Method::DELETE);

        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.user_client.hyper_client().request(req),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
