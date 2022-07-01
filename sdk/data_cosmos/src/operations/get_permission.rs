use crate::{prelude::*, resources::permission::PermissionResponse as GetPermissionResponse};

operation! {
    GetPermission,
    client: PermissionClient,
    ?consistency_level: ConsistencyLevel
}

impl GetPermissionBuilder {
    pub fn into_future(self) -> GetPermission {
        Box::pin(async move {
            let mut request = self.client.permission_request(azure_core::Method::Get);

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Permissions),
                    &mut request,
                )
                .await?;

            GetPermissionResponse::try_from(response).await
        })
    }
}
