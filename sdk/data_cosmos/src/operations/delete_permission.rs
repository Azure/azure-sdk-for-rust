use crate::headers::from_headers::*;
use crate::prelude::*;

use azure_core::headers::session_token_from_headers;
use azure_core::Response as HttpResponse;

operation! {
    DeletePermission,
    client: PermissionClient,
    ?consistency_level: ConsistencyLevel
}

impl DeletePermissionBuilder {
    pub fn into_future(self) -> DeletePermission {
        Box::pin(async move {
            let mut request = self.client.permission_request(azure_core::Method::Delete);

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

            DeletePermissionResponse::try_from(response).await
        })
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
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
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
