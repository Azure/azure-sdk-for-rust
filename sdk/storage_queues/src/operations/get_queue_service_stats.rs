use crate::QueueServiceClient;
use azure_core::{
    collect_pinned_stream,
    error::{ErrorKind, ResultExt},
    headers::Headers,
    Method, Response as AzureResponse,
};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use chrono::{DateTime, Utc};
use std::convert::TryInto;

operation! {
    GetQueueServiceStats,
    client: QueueServiceClient,
}

impl GetQueueServiceStatsBuilder {
    pub fn into_future(mut self) -> GetQueueServiceStats {
        Box::pin(async move {
            let mut url = self
                .client
                .storage_client
                .queue_storage_secondary_url()
                .to_owned();

            url.query_pairs_mut().append_pair("restype", "service");
            url.query_pairs_mut().append_pair("comp", "stats");

            let mut request = self.client.storage_client.finalize_request(
                url,
                Method::Get,
                Headers::new(),
                None,
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            GetQueueServiceStatsResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Live,
    Bootstrap,
    Unavailable,
}

#[derive(Debug, Clone)]
pub struct GetQueueServiceStatsResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub status: Status,
    pub last_sync_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GetQueueServiceStatsResponseInternal {
    pub geo_replication: GeoReplication,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GeoReplication {
    pub status: Status,
    pub last_sync_time: Option<String>,
}

impl GetQueueServiceStatsResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let response: GetQueueServiceStatsResponseInternal = read_xml(&body)?;

        Ok(GetQueueServiceStatsResponse {
            common_storage_response_headers: (&headers).try_into()?,
            status: response.geo_replication.status,
            last_sync_time: response
                .geo_replication
                .last_sync_time
                .map(|t| DateTime::parse_from_rfc2822(&t))
                .transpose()
                .context(ErrorKind::DataConversion, "failed to parse last sync time")?
                .map(|t| DateTime::from_utc(t.naive_utc(), Utc)),
        })
    }
}
