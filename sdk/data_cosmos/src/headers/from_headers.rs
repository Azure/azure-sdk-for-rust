use crate::headers::*;
use crate::resource_quota::resource_quotas_from_str;
use crate::resources::document::IndexingDirective;
use crate::ResourceQuota;

use azure_core::error::{Error, ErrorKind};
use azure_core::headers::{self, parse_int, Headers};
use chrono::{DateTime, Utc};
use http::HeaderMap;

pub(crate) fn request_charge_from_headers(headers: &Headers) -> azure_core::Result<f64> {
    headers::get_from_headers(headers, HEADER_REQUEST_CHARGE)
}

pub(crate) fn role_from_headers(headers: &Headers) -> azure_core::Result<u32> {
    headers::get_from_headers(headers, HEADER_ROLE)
}

pub(crate) fn number_of_read_regions_from_headers(headers: &Headers) -> azure_core::Result<u32> {
    headers::get_from_headers(headers, HEADER_NUMBER_OF_READ_REGIONS)
}

pub(crate) fn activity_id_from_headers(headers: &Headers) -> azure_core::Result<uuid::Uuid> {
    headers::get_from_headers(headers, HEADER_ACTIVITY_ID)
}

pub(crate) fn content_path_from_headers(headers: &Headers) -> azure_core::Result<&str> {
    headers::get_str_from_headers(headers, HEADER_CONTENT_PATH)
}

pub(crate) fn alt_content_path_from_headers(headers: &Headers) -> azure_core::Result<&str> {
    headers::get_str_from_headers(headers, HEADER_ALT_CONTENT_PATH)
}

pub(crate) fn resource_quota_from_headers(
    headers: &Headers,
) -> azure_core::Result<Vec<ResourceQuota>> {
    let s = headers::get_str_from_headers(headers, HEADER_RESOURCE_QUOTA)?;
    resource_quotas_from_str(s)
}

pub(crate) fn resource_usage_from_headers(
    headers: &Headers,
) -> azure_core::Result<Vec<ResourceQuota>> {
    let s = headers::get_str_from_headers(headers, HEADER_RESOURCE_USAGE)?;
    resource_quotas_from_str(s)
}

pub(crate) fn quorum_acked_lsn_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_QUORUM_ACKED_LSN)
}

pub(crate) fn quorum_acked_lsn_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<u64>> {
    headers::get_option_from_headers(headers, HEADER_QUORUM_ACKED_LSN)
}

pub(crate) fn cosmos_quorum_acked_llsn_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_COSMOS_QUORUM_ACKED_LLSN)
}

pub(crate) fn cosmos_quorum_acked_llsn_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<u64>> {
    headers::get_option_from_headers(headers, HEADER_COSMOS_QUORUM_ACKED_LLSN)
}

pub(crate) fn current_write_quorum_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_CURRENT_WRITE_QUORUM)
}

pub(crate) fn current_write_quorum_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<u64>> {
    headers::get_option_from_headers(headers, HEADER_CURRENT_WRITE_QUORUM)
}

pub(crate) fn collection_partition_index_from_headers(
    headers: &Headers,
) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_COLLECTION_PARTITION_INDEX)
}

pub(crate) fn indexing_directive_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<IndexingDirective>> {
    headers::get_option_from_headers(headers, HEADER_INDEXING_DIRECTIVE)
}

pub(crate) fn collection_service_index_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_COLLECTION_SERVICE_INDEX)
}

pub(crate) fn lsn_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_LSN)
}

pub(crate) fn item_lsn_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_ITEM_LSN)
}

pub(crate) fn transport_request_id_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_TRANSPORT_REQUEST_ID)
}

pub(crate) fn global_committed_lsn_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    let s = headers::get_str_from_headers(headers, HEADER_GLOBAL_COMMITTED_LSN)?;
    Ok(if s == "-1" {
        0
    } else {
        parse_int(s).map_err(|e| {
            Error::full(
                ErrorKind::DataConversion,
                e,
                format!(
                    "failed to parse header '{}' as int",
                    HEADER_GLOBAL_COMMITTED_LSN,
                ),
            )
        })?
    })
}

pub(crate) fn cosmos_llsn_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_COSMOS_LLSN)
}

pub(crate) fn cosmos_item_llsn_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_COSMOS_ITEM_LLSN)
}

pub(crate) fn current_replica_set_size_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_CURRENT_REPLICA_SET_SIZE)
}

pub(crate) fn current_replica_set_size_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<u64>> {
    headers::get_option_from_headers(headers, HEADER_CURRENT_REPLICA_SET_SIZE)
}

pub(crate) fn schema_version_from_headers(headers: &Headers) -> azure_core::Result<&str> {
    headers::get_str_from_headers(headers, HEADER_SCHEMA_VERSION)
}

pub(crate) fn server_from_headers(headers: &Headers) -> azure_core::Result<&str> {
    headers::get_str_from_headers(headers, &http::header::SERVER.to_string())
}

pub(crate) fn service_version_from_headers(headers: &Headers) -> azure_core::Result<&str> {
    headers::get_str_from_headers(headers, HEADER_SERVICE_VERSION)
}

pub(crate) fn content_location_from_headers(headers: &Headers) -> azure_core::Result<&str> {
    headers::get_str_from_headers(headers, &http::header::CONTENT_LOCATION.to_string())
}

pub(crate) fn gateway_version_from_headers(headers: &Headers) -> azure_core::Result<&str> {
    headers::get_str_from_headers(headers, HEADER_GATEWAY_VERSION)
}

pub(crate) fn max_media_storage_usage_mb_from_headers(
    headers: &Headers,
) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_MAX_MEDIA_STORAGE_USAGE_MB)
}

pub(crate) fn media_storage_usage_mb_from_headers(headers: &Headers) -> azure_core::Result<u64> {
    headers::get_from_headers(headers, HEADER_MEDIA_STORAGE_USAGE_MB)
}

fn _date_from_headers(headers: &Headers, header_name: &str) -> azure_core::Result<DateTime<Utc>> {
    let date = headers::get_str_from_headers(headers, header_name)?;
    // since Azure returns "GMT" instead of +0000 as timezone we replace it ourselves.
    // For example: Wed, 15 Jan 2020 23:39:44.369 GMT
    let date = date.replace("GMT", "+0000");
    let date = headers::parse_date_from_str(&date, "%a, %e %h %Y %H:%M:%S%.f %z")?;
    Ok(DateTime::from_utc(date.naive_utc(), Utc))
}

pub(crate) fn last_state_change_from_headers(
    headers: &Headers,
) -> azure_core::Result<DateTime<Utc>> {
    _date_from_headers(headers, HEADER_LAST_STATE_CHANGE_UTC)
}

pub(crate) fn date_from_headers(headers: &Headers) -> azure_core::Result<DateTime<Utc>> {
    let header = http::header::DATE;
    _date_from_headers(headers, header.as_str())
}
