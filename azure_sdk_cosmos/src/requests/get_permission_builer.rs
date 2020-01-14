use crate::clients::{CosmosUriBuilder, PermissionClient, ResourceType};
use crate::prelude::*;
use crate::responses::GetPermissionResponse;
use azure_sdk_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetPermissionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    permission_client: &'a PermissionClient<'a, CUB>,
}

impl<'a, CUB> GetPermissionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        permission_client: &'a PermissionClient<'a, CUB>,
    ) -> GetPermissionBuilder<'a, CUB> {
        GetPermissionBuilder { permission_client }
    }
}

impl<'a, CUB> PermissionClientRequired<'a, CUB> for GetPermissionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn permission_client(&self) -> &'a PermissionClient<'a, CUB> {
        self.permission_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> GetPermissionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<Option<GetPermissionResponse<'a>>, AzureError> {
        trace!("GetPermissionBuilder::execute called");

        let req = self.permission_client.main_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions/{}",
                self.permission_client.database_name().name(),
                self.permission_client.user_name().id(),
                self.permission_client.permission_name().name(),
            ),
            hyper::Method::GET,
            ResourceType::Permissions,
        );

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:#?}", req);

        let (status, headers, body) =
            extract_status_headers_and_body(self.permission_client.hyper_client().request(req))
                .await?;

        match status {
            StatusCode::OK => Ok(Some((&headers, &body as &[u8]).try_into()?)),
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::OK, StatusCode::NOT_FOUND],
                status,
                std::str::from_utf8(&body)?,
            )
            .into()),
        }
    }
}
