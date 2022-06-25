use crate::{prelude::*, resources::permission::PermissionResponse};

use azure_core::Context;

#[derive(Debug, Clone)]
pub struct GetPermissionBuilder {
    client: PermissionClient,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl GetPermissionBuilder {
    pub fn new(client: PermissionClient) -> Self {
        Self {
            client,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> GetPermission {
        Box::pin(async move {
            let mut request = self.client.permission_request(azure_core::Method::GET);

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

            PermissionResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type GetPermission =
    futures::future::BoxFuture<'static, azure_core::Result<PermissionResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetPermissionBuilder {
    type IntoFuture = GetPermission;
    type Output = <GetPermission as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
