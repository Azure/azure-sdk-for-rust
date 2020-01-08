use crate::clients::{CosmosUriBuilder, ResourceType, UserClient};
use crate::prelude::*;
use crate::responses::DeleteUserResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    user_client: &'a UserClient<'a, CUB>,
}

impl<'a, CUB> DeleteUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(user_client: &'a UserClient<'a, CUB>) -> DeleteUserBuilder<'a, CUB> {
        DeleteUserBuilder { user_client }
    }
}

impl<'a, CUB> UserClientRequired<'a, CUB> for DeleteUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_client(&self) -> &'a UserClient<'a, CUB> {
        self.user_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> DeleteUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<DeleteUserResponse, AzureError> {
        trace!("DeleteUserBuilder::execute called");

        let req = self.user_client.main_client().prepare_request(
            &format!(
                "dbs/{}/users/{}",
                self.user_client.database_name().name(),
                self.user_client.user_name().id()
            ),
            hyper::Method::DELETE,
            ResourceType::Users,
        );

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.user_client.hyper_client().request(req),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
