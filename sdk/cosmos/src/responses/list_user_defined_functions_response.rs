use crate::headers::from_headers::*;
use crate::resources::UserDefinedFunction;
use crate::ResourceQuota;
use azure_core::headers::{
    continuation_token_from_headers_optional, item_count_from_headers, session_token_from_headers,
};
use chrono::{DateTime, Utc};
use http::response::Response;

#[derive(Debug, Clone, PartialEq)]
pub struct ListUserDefinedFunctionsResponse {
    pub rid: String,
    pub user_defined_functions: Vec<UserDefinedFunction>,
    pub content_location: String,
    pub server: String,
    pub last_state_change: DateTime<Utc>,
    pub continuation_token: Option<String>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
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
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for ListUserDefinedFunctionsResponse {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = response.body();

        debug!("{:#?}", headers);
        debug!("{:#?}", std::str::from_utf8(&body)?);

        #[derive(Debug, Deserialize)]
        struct Response<'a> {
            #[serde(rename = "_rid")]
            rid: &'a str,
            #[serde(rename = "UserDefinedFunctions")]
            user_defined_functions: Vec<UserDefinedFunction>,
            #[serde(rename = "_count")]
            count: u32,
        }
        let response: Response = serde_json::from_slice(body)?;

        Ok(Self {
            rid: response.rid.to_owned(),
            user_defined_functions: response.user_defined_functions,
            content_location: content_location_from_headers(headers)?.to_owned(),
            server: server_from_headers(headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(headers)?,
            continuation_token: continuation_token_from_headers_optional(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
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
            session_token: session_token_from_headers(headers)?,
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?.to_owned(),
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
        })
    }
}
