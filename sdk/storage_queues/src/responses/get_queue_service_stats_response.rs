use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use azure_storage::core::xml::read_xml;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::response::Response;
use std::convert::TryInto;

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

impl std::convert::TryFrom<CollectedResponse> for GetQueueServiceStatsResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let headers = response.headers();
        let body = response.body();

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);
        let response: GetQueueServiceStatsResponseInternal =
            read_xml(body).map_kind(ErrorKind::DataConversion)?;
        debug!("deserde == {:#?}", response);

        Ok(GetQueueServiceStatsResponse {
            common_storage_response_headers: headers.try_into()?,
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
