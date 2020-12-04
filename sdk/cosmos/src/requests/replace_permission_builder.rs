use crate::prelude::*;
use crate::resources::permission::{ExpirySeconds, PermissionMode};
use crate::responses::ReplacePermissionResponse;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ReplacePermissionBuilder<'a, 'b> {
    permission_client: &'a PermissionClient,
    expiry_seconds: ExpirySeconds,
    user_agent: Option<azure_core::UserAgent<'b>>,
    activity_id: Option<azure_core::ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplacePermissionBuilder<'a, 'b> {
    pub(crate) fn new(permission_client: &'a PermissionClient) -> Self {
        Self {
            permission_client,
            expiry_seconds: ExpirySeconds::new(3600),
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    pub fn permission_client(&self) -> &'a PermissionClient {
        self.permission_client
    }

    // TODO: Use this in request
    #[allow(unused)]
    fn expiry_seconds(&self) -> ExpirySeconds {
        self.expiry_seconds
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

    pub fn with_expiry_seconds(self, expiry_seconds: u64) -> Self {
        Self {
            expiry_seconds: ExpirySeconds::new(expiry_seconds),
            ..self
        }
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

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub async fn execute_with_permission(
        &self,
        permission_mode: &PermissionMode<'a>,
    ) -> Result<ReplacePermissionResponse<'a>, CosmosError> {
        trace!("ReplacePermissionBuilder::execute called");

        let request = self
            .permission_client
            .prepare_request_with_permission_name(http::Method::PUT);

        let request = crate::headers::add_optional_header(self.user_agent(), request);
        let request = crate::headers::add_optional_header(self.activity_id(), request);
        let request = crate::headers::add_optional_header(self.consistency_level(), request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'a> {
            id: &'a str,
            #[serde(rename = "permissionMode")]
            permission_mode: &'a str,
            resource: &'a str,
        }

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
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
