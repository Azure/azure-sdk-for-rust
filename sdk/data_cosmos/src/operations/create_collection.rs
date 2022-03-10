use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::collection::{IndexingPolicy, PartitionKey};
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::{collect_pinned_stream, Context, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CreateCollectionBuilder {
    client: DatabaseClient,
    partition_key: PartitionKey,
    consistency_level: Option<ConsistencyLevel>,
    indexing_policy: Option<IndexingPolicy>,
    collection_name: String,
    offer: Option<Offer>,
    context: Context,
}

impl CreateCollectionBuilder {
    pub(crate) fn new(
        client: DatabaseClient,
        collection_name: String,
        partition_key: PartitionKey,
    ) -> Self {
        Self {
            client,
            collection_name,
            partition_key,
            consistency_level: None,
            indexing_policy: None,
            offer: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        indexing_policy: IndexingPolicy => Some(indexing_policy),
        offer: Offer => Some(offer),
        context: Context => context,
    }

    pub fn into_future(self) -> CreateCollection {
        Box::pin(async move {
            let mut request = self.client.cosmos_client().prepare_request_pipeline(
                &format!("dbs/{}/colls", self.client.database_name()),
                http::Method::POST,
            );
            azure_core::headers::add_optional_header2(&self.offer, &mut request)?;
            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;

            let collection = CreateCollectionBody {
                id: &self.collection_name,
                indexing_policy: &self.indexing_policy,
                partition_key: &self.partition_key,
            };

            request.set_body(bytes::Bytes::from(serde_json::to_string(&collection)?).into());

            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Collections),
                    &mut request,
                )
                .await?;

            CreateCollectionResponse::try_from(response).await
        })
    }
}

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateCollectionBuilder {
    type Future = CreateCollection;
    type Output = <CreateCollection as std::future::Future>::Output;
    fn into_future(self) -> Self::Future {
        Self::into_future(self)
    }
}

/// The future returned by calling `into_future` on the builder.
pub type CreateCollection =
    futures::future::BoxFuture<'static, crate::Result<CreateCollectionResponse>>;

/// Body for the create collection request
#[derive(Serialize, Debug)]
struct CreateCollectionBody<'a> {
    pub id: &'a str,
    #[serde(rename = "indexingPolicy", skip_serializing_if = "Option::is_none")]
    pub indexing_policy: &'a Option<IndexingPolicy>,
    #[serde(rename = "partitionKey")]
    pub partition_key: &'a PartitionKey,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateCollectionResponse {
    pub collection: Collection,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub last_state_change: DateTime<Utc>,
    pub schema_version: String,
    pub service_version: String,
    pub gateway_version: String,
    pub alt_content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
}

impl CreateCollectionResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(Self {
            collection: serde_json::from_slice(&body)?,
            charge: request_charge_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            service_version: service_version_from_headers(&headers)?.to_owned(),
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
        })
    }
}
