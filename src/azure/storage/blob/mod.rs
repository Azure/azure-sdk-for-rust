mod lease_blob_options;
pub use self::lease_blob_options::{LeaseBlobOptions, LEASE_BLOB_OPTIONS_DEFAULT};

mod blob_block_type;
mod blob_stream;
pub use self::blob_block_type::BlobBlockType;

mod block_list_type;
pub use self::block_list_type::BlockListType;

mod blob_block_with_size;
pub use self::blob_block_with_size::BlobBlockWithSize;

mod block_with_size_list;
pub use self::block_with_size_list::BlockWithSizeList;

use azure::storage::IntoAzurePath;

mod block_list;
pub use self::block_list::BlockList;

pub mod requests;
pub mod responses;

mod get_block_list_response;
pub use self::get_block_list_response::GetBlockListResponse;

use azure::core::headers::{
    BLOB_SEQUENCE_NUMBER, BLOB_TYPE, CLIENT_REQUEST_ID, CONTENT_MD5, COPY_COMPLETION_TIME, COPY_ID, COPY_PROGRESS, COPY_SOURCE,
    COPY_STATUS, COPY_STATUS_DESCRIPTION, CREATION_TIME, LEASE_ACTION, LEASE_BREAK_PERIOD, LEASE_DURATION, LEASE_ID, LEASE_STATE,
    LEASE_STATUS, PROPOSED_LEASE_ID, REQUEST_ID, SERVER_ENCRYPTED,
};
use base64;
use chrono::{DateTime, Utc};
use futures::{future::*, prelude::*};
use hyper::{self, header, Method, StatusCode};
use md5;
use std::collections::HashMap;
use std::{borrow::Borrow, fmt, str::FromStr};
use uuid::Uuid;
use xml::Element;
use xml::Xml::ElementNode;

use azure::core::{
    enumerations,
    errors::{check_status_extract_body, check_status_extract_headers_and_body, AzureError, TraversingError},
    incompletevector::IncompleteVector,
    lease::{LeaseAction, LeaseDuration, LeaseId, LeaseState, LeaseStatus},
    parsing::{cast_must, cast_optional, from_azure_time, inner_text, traverse, FromStringOptional},
    range::Range,
    util::{HeaderMapExt, RequestBuilderExt},
};
use azure::storage::client::Client;

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
    pub creation_time: DateTime<Utc>,
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
    pub fn parse(elem: &Element, container_name: &str) -> Result<Blob, AzureError> {
        let name = cast_must::<String>(elem, &["Name"])?;
        let snapshot_time = cast_optional::<DateTime<Utc>>(elem, &["Snapshot"])?;
        let creation_time = cast_must::<DateTime<Utc>>(elem, &["Properties", "Creation-Time"])?;
        let last_modified = cast_optional::<DateTime<Utc>>(elem, &["Properties", "Last-Modified"])?;
        let etag = cast_optional::<String>(elem, &["Properties", "Etag"])?;

        let content_length = cast_must::<u64>(elem, &["Properties", "Content-Length"])?;

        let content_type = cast_must::<String>(elem, &["Properties", "Content-Type"])?;
        let content_encoding = cast_optional::<String>(elem, &["Properties", "Content-Encoding"])?;
        let content_language = cast_optional::<String>(elem, &["Properties", "Content-Language"])?;
        let content_md5 = cast_optional::<String>(elem, &["Properties", "Content-MD5"])?;
        let cache_control = cast_optional::<String>(elem, &["Properties", "Cache-Control"])?;
        let content_disposition = cast_optional::<String>(elem, &["Properties", "Content-Disposition"])?;
        let x_ms_blob_sequence_number = cast_optional::<u64>(elem, &["Properties", "x-ms-blob-sequence-number"])?;

        let blob_type = cast_must::<BlobType>(elem, &["Properties", "BlobType"])?;
        let access_tier = cast_optional::<String>(elem, &["Properties", "AccessTier"])?;

        let lease_status = cast_optional::<LeaseStatus>(elem, &["Properties", "LeaseStatus"])?;
        let lease_state = cast_must::<LeaseState>(elem, &["Properties", "LeaseState"])?;
        let lease_duration = cast_optional::<LeaseDuration>(elem, &["Properties", "LeaseDuration"])?;

        let copy_id = cast_optional::<String>(elem, &["Properties", "CopyId"])?;
        let copy_status = cast_optional::<CopyStatus>(elem, &["Properties", "CopyStatus"])?;
        let copy_source = cast_optional::<String>(elem, &["Properties", "CopySource"])?;
        let copy_progress = cast_optional::<String>(elem, &["Properties", "CopyProgress"])?;
        let copy_completion_time = cast_optional::<DateTime<Utc>>(elem, &["Properties", "CopyCompletionTime"])?;
        let copy_status_description = cast_optional::<String>(elem, &["Properties", "CopyStatusDescription"])?;

        let server_encrypted = cast_must::<bool>(elem, &["Properties", "ServerEncrypted"])?;
        let incremental_copy = cast_optional::<bool>(elem, &["Properties", "IncrementalCopy"])?;

        // this seems to be either true or absent. We handle absent with None.
        // Previously we returned false in case of absent value but that was
        // misleading.
        // TOCHECK
        let access_tier_inferred = cast_optional::<bool>(elem, &["Properties", "AccessTierInferred"])?;

        let access_tier_change_time = cast_optional::<DateTime<Utc>>(elem, &["Properties", "AccessTierChangeTime"])?;
        let deleted_time = cast_optional::<DateTime<Utc>>(elem, &["Properties", "DeletedTime"])?;
        let remaining_retention_days = cast_optional::<u64>(elem, &["Properties", "RemainingRetentionDays"])?;

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
                        _ => return Err(TraversingError::UnexpectedNodeTypeError("ElementNode".to_owned()).into()),
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
            access_tier_inferred: access_tier_inferred,
            access_tier_change_time,
            deleted_time,
            remaining_retention_days,
            metadata,
        })
    }

    pub fn from_headers(
        blob_name: &str,
        container_name: &str,
        snapshot_time: Option<DateTime<Utc>>,
        h: &header::HeaderMap,
    ) -> Result<Blob, AzureError> {
        trace!("\n{:?}", h);

        let creation_time = h
            .get(CREATION_TIME)
            .ok_or_else(|| AzureError::HeaderNotFound(CREATION_TIME.to_owned()))?
            .to_str()?;
        let creation_time = DateTime::parse_from_rfc2822(creation_time)?;
        let creation_time = DateTime::from_utc(creation_time.naive_utc(), Utc);
        trace!("creation_time == {:?}", creation_time);

        let content_type = h
            .get_as_string(header::CONTENT_TYPE)
            .unwrap_or_else(|| "application/octet-stream".to_owned());
        trace!("content_type == {:?}", content_type);

        let content_length = h
            .get(header::CONTENT_LENGTH)
            .ok_or_else(|| AzureError::HeaderNotFound(header::CONTENT_LENGTH.as_str().to_owned()))?
            .to_str()?
            .parse::<u64>()?;
        trace!("content_length == {:?}", content_length);

        let last_modified = h
            .get_as_str(header::LAST_MODIFIED)
            .ok_or_else(|| AzureError::HeaderNotFound(header::LAST_MODIFIED.as_str().to_owned()))?;
        let last_modified = from_azure_time(last_modified)?;
        trace!("last_modified == {:?}", last_modified);

        let etag = h
            .get_as_string(header::ETAG)
            .ok_or_else(|| AzureError::HeaderNotFound(header::ETAG.as_str().to_owned()))?;
        trace!("etag == {:?}", etag);

        let x_ms_blob_sequence_number = h.get_as_u64(BLOB_SEQUENCE_NUMBER);
        trace!("x_ms_blob_sequence_number == {:?}", x_ms_blob_sequence_number);

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

        let copy_progress = h.get_as_str(COPY_PROGRESS).and_then(|cp| Some(Range::from_str(cp).ok()?));
        trace!("copy_progress == {:?}", copy_progress);

        let copy_completion_time: Option<DateTime<Utc>> = h
            .get_as_str(COPY_COMPLETION_TIME)
            .and_then(|cct| Some(DateTime::from_utc(DateTime::parse_from_rfc2822(cct).ok()?.naive_utc(), Utc)));
        trace!("copy_completion_time == {:?}", copy_completion_time);

        let copy_status_description = h.get_as_string(COPY_STATUS_DESCRIPTION);
        trace!("copy_status_description == {:?}", copy_status_description);

        let server_encrypted = h
            .get_as_str(SERVER_ENCRYPTED)
            .ok_or_else(|| AzureError::HeaderNotFound(SERVER_ENCRYPTED.to_owned()))?
            .parse::<bool>()?;

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
            metadata: HashMap::new(),       // TODO: Not present or documentation bug?
        })
    }

    pub fn stream<'a>(
        c: &'a Client,
        container_name: &'a str,
        blob_name: &'a str,
        snapshot: Option<&'a DateTime<Utc>>,
        range: &Range,
        lease_id: Option<&'a LeaseId>,
        increment: u64,
    ) -> impl Stream<Item = Vec<u8>, Error = AzureError> + 'a {
        blob_stream::BlobStream::new(c, container_name, blob_name, snapshot, range, lease_id, increment)
    }

    pub fn lease(&self, c: &Client, la: LeaseAction, lbo: &LeaseBlobOptions) -> impl Future<Item = LeaseId, Error = AzureError> {
        let mut uri = format!(
            "https://{}.blob.core.windows.net/{}/{}?comp=lease",
            c.account(),
            self.container_name,
            self.name
        );
        if let Some(ref timeout) = lbo.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let req = c.perform_request(
            &uri,
            Method::PUT,
            move |ref mut request| {
                if let Some(ref lease_id) = lbo.lease_id {
                    request.header_formatted(LEASE_ID, lease_id);
                }

                request.header_formatted(LEASE_ACTION, la);

                if let Some(lease_break_period) = lbo.lease_break_period {
                    request.header_formatted(LEASE_BREAK_PERIOD, lease_break_period);
                }
                if let Some(lease_duration) = lbo.lease_duration {
                    request.header_formatted(LEASE_DURATION, lease_duration);
                }
                if let Some(ref proposed_lease_id) = lbo.proposed_lease_id {
                    request.header_formatted(PROPOSED_LEASE_ID, proposed_lease_id);
                }
                if let Some(ref request_id) = lbo.request_id {
                    request.header_formatted(CLIENT_REQUEST_ID, request_id);
                }
            },
            // this fix is needed to avoid
            // receiving HTTP Error 411. The request must be chunked or have a content length
            // This happens since hyper 0.11.2
            Some(b""),
        );

        let expected_result = match la {
            LeaseAction::Acquire => StatusCode::CREATED,
            LeaseAction::Renew | LeaseAction::Change | LeaseAction::Release => StatusCode::OK,
            LeaseAction::Break => StatusCode::ACCEPTED,
        };

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_headers_and_body(future_response, expected_result))
            .and_then(|(headers, _)| {
                headers
                    .get_as_str(LEASE_ID)
                    .and_then(|s| s.parse::<Uuid>().ok())
                    .ok_or_else(|| AzureError::HeaderNotFound("x-ms-lease-id".to_owned()))
            })
    }

    pub fn delete(
        c: &Client,
        container_name: &str,
        blob_name: &str,
        lease_id: Option<&LeaseId>,
    ) -> impl Future<Item = (), Error = AzureError> {
        let uri = format!("https://{}.blob.core.windows.net/{}/{}", c.account(), container_name, blob_name);

        let req = c.perform_request(
            &uri,
            Method::DELETE,
            |ref mut request| {
                if let Some(lease_id) = lease_id {
                    request.header_formatted(LEASE_ID, lease_id);
                }
            },
            None,
        );

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::ACCEPTED))
            .and_then(|_| ok(()))
    }
}

fn put_block_list_prepare_request<P, T>(
    c: &Client,
    path: &P,
    timeout: Option<u64>,
    lease_id: Option<&LeaseId>,
    block_ids: &BlockList<T>,
) -> Result<hyper::client::ResponseFuture, AzureError>
where
    P: IntoAzurePath,
    T: Borrow<str>,
{
    let container_name = path.container_name()?;
    let blob_name = path.blob_name()?;

    let mut uri = format!(
        "https://{}.blob.core.windows.net/{}/{}?comp=blocklist",
        c.account(),
        container_name,
        blob_name,
    );

    if let Some(ref timeout) = timeout {
        uri = format!("{}&timeout={}", uri, timeout);
    }

    // create the blocklist XML
    let xml = block_ids.to_xml();
    let xml_bytes = xml.as_bytes();

    // calculate the xml MD5. This can be made optional
    // in a future version.
    let md5 = {
        let hash = md5::compute(xml_bytes);
        debug!("md5 hash: {:02X}", hash);
        base64::encode(&*hash)
    };

    // now create the request
    c.perform_request(
        &uri,
        Method::PUT,
        move |ref mut request| {
            request.header_formatted(header::CONTENT_LENGTH, xml_bytes.len());
            request.header_formatted(CONTENT_MD5, md5);
            if let Some(lease_id) = lease_id {
                request.header_formatted(LEASE_ID, *lease_id);
            }
        },
        Some(xml_bytes),
    )
}

pub fn put_block_list<P, T>(
    c: &Client,
    path: &P,
    timeout: Option<u64>,
    lease_id: Option<&LeaseId>,
    block_ids: &BlockList<T>,
) -> impl Future<Item = (), Error = AzureError>
where
    P: IntoAzurePath,
    T: Borrow<str>,
{
    done(put_block_list_prepare_request(c, path, timeout, lease_id, block_ids))
        .from_err()
        .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::CREATED))
        .and_then(|_| ok(()))
}

fn get_block_list_create_request<P>(
    c: &Client,
    path: &P,
    bl: &BlockListType,
    timeout: Option<u64>,
    lease_id: Option<&LeaseId>,
    request_id: Option<String>,
    snapshot: Option<&DateTime<Utc>>,
) -> Result<hyper::client::ResponseFuture, AzureError>
where
    P: IntoAzurePath,
{
    let container_name = path.container_name()?;
    let blob_name = path.blob_name()?;

    let mut uri = format!(
        "https://{}.blob.core.windows.net/{}/{}?comp=blocklist&blocklisttype={}",
        c.account(),
        container_name,
        blob_name,
        match bl {
            BlockListType::Committed => "committed",
            BlockListType::Uncommitted => "uncommitted",
            BlockListType::All => "all",
        }
    );

    if let Some(ref timeout) = timeout {
        uri = format!("{}&timeout={}", uri, timeout);
    }

    if let Some(snapshot) = snapshot {
        uri = format!("{}&snapshot={}", uri, snapshot.to_rfc2822());
    }

    c.perform_request(
        &uri,
        Method::GET,
        move |ref mut request| {
            if let Some(lease_id) = lease_id {
                request.header_formatted(LEASE_ID, lease_id);
            };
            if let Some(request_id) = request_id {
                request.header_bytes(CLIENT_REQUEST_ID, request_id);
            };
        },
        None,
    )
}

pub fn get_block_list<P>(
    c: &Client,
    path: &P,
    bl: &BlockListType,
    timeout: Option<u64>,
    lease_id: Option<&LeaseId>,
    request_id: Option<String>,
    snapshot: Option<&DateTime<Utc>>,
) -> impl Future<Item = GetBlockListResponse, Error = AzureError>
where
    P: IntoAzurePath,
{
    done(get_block_list_create_request(c, path, bl, timeout, lease_id, request_id, snapshot))
        .from_err()
        .and_then(move |future_response| check_status_extract_headers_and_body(future_response, StatusCode::OK))
        .and_then(move |(headers, body)| {
            done(match String::from_utf8(body.to_owned()) {
                Ok(body) => Ok((headers, body)),
                Err(err) => Err(AzureError::FromUtf8Error(err)),
            })
        }).and_then(move |(headers, body)| {
            debug!("response headers == {:?}", headers);

            // extract headers
            let etag = headers.get_as_string(header::ETAG);
            debug!("etag == {:?}", etag);

            let content_type = headers.get_as_string(header::CONTENT_TYPE).unwrap();
            debug!("content_type == {:?}", content_type);

            let request_id = Uuid::parse_str(headers.get_as_str(REQUEST_ID).unwrap()).unwrap();

            debug!("request_id == {}", request_id);

            let last_modified = headers
                .get_as_str(header::LAST_MODIFIED)
                .map(|s| DateTime::parse_from_rfc2822(s).unwrap());

            let date = headers.get_as_str(header::DATE).unwrap();
            debug!("date == {}", date);
            let date = DateTime::parse_from_rfc2822(&date).unwrap();

            debug!("body == {:?}", body);

            done(match BlockWithSizeList::try_from(&body[3..] as &str) {
                Ok(block_list) => Ok(GetBlockListResponse {
                    block_list,
                    last_modified,
                    etag,
                    content_type: content_type.clone(),
                    request_id,
                    date,
                }),
                Err(error) => Err(AzureError::SerdeXMLDeserializationError(error)),
            })
        })
}

#[inline]
pub(crate) fn incomplete_vector_from_response(body: &str, container_name: &str) -> Result<IncompleteVector<Blob>, AzureError> {
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
