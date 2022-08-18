use crate::prelude::*;

use crate::headers::{from_headers::*, CommonHeaders};
use azure_core::Response as HttpResponse;

operation! {
    GetCollection,
    client: CollectionClient,
    ?consistency_level: ConsistencyLevel
}

impl GetCollectionBuilder {
    pub fn into_future(self) -> GetCollection {
        Box::pin(async move {
            let mut request = self.client.collection_request(azure_core::Method::Get);

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Collections),
                    &mut request,
                )
                .await?;

            GetCollectionResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetCollectionResponse {
    pub collection: Collection,
    pub common: CommonHeaders,
    pub collection_partition_index: u64,
    pub collection_service_index: u64,
    pub content_path: String,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub item_lsn: u64,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_item_llsn: u64,
    pub xp_role: u32,
    pub content_location: Option<String>,
}

impl GetCollectionResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            collection: serde_json::from_slice(&body)?,
            common: CommonHeaders::try_from(&headers)?,
            collection_partition_index: collection_partition_index_from_headers(&headers)?,
            collection_service_index: collection_service_index_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            item_lsn: item_lsn_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_item_llsn: cosmos_item_llsn_from_headers(&headers)?,
            xp_role: role_from_headers(&headers)?,
            content_location: content_location_from_headers(&headers)?,
        })
    }
}
