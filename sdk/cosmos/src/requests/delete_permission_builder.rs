use crate::prelude::*;
use crate::responses::DeletePermissionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeletePermissionsBuilder<'a, 'b> {
    permission_client: &'a PermissionClient,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
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
}

impl<'a, 'b> DeletePermissionsBuilder<'a, 'b> {
    pub fn permission_client(&self) -> &'a PermissionClient {
        self.permission_client
    }
}

impl<'a, 'b> DeletePermissionsBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }
}

impl<'a, 'b> DeletePermissionsBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }
}

impl<'a, 'b> DeletePermissionsBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> DeletePermissionsBuilder<'a, 'b> {
    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for DeletePermissionsBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for DeletePermissionsBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> DeletePermissionsBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<DeletePermissionResponse, CosmosError> {
        trace!("DeletePermissionBuilder::execute called");

        let request = self
            .permission_client
            .prepare_request_with_permission_name(http::Method::DELETE);

        let request = crate::headers::add_header(self.user_agent(), request);
        let request = crate::headers::add_header(self.activity_id(), request);
        let request = crate::headers::add_header(self.consistency_level(), request);

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
