use crate::prelude::*;
use crate::resources::permission::{ExpirySeconds, PermissionMode};
use crate::resources::ResourceType;
use crate::responses::CreatePermissionResponse;

use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreatePermissionBuilder<'a, 'b> {
    permission_client: &'a PermissionClient,
    // TODO: use this field
    expiry_seconds: ExpirySeconds,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreatePermissionBuilder<'a, 'b> {
    pub(crate) fn new(permission_client: &'a PermissionClient) -> Self {
        Self {
            permission_client,
            expiry_seconds: ExpirySeconds::new(3600),
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> CreatePermissionBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        expiry_seconds: u64 => ExpirySeconds::new(expiry_seconds),
    }
}

impl<'a, 'b> CreatePermissionBuilder<'a, 'b> {
    pub async fn execute(
        &self,
        permission_mode: &PermissionMode<'a>,
    ) -> Result<CreatePermissionResponse<'a>, CosmosError> {
        trace!("CreatePermissionBuilder::execute called");

        let request = self.permission_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions",
                self.permission_client.database_client().database_name(),
                self.permission_client.user_client().user_name(),
            ),
            http::Method::POST,
            ResourceType::Permissions,
        );

        let request = azure_core::headers::add_optional_header(&self.user_agent, request);
        let request = azure_core::headers::add_optional_header(&self.activity_id, request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level, request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'x> {
            id: &'x str,
            #[serde(rename = "permissionMode")]
            permission_mode: &'x str,
            resource: &'x str,
        }

        let request_body = RequestBody {
            id: self.permission_client.permission_name(),
            permission_mode: permission_mode.kind(),
            resource: permission_mode.resource(),
        };
        let request_body = azure_core::to_json(&request_body)?;

        let request = request.body(request_body)?;
        debug!("\nrequest == {:#?}", request);

        Ok(self
            .permission_client
            .http_client()
            .execute_request_check_status(request, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
