use crate::prelude::*;
use crate::responses::ReplacePermissionResponse;
use crate::{PermissionMode, PermissionResource};
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ReplacePermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    permission_client: &'a dyn PermissionClient<C, D, USER>,
    expiry_seconds: u64,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, C, D, USER> ReplacePermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    pub(crate) fn new(
        permission_client: &'a dyn PermissionClient<C, D, USER>,
    ) -> ReplacePermissionBuilder<'a, 'b, C, D, USER> {
        ReplacePermissionBuilder {
            permission_client,
            expiry_seconds: 3600,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, C, D, USER> PermissionClientRequired<'a, C, D, USER>
    for ReplacePermissionBuilder<'a, 'b, C, D, USER>
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
impl<'a, 'b, C, D, USER> ExpirySecondsOption for ReplacePermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn expiry_seconds(&self) -> u64 {
        self.expiry_seconds
    }
}

impl<'a, 'b, C, D, USER> UserAgentOption<'b> for ReplacePermissionBuilder<'a, 'b, C, D, USER>
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

impl<'a, 'b, C, D, USER> ActivityIdOption<'b> for ReplacePermissionBuilder<'a, 'b, C, D, USER>
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

impl<'a, 'b, C, D, USER> ConsistencyLevelOption<'b> for ReplacePermissionBuilder<'a, 'b, C, D, USER>
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

impl<'a, 'b, C, D, USER> ExpirySecondsSupport for ReplacePermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    type O = ReplacePermissionBuilder<'a, 'b, C, D, USER>;

    #[inline]
    fn with_expiry_seconds(self, expiry_seconds: u64) -> Self::O {
        ReplacePermissionBuilder {
            permission_client: self.permission_client,
            expiry_seconds,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, USER> UserAgentSupport<'b> for ReplacePermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    type O = ReplacePermissionBuilder<'a, 'b, C, D, USER>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ReplacePermissionBuilder {
            permission_client: self.permission_client,
            expiry_seconds: self.expiry_seconds,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, USER> ActivityIdSupport<'b> for ReplacePermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    type O = ReplacePermissionBuilder<'a, 'b, C, D, USER>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ReplacePermissionBuilder {
            permission_client: self.permission_client,
            expiry_seconds: self.expiry_seconds,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, C, D, USER> ConsistencyLevelSupport<'b>
    for ReplacePermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    type O = ReplacePermissionBuilder<'a, 'b, C, D, USER>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ReplacePermissionBuilder {
            permission_client: self.permission_client,
            expiry_seconds: self.expiry_seconds,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C, D, USER> ReplacePermissionBuilder<'a, 'b, C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    pub async fn execute_with_permission<R>(
        &self,
        permission_mode: &PermissionMode<R>,
    ) -> Result<ReplacePermissionResponse<'a>, AzureError>
    where
        R: PermissionResource,
    {
        trace!("ReplacePermissionBuilder::execute called");

        let request = self
            .permission_client
            .prepare_request_with_permission_name(hyper::Method::PUT);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'a> {
            id: &'a str,
            #[serde(rename = "permissionMode")]
            permission_mode: &'a str,
            resource: &'a str,
        }

        let (permission_mode, resource) = permission_mode.to_elements();

        let request_body = RequestBody {
            id: self.permission_client.permission_name(),
            permission_mode,
            resource: resource.uri(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let request = request.body(hyper::Body::from(request_body))?;
        debug!("\nrequest == {:#?}", request);

        let (headers, body) = check_status_extract_headers_and_body(
            self.permission_client.http_client().request(request),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
