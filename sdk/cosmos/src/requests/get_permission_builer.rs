use crate::prelude::*;
use crate::responses::GetPermissionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetPermissionBuilder<'a, 'b> {
    permission_client: &'a PermissionClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
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

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub async fn execute(&self) -> Result<Option<GetPermissionResponse<'a>>, CosmosError> {
        trace!("GetPermissionBuilder::execute called");

        let request = self
            .permission_client
            .prepare_request_with_permission_name(http::Method::GET);

        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);

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
