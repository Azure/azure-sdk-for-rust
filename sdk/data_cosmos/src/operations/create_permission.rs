use crate::prelude::*;
use crate::resources::permission::{
    ExpirySeconds, PermissionMode, PermissionResponse as CreatePermissionResponse,
};

operation! {
    CreatePermission,
    client: PermissionClient,
    permission_mode: PermissionMode,
    ?expiry_seconds: ExpirySeconds,
    ?consistency_level: ConsistencyLevel
}

impl CreatePermissionBuilder {
    pub fn into_future(self) -> CreatePermission {
        Box::pin(async move {
            let mut request = self.client.cosmos_client().request(
                &format!(
                    "dbs/{}/users/{}/permissions",
                    self.client.database_client().database_name(),
                    self.client.user_client().user_name()
                ),
                azure_core::Method::Post,
            );

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            request.insert_headers(&self.expiry_seconds);

            #[derive(Serialize, Deserialize)]
            struct RequestBody<'x> {
                id: &'x str,
                #[serde(rename = "permissionMode")]
                permission_mode: &'x str,
                resource: &'x str,
            }

            let request_body = RequestBody {
                id: self.client.permission_name(),
                permission_mode: self.permission_mode.kind(),
                resource: self.permission_mode.resource(),
            };

            request.set_body(serde_json::to_vec(&request_body)?);
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Permissions),
                    &mut request,
                )
                .await?;

            CreatePermissionResponse::try_from(response).await
        })
    }
}
