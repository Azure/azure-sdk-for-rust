use crate::from_headers::*;
use crate::PartitionKeyRange;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::session_token_from_headers;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetPartitionKeyRangesResponse {
    pub rid: String,
    pub content_location: String,
    pub server: String,
    pub last_state_change: DateTime<Utc>,
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
    pub date: DateTime<Utc>,
    pub partition_key_ranges: Vec<PartitionKeyRange>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for GetPartitionKeyRangesResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("body == {}", std::str::from_utf8(body)?);

        debug!("headers == {:#?}", headers);

        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "_rid")]
            pub rid: String,
            #[serde(rename = "PartitionKeyRanges")]
            pub partition_key_ranges: Vec<PartitionKeyRange>,
        }

        let r: Response = serde_json::from_slice(body)?;

        Ok(Self {
            rid: r.rid,
            content_location: content_location_from_headers(headers)?.to_owned(),
            server: server_from_headers(headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            item_count: item_count_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            role: role_from_headers(headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
            partition_key_ranges: r.partition_key_ranges,
        })
    }
}
