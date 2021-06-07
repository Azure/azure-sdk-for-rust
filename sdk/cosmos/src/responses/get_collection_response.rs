use crate::headers::from_headers::*;
use crate::resources::Collection;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use chrono::{DateTime, Utc};
use http::response::Response;

#[derive(Debug, Clone, PartialEq)]
pub struct GetCollectionResponse {
    pub collection: Collection,
    pub last_state_change: DateTime<Utc>,
    pub etag: String,
    pub collection_partition_index: u64,
    pub collection_service_index: u64,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub item_lsn: u64,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_item_llsn: u64,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub gateway_version: String,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for GetCollectionResponse {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = response.body();

        debug!("headers == {:#?}", headers);
        debug!("body == {}", std::str::from_utf8(body)?);

        Ok(Self {
            collection: serde_json::from_slice(body)?,
            last_state_change: last_state_change_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            collection_partition_index: collection_partition_index_from_headers(headers)?,
            collection_service_index: collection_service_index_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
            content_path: content_path_from_headers(headers)?.to_owned(),
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            item_lsn: item_lsn_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_item_llsn: cosmos_item_llsn_from_headers(headers)?,
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
        })
    }
}
