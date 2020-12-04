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
use crate::core::Client;
use azure_core::headers::{
    BLOB_SEQUENCE_NUMBER, BLOB_TYPE, CONTENT_MD5, COPY_COMPLETION_TIME, COPY_ID, COPY_PROGRESS,
    COPY_SOURCE, COPY_STATUS, COPY_STATUS_DESCRIPTION, CREATION_TIME, LEASE_DURATION, LEASE_STATE,
    LEASE_STATUS, META_PREFIX, SERVER_ENCRYPTED,
};
use azure_core::{
    errors::{AzureError, TraversingError},
    incompletevector::IncompleteVector,
    lease::{LeaseDuration, LeaseState, LeaseStatus},
    parsing::{cast_must, cast_optional, from_azure_time, inner_text, traverse},
    range::Range,
    util::HeaderMapExt,
};
use chrono::{DateTime, Utc};
use hyper::header;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::str::FromStr;
use xml::Element;
use xml::Xml::ElementNode;

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

pub trait BlockListTypeSupport {
    type O;
    fn with_block_list_type(self, block_list_type: BlockListType) -> Self::O;
}

pub trait BlockListTypeRequired {
    fn block_list_type(&self) -> BlockListType;

    fn to_uri_parameter(&self) -> String {
        format!("blocklisttype={}", self.block_list_type().to_str())
    }
}

pub trait BlockListSupport<'a, T>
where
    T: Borrow<[u8]>,
{
    type O;
    fn with_block_list(self, _: &'a BlockList<T>) -> Self::O;
}

pub trait BlockListRequired<'a, T>
where
    T: Borrow<[u8]> + 'a,
{
    fn block_list(&self) -> &'a BlockList<T>;

    fn to_string(&self) -> String {
        self.block_list().to_xml()
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

create_enum!(PageWriteType, (Update, "update"), (Clear, "clear"));

#[derive(Debug, Clone, PartialEq)]
pub struct Blob {
    pub name: String,
    pub container_name: String,
    pub snapshot_time: Option<DateTime<Utc>>,
    #[cfg(not(feature = "azurite_workaround"))]
    pub creation_time: DateTime<Utc>,
    #[cfg(feature = "azurite_workaround")]
    pub creation_time: Option<DateTime<Utc>>,
    pub last_modified: Option<DateTime<Utc>>, // optional because unavailable in uncommitted blobs
    pub etag: Option<String>,                 // optional because unavailable in uncommitted blobs
    pub content_length: u64,
    pub content_type: Option<String>,
    pub content_encoding: Option<String>,
    pub content_language: Option<String>,
    pub content_md5: Option<String>,
    pub cache_control: Option<String>,
    pub content_disposition: Option<String>,
    pub x_ms_blob_sequence_number: Option<u64>,
    pub blob_type: BlobType,
    pub access_tier: Option<String>,
    pub lease_status: Option<LeaseStatus>,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
    pub copy_id: Option<String>,
    pub copy_status: Option<CopyStatus>,
    pub copy_source: Option<String>,
    pub copy_progress: Option<Range>,
    pub copy_completion_time: Option<DateTime<Utc>>,
    pub copy_status_description: Option<String>,
    pub incremental_copy: Option<bool>,
    pub server_encrypted: bool,
    pub access_tier_inferred: Option<bool>,
    pub access_tier_change_time: Option<DateTime<Utc>>,
    pub deleted_time: Option<DateTime<Utc>>,
    pub remaining_retention_days: Option<u64>,
    pub metadata: HashMap<String, String>,
}

impl Blob {
    pub(crate) fn parse(elem: &Element, container_name: &str) -> Result<Blob, AzureError> {
        let name = cast_must::<String>(elem, &["Name"])?;
        let snapshot_time = cast_optional::<DateTime<Utc>>(elem, &["Snapshot"])?;

        #[cfg(feature = "azurite_workaround")]
        let creation_time = cast_optional::<DateTime<Utc>>(elem, &["Properties", "Creation-Time"])?;
        #[cfg(not(feature = "azurite_workaround"))]
        let creation_time = cast_must::<DateTime<Utc>>(elem, &["Properties", "Creation-Time"])?;

        let last_modified = cast_optional::<DateTime<Utc>>(elem, &["Properties", "Last-Modified"])?;
        let etag = cast_optional::<String>(elem, &["Properties", "Etag"])?;

        let content_length = cast_must::<u64>(elem, &["Properties", "Content-Length"])?;

        let content_type = cast_must::<String>(elem, &["Properties", "Content-Type"])?;
        let content_encoding = cast_optional::<String>(elem, &["Properties", "Content-Encoding"])?;
        let content_language = cast_optional::<String>(elem, &["Properties", "Content-Language"])?;
        let content_md5 = cast_optional::<String>(elem, &["Properties", "Content-MD5"])?;
        let cache_control = cast_optional::<String>(elem, &["Properties", "Cache-Control"])?;
        let content_disposition =
            cast_optional::<String>(elem, &["Properties", "Content-Disposition"])?;
        let x_ms_blob_sequence_number =
            cast_optional::<u64>(elem, &["Properties", "x-ms-blob-sequence-number"])?;

        let blob_type = cast_must::<BlobType>(elem, &["Properties", "BlobType"])?;
        let access_tier = cast_optional::<String>(elem, &["Properties", "AccessTier"])?;

        let lease_status = cast_optional::<LeaseStatus>(elem, &["Properties", "LeaseStatus"])?;
        let lease_state = cast_must::<LeaseState>(elem, &["Properties", "LeaseState"])?;
        let lease_duration =
            cast_optional::<LeaseDuration>(elem, &["Properties", "LeaseDuration"])?;

        let copy_id = cast_optional::<String>(elem, &["Properties", "CopyId"])?;
        let copy_status = cast_optional::<CopyStatus>(elem, &["Properties", "CopyStatus"])?;
        let copy_source = cast_optional::<String>(elem, &["Properties", "CopySource"])?;
        let copy_progress = cast_optional::<String>(elem, &["Properties", "CopyProgress"])?;
        let copy_completion_time =
            cast_optional::<DateTime<Utc>>(elem, &["Properties", "CopyCompletionTime"])?;
        let copy_status_description =
            cast_optional::<String>(elem, &["Properties", "CopyStatusDescription"])?;

        let server_encrypted = cast_must::<bool>(elem, &["Properties", "ServerEncrypted"])?;
        let incremental_copy = cast_optional::<bool>(elem, &["Properties", "IncrementalCopy"])?;

        // this seems to be either true or absent. We handle absent with None.
        // Previously we returned false in case of absent value but that was
        // misleading.
        // TOCHECK
        let access_tier_inferred =
            cast_optional::<bool>(elem, &["Properties", "AccessTierInferred"])?;

        let access_tier_change_time =
            cast_optional::<DateTime<Utc>>(elem, &["Properties", "AccessTierChangeTime"])?;
        let deleted_time = cast_optional::<DateTime<Utc>>(elem, &["Properties", "DeletedTime"])?;
        let remaining_retention_days =
            cast_optional::<u64>(elem, &["Properties", "RemainingRetentionDays"])?;

        let mut cp_bytes: Option<Range> = None;
        if let Some(txt) = copy_progress {
            cp_bytes = Some(txt.parse::<Range>()?);
        }

        // metadata parsing
        let metadata = {
            let mut metadata: HashMap<String, String> = HashMap::new();
            let mds = traverse(elem, &["Metadata"], true)?;
            for md_node in mds {
                for node in md_node.children.iter() {
                    match node {
                        ElementNode(elem) => {
                            let key = elem.name.to_owned();
                            let value = inner_text(elem)?.to_owned();
                            debug!("key == {:?}, value == {:?}", key, value);
                            metadata.insert(key, value);
                        }
                        _ => {
                            return Err(TraversingError::UnexpectedNodeTypeError(
                                "ElementNode".to_owned(),
                            )
                            .into())
                        }
                    }
                }
            }
            metadata
        };

        Ok(Blob {
            name,
            container_name: container_name.to_owned(),
            snapshot_time,
            creation_time,
            last_modified,
            etag,
            content_length,
            content_type: Some(content_type),
            content_encoding,
            content_language,
            content_md5,
            cache_control,
            content_disposition,
            x_ms_blob_sequence_number,
            blob_type,
            access_tier,
            lease_status,
            lease_state,
            lease_duration,
            copy_id,
            copy_status,
            copy_source,
            copy_progress: cp_bytes,
            copy_completion_time,
            copy_status_description,
            incremental_copy,
            server_encrypted,
            access_tier_inferred,
            access_tier_change_time,
            deleted_time,
            remaining_retention_days,
            metadata,
        })
    }

    pub(crate) fn from_headers(
        blob_name: &str,
        container_name: &str,
        snapshot_time: Option<DateTime<Utc>>,
        h: &header::HeaderMap,
    ) -> Result<Blob, AzureError> {
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
        trace!("etag == {:?}", etag);

        let x_ms_blob_sequence_number = h.get_as_u64(BLOB_SEQUENCE_NUMBER);
        trace!(
            "x_ms_blob_sequence_number == {:?}",
            x_ms_blob_sequence_number
        );

        let blob_type = h
            .get_as_str(BLOB_TYPE)
            .ok_or_else(|| AzureError::HeaderNotFound(BLOB_TYPE.to_owned()))?
            .parse::<BlobType>()?;
        trace!("blob_type == {:?}", blob_type);

        let content_encoding = h.get_as_string(header::CONTENT_ENCODING);
        trace!("content_encoding == {:?}", content_encoding);

        let content_language = h.get_as_string(header::CONTENT_LANGUAGE);
        trace!("content_language == {:?}", content_language);

        let content_md5 = h.get_as_string(CONTENT_MD5);
        trace!("content_md5 == {:?}", content_md5);

        let cache_control = h.get_as_string(header::CACHE_CONTROL);

        let content_disposition = h.get_as_string(header::CONTENT_DISPOSITION);

        let lease_status = Some(
            h.get_as_enum(LEASE_STATUS)?
                .ok_or_else(|| AzureError::HeaderNotFound(LEASE_STATUS.to_owned()))?,
        );
        trace!("lease_status == {:?}", lease_status);

        let lease_state = h
            .get_as_enum(LEASE_STATE)?
            .ok_or_else(|| AzureError::HeaderNotFound(LEASE_STATE.to_owned()))?;
        trace!("lease_state == {:?}", lease_state);

        let lease_duration = h.get_as_enum(LEASE_DURATION)?;
        trace!("lease_duration == {:?}", lease_duration);

        let copy_id = h.get_as_string(COPY_ID);
        trace!("copy_id == {:?}", copy_id);

        let copy_status = h.get_as_enum(COPY_STATUS)?;
        trace!("copy_status == {:?}", copy_status);

        let copy_source = h.get_as_string(COPY_SOURCE);
        trace!("copy_source == {:?}", copy_source);

        let copy_progress = h
            .get_as_str(COPY_PROGRESS)
            .and_then(|cp| Some(Range::from_str(cp).ok()?));
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

        Ok(Blob {
            name: blob_name.to_owned(),
            container_name: container_name.to_owned(),
            snapshot_time,
            creation_time,
            last_modified: Some(last_modified),
            etag: Some(etag),
            content_length,
            content_type: Some(content_type),
            content_encoding,
            content_language,
            content_md5,
            cache_control,
            content_disposition,
            x_ms_blob_sequence_number,
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
            access_tier_inferred: None,     // TODO: Not present
            access_tier_change_time: None,  // TODO: Not present
            deleted_time: None,             // TODO
            remaining_retention_days: None, // TODO: Not present or documentation bug?
            metadata,
        })
    }
}

#[inline]
pub(crate) fn incomplete_vector_from_response(
    body: &str,
    container_name: &str,
) -> Result<IncompleteVector<Blob>, AzureError> {
    trace!("body = {}", body);
    trace!("body = {}", body);

    let elem: Element = body.parse()?;

    let next_marker = match cast_optional::<String>(&elem, &["NextMarker"])? {
        Some(ref nm) if nm == "" => None,
        Some(nm) => Some(nm),
        None => None,
    };

    debug!("next_marker == {:?}", next_marker);

    let mut v = Vec::new();
    for node_blob in traverse(&elem, &["Blobs", "Blob"], true)? {
        //trace!("{:?}", node_blob);
        v.push(Blob::parse(node_blob, container_name)?);
    }

    Ok(IncompleteVector::<Blob>::new(next_marker, v))
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
