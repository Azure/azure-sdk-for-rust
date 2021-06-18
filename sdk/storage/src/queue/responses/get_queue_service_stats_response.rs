use azure_core::headers::CommonStorageResponseHeaders;
use azure_core::util::to_str_without_bom;
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

impl std::convert::TryFrom<&Response<Bytes>> for GetQueueServiceStatsResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = to_str_without_bom(response.body())?;

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);
        let response: GetQueueServiceStatsResponseInternal = serde_xml_rs::from_str(body)?;
        debug!("deserde == {:#?}", response);

        Ok(GetQueueServiceStatsResponse {
            common_storage_response_headers: headers.try_into()?,
            status: response.geo_replication.status,
            last_sync_time: response
                .geo_replication
                .last_sync_time
                .map(|t| DateTime::parse_from_rfc2822(&t))
                .transpose()?
                .map(|t| DateTime::from_utc(t.naive_utc(), Utc)),
        })
    }
}
