use crate::headers::*;
use crate::resource_quota::resource_quotas_from_str;
use crate::resources::document::IndexingDirective;
use crate::ResourceQuota;
use azure_core::errors::HeaderError;
use chrono::{DateTime, Utc};
use http::HeaderMap;

pub(crate) fn request_charge_from_headers(headers: &HeaderMap) -> Result<f64, HeaderError> {
    parse(headers, HEADER_REQUEST_CHARGE)
}

pub(crate) fn item_count_from_headers(headers: &HeaderMap) -> Result<u32, HeaderError> {
    parse(headers, HEADER_ITEM_COUNT)
}

pub(crate) fn role_from_headers(headers: &HeaderMap) -> Result<u32, HeaderError> {
    parse(headers, HEADER_ROLE)
}

pub(crate) fn number_of_read_regions_from_headers(headers: &HeaderMap) -> Result<u32, HeaderError> {
    parse(headers, HEADER_NUMBER_OF_READ_REGIONS)
}

pub(crate) fn activity_id_from_headers(headers: &HeaderMap) -> Result<uuid::Uuid, HeaderError> {
    extract_with_parse(headers, HEADER_ACTIVITY_ID, uuid::Uuid::parse_str)
}

pub(crate) fn content_path_from_headers(headers: &HeaderMap) -> Result<&str, HeaderError> {
    extract(headers, HEADER_CONTENT_PATH)
}

pub(crate) fn alt_content_path_from_headers(headers: &HeaderMap) -> Result<&str, HeaderError> {
    extract(headers, HEADER_ALT_CONTENT_PATH)
}

pub(crate) fn resource_quota_from_headers(
    headers: &HeaderMap,
) -> Result<Vec<ResourceQuota>, HeaderError> {
    extract_with_parse(headers, HEADER_RESOURCE_QUOTA, resource_quotas_from_str)
}

pub(crate) fn resource_usage_from_headers(
    headers: &HeaderMap,
) -> Result<Vec<ResourceQuota>, HeaderError> {
    extract_with_parse(headers, HEADER_RESOURCE_USAGE, resource_quotas_from_str)
}

pub(crate) fn quorum_acked_lsn_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    parse(headers, HEADER_QUORUM_ACKED_LSN)
}

pub(crate) fn quorum_acked_lsn_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<u64>, HeaderError> {
    parse_optional(headers, HEADER_QUORUM_ACKED_LSN)
}

pub(crate) fn cosmos_quorum_acked_llsn_from_headers(
    headers: &HeaderMap,
) -> Result<u64, HeaderError> {
    parse(headers, HEADER_COSMOS_QUORUM_ACKED_LLSN)
}

pub(crate) fn cosmos_quorum_acked_llsn_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<u64>, HeaderError> {
    parse_optional(headers, HEADER_COSMOS_QUORUM_ACKED_LLSN)
}

pub(crate) fn current_write_quorum_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    parse(headers, HEADER_CURRENT_WRITE_QUORUM)
}

pub(crate) fn current_write_quorum_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<u64>, HeaderError> {
    parse_optional(headers, HEADER_CURRENT_WRITE_QUORUM)
}

pub(crate) fn collection_partition_index_from_headers(
    headers: &HeaderMap,
) -> Result<u64, HeaderError> {
    parse(headers, HEADER_COLLECTION_PARTITION_INDEX)
}

pub(crate) fn indexing_directive_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<IndexingDirective>, HeaderError> {
    parse_optional(headers, HEADER_INDEXING_DIRECTIVE)
}

pub(crate) fn collection_service_index_from_headers(
    headers: &HeaderMap,
) -> Result<u64, HeaderError> {
    parse(headers, HEADER_COLLECTION_SERVICE_INDEX)
}

pub(crate) fn lsn_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    parse(headers, HEADER_LSN)
}

pub(crate) fn item_lsn_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    parse(headers, HEADER_ITEM_LSN)
}

pub(crate) fn transport_request_id_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    parse(headers, HEADER_TRANSPORT_REQUEST_ID)
}

pub(crate) fn global_committed_lsn_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    extract_with_parse(headers, HEADER_GLOBAL_COMMITTED_LSN, |s| {
        if s == "-1" {
            Ok(0)
        } else {
            s.parse()
        }
    })
}

pub(crate) fn cosmos_llsn_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    parse(headers, HEADER_COSMOS_LLSN)
}

pub(crate) fn cosmos_item_llsn_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    parse(headers, HEADER_COSMOS_ITEM_LLSN)
}

pub(crate) fn current_replica_set_size_from_headers(
    headers: &HeaderMap,
) -> Result<u64, HeaderError> {
    parse(headers, HEADER_CURRENT_REPLICA_SET_SIZE)
}

pub(crate) fn current_replica_set_size_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<u64>, HeaderError> {
    parse_optional(headers, HEADER_CURRENT_REPLICA_SET_SIZE)
}

pub(crate) fn schema_version_from_headers(headers: &HeaderMap) -> Result<&str, HeaderError> {
    extract(headers, HEADER_SCHEMA_VERSION)
}

pub(crate) fn server_from_headers(headers: &HeaderMap) -> Result<&str, HeaderError> {
    extract(headers, http::header::SERVER.as_str())
}

pub(crate) fn service_version_from_headers(headers: &HeaderMap) -> Result<&str, HeaderError> {
    extract(headers, HEADER_SERVICE_VERSION)
}

pub(crate) fn content_location_from_headers(headers: &HeaderMap) -> Result<&str, HeaderError> {
    extract(headers, http::header::CONTENT_LOCATION.as_str())
}

pub(crate) fn gateway_version_from_headers(headers: &HeaderMap) -> Result<&str, HeaderError> {
    extract(headers, HEADER_GATEWAY_VERSION)
}

pub(crate) fn max_media_storage_usage_mb_from_headers(
    headers: &HeaderMap,
) -> Result<u64, HeaderError> {
    parse(headers, HEADER_MAX_MEDIA_STORAGE_USAGE_MB)
}

pub(crate) fn media_storage_usage_mb_from_headers(headers: &HeaderMap) -> Result<u64, HeaderError> {
    parse(headers, HEADER_MEDIA_STORAGE_USAGE_MB)
}

fn _date_from_headers(
    headers: &HeaderMap,
    header_name: &str,
) -> Result<DateTime<Utc>, HeaderError> {
    let date = extract(headers, header_name)?;
    debug!("date == {:#}", date);

    // since Azure returns "GMT" instead of +0000 as timezone we replace it ourselves.
    // For example: Wed, 15 Jan 2020 23:39:44.369 GMT
    let date = date.replace("GMT", "+0000");
    debug!("date == {:#}", date);

    let date = DateTime::parse_from_str(&date, "%a, %e %h %Y %H:%M:%S%.f %z").map_err(|e| {
        HeaderError::ParsingError {
            name: header_name.to_owned(),
            error: e.into(),
        }
    })?;
    debug!("date == {:#}", date);

    let date = DateTime::from_utc(date.naive_utc(), Utc);
    debug!("date == {:#}", date);

    Ok(date)
}

pub(crate) fn last_state_change_from_headers(
    headers: &HeaderMap,
) -> Result<DateTime<Utc>, HeaderError> {
    _date_from_headers(headers, HEADER_LAST_STATE_CHANGE_UTC)
}

pub(crate) fn date_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>, HeaderError> {
    _date_from_headers(headers, http::header::DATE.as_str())
}

/// Get header with `name` from `headers` and parse it as `T`
fn parse<T>(headers: &HeaderMap, name: &str) -> Result<T, HeaderError>
where
    T: std::str::FromStr,
    T::Err: Send + Sync + std::error::Error + 'static,
{
    extract_with_parse(headers, name, |val| val.parse())
}

/// Get header with `name` from `headers` and parse it as `T` or return `None` if it's not there
fn parse_optional<T>(headers: &HeaderMap, name: &str) -> Result<Option<T>, HeaderError>
where
    T: std::str::FromStr,
    T::Err: Send + Sync + std::error::Error + 'static,
{
    extract_optional_with_parse(headers, name, |val| val.parse())
}

fn extract_optional_with_parse<'a, F, T, E>(
    headers: &'a HeaderMap,
    name: &str,
    parse: F,
) -> Result<Option<T>, HeaderError>
where
    F: Fn(&'a str) -> Result<T, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let val = extract_optional(headers, name)?;
    val.map(parse)
        .transpose()
        .map_err(|e| HeaderError::ParsingError {
            name: name.to_owned(),
            error: e.into(),
        })
}

fn extract_with_parse<'a, F, T, E>(
    headers: &'a HeaderMap,
    name: &str,
    parse: F,
) -> Result<T, HeaderError>
where
    F: Fn(&'a str) -> Result<T, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let val = extract(headers, name)?;
    parse(val).map_err(|e| HeaderError::ParsingError {
        name: name.to_owned(),
        error: e.into(),
    })
}

fn extract<'a>(headers: &'a HeaderMap, name: &str) -> Result<&'a str, HeaderError> {
    extract_optional(headers, name)?.ok_or_else(|| HeaderError::not_found(name.to_owned()))
}

fn extract_optional<'a>(
    headers: &'a HeaderMap,
    name: &str,
) -> Result<Option<&'a str>, HeaderError> {
    headers
        .get(name)
        .map(|v| {
            v.to_str().map_err(|e| HeaderError::ValueNotUtf8 {
                name: name.to_owned(),
            })
        })
        .transpose()
}
