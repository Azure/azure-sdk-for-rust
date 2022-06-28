use crate::QueueServiceClient;
use azure_core::{
    collect_pinned_stream,
    error::{ErrorKind, ResultExt},
    prelude::*,
    Method, Response as AzureResponse,
};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use chrono::{DateTime, Utc};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueServiceStatsBuilder {
    service_client: QueueServiceClient,
    timeout: Option<Timeout>,
    context: Context,
}

impl GetQueueServiceStatsBuilder {
    pub(crate) fn new(service_client: QueueServiceClient) -> Self {
        Self {
            service_client,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self
                .service_client
                .storage_client
                .storage_account_client()
                .queue_storage_secondary_url()
                .to_owned();

            url.query_pairs_mut().append_pair("restype", "service");
            url.query_pairs_mut().append_pair("comp", "stats");

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.service_client
                    .storage_client
                    .prepare_request(url, Method::GET, None)?;

            let response = self
                .service_client
                .send(&mut self.context, &mut request)
                .await?;

            GetQueueServiceStatsResponse::try_from(response).await
        })
    }
}

pub type Response =
    futures::future::BoxFuture<'static, azure_core::Result<GetQueueServiceStatsResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetQueueServiceStatsBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
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
