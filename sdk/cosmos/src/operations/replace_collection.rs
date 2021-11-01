use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::collection::{IndexingPolicy, PartitionKey};
use azure_core::headers::{
    content_type_from_headers, etag_from_headers, session_token_from_headers,
};
use azure_core::{collect_pinned_stream, Request as HttpRequest, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ReplaceCollectionOptions {
    partition_key: PartitionKey,
    consistency_level: Option<ConsistencyLevel>,
    indexing_policy: Option<IndexingPolicy>,
}

impl ReplaceCollectionOptions {
    pub fn new<P: Into<PartitionKey>>(partition_key: P) -> Self {
        Self {
            partition_key: partition_key.into(),
            consistency_level: None,
            indexing_policy: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        indexing_policy: IndexingPolicy => Some(indexing_policy),
    }

    pub(crate) fn decorate_request(
        &self,
        request: &mut HttpRequest,
        collection_name: &str,
    ) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;

        let collection = ReplaceCollectionBody {
            id: collection_name,
            indexing_policy: &self.indexing_policy,
            partition_key: &self.partition_key,
        };

        request.set_body(bytes::Bytes::from(serde_json::to_string(&collection)?).into());
        Ok(())
    }
}

#[derive(Serialize, Debug)]
struct ReplaceCollectionBody<'a> {
    pub id: &'a str,
    #[serde(rename = "indexingPolicy", skip_serializing_if = "Option::is_none")]
    pub indexing_policy: &'a Option<IndexingPolicy>,
    #[serde(rename = "partitionKey")]
    pub partition_key: &'a PartitionKey,
}

#[derive(Debug, Clone)]
pub struct ReplaceCollectionResponse {
    pub collection: Collection,
    pub lsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
    pub current_replica_set_size: u64,
    pub number_of_read_regions: u32,
    pub etag: String,
    pub charge: f64,
    pub current_write_quorum: u64,
    pub server: String,
    pub collection_partition_index: u64,
    pub global_committed_lsn: u64,
    pub session_token: String,
    pub cosmos_llsn: u64,
    pub xp_role: u32,
    pub gateway_version: String,
    pub collection_service_index: u64,
    pub content_type: String,
    pub transport_request_id: u64,
    pub alt_content_path: String,
    pub service_version: String,
    pub quorum_acked_lsn: u64,
    pub last_state_change: DateTime<Utc>,
    pub date: DateTime<Utc>,
    pub content_location: String,
    pub activity_id: uuid::Uuid,
    pub schema_version: String,
}

impl ReplaceCollectionResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(Self {
            collection: serde_json::from_slice(&body)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            collection_partition_index: collection_partition_index_from_headers(&headers)?,
            collection_service_index: collection_service_index_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(&headers)?,
            server: server_from_headers(&headers)?.to_owned(),
            xp_role: role_from_headers(&headers)?,
            content_type: content_type_from_headers(&headers)?.to_owned(),
            content_location: content_location_from_headers(&headers)?.to_owned(),
            transport_request_id: transport_request_id_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
        })
    }
}
