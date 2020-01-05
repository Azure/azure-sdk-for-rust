use crate::clients::{CosmosUriBuilder, DatabaseClient, ResourceType};
use crate::responses::ListCollectionsResponse;
use crate::DatabaseClientRequired;
use crate::DatabaseTrait;
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct ListCollectionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
}

impl<'a, CUB> ListCollectionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        database_client: &'a DatabaseClient<'a, CUB>,
    ) -> ListCollectionsBuilder<'a, CUB> {
        ListCollectionsBuilder { database_client }
    }
}

impl<'a, CUB> DatabaseClientRequired<'a, CUB> for ListCollectionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_client(&self) -> &'a DatabaseClient<'a, CUB> {
        self.database_client
    }
}

// methods callable regardless
impl<'a, CUB> ListCollectionsBuilder<'a, CUB> where CUB: CosmosUriBuilder {}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListCollectionsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(self) -> Result<ListCollectionsResponse, AzureError> {
        trace!("ListCollectionsBuilder::execute called");
        let request = self
            .database_client
            .main_client()
            .prepare_request(
                &format!("dbs/{}/colls", self.database_client.database_name().name()),
                hyper::Method::GET,
                ResourceType::Collections,
            )
            .body(hyper::Body::empty())?;

        trace!("request prepared == {:?}", request);

        let future_response = self.database_client.hyper_client().request(request);
        let body = check_status_extract_body(future_response, StatusCode::OK).await?;
        let response = serde_json::from_str::<ListCollectionsResponse>(&body)?;
        Ok(response)
    }
}
