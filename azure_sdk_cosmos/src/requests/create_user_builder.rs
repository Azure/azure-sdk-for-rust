use crate::clients::{CosmosUriBuilder, ResourceType, UserClient};
use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    user_client: &'a UserClient<'a, CUB>,
}

impl<'a, CUB> CreateUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(user_client: &'a UserClient<'a, CUB>) -> CreateUserBuilder<'a, CUB> {
        CreateUserBuilder { user_client }
    }
}

impl<'a, CUB> UserClientRequired<'a, CUB> for CreateUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_client(&self) -> &'a UserClient<'a, CUB> {
        self.user_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> CreateUserBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateUserResponse, AzureError> {
        trace!("CreateUserBuilder::execute called");

        let mut req = self.user_client.main_client().prepare_request(
            &format!("dbs/{}/users", self.user_client.database_name().name()),
            hyper::Method::POST,
            ResourceType::Users,
        );

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'a> {
            id: &'a str,
        }
        let request_body = RequestBody {
            id: self.user_client().user_name().id(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let req = req.body(hyper::Body::from(request_body))?;
        debug!("\nreq == {:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.user_client.hyper_client().request(req),
            StatusCode::CREATED,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
