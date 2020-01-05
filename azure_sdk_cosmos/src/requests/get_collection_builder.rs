use crate::clients::{CollectionClient, CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::GetCollectionResponse;
use crate::CollectionClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
}

impl<'a, CUB> GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> GetCollectionBuilder<'a, CUB> {
        GetCollectionBuilder { collection_client }
    }
}

impl<'a, CUB> CollectionClientRequired<'a, CUB> for GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<GetCollectionResponse, AzureError> {
        trace!("GetCollectionResponse::execute called");

        let request = self
            .collection_client()
            .main_client()
            .prepare_request(
                &format!(
                    "dbs/{}/colls/{}",
                    self.collection_client.database_name().name(),
                    self.collection_client.collection_name().name()
                ),
                hyper::Method::GET,
                ResourceType::Collections,
            )
            .body(hyper::Body::empty())?;

        let future_response = self.collection_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
