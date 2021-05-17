use crate::resource_quota::resource_quotas_from_str;
use crate::resources::document::IndexingDirective;
use crate::ResourceQuota;
use crate::{headers::*, CosmosError};
use chrono::{DateTime, Utc};
use http::HeaderMap;

pub(crate) fn request_charge_from_headers(headers: &HeaderMap) -> Result<f64, CosmosError> {
    Ok(headers
        .get(HEADER_REQUEST_CHARGE)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_REQUEST_CHARGE.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn item_count_from_headers(headers: &HeaderMap) -> Result<u32, CosmosError> {
    Ok(headers
        .get(HEADER_ITEM_COUNT)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_ITEM_COUNT.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn role_from_headers(headers: &HeaderMap) -> Result<u32, CosmosError> {
    Ok(headers
        .get(HEADER_ROLE)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_ROLE.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn number_of_read_regions_from_headers(headers: &HeaderMap) -> Result<u32, CosmosError> {
    Ok(headers
        .get(HEADER_NUMBER_OF_READ_REGIONS)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_NUMBER_OF_READ_REGIONS.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn activity_id_from_headers(headers: &HeaderMap) -> Result<uuid::Uuid, CosmosError> {
    let s = headers
        .get(HEADER_ACTIVITY_ID)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_ACTIVITY_ID.to_owned()))?
        .to_str()?;
    Ok(uuid::Uuid::parse_str(s)?)
}

pub(crate) fn content_path_from_headers(headers: &HeaderMap) -> Result<&str, CosmosError> {
    Ok(headers
        .get(HEADER_CONTENT_PATH)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_CONTENT_PATH.to_owned()))?
        .to_str()?)
}

pub(crate) fn alt_content_path_from_headers(headers: &HeaderMap) -> Result<&str, CosmosError> {
    Ok(headers
        .get(HEADER_ALT_CONTENT_PATH)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_ALT_CONTENT_PATH.to_owned()))?
        .to_str()?)
}

pub(crate) fn resource_quota_from_headers(
    headers: &HeaderMap,
) -> Result<Vec<ResourceQuota>, CosmosError> {
    let s = headers
        .get(HEADER_RESOURCE_QUOTA)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_RESOURCE_QUOTA.to_owned()))?
        .to_str()?;
    Ok(resource_quotas_from_str(s)
        .map_err(|err| CosmosError::GenericErrorWithText(err.to_string()))?)
}

pub(crate) fn resource_usage_from_headers(
    headers: &HeaderMap,
) -> Result<Vec<ResourceQuota>, CosmosError> {
    let s = headers
        .get(HEADER_RESOURCE_USAGE)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_RESOURCE_USAGE.to_owned()))?
        .to_str()?;
    Ok(resource_quotas_from_str(s)
        .map_err(|err| CosmosError::GenericErrorWithText(err.to_string()))?)
}

pub(crate) fn quorum_acked_lsn_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_QUORUM_ACKED_LSN)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_QUORUM_ACKED_LSN.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn quorum_acked_lsn_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<u64>, CosmosError> {
    Ok(match headers.get(HEADER_QUORUM_ACKED_LSN) {
        Some(val) => Some(val.to_str()?.parse()?),
        None => None,
    })
}

pub(crate) fn cosmos_quorum_acked_llsn_from_headers(
    headers: &HeaderMap,
) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_COSMOS_QUORUM_ACKED_LLSN)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_COSMOS_QUORUM_ACKED_LLSN.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn cosmos_quorum_acked_llsn_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<u64>, CosmosError> {
    Ok(match headers.get(HEADER_COSMOS_QUORUM_ACKED_LLSN) {
        Some(val) => Some(val.to_str()?.parse()?),
        None => None,
    })
}

pub(crate) fn current_write_quorum_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_CURRENT_WRITE_QUORUM)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_CURRENT_WRITE_QUORUM.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn current_write_quorum_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<u64>, CosmosError> {
    Ok(match headers.get(HEADER_CURRENT_WRITE_QUORUM) {
        Some(val) => Some(val.to_str()?.parse()?),
        None => None,
    })
}

pub(crate) fn collection_partition_index_from_headers(
    headers: &HeaderMap,
) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_COLLECTION_PARTITION_INDEX)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_COLLECTION_PARTITION_INDEX.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn indexing_directive_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<IndexingDirective>, CosmosError> {
    match headers.get(HEADER_INDEXING_DIRECTIVE) {
        Some(header) => Ok(Some(header.to_str()?.parse()?)),
        None => Ok(None),
    }
}

pub(crate) fn collection_service_index_from_headers(
    headers: &HeaderMap,
) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_COLLECTION_SERVICE_INDEX)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_COLLECTION_SERVICE_INDEX.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn lsn_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_LSN)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_LSN.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn item_lsn_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_ITEM_LSN)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_ITEM_LSN.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn transport_request_id_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_TRANSPORT_REQUEST_ID)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_TRANSPORT_REQUEST_ID.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn global_committed_lsn_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    let s = headers
        .get(HEADER_GLOBAL_COMMITTED_LSN)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_GLOBAL_COMMITTED_LSN.to_owned()))?
        .to_str()?;
    Ok(if s == "-1" { 0 } else { s.parse()? })
}

pub(crate) fn cosmos_llsn_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_COSMOS_LLSN)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_COSMOS_LLSN.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn cosmos_item_llsn_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_COSMOS_ITEM_LLSN)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_COSMOS_ITEM_LLSN.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn current_replica_set_size_from_headers(
    headers: &HeaderMap,
) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_CURRENT_REPLICA_SET_SIZE)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_CURRENT_REPLICA_SET_SIZE.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn current_replica_set_size_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<u64>, CosmosError> {
    Ok(match headers.get(HEADER_CURRENT_REPLICA_SET_SIZE) {
        Some(val) => Some(val.to_str()?.parse()?),
        None => None,
    })
}

pub(crate) fn schema_version_from_headers(headers: &HeaderMap) -> Result<&str, CosmosError> {
    Ok(headers
        .get(HEADER_SCHEMA_VERSION)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_SCHEMA_VERSION.to_owned()))?
        .to_str()?)
}

pub(crate) fn server_from_headers(headers: &HeaderMap) -> Result<&str, CosmosError> {
    let header_server = http::header::SERVER;

    Ok(headers
        .get(http::header::SERVER)
        .ok_or_else(|| CosmosError::HeaderNotFound(header_server.to_string()))?
        .to_str()?)
}

pub(crate) fn service_version_from_headers(headers: &HeaderMap) -> Result<&str, CosmosError> {
    Ok(headers
        .get(HEADER_SERVICE_VERSION)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_SERVICE_VERSION.to_owned()))?
        .to_str()?)
}

pub(crate) fn content_location_from_headers(headers: &HeaderMap) -> Result<&str, CosmosError> {
    Ok(headers
        .get(http::header::CONTENT_LOCATION)
        .ok_or_else(|| {
            let header = http::header::CONTENT_LOCATION;
            CosmosError::HeaderNotFound(header.as_str().to_owned())
        })?
        .to_str()?)
}

pub(crate) fn gateway_version_from_headers(headers: &HeaderMap) -> Result<&str, CosmosError> {
    Ok(headers
        .get(HEADER_GATEWAY_VERSION)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_GATEWAY_VERSION.to_owned()))?
        .to_str()?)
}

pub(crate) fn max_media_storage_usage_mb_from_headers(
    headers: &HeaderMap,
) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_MAX_MEDIA_STORAGE_USAGE_MB)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_MAX_MEDIA_STORAGE_USAGE_MB.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn media_storage_usage_mb_from_headers(headers: &HeaderMap) -> Result<u64, CosmosError> {
    Ok(headers
        .get(HEADER_MEDIA_STORAGE_USAGE_MB)
        .ok_or_else(|| CosmosError::HeaderNotFound(HEADER_MEDIA_STORAGE_USAGE_MB.to_owned()))?
        .to_str()?
        .parse()?)
}

fn _date_from_headers(
    headers: &HeaderMap,
    header_name: &str,
) -> Result<DateTime<Utc>, CosmosError> {
    let date = headers
        .get(header_name)
        .ok_or_else(|| CosmosError::HeaderNotFound(header_name.to_owned()))?
        .to_str()?;
    debug!("date == {:#}", date);

    // since Azure returns "GMT" instead of +0000 as timezone we replace it
    // ourselves.
    // For example: Wed, 15 Jan 2020 23:39:44.369 GMT
    let date = date.replace("GMT", "+0000");
    debug!("date == {:#}", date);

    let date = DateTime::parse_from_str(&date, "%a, %e %h %Y %H:%M:%S%.f %z")?;
    debug!("date == {:#}", date);

    let date = DateTime::from_utc(date.naive_utc(), Utc);
    debug!("date == {:#}", date);

    Ok(date)
}

pub(crate) fn last_state_change_from_headers(
    headers: &HeaderMap,
) -> Result<DateTime<Utc>, CosmosError> {
    _date_from_headers(headers, HEADER_LAST_STATE_CHANGE_UTC)
}

pub(crate) fn date_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>, CosmosError> {
    let header = http::header::DATE;
    _date_from_headers(headers, header.as_str())
}
