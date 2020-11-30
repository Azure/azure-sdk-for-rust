use crate::prelude::*;
use crate::resources::permission::PermissionMode;
use crate::resources::ResourceType;
use crate::responses::CreatePermissionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreatePermissionBuilder<'a, 'b> {
    permission_client: &'a PermissionClient,
    expiry_seconds: u64,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreatePermissionBuilder<'a, 'b> {
    pub(crate) fn new(permission_client: &'a PermissionClient) -> Self {
        Self {
            permission_client,
            expiry_seconds: 3600,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> PermissionClientRequired<'a> for CreatePermissionBuilder<'a, 'b> {
    fn permission_client(&self) -> &'a PermissionClient {
        self.permission_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b> ExpirySecondsOption for CreatePermissionBuilder<'a, 'b> {
    fn expiry_seconds(&self) -> u64 {
        self.expiry_seconds
    }
}

impl<'a, 'b> UserAgentOption<'b> for CreatePermissionBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for CreatePermissionBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for CreatePermissionBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> ExpirySecondsSupport for CreatePermissionBuilder<'a, 'b> {
    type O = Self;

    fn with_expiry_seconds(self, expiry_seconds: u64) -> Self::O {
        Self {
            expiry_seconds,
            ..self
        }
    }
}

impl<'a, 'b> UserAgentSupport<'b> for CreatePermissionBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for CreatePermissionBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for CreatePermissionBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
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

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

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
