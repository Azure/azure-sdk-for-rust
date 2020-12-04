use crate::prelude::*;
use crate::resources::permission::{ExpirySeconds, PermissionMode};
use crate::resources::ResourceType;
use crate::responses::CreatePermissionResponse;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreatePermissionBuilder<'a, 'b> {
    permission_client: &'a PermissionClient,
    expiry_seconds: ExpirySeconds,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
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
    pub fn permission_client(&self) -> &'a PermissionClient {
        self.permission_client
    }

    pub fn with_user_agent(self, user_agent: &'b str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'b str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_expiry_seconds(self, expiry_seconds: u64) -> Self {
        Self {
            expiry_seconds: ExpirySeconds::new(expiry_seconds),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'b>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'b>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    // TODO: Use this in request
    #[allow(unused)]
    fn expiry_seconds(&self) -> ExpirySeconds {
        self.expiry_seconds
    }
}

impl<'a, 'b> CreatePermissionBuilder<'a, 'b> {
    pub async fn execute_with_permission(
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

        let request = azure_core::headers::add_optional_header(&self.user_agent(), request);
        let request = azure_core::headers::add_optional_header(&self.activity_id(), request);
        let request = azure_core::headers::add_optional_header(&self.consistency_level(), request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'x> {
            id: &'x str,
            #[serde(rename = "permissionMode")]
            permission_mode: &'x str,
            resource: &'x str,
        };

        let request_body = RequestBody {
            id: self.permission_client.permission_name(),
            permission_mode: permission_mode.kind(),
            resource: permission_mode.resource(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let request = request.body(request_body.as_bytes())?;
        debug!("\nrequest == {:#?}", request);

        Ok(self
            .permission_client
            .http_client()
            .execute_request_check_status(request, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
