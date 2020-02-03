use crate::clients::{CosmosUriBuilder, ResourceType, UserClient};
use crate::prelude::*;
use crate::responses::CreateUserResponse;
use azure_sdk_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ReplaceUserBuilder<'a, CUB, UserNameSet>
where
    UserNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    user_client: &'a UserClient<'a, CUB>,
    p_user_name: PhantomData<UserNameSet>,
    user_name: Option<&'a dyn UserName>,
}

impl<'a, CUB> ReplaceUserBuilder<'a, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(user_client: &'a UserClient<'a, CUB>) -> ReplaceUserBuilder<'a, CUB, No> {
        ReplaceUserBuilder {
            user_client,
            p_user_name: PhantomData {},
            user_name: None,
        }
    }
}

impl<'a, CUB, UserNameSet> UserClientRequired<'a, CUB> for ReplaceUserBuilder<'a, CUB, UserNameSet>
where
    UserNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_client(&self) -> &'a UserClient<'a, CUB> {
        self.user_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> UserNameRequired<'a> for ReplaceUserBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_name(&self) -> &'a dyn UserName {
        self.user_name.unwrap()
    }
}

impl<'a, CUB> UserNameSupport<'a> for ReplaceUserBuilder<'a, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = ReplaceUserBuilder<'a, CUB, Yes>;

    #[inline]
    fn with_user_name(self, user_name: &'a dyn UserName) -> Self::O {
        ReplaceUserBuilder {
            user_client: self.user_client,
            p_user_name: PhantomData {},
            user_name: Some(user_name),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ReplaceUserBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<Option<CreateUserResponse>, AzureError> {
        trace!("ReplaceUserBuilder::execute called");

        let req = self.user_client.main_client().prepare_request(
            &format!(
                "dbs/{}/users/{}",
                self.user_client.database_name().name(),
                self.user_client.user_name().id()
            ),
            hyper::Method::PUT,
            ResourceType::Users,
        );

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'a> {
            id: &'a str,
        }
        let request_body = RequestBody {
            id: self.user_name().id(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let req = req.body(hyper::Body::from(request_body))?;
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
