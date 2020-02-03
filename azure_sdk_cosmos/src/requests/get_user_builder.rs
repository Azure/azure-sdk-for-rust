use crate::clients::{CosmosUriBuilder, ResourceType, UserClient};
use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_sdk_core::errors::UnexpectedHTTPResult;
use azure_sdk_core::errors::{extract_status_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    user_client: &'a UserClient<'a, CUB>,
}

impl<'a, CUB> GetUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(user_client: &'a UserClient<'a, CUB>) -> GetUserBuilder<'a, CUB> {
        GetUserBuilder { user_client }
    }
}

impl<'a, CUB> UserClientRequired<'a, CUB> for GetUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_client(&self) -> &'a UserClient<'a, CUB> {
        self.user_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> GetUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, AzureError> {
        trace!("GetUserBuilder::execute called");

        let req = self.user_client.main_client().prepare_request(
            &format!(
                "dbs/{}/users/{}",
                self.user_client.database_name().name(),
                self.user_client.user_name().id()
            ),
            hyper::Method::GET,
            ResourceType::Users,
        );

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:?}", req);

        let (status_code, headers, body) =
            extract_status_headers_and_body(self.user_client.hyper_client().request(req)).await?;

        match status_code {
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::OK => Ok(Some((&headers, &body as &[u8]).try_into()?)),
            _ => Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::OK, StatusCode::NOT_FOUND],
                status_code,
                std::str::from_utf8(&body)?,
            )
            .into()),
        }
    }
}
