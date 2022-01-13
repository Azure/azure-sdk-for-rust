use crate::prelude::*;
use crate::resources::permission::{ExpirySeconds, PermissionMode};

use azure_core::Request as HttpRequest;

#[derive(Debug, Clone)]
pub struct CreatePermissionOptions {
    expiry_seconds: Option<ExpirySeconds>,
    consistency_level: Option<ConsistencyLevel>,
}

impl CreatePermissionOptions {
    pub fn new() -> Self {
        Self {
            expiry_seconds: Some(ExpirySeconds::new(3600)),
            consistency_level: None,
        }
    }

    setters! {
        expiry_seconds: u64 => Some(ExpirySeconds::new(expiry_seconds)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub(crate) fn decorate_request(
        &self,
        request: &mut HttpRequest,
        id: &str,
        permission_mode: &PermissionMode<'_>,
    ) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        azure_core::headers::add_optional_header2(&self.expiry_seconds, request)?;

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'x> {
            id: &'x str,
            #[serde(rename = "permissionMode")]
            permission_mode: &'x str,
            resource: &'x str,
        }

        let request_body = RequestBody {
            id,
            permission_mode: permission_mode.kind(),
            resource: permission_mode.resource(),
        };

        request.set_body(bytes::Bytes::from(serde_json::to_string(&request_body)?).into());
        Ok(())
    }
}
