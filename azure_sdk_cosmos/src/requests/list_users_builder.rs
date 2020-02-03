use crate::clients::{CosmosUriBuilder, DatabaseClient, ResourceType};
use crate::responses::ListUsersResponse;
use crate::DatabaseClientRequired;
use crate::DatabaseTrait;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListUsersBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
}

impl<'a, CUB> ListUsersBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(database_client: &'a DatabaseClient<'a, CUB>) -> ListUsersBuilder<'a, CUB> {
        ListUsersBuilder { database_client }
    }
}

impl<'a, CUB> DatabaseClientRequired<'a, CUB> for ListUsersBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_client(&self) -> &'a DatabaseClient<'a, CUB> {
        self.database_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListUsersBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ListUsersResponse, AzureError> {
        trace!("ListUsersBuilder::execute called");

        let req = self.database_client.main_client().prepare_request(
            &format!("dbs/{}/users", self.database_client.database_name().name()),
            hyper::Method::GET,
            ResourceType::Users,
        );

        let req = req.body(hyper::Body::empty())?;
        debug!("\nreq == {:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.database_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
