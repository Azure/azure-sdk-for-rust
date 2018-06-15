mod put_options;
pub use self::put_options::{PutOptions, PUT_OPTIONS_DEFAULT};

mod list_blob_options;
pub use self::list_blob_options::{ListBlobOptions, LIST_BLOB_OPTIONS_DEFAULT};

mod put_block_options;
pub use self::put_block_options::{PutBlockOptions, PUT_BLOCK_OPTIONS_DEFAULT};

mod put_page_options;
pub use self::put_page_options::{PutPageOptions, PUT_PAGE_OPTIONS_DEFAULT};

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

mod get_block_list_response;
pub use self::get_block_list_response::GetBlockListResponse;

use base64;
use chrono::{DateTime, Utc};
use futures::{future::*, prelude::*};
use hyper::{self, header, Method, StatusCode};
use md5;
use std::{borrow::Borrow, fmt, str::FromStr};
use uuid::Uuid;
use xml::Element;

use azure::core::{
    ba512_range::BA512Range,
    enumerations,
    errors::{check_status_extract_body, check_status_extract_headers_and_body, AzureError, TraversingError},
    incompletevector::IncompleteVector,
    lease::{LeaseAction, LeaseDuration, LeaseId, LeaseState, LeaseStatus},
    parsing::{cast_must, cast_optional, from_azure_time, traverse, FromStringOptional},
    range::Range,
    util::{HeaderMapExt, RequestBuilderExt},
};
use azure::storage::{client::Client, rest_client::*};

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

const HEADER_BLOB_CONTENT_LENGTH: &str = "x-ms-blob-content-length";
const HEADER_BLOB_SEQUENCE_NUMBER: &str = "x-ms-blob-sequence-number";
const HEADER_BLOB_TYPE: &str = "x-ms-blob-type";
const HEADER_BLOB_CONTENT_DISPOSITION: &str = "x-ms-blob-content-disposition";
const HEADER_PAGE_WRITE: &str = "x-ms-blob-page-write";

#[derive(Debug, Clone)]
pub struct Blob {
    pub name: String,
    pub container_name: String,
    pub snapshot_time: Option<DateTime<Utc>>,
    pub last_modified: DateTime<Utc>,
    pub etag: String,
    pub content_length: u64,
    pub content_type: Option<String>,
    pub content_encoding: Option<String>,
    pub content_language: Option<String>,
    pub content_md5: Option<String>,
    pub cache_control: Option<String>,
    pub x_ms_blob_sequence_number: Option<u64>,
    pub blob_type: BlobType,
    pub lease_status: LeaseStatus,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
    pub copy_id: Option<String>,
    pub copy_status: Option<CopyStatus>,
    pub copy_source: Option<String>,
    pub copy_progress: Option<Range>,
    pub copy_completion: Option<DateTime<Utc>>,
    pub copy_status_description: Option<String>,
}

impl Blob {
    pub fn parse(elem: &Element, container_name: &str) -> Result<Blob, AzureError> {
        let name = cast_must::<String>(elem, &["Name"])?;
        let snapshot_time = cast_optional::<DateTime<Utc>>(elem, &["Snapshot"])?;
        let last_modified = cast_must::<DateTime<Utc>>(elem, &["Properties", "Last-Modified"])?;
        let etag = cast_must::<String>(elem, &["Properties", "Etag"])?;

        let content_length = cast_must::<u64>(elem, &["Properties", "Content-Length"])?;

        let content_type = cast_must::<String>(elem, &["Properties", "Content-Type"])?;
        let content_encoding = cast_optional::<String>(elem, &["Properties", "Content-Encoding"])?;
        let content_language = cast_optional::<String>(elem, &["Properties", "Content-Language"])?;
        let content_md5 = cast_optional::<String>(elem, &["Properties", "Content-MD5"])?;
        let cache_control = cast_optional::<String>(elem, &["Properties", "Cache-Control"])?;
        let x_ms_blob_sequence_number = cast_optional::<u64>(elem, &["Properties", "x-ms-blob-sequence-number"])?;

        let blob_type = cast_must::<BlobType>(elem, &["Properties", "BlobType"])?;

        let lease_status = cast_must::<LeaseStatus>(elem, &["Properties", "LeaseStatus"])?;
        let lease_state = cast_must::<LeaseState>(elem, &["Properties", "LeaseState"])?;
        let lease_duration = cast_optional::<LeaseDuration>(elem, &["Properties", "LeaseDuration"])?;
        let copy_id = cast_optional::<String>(elem, &["Properties", "CopyId"])?;
        let copy_status = cast_optional::<CopyStatus>(elem, &["Properties", "CopyStatus"])?;
        let copy_source = cast_optional::<String>(elem, &["Properties", "CopySource"])?;
        let copy_progress = cast_optional::<String>(elem, &["Properties", "CopyProgress"])?;
        let copy_completion = cast_optional::<DateTime<Utc>>(elem, &["Properties", "CopyCompletionTime"])?;
        let copy_status_description = cast_optional::<String>(elem, &["Properties", "CopyStatusDescription"])?;

        let mut cp_bytes: Option<Range> = None;
        if let Some(txt) = copy_progress {
            cp_bytes = Some(txt.parse::<Range>()?);
        }

        Ok(Blob {
            name,
            container_name: container_name.to_owned(),
            snapshot_time,
            last_modified,
            etag,
            content_length,
            content_type: Some(content_type),
            content_encoding,
            content_language,
            content_md5,
            cache_control,
            x_ms_blob_sequence_number,
            blob_type,
            lease_status,
            lease_state,
            lease_duration,
            copy_id,
            copy_status,
            copy_source,
            copy_progress: cp_bytes,
            copy_completion,
            copy_status_description,
        })
    }

    pub fn from_headers(blob_name: &str, container_name: &str, h: &header::HeaderMap) -> Result<Blob, AzureError> {
        let content_type = h
            .get_as_string(header::CONTENT_TYPE)
            .unwrap_or_else(|| "application/octet-stream".to_owned());
        trace!("content_type == {:?}", content_type);

        let content_length = h
            .get(header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| AzureError::HeaderNotFound("Content-Length".to_owned()))?;
        trace!("content_length == {:?}", content_length);

        let last_modified = h
            .get_as_str(header::LAST_MODIFIED)
            .ok_or_else(|| AzureError::HeaderNotFound("Last-Modified".to_owned()))?;
        let last_modified = from_azure_time(last_modified)?;
        trace!("last_modified == {:?}", last_modified);

        let etag = h
            .get_as_string(header::ETAG)
            .ok_or_else(|| AzureError::HeaderNotFound("ETag".to_owned()))?;
        trace!("etag == {:?}", etag);

        let x_ms_blob_sequence_number = h.get_as_u64(HEADER_BLOB_SEQUENCE_NUMBER);
        trace!("x_ms_blob_sequence_number == {:?}", x_ms_blob_sequence_number);

        let blob_type = h
            .get_as_str(HEADER_BLOB_TYPE)
            .ok_or_else(|| AzureError::HeaderNotFound("x-ms-blob-type".to_owned()))?
            .parse::<BlobType>()?;
        trace!("blob_type == {:?}", blob_type);

        let content_encoding = h.get_as_string(header::CONTENT_ENCODING);
        trace!("content_encoding == {:?}", content_encoding);

        let content_language = h.get_as_string(header::CONTENT_LANGUAGE);
        trace!("content_language == {:?}", content_language);

        let content_md5 = h.get_as_string(HEADER_CONTENT_MD5);
        trace!("content_md5 == {:?}", content_md5);

        // TODO
        // let cache_control = match h.get::<CacheControl>() {
        //     Some(cc) => Some(cc.to_string()),
        //     None => None
        // };
        // println!("cache_control == {:?}", cache_control);

        //println!(
        //    "h.get::<XMSLeaseStatus>() == {:?}",
        //    h.get::<XMSLeaseStatus>()
        //);

        let lease_status = h
            .get_as_enum(HEADER_LEASE_STATUS)?
            .ok_or_else(|| AzureError::HeaderNotFound("x-ms-lease-status".to_owned()))?;
        trace!("lease_status == {:?}", lease_status);

        let lease_state = h
            .get_as_enum(HEADER_LEASE_STATE)?
            .ok_or_else(|| AzureError::HeaderNotFound("x-ms-lease-state".to_owned()))?;
        trace!("lease_state == {:?}", lease_state);

        let lease_duration = h.get_as_enum(HEADER_LEASE_DURATION)?;
        trace!("lease_duration == {:?}", lease_duration);

        // TODO: get the remaining headers
        // (https://msdn.microsoft.com/en-us/library/azure/dd179440.aspx)

        Ok(Blob {
            name: blob_name.to_owned(),
            container_name: container_name.to_owned(),
            snapshot_time: None,
            last_modified,
            etag,
            content_length,
            content_type: Some(content_type),
            content_encoding,
            content_language,
            content_md5,
            cache_control: None, // TODO
            x_ms_blob_sequence_number,
            blob_type,
            lease_status,
            lease_state,
            lease_duration,
            copy_id: None,                 // TODO
            copy_status: None,             // TODO
            copy_source: None,             // TODO
            copy_progress: None,           // TODO
            copy_completion: None,         // TODO
            copy_status_description: None, // TODO
        })
    }

    pub fn list(c: &Client, container_name: &str, lbo: &ListBlobOptions) -> impl Future<Item = IncompleteVector<Blob>, Error = AzureError> {
        let mut include = String::new();
        if lbo.include_snapshots {
            include += "snapshots";
        }
        if lbo.include_metadata {
            if include.is_empty() {
                include += ",";
            }
            include += "metadata";
        }
        if lbo.include_uncommittedblobs {
            if include.is_empty() {
                include += ",";
            }
            include += "uncommittedblobs";
        }
        if lbo.include_copy {
            if include.is_empty() {
                include += ",";
            }
            include += "copy";
        }

        let mut uri = format!(
            "https://{}.blob.core.windows.\
             net/{}?restype=container&comp=list&maxresults={}",
            c.account(),
            container_name,
            lbo.max_results
        );

        if !include.is_empty() {
            uri = format!("{}&include={}", uri, include);
        }

        if let Some(nm) = lbo.next_marker {
            uri = format!("{}&marker={}", uri, nm);
        }

        if let Some(ref pref) = lbo.prefix {
            uri = format!("{}&prefix={}", uri, pref);
        }

        if let Some(ref timeout) = lbo.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let req = c.perform_request(&uri, Method::GET, |_| {}, None);

        // we create a copy to move into the future's closure.
        // We need to do this since the closure only accepts
        // 'static lifetimes.
        let container_name = container_name.to_owned();

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::OK)
                .and_then(move |body| done(incomplete_vector_from_response(&body, &container_name)).from_err())
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

    pub fn get(
        c: &Client,
        container_name: &str,
        blob_name: &str,
        snapshot: Option<&DateTime<Utc>>,
        range: Option<&Range>,
        lease_id: Option<&LeaseId>,
    ) -> impl Future<Item = (Blob, Vec<u8>), Error = AzureError> {
        let mut uri = format!("https://{}.blob.core.windows.net/{}/{}", c.account(), container_name, blob_name);

        if let Some(snapshot) = snapshot {
            uri = format!("{}?snapshot={}", uri, snapshot.to_rfc2822());
        }

        trace!("uri == {:?}", uri);

        let req = c.perform_request(
            &uri,
            Method::GET,
            |ref mut request| {
                if let Some(r) = range {
                    request.header_formatted(HEADER_RANGE, r);

                    // if range is < 4MB request md5
                    if r.end - r.start <= 1024 * 1024 * 4 {
                        request.header_static(HEADER_RANGE_GET_CONTENT_MD5, "true");
                    }
                }
                if let Some(l) = lease_id {
                    request.header_formatted(HEADER_LEASE_ID, l);
                }
            },
            None,
        );

        let expected_status_code = if range.is_some() {
            StatusCode::PARTIAL_CONTENT
        } else {
            StatusCode::OK
        };

        let container_name = container_name.to_owned();
        let blob_name = blob_name.to_owned();

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_headers_and_body(future_response, expected_status_code).and_then(move |(headers, body)| {
                done(Blob::from_headers(&blob_name, &container_name, &headers)).and_then(move |blob| ok((blob, body)))
            })
        })
    }

    fn put_create_request(&self, c: &Client, po: &PutOptions, r: Option<&[u8]>) -> Result<hyper::client::ResponseFuture, AzureError> {
        // parameter sanity check
        match self.blob_type {
            BlobType::BlockBlob => if r.is_none() {
                return Err(AzureError::InputParametersError(
                    "cannot use put_blob with \
                     BlockBlob without a Read"
                        .to_owned(),
                ));
            },
            BlobType::PageBlob => {
                if r.is_some() {
                    return Err(AzureError::InputParametersError(
                        "cannot use put_blob with \
                         PageBlob with a Read"
                            .to_owned(),
                    ));
                }

                if self.content_length % 512 != 0 {
                    return Err(AzureError::InputParametersError(
                        "PageBlob size must be aligned \
                         to 512 bytes boundary"
                            .to_owned(),
                    ));
                }
            }
            BlobType::AppendBlob => if r.is_some() {
                return Err(AzureError::InputParametersError(
                    "cannot use put_blob with \
                     AppendBlob with a Read"
                        .to_owned(),
                ));
            },
        };

        let mut uri = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            c.account(),
            self.container_name,
            self.name
        );

        if let Some(ref timeout) = po.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        c.perform_request(
            &uri,
            Method::PUT,
            move |ref mut request| {
                if let Some(ref ct) = self.content_type {
                    request.header_formatted(header::CONTENT_TYPE, ct);
                }

                if let Some(ref ce) = self.content_encoding {
                    request.header_formatted(header::CONTENT_ENCODING, ce);
                }

                // TODO Content-Language

                if let Some(ref content_md5) = self.content_md5 {
                    request.header_formatted(HEADER_CONTENT_MD5, content_md5);
                };

                request.header_formatted(HEADER_BLOB_TYPE, self.blob_type);

                if let Some(ref lease_id) = po.lease_id {
                    request.header_formatted(HEADER_LEASE_ID, lease_id);
                }

                // TODO x-ms-blob-content-disposition

                if self.blob_type == BlobType::PageBlob {
                    request.header_formatted(HEADER_BLOB_CONTENT_LENGTH, self.content_length);
                }
            },
            r,
        )
    }

    pub fn put(&self, c: &Client, po: &PutOptions, r: Option<&[u8]>) -> impl Future<Item = (), Error = AzureError> {
        ok(self.put_create_request(c, po, r)).and_then(|req| {
            done(req)
                .from_err()
                .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::CREATED))
                .and_then(|_| ok(()))
        })
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
                    request.header_formatted(HEADER_LEASE_ID, lease_id);
                }

                request.header_formatted(HEADER_LEASE_ACTION, la);

                if let Some(lease_break_period) = lbo.lease_break_period {
                    request.header_formatted(HEADER_LEASE_BREAK_PERIOD, lease_break_period);
                }
                if let Some(lease_duration) = lbo.lease_duration {
                    request.header_formatted(HEADER_LEASE_DURATION, lease_duration);
                }
                if let Some(ref proposed_lease_id) = lbo.proposed_lease_id {
                    request.header_formatted(HEADER_PROPOSED_LEASE_ID, proposed_lease_id);
                }
                if let Some(ref request_id) = lbo.request_id {
                    request.header_formatted(HEADER_CLIENT_REQUEST_ID, request_id);
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
                    .get_as_str(HEADER_LEASE_ID)
                    .and_then(|s| s.parse::<Uuid>().ok())
                    .ok_or_else(|| AzureError::HeaderNotFound("x-ms-lease-id".to_owned()))
            })
    }

    pub fn put_page(
        &self,
        c: &Client,
        range: &BA512Range,
        ppo: &PutPageOptions,
        content: &[u8],
    ) -> impl Future<Item = (), Error = AzureError> {
        let mut uri = format!(
            "https://{}.blob.core.windows.net/{}/{}?comp=page",
            c.account(),
            self.container_name,
            self.name
        );

        if let Some(ref timeout) = ppo.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let req = c.perform_request(
            &uri,
            Method::PUT,
            move |ref mut request| {
                let range: Range = range.into();
                request.header_formatted(HEADER_RANGE, range);
                request.header_formatted(HEADER_BLOB_CONTENT_LENGTH, content.len());
                if let Some(lease_id) = ppo.lease_id {
                    request.header_formatted(HEADER_LEASE_ID, lease_id);
                }

                request.header_formatted(HEADER_PAGE_WRITE, PageWriteType::Update);
            },
            Some(content),
        );

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::CREATED))
            .and_then(|_| ok(()))
    }

    fn put_block_create_request(
        &self,
        c: &Client,
        encoded_block_id: &str,
        pbo: &PutBlockOptions,
        content: &[u8],
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        // parameter sanity check
        match self.blob_type {
            BlobType::BlockBlob => {}
            BlobType::PageBlob => {
                return Err(AzureError::InputParametersError(String::from(
                    "cannot use put_block_blob with a page blob",
                )));
            }
            BlobType::AppendBlob => {
                return Err(AzureError::InputParametersError(String::from(
                    "cannot use put_block_blob with an AppendBlob",
                )));
            }
        };

        let mut uri = format!(
            "https://{}.blob.core.windows.net/{}/{}?comp=block&blockid={}",
            c.account(),
            self.container_name,
            self.name,
            encoded_block_id
        );

        if let Some(ref timeout) = pbo.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        c.perform_request(
            &uri,
            Method::PUT,
            move |ref mut request| {
                if let Some(ref ct) = self.content_type {
                    request.header_formatted(header::CONTENT_TYPE, ct);
                }

                if let Some(ref ce) = self.content_encoding {
                    request.header_formatted(header::CONTENT_ENCODING, ce);
                }

                // TODO Content-Language

                if let Some(ref content_md5) = self.content_md5 {
                    request.header_formatted(HEADER_CONTENT_MD5, content_md5);
                };

                request.header_formatted(HEADER_BLOB_TYPE, self.blob_type);

                if let Some(ref lease_id) = pbo.lease_id {
                    request.header_formatted(HEADER_LEASE_ID, lease_id);
                }

                // TODO x-ms-blob-content-disposition

                if self.blob_type == BlobType::PageBlob {
                    request.header_formatted(HEADER_BLOB_CONTENT_LENGTH, self.content_length);
                }

                if let Some(ref request_id) = pbo.request_id {
                    request.header_formatted(HEADER_CLIENT_REQUEST_ID, request_id.to_owned());
                }
            },
            Some(content),
        )
    }

    pub fn put_block(
        &self,
        c: &Client,
        block_id: &str,
        pbo: &PutBlockOptions,
        content: &[u8],
    ) -> impl Future<Item = String, Error = AzureError> {
        let encoded_block_id = base64::encode(block_id.as_bytes());
        ok(self.put_block_create_request(c, &encoded_block_id, pbo, content)).and_then(|req| {
            done(req)
                .from_err()
                .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::CREATED))
                .and_then(|_| ok(encoded_block_id))
        })
    }

    pub fn clear_page(&self, c: &Client, range: &BA512Range, lease_id: Option<&LeaseId>) -> impl Future<Item = (), Error = AzureError> {
        let uri = format!(
            "https://{}.blob.core.windows.net/{}/{}?comp=page",
            c.account(),
            self.container_name,
            self.name
        );
        let req = c.perform_request(
            &uri,
            Method::PUT,
            move |ref mut request| {
                request.header_formatted(HEADER_RANGE, Range::from(range));
                request.header_static(HEADER_BLOB_CONTENT_LENGTH, "0");
                if let Some(lease_id) = lease_id {
                    request.header_formatted(HEADER_LEASE_ID, lease_id);
                }

                request.header_formatted(HEADER_PAGE_WRITE, PageWriteType::Clear);
            },
            None,
        );

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::CREATED))
            .and_then(|_| ok(()))
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
                    request.header_formatted(HEADER_LEASE_ID, lease_id);
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
            request.header_formatted(HEADER_CONTENT_MD5, md5);
            if let Some(lease_id) = lease_id {
                request.header_formatted(HEADER_LEASE_ID, *lease_id);
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
                request.header_formatted(HEADER_LEASE_ID, lease_id);
            };
            if let Some(request_id) = request_id {
                request.header_bytes(HEADER_CLIENT_REQUEST_ID, request_id);
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
            done(match String::from_utf8(body) {
                Ok(body) => Ok((headers, body)),
                Err(err) => Err(AzureError::FromUtf8Error(err)),
            })
        })
        .and_then(move |(headers, body)| {
            debug!("response headers == {:?}", headers);

            // extract headers
            let etag = headers.get_as_string(header::ETAG);
            debug!("etag == {:?}", etag);

            let content_type = headers.get_as_string(header::CONTENT_TYPE).unwrap();
            debug!("content_type == {:?}", content_type);

            let request_id = Uuid::parse_str(headers.get_as_str(HEADER_REQUEST_ID).unwrap()).unwrap();

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
fn incomplete_vector_from_response(body: &str, container_name: &str) -> Result<IncompleteVector<Blob>, AzureError> {
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
        //println!("{:?}", node_blob);
        v.push(Blob::parse(node_blob, container_name)?);
    }

    Ok(IncompleteVector::<Blob>::new(next_marker, v))
}
