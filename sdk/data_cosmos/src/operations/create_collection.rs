use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::collection::{IndexingPolicy, PartitionKey};
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::Response as HttpResponse;
use time::OffsetDateTime;

operation! {
    CreateCollection,
    client: DatabaseClient,
    collection_name: String,
    partition_key: PartitionKey,
    ?consistency_level: ConsistencyLevel,
    ?indexing_policy: IndexingPolicy,
    ?offer: Offer
}

impl CreateCollectionBuilder {
    pub fn into_future(self) -> CreateCollection {
        Box::pin(async move {
            let mut request = self.client.collections_request(azure_core::Method::Post);
            request.insert_headers(&self.offer);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            /// Body for the create collection request
            #[derive(Serialize)]
            struct CreateCollectionBody<'a> {
                pub id: &'a str,
                #[serde(rename = "indexingPolicy", skip_serializing_if = "Option::is_none")]
                pub indexing_policy: &'a Option<IndexingPolicy>,
                #[serde(rename = "partitionKey")]
                pub partition_key: &'a PartitionKey,
            }

            let collection = CreateCollectionBody {
                id: &self.collection_name,
                indexing_policy: &self.indexing_policy,
                partition_key: &self.partition_key,
            };

            request.set_body(serde_json::to_vec(&collection)?);

            let response = self
                .client
                .cosmos_client()
                .send(request, self.context.clone(), ResourceType::Collections)
                .await?;

            CreateCollectionResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateCollectionResponse {
    pub collection: Collection,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub last_state_change: OffsetDateTime,
    pub schema_version: String,
    pub service_version: String,
    pub gateway_version: String,
    pub alt_content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
}

impl CreateCollectionResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        Ok(Self {
            collection: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
        })
    }
}
