use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_core::errors::UnexpectedHTTPResult;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use http::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>
where
    UserNameSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    user_client: &'a dyn UserClient<C, D>,
    p_user_name: PhantomData<UserNameSet>,
    user_name: Option<&'a dyn UserName>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D> ReplaceUserBuilder<'a, 'b, C, D, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    pub(crate) fn new(
        user_client: &'a dyn UserClient<C, D>,
    ) -> ReplaceUserBuilder<'a, 'b, C, D, No> {
        ReplaceUserBuilder {
            user_client,
            p_user_name: PhantomData {},
            user_name: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, UserNameSet> UserClientRequired<'a, C, D>
    for ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>
where
    UserNameSet: ToAssign,
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
impl<'a, 'b, C, D> UserNameRequired<'a> for ReplaceUserBuilder<'a, 'b, C, D, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn user_name(&self) -> &'a dyn UserName {
        self.user_name.unwrap()
    }
}

impl<'a, 'b, C, D, UserNameSet> UserAgentOption<'b>
    for ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>
where
    UserNameSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, UserNameSet> ActivityIdOption<'b>
    for ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>
where
    UserNameSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, UserNameSet> ConsistencyLevelOption<'b>
    for ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>
where
    UserNameSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D> UserNameSupport<'a> for ReplaceUserBuilder<'a, 'b, C, D, No>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceUserBuilder<'a, 'b, C, D, Yes>;

    #[inline]
    fn with_user_name(self, user_name: &'a dyn UserName) -> Self::O {
        ReplaceUserBuilder {
            user_client: self.user_client,
            p_user_name: PhantomData {},
            user_name: Some(user_name),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, UserNameSet> UserAgentSupport<'b>
    for ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>
where
    UserNameSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ReplaceUserBuilder {
            user_client: self.user_client,
            p_user_name: PhantomData {},
            user_name: self.user_name,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, UserNameSet> ActivityIdSupport<'b>
    for ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>
where
    UserNameSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ReplaceUserBuilder {
            user_client: self.user_client,
            p_user_name: PhantomData {},
            user_name: self.user_name,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, UserNameSet> ConsistencyLevelSupport<'b>
    for ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>
where
    UserNameSet: ToAssign,
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    type O = ReplaceUserBuilder<'a, 'b, C, D, UserNameSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ReplaceUserBuilder {
            user_client: self.user_client,
            p_user_name: PhantomData {},
            user_name: self.user_name,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D> ReplaceUserBuilder<'a, 'b, C, D, Yes>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, CosmosError> {
        trace!("ReplaceUserBuilder::execute called");

        let req = self
            .user_client
            .prepare_request_with_user_name(http::Method::PUT);

        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'x> {
            id: &'x str,
        }
        let request_body = RequestBody {
            id: self.user_name().id(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let req = req.body(request_body.as_bytes())?;
        debug!("\nreq == {:?}", req);

        let response = self.user_client.http_client().execute_request(req).await?;

        match response.status() {
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::OK => Ok(Some(
                (response.headers(), response.body().as_ref()).try_into()?,
            )),
            _ => Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::OK, StatusCode::NOT_FOUND],
                response.status(),
                std::str::from_utf8(response.body())?,
            )
            .into()),
        }
    }
}
