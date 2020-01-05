use crate::clients::{Client, CosmosUriBuilder, ResourceType};
use crate::responses::ListDatabasesResponse;
use crate::ClientRequired;
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    client: &'a Client<CUB>,
}

impl<'a, CUB> ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(client: &'a Client<CUB>) -> ListDatabasesBuilder<'a, CUB> {
        ListDatabasesBuilder { client }
    }
}

impl<'a, CUB> ClientRequired<'a, CUB> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn client(&self) -> &'a Client<CUB> {
        self.client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(self) -> Result<ListDatabasesResponse, AzureError> {
        trace!("ListDatabasesBuilder::execute called");

        let request = self
            .client
            .prepare_request("dbs", hyper::Method::GET, ResourceType::Databases)
            .body(hyper::Body::empty())?;

        let future_response = self.client.hyper_client().request(request);
        let body = check_status_extract_body(future_response, StatusCode::OK).await?;
        let res = serde_json::from_str::<ListDatabasesResponse>(&body)?;
        Ok(res)
    }
}
