use crate::prelude::*;
use crate::responses::GetPermissionResponse;
use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    permission_client: &'a dyn PermissionClient<C, D, USER>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, USER> GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    pub(crate) fn new(
        permission_client: &'a dyn PermissionClient<C, D, USER>,
    ) -> GetPermissionBuilder<'a, 'b, C, D, USER> {
        GetPermissionBuilder {
            permission_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, USER> PermissionClientRequired<'a, C, D, USER>
    for GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn permission_client(&self) -> &'a dyn PermissionClient<C, D, USER> {
        self.permission_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, C, D, USER> UserAgentOption<'b> for GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, C, D, USER> ActivityIdOption<'b> for GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, C, D, USER> ConsistencyLevelOption<'b> for GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, C, D, USER> UserAgentSupport<'b> for GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    type O = GetPermissionBuilder<'a, 'b, C, D, USER>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        GetPermissionBuilder {
            permission_client: self.permission_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, USER> ActivityIdSupport<'b> for GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    type O = GetPermissionBuilder<'a, 'b, C, D, USER>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        GetPermissionBuilder {
            permission_client: self.permission_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, USER> ConsistencyLevelSupport<'b> for GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    type O = GetPermissionBuilder<'a, 'b, C, D, USER>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        GetPermissionBuilder {
            permission_client: self.permission_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, USER> GetPermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    pub async fn execute(&self) -> Result<Option<GetPermissionResponse<'a>>, AzureError> {
        trace!("GetPermissionBuilder::execute called");

        let request = self
            .permission_client
            .prepare_request_with_permission_name(hyper::Method::GET);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;
        debug!("\nrequest == {:#?}", request);

        let (status, headers, body) =
            extract_status_headers_and_body(self.permission_client.http_client().request(request))
                .await?;

        match status {
            StatusCode::OK => Ok(Some((&headers, &body as &[u8]).try_into()?)),
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::OK, StatusCode::NOT_FOUND],
                status,
                std::str::from_utf8(&body)?,
            )
            .into()),
        }
    }
}
