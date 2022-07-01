use crate::prelude::*;
use crate::resources::permission::{
    ExpirySeconds, PermissionMode, PermissionResponse as ReplacePermissionResponse,
};

operation! {
    ReplacePermission,
    client: PermissionClient,
    permission_mode: PermissionMode,
    ?expiry_seconds: ExpirySeconds,
    ?consistency_level: ConsistencyLevel
}

impl ReplacePermissionBuilder {
    pub fn into_future(self) -> ReplacePermission {
        Box::pin(async move {
            let mut request = self.client.permission_request(azure_core::Method::Put);

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

            ReplacePermissionResponse::try_from(response).await
        })
    }
}
