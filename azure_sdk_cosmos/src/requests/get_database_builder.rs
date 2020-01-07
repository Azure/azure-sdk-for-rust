use crate::clients::{CosmosUriBuilder, DatabaseClient, ResourceType};
use crate::responses::GetDatabaseResponse;
use crate::DatabaseClientRequired;
use crate::DatabaseTrait;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
}

impl<'a, CUB> GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(database_client: &'a DatabaseClient<CUB>) -> GetDatabaseBuilder<'a, CUB> {
        GetDatabaseBuilder { database_client }
    }
}

impl<'a, CUB> DatabaseClientRequired<'a, CUB> for GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_client(&self) -> &'a DatabaseClient<'a, CUB> {
        self.database_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> GetDatabaseBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<GetDatabaseResponse, AzureError> {
        trace!("GetDatabaseResponse::execute called");

        let request = self
            .database_client()
            .main_client()
            .prepare_request(
                &format!("dbs/{}", self.database_client().database_name().name()),
                hyper::Method::GET,
                ResourceType::Databases,
            )
            .body(hyper::Body::empty())?;

        trace!("request prepared == {:?}", request);

        let future_response = self.database_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok(GetDatabaseResponse::try_from((&headers, &body as &[u8]))?)
    }
}
