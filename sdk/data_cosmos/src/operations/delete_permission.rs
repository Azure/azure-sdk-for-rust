use crate::headers::from_headers::*;
use crate::prelude::*;

use azure_core::headers::session_token_from_headers;
use azure_core::Context;
use azure_core::Response as HttpResponse;

#[derive(Debug, Clone)]
pub struct DeletePermissionBuilder {
    client: PermissionClient,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl DeletePermissionBuilder {
    pub(crate) fn new(client: PermissionClient) -> Self {
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

    pub fn into_future(self) -> DeletePermission {
        Box::pin(async move {
            let mut request = self
                .client
                .prepare_request_with_permission_name(http::Method::DELETE);

            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Permissions),
                    &mut request,
                )
                .await?;

            DeletePermissionResponse::try_from(response).await
        })
    }
}

type DeletePermission =
    futures::future::BoxFuture<'static, crate::Result<DeletePermissionResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeletePermissionBuilder {
    type Future = DeletePermission;
    type Output = <DeletePermission as std::future::Future>::Output;
    fn into_future(self) -> Self::Future {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeletePermissionResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl DeletePermissionResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            content_path: String::from(content_path_from_headers(&headers)?),
            alt_content_path: String::from(alt_content_path_from_headers(&headers)?),
        })
    }
}
