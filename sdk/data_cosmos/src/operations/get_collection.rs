use crate::prelude::*;

use crate::headers::from_headers::*;
use azure_core::headers::{
    content_type_from_headers, etag_from_headers, session_token_from_headers,
};
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

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
    pub last_state_change: OffsetDateTime,
    pub etag: String,
    pub collection_partition_index: u64,
    pub collection_service_index: u64,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub item_lsn: u64,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_item_llsn: u64,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub gateway_version: String,
    pub server: String,
    pub xp_role: u32,
    pub content_type: String,
    pub content_location: String,
    pub date: OffsetDateTime,
}

impl GetCollectionResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            collection: serde_json::from_slice(&body)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            collection_partition_index: collection_partition_index_from_headers(&headers)?,
            collection_service_index: collection_service_index_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            item_lsn: item_lsn_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_item_llsn: cosmos_item_llsn_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            server: server_from_headers(&headers)?,
            xp_role: role_from_headers(&headers)?,
            content_type: content_type_from_headers(&headers)?,
            content_location: content_location_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
        })
    }
}
