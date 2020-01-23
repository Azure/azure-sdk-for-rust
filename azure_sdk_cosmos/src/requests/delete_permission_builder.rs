use crate::clients::{CosmosUriBuilder, PermissionClient, ResourceType};
use crate::prelude::*;
use crate::responses::DeletePermissionResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeletePermissionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    permission_client: &'a PermissionClient<'a, CUB>,
}

impl<'a, CUB> DeletePermissionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        permission_client: &'a PermissionClient<'a, CUB>,
    ) -> DeletePermissionsBuilder<'a, CUB> {
        DeletePermissionsBuilder { permission_client }
    }
}

impl<'a, CUB> PermissionClientRequired<'a, CUB> for DeletePermissionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn permission_client(&self) -> &'a PermissionClient<'a, CUB> {
        self.permission_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> DeletePermissionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<DeletePermissionResponse, AzureError> {
        trace!("DeletePermissionBuilder::execute called");

        let req = self.permission_client.main_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions/{}",
                self.permission_client.database_name().name(),
                self.permission_client.user_name().id(),
                self.permission_client.permission_name().name()
            ),
            hyper::Method::DELETE,
            ResourceType::Permissions,
        );

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:#?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.permission_client.hyper_client().request(req),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
