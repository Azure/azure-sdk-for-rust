use crate::prelude::*;
use crate::resources::permission::{ExpirySeconds, PermissionMode, PermissionResponse};

use azure_core::Context;

#[derive(Debug, Clone)]
pub struct CreatePermissionBuilder {
    client: PermissionClient,
    expiry_seconds: Option<ExpirySeconds>,
    consistency_level: Option<ConsistencyLevel>,
    permission_mode: PermissionMode,
    context: Context,
}

impl CreatePermissionBuilder {
    pub(crate) fn new(client: PermissionClient, permission_mode: PermissionMode) -> Self {
        Self {
            client,
            expiry_seconds: Some(ExpirySeconds::new(3600)),
            consistency_level: None,
            permission_mode,
            context: Context::new(),
        }
    }

    setters! {
        expiry_seconds: u64 => Some(ExpirySeconds::new(expiry_seconds)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> CreatePermission {
        Box::pin(async move {
            let mut request = self.client.cosmos_client().prepare_request_pipeline(
                &format!(
                    "dbs/{}/users/{}/permissions",
                    self.client.database_client().database_name(),
                    self.client.user_client().user_name()
                ),
                http::Method::POST,
            );

            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;
            azure_core::headers::add_optional_header2(&self.expiry_seconds, &mut request)?;

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

            request.set_body(bytes::Bytes::from(serde_json::to_string(&request_body)?).into());
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Permissions),
                    &mut request,
                )
                .await?;

            PermissionResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type CreatePermission = futures::future::BoxFuture<'static, crate::Result<PermissionResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreatePermissionBuilder {
    type IntoFuture = CreatePermission;
    type Output = <CreatePermission as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
