use crate::clients::{CollectionClient, CosmosUriBuilder};
use crate::responses::DeleteCollectionResponse;
use crate::{CollectionBuilderTrait, CollectionClientRequired};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
}

impl<'a, CUB> DeleteCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> DeleteCollectionBuilder<'a, CUB> {
        DeleteCollectionBuilder { collection_client }
    }
}

impl<'a, CUB> CollectionClientRequired<'a, CUB> for DeleteCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> DeleteCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<DeleteCollectionResponse, AzureError> {
        trace!("DeleteCollectionBuilder::execute called");

        let req = self
            .collection_client()
            .prepare_request(hyper::Method::DELETE)
            .body(hyper::Body::empty())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client().hyper_client().request(req),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
