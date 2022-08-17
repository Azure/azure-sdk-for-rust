use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::ResourceType;
use azure_core::headers::{item_count_from_headers, session_token_from_headers};
use azure_core::{prelude::*, Response as HttpResponse};
use time::OffsetDateTime;

operation! {
    GetPartitionKeyRanges,
    client: CollectionClient,
    ?if_match_condition: IfMatchCondition,
    ?if_modified_since: IfModifiedSince,
    ?consistency_level: ConsistencyLevel
}

impl GetPartitionKeyRangesBuilder {
    pub fn into_future(self) -> GetPartitionKeyRanges {
        Box::pin(async move {
            let mut request = self.client.cosmos_client().request(
                &format!(
                    "dbs/{}/colls/{}/pkranges",
                    self.client.database_client().database_name(),
                    self.client.collection_name()
                ),
                azure_core::Method::Get,
            );

            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            let response = self
                .client
                .pipeline()
                .send(
                    self.context
                        .clone()
                        .insert(ResourceType::PartitionKeyRanges),
                    &mut request,
                )
                .await?;

            GetPartitionKeyRangesResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPartitionKeyRangesResponse {
    pub rid: String,
    pub content_location: Option<String>,
    pub server: String,
    pub last_state_change: OffsetDateTime,
    pub lsn: u64,
    pub item_count: u32,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub session_token: String,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: OffsetDateTime,
    pub partition_key_ranges: Vec<PartitionKeyRange>,
}

impl GetPartitionKeyRangesResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "_rid")]
            pub rid: String,
            #[serde(rename = "PartitionKeyRanges")]
            pub partition_key_ranges: Vec<PartitionKeyRange>,
        }

        let r: Response = serde_json::from_slice(&body)?;

        Ok(Self {
            rid: r.rid,
            content_location: content_location_from_headers(&headers)?,
            server: server_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            item_count: item_count_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
            partition_key_ranges: r.partition_key_ranges,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Deserialize)]
pub struct PartitionKeyRange {
    #[serde(rename = "_rid")]
    pub rid: String,
    pub id: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(rename = "minInclusive")]
    pub min_exclusive: String,
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: String,
    #[serde(rename = "ridPrefix")]
    pub rid_prefix: u64,
    pub _self: String,
    #[serde(rename = "throughputFraction")]
    pub throughput_fraction: u64,
    pub status: String,
    // TODO: parents
    #[serde(rename = "_ts")]
    pub ts: u64,
}
