use crate::headers::from_headers::*;

use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::{collect_pinned_stream, Response as HttpResponse};

use super::Permission;

#[derive(Debug, Clone, PartialEq)]
pub struct PermissionResponse {
    pub permission: Permission,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl PermissionResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::error::Result<PermissionResponse> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(Self {
            permission: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            content_path: String::from(content_path_from_headers(&headers)?),
            alt_content_path: String::from(alt_content_path_from_headers(&headers)?),
        })
    }
}
