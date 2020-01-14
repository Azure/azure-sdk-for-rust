use crate::clients::{CosmosUriBuilder, ResourceType, UserClient};
use crate::prelude::*;
use crate::responses::ListPermissionsResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListPermissionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    user_client: &'a UserClient<'a, CUB>,
}

impl<'a, CUB> ListPermissionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(user_client: &'a UserClient<'a, CUB>) -> ListPermissionsBuilder<'a, CUB> {
        ListPermissionsBuilder { user_client }
    }
}

impl<'a, CUB> UserClientRequired<'a, CUB> for ListPermissionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_client(&self) -> &'a UserClient<'a, CUB> {
        self.user_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListPermissionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ListPermissionsResponse<'a>, AzureError> {
        trace!("ListPermissionsBuilder::execute called");

        let mut req = self.user_client.main_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions",
                self.user_client.database_name().name(),
                self.user_client.user_name().id(),
            ),
            hyper::Method::GET,
            ResourceType::Permissions,
        );

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:#?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.user_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
