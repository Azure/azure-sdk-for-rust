use crate::prelude::*;
use crate::responses::GetPermissionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetPermissionBuilder<'a, 'b> {
    permission_client: &'a PermissionClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> GetPermissionBuilder<'a, 'b> {
    pub(crate) fn new(permission_client: &'a PermissionClient) -> Self {
        Self {
            permission_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> GetPermissionBuilder<'a, 'b> {
    pub fn permission_client(&self) -> &'a PermissionClient {
        self.permission_client
    }
}

impl<'a, 'b> GetPermissionBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for GetPermissionBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for GetPermissionBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> UserAgentSupport<'b> for GetPermissionBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for GetPermissionBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for GetPermissionBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> GetPermissionBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<Option<GetPermissionResponse<'a>>, CosmosError> {
        trace!("GetPermissionBuilder::execute called");

        let request = self
            .permission_client
            .prepare_request_with_permission_name(http::Method::GET);

        let request = crate::headers::add_header(self.user_agent(), request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;
        debug!("\nrequest == {:#?}", request);

        let response = self
            .permission_client
            .http_client()
            .execute_request_check_statuses(request, &[StatusCode::OK, StatusCode::NOT_FOUND])
            .await?;

        match response.status() {
            StatusCode::OK => Ok(Some(response.try_into()?)),
            StatusCode::NOT_FOUND => Ok(None),
            _ => unreachable!(),
        }
    }
}
