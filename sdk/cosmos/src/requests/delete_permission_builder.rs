use crate::prelude::*;
use crate::responses::DeletePermissionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeletePermissionsBuilder<'a, 'b> {
    permission_client: &'a PermissionClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> DeletePermissionsBuilder<'a, 'b> {
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

    pub async fn execute(&self) -> Result<DeletePermissionResponse, CosmosError> {
        trace!("DeletePermissionBuilder::execute called");

        let request = self
            .permission_client
            .prepare_request_with_permission_name(http::Method::DELETE);

        let request = crate::headers::add_header(self.user_agent, request);
        let request = crate::headers::add_header(self.activity_id, request);
        let request = crate::headers::add_header(self.consistency_level.clone(), request);

        let request = request.body(EMPTY_BODY.as_ref())?;
        debug!("\nrequest == {:#?}", request);

        Ok(self
            .permission_client
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
