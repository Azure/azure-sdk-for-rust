use crate::clients::{CosmosUriBuilder, DatabaseClient, ResourceType};
use crate::prelude::*;
use crate::responses::DeleteDatabaseResponse;
use crate::DatabaseTrait;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
}

impl<'a, CUB> DeleteDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        database_client: &'a DatabaseClient<'a, CUB>,
    ) -> DeleteDatabaseBuilder<'a, CUB> {
        DeleteDatabaseBuilder { database_client }
    }
}

impl<'a, CUB> DatabaseClientRequired<'a, CUB> for DeleteDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_client(&self) -> &'a DatabaseClient<'a, CUB> {
        self.database_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> DeleteDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<DeleteDatabaseResponse, AzureError> {
        trace!("DeleteDatabaseResponse::execute called");

        let request = self
            .database_client()
            .main_client()
            .prepare_request(
                &format!("dbs/{}", self.database_client().database_name().name()),
                hyper::Method::DELETE,
                ResourceType::Databases,
            )
            .body(hyper::Body::empty())?;

        trace!("request prepared == {:?}", request);

        let future_response = self.database_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::NO_CONTENT).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
