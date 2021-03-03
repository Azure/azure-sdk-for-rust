mod lease_blob_options;
pub use self::lease_blob_options::{LeaseBlobOptions, LEASE_BLOB_OPTIONS_DEFAULT};
//mod blob_stream_builder;
//pub use self::blob_stream_builder::BlobStreamBuilder;
mod blob_block_type;
pub use self::blob_block_type::BlobBlockType;
//mod list_blob_stream_builder;
//pub use self::list_blob_stream_builder::ListBlobStreamBuilder;
mod block_list_type;
pub use self::block_list_type::BlockListType;
mod blob_block_with_size;
pub use self::blob_block_with_size::BlobBlockWithSize;
mod block_with_size_list;
pub use self::block_with_size_list::BlockWithSizeList;
mod block_list;
pub use self::block_list::BlockList;
pub mod requests;
pub mod responses;
use crate::AccessTier;
use crate::{core::Client, CopyId, CopyProgress};
use azure_core::headers::{
    BLOB_SEQUENCE_NUMBER, BLOB_TYPE, CONTENT_MD5, COPY_COMPLETION_TIME, COPY_ID, COPY_PROGRESS,
    COPY_SOURCE, COPY_STATUS, COPY_STATUS_DESCRIPTION, CREATION_TIME, LEASE_DURATION, LEASE_STATE,
    LEASE_STATUS, META_PREFIX, SERVER_ENCRYPTED,
};
use azure_core::{
    errors::AzureError,
    lease::{LeaseDuration, LeaseState, LeaseStatus},
    parsing::from_azure_time,
    prelude::*,
    util::HeaderMapExt,
};
use chrono::{DateTime, Utc};
use hyper::header;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::str::FromStr;
use std::{collections::HashMap, convert::TryInto};

#[cfg(feature = "azurite_workaround")]
fn get_creation_time(h: &header::HeaderMap) -> Result<Option<DateTime<Utc>>, AzureError> {
    if let Some(creation_time) = h.get(CREATION_TIME) {
        // Check that the creation time is valid
        let creation_time = creation_time.to_str()?;
        let creation_time = DateTime::parse_from_rfc2822(creation_time)?;
        let creation_time = DateTime::from_utc(creation_time.naive_utc(), Utc);
        Ok(Some(creation_time))
    } else {
        // Not having a creation time is ok
        Ok(None)
    }
}

create_enum!(
    BlobType,
    (BlockBlob, "BlockBlob"),
    (PageBlob, "PageBlob"),
    (AppendBlob, "AppendBlob")
);

create_enum!(
    CopyStatus,
    (Pending, "pending"),
    (Success, "success"),
    (Aborted, "aborted"),
    (Failed, "failed")
);

create_enum!(RehydratePriority, (High, "High"), (Standard, "Standard"));

create_enum!(PageWriteType, (Update, "update"), (Clear, "clear"));

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Blob {
    pub name: String,
    pub snapshot: Option<DateTime<Utc>>,
    pub version_id: Option<String>,
    pub is_current_version: Option<bool>,
    pub deleted: Option<bool>,
    pub properties: BlobProperties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlobProperties {
    #[serde(rename = "Creation-Time")]
    #[serde(with = "azure_core::parsing::rfc2822_time_format")]
    pub creation_time: DateTime<Utc>,
    #[serde(rename = "Last-Modified")]
    #[serde(with = "azure_core::parsing::rfc2822_time_format")]
    pub last_modified: DateTime<Utc>,
    #[serde(default)]
    #[serde(with = "azure_core::parsing::rfc2822_time_format_optional")]
    pub last_access_time: Option<DateTime<Utc>>,
    pub etag: Etag,
    #[serde(rename = "Content-Length")]
    pub content_length: u64,
    #[serde(rename = "Content-Type")]
    pub content_type: String,
    #[serde(rename = "Content-Encoding")]
    pub content_encoding: String,
    #[serde(rename = "Content-Language")]
    pub content_language: String,
    #[serde(rename = "Content-Disposition")]
    pub content_disposition: String,
    #[serde(rename = "Content-MD5")]
    pub content_md5: String,
    #[serde(rename = "Content-CRC64")]
    pub content_crc64: String,
    #[serde(rename = "Cache-Control")]
    pub cache_control: String,
    #[serde(rename = "x-ms-blob-sequence-number")]
    pub blob_sequence_number: Option<u64>,
    pub blob_type: BlobType,
    pub access_tier: Option<AccessTier>,
    #[serde(default)]
    #[serde(with = "azure_core::parsing::rfc2822_time_format_optional")]
    pub access_tier_change_time: Option<DateTime<Utc>>,
    pub lease_status: LeaseStatus,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
    pub copy_id: Option<CopyId>,
    pub copy_status: Option<CopyStatus>,
    pub copy_source: Option<String>,
    pub copy_progress: Option<CopyProgress>,
    #[serde(default)]
    #[serde(with = "azure_core::parsing::rfc2822_time_format_optional")]
    pub copy_completion_time: Option<DateTime<Utc>>,
    pub copy_status_description: Option<String>,
    pub server_encrypted: bool,
    pub customer_provided_key_sha256: Option<String>,
    pub encryption_scope: Option<String>,
    pub incremental_copy: Option<bool>,
    pub access_tier_inferred: Option<bool>,
    #[serde(default)]
    #[serde(with = "azure_core::parsing::rfc2822_time_format_optional")]
    pub deleted_time: Option<DateTime<Utc>>,
    pub remaining_retention_days: Option<u32>,
    pub tag_count: Option<u32>,
    pub rehydrate_priority: Option<RehydratePriority>,
    pub metadata: Option<HashMap<String, String>>,
    #[serde(flatten)]
    extra: HashMap<String, String>, // For debug purposes, should be compiled out in the future
}

impl Blob {
    pub(crate) fn from_headers(blob_name: &str, h: &header::HeaderMap) -> Result<Blob, AzureError> {
        trace!("\n{:?}", h);

        #[cfg(not(feature = "azurite_workaround"))]
        let creation_time = {
            let creation_time = h
                .get(CREATION_TIME)
                .ok_or_else(|| AzureError::HeaderNotFound(CREATION_TIME.to_owned()))?
                .to_str()?;
            let creation_time = DateTime::parse_from_rfc2822(creation_time)?;
            let creation_time = DateTime::from_utc(creation_time.naive_utc(), Utc);
            trace!("creation_time == {:?}", creation_time);
            creation_time
        };
        #[cfg(feature = "azurite_workaround")]
        let creation_time = get_creation_time(h)?;

        let content_type = h
            .get_as_string(header::CONTENT_TYPE)
            .unwrap_or_else(|| "application/octet-stream".to_owned());
        trace!("content_type == {:?}", content_type);

        let content_length = h
            .get(header::CONTENT_LENGTH)
            .ok_or_else(|| {
                static CL: header::HeaderName = header::CONTENT_LENGTH;
                AzureError::HeaderNotFound(CL.as_str().to_owned())
            })?
            .to_str()?
            .parse::<u64>()?;
        trace!("content_length == {:?}", content_length);

        let last_modified = h.get_as_str(header::LAST_MODIFIED).ok_or_else(|| {
            static LM: header::HeaderName = header::LAST_MODIFIED;
            AzureError::HeaderNotFound(LM.as_str().to_owned())
        })?;
        let last_modified = from_azure_time(last_modified)?;
        trace!("last_modified == {:?}", last_modified);

        let etag = h.get_as_string(header::ETAG).ok_or_else(|| {
            static E: header::HeaderName = header::ETAG;
            AzureError::HeaderNotFound(E.as_str().to_owned())
        })?;
        let etag = etag.into();
        trace!("etag == {:?}", etag);

        let blob_sequence_number = h.get_as_u64(BLOB_SEQUENCE_NUMBER);
        trace!("blob_sequence_number == {:?}", blob_sequence_number);

        let blob_type = h
            .get_as_str(BLOB_TYPE)
            .ok_or_else(|| AzureError::HeaderNotFound(BLOB_TYPE.to_owned()))?
            .parse::<BlobType>()?;
        trace!("blob_type == {:?}", blob_type);

        let content_encoding = h
            .get_as_string(header::CONTENT_ENCODING)
            .unwrap_or("".to_owned());
        trace!("content_encoding == {:?}", content_encoding);

        let content_language = h
            .get_as_string(header::CONTENT_LANGUAGE)
            .unwrap_or("".to_owned());
        trace!("content_language == {:?}", content_language);

        let content_md5 = h.get_as_string(CONTENT_MD5).unwrap_or("".to_owned());
        trace!("content_md5 == {:?}", content_md5);

        let cache_control = h
            .get_as_string(header::CACHE_CONTROL)
            .unwrap_or("".to_owned());

        let content_disposition = h
            .get_as_string(header::CONTENT_DISPOSITION)
            .unwrap_or("".to_owned());

        let lease_status = h
            .get_as_enum(LEASE_STATUS)?
            .ok_or_else(|| AzureError::HeaderNotFound(LEASE_STATUS.to_owned()))?;
        trace!("lease_status == {:?}", lease_status);

        let lease_state = h
            .get_as_enum(LEASE_STATE)?
            .ok_or_else(|| AzureError::HeaderNotFound(LEASE_STATE.to_owned()))?;
        trace!("lease_state == {:?}", lease_state);

        let lease_duration = h.get_as_enum(LEASE_DURATION)?;
        trace!("lease_duration == {:?}", lease_duration);

        let copy_id = h
            .get_as_string(COPY_ID)
            .map(|c| (&c as &str).try_into())
            .transpose()?;
        trace!("copy_id == {:?}", copy_id);

        let copy_status = h.get_as_enum(COPY_STATUS)?;
        trace!("copy_status == {:?}", copy_status);

        let copy_source = h.get_as_string(COPY_SOURCE);
        trace!("copy_source == {:?}", copy_source);

        let copy_progress = h
            .get_as_str(COPY_PROGRESS)
            .and_then(|cp| Some(CopyProgress::from_str(cp).ok()?));
        trace!("copy_progress == {:?}", copy_progress);

        let copy_completion_time: Option<DateTime<Utc>> =
            h.get_as_str(COPY_COMPLETION_TIME).and_then(|cct| {
                Some(DateTime::from_utc(
                    DateTime::parse_from_rfc2822(cct).ok()?.naive_utc(),
                    Utc,
                ))
            });
        trace!("copy_completion_time == {:?}", copy_completion_time);

        let copy_status_description = h.get_as_string(COPY_STATUS_DESCRIPTION);
        trace!("copy_status_description == {:?}", copy_status_description);

        let server_encrypted = h
            .get_as_str(SERVER_ENCRYPTED)
            .ok_or_else(|| AzureError::HeaderNotFound(SERVER_ENCRYPTED.to_owned()))?
            .parse::<bool>()?;

        let mut metadata = HashMap::new();
        for (name, value) in h.iter() {
            let name = name.as_str();
            if let Some(name) = name.strip_prefix(META_PREFIX) {
                if let Ok(value) = value.to_str() {
                    metadata.insert(name.to_string(), value.to_string());
                }
            }
        }
        let metadata = if metadata.is_empty() {
            None
        } else {
            Some(metadata)
        };

        // TODO: Retrieve the snapshot time from
        // the headers
        let snapshot = None;

        Ok(Blob {
            name: blob_name.to_owned(),
            snapshot,
            deleted: None,            //TODO
            is_current_version: None, //TODO
            version_id: None,         //TODO
            properties: BlobProperties {
                creation_time,
                last_modified: last_modified,
                last_access_time: None, // TODO
                etag: etag,
                content_length,
                content_type: content_type,
                content_encoding,
                content_language,
                content_md5,
                content_crc64: "".to_owned(), // TODO
                cache_control,
                content_disposition,
                blob_sequence_number,
                blob_type,
                access_tier: None,
                lease_status,
                lease_state,
                lease_duration,
                copy_id,
                copy_status,
                copy_source,
                copy_progress,
                copy_completion_time,
                copy_status_description,
                incremental_copy: None, // TODO: Not present or documentation bug?
                server_encrypted,
                customer_provided_key_sha256: None, // TODO
                encryption_scope: None,             // TODO
                access_tier_inferred: None,         // TODO: Not present
                access_tier_change_time: None,      // TODO: Not present
                deleted_time: None,                 // TODO
                remaining_retention_days: None,     // TODO: Not present or documentation bug?
                tag_count: None,                    // TODO
                rehydrate_priority: None,           // TODO
                metadata,
                extra: HashMap::new(),
            },
        })
    }
}

#[inline]
pub(crate) fn generate_blob_uri<C>(
    t: &C,
    container_name: &str,
    blob_name: &str,
    params: Option<&str>,
) -> String
where
    C: Client,
{
    match params {
        Some(ref params) => format!(
            "{}/{}/{}?{}",
            t.blob_uri(),
            utf8_percent_encode(container_name, NON_ALPHANUMERIC),
            utf8_percent_encode(blob_name, NON_ALPHANUMERIC),
            params
        ),
        None => format!(
            "{}/{}/{}",
            t.blob_uri(),
            utf8_percent_encode(container_name, NON_ALPHANUMERIC),
            utf8_percent_encode(blob_name, NON_ALPHANUMERIC),
        ),
    }
}

pub(crate) fn copy_status_from_headers(
    headers: &http::HeaderMap,
) -> Result<CopyStatus, AzureError> {
    let val = headers
        .get_as_str(azure_core::headers::COPY_STATUS)
        .ok_or_else(|| AzureError::HeaderNotFound(azure_core::headers::COPY_STATUS.to_owned()))?;
    Ok(CopyStatus::from_str(val)?)
}
