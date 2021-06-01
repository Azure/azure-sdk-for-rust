use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::collection::{IndexingPolicy, PartitionKey};
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::{collect_pinned_stream, Request as HttpRequest, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Options {
    partition_key: PartitionKey,
    consistency_level: Option<ConsistencyLevel>,
    indexing_policy: Option<IndexingPolicy>,
    offer: Option<Offer>,
}

impl Options {
    pub fn new<P: Into<PartitionKey>>(partition_key: P) -> Self {
        Self {
            partition_key: partition_key.into(),
            consistency_level: None,
            indexing_policy: None,
            offer: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        indexing_policy: IndexingPolicy => Some(indexing_policy),
        offer: Offer => Some(offer),
    }
}

impl Options {
    pub(crate) fn decorate_request(
        &self,
        request: &mut HttpRequest,
        collection_name: &str,
    ) -> Result<(), CosmosError> {
        azure_core::headers::add_optional_header2(&self.offer, request);
        azure_core::headers::add_optional_header2(&self.consistency_level, request);

        let collection = CreateCollectionBody {
            id: collection_name.as_ref(),
            indexing_policy: &self.indexing_policy,
            partition_key: &self.partition_key,
        };

        request.set_body(bytes::Bytes::from(serde_json::to_string(&collection)?).into());
        Ok(())
    }
}

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
pub struct Response {
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

impl Response {
    pub async fn try_from(response: HttpResponse) -> Result<Self, CosmosError> {
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
