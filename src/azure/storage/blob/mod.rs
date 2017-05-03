extern crate uuid;

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

use chrono::datetime::DateTime;
use chrono::UTC;

use azure::core::lease::{LeaseId, LeaseStatus, LeaseState, LeaseDuration, LeaseAction};
use azure::storage::client::Client;

use azure::core;
use azure::storage::rest_client::{XMSRange, ContentMD5, XMSLeaseStatus, XMSLeaseDuration,
                                  XMSLeaseState, XMSLeaseId, XMSRangeGetContentMD5,
                                  XMSClientRequestId, XMSLeaseAction, XMSLeaseDurationSeconds,
                                  XMSLeaseBreakPeriod, XMSProposedLeaseId, ETag};

use azure::core::parsing::{cast_must, cast_optional, from_azure_time, traverse};

use xml::Element;

use std::str::FromStr;
use azure::core::enumerations;
use std::fmt;

use std::io::Read;

use azure::core::errors::{TraversingError, AzureError, check_status};
use azure::core::parsing::FromStringOptional;

use azure::core::range::Range;
use azure::core::ba512_range::BA512Range;
use azure::core::incompletevector::IncompleteVector;

use mime::Mime;

use hyper::status::StatusCode;
use hyper::header::{Headers, ContentType, ContentLength, LastModified, ContentEncoding,
                    ContentLanguage};

use base64;

use uuid::Uuid;

create_enum!(BlobType,
             (BlockBlob, "BlockBlob"),
             (PageBlob, "PageBlob"),
             (AppendBlob, "AppendBlob"));

create_enum!(CopyStatus,
             (Pending, "pending"),
             (Success, "success"),
             (Aborted, "aborted"),
             (Failed, "failed"));

create_enum!(PageWriteType, (Update, "update"), (Clear, "clear"));

header! { (XMSBlobContentLength, "x-ms-blob-content-length") => [u64] }
header! { (XMSBlobSequenceNumber, "x-ms-blob-sequence-number") => [u64] }
header! { (XMSBlobType, "x-ms-blob-type") => [BlobType] }
header! { (XMSBlobContentDisposition, "x-ms-blob-content-disposition") => [String] }
header! { (XMSPageWrite, "x-ms-page-write") => [PageWriteType] }

#[derive(Debug)]
pub struct Blob {
    pub name: String,
    pub container_name: String,
    pub snapshot_time: Option<DateTime<UTC>>,
    pub last_modified: DateTime<UTC>,
    pub etag: String,
    pub content_length: u64,
    pub content_type: Mime,
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
    pub copy_completion: Option<DateTime<UTC>>,
    pub copy_status_description: Option<String>,
}

impl Blob {
    pub fn parse(elem: &Element, container_name: &str) -> Result<Blob, AzureError> {
        let name = try!(cast_must::<String>(elem, &["Name"]));
        let snapshot_time = try!(cast_optional::<DateTime<UTC>>(elem, &["Snapshot"]));
        let last_modified = try!(cast_must::<DateTime<UTC>>(elem,
                                                            &["Properties", "Last-Modified"]));
        let etag = try!(cast_must::<String>(elem, &["Properties", "Etag"]));

        let content_length = try!(cast_must::<u64>(elem, &["Properties", "Content-Length"]));

        let content_type = try!(cast_must::<String>(elem, &["Properties", "Content-Type"]));
        let content_encoding = try!(cast_optional::<String>(elem,
                                                            &["Properties", "Content-Encoding"]));
        let content_language = try!(cast_optional::<String>(elem,
                                                            &["Properties", "Content-Language"]));
        let content_md5 = try!(cast_optional::<String>(elem, &["Properties", "Content-MD5"]));
        let cache_control = try!(cast_optional::<String>(elem, &["Properties", "Cache-Control"]));
        let x_ms_blob_sequence_number =
            try!(cast_optional::<u64>(elem, &["Properties", "x-ms-blob-sequence-number"]));

        let blob_type = try!(cast_must::<BlobType>(elem, &["Properties", "BlobType"]));

        let lease_status = try!(cast_must::<LeaseStatus>(elem, &["Properties", "LeaseStatus"]));
        let lease_state = try!(cast_must::<LeaseState>(elem, &["Properties", "LeaseState"]));
        let lease_duration = try!(cast_optional::<LeaseDuration>(elem,
                                                                 &["Properties", "LeaseDuration"]));
        let copy_id = try!(cast_optional::<String>(elem, &["Properties", "CopyId"]));
        let copy_status = try!(cast_optional::<CopyStatus>(elem, &["Properties", "CopyStatus"]));
        let copy_source = try!(cast_optional::<String>(elem, &["Properties", "CopySource"]));
        let copy_progress = try!(cast_optional::<String>(elem, &["Properties", "CopyProgress"]));
        let copy_completion =
            try!(cast_optional::<DateTime<UTC>>(elem, &["Properties", "CopyCompletionTime"]));
        let copy_status_description =
            try!(cast_optional::<String>(elem, &["Properties", "CopyStatusDescription"]));

        let mut cp_bytes: Option<Range> = None;
        if let Some(txt) = copy_progress {
            cp_bytes = Some(try!(txt.parse::<Range>()));
        }

        let ctype = try!(content_type.parse::<Mime>());

        Ok(Blob {
               name: name,
               container_name: container_name.to_owned(),
               snapshot_time: snapshot_time,
               last_modified: last_modified,
               etag: etag,
               content_length: content_length,
               content_type: ctype,
               content_encoding: content_encoding,
               content_language: content_language,
               content_md5: content_md5,
               cache_control: cache_control,
               x_ms_blob_sequence_number: x_ms_blob_sequence_number,
               blob_type: blob_type,
               lease_status: lease_status,
               lease_state: lease_state,
               lease_duration: lease_duration,
               copy_id: copy_id,
               copy_status: copy_status,
               copy_source: copy_source,
               copy_progress: cp_bytes,
               copy_completion: copy_completion,
               copy_status_description: copy_status_description,
           })
    }

    pub fn from_headers(blob_name: &str,
                        container_name: &str,
                        h: &Headers)
                        -> Result<Blob, AzureError> {
        let content_type = match h.get::<ContentType>() {
            Some(ct) => (ct as &Mime).clone(),
            None => try!("application/octet-stream".parse::<Mime>()),
        };
        trace!("content_type == {:?}", content_type);

        let content_length = match h.get::<ContentLength>() {
            Some(cl) => *(cl as &u64),
            None => return Err(AzureError::HeaderNotFound("Content-Length".to_owned())),
        };
        trace!("content_length == {:?}", content_length);

        let last_modified = match h.get::<LastModified>() {
            Some(lm) => {
                try!(from_azure_time(&lm.to_string()))
                //{let te: TraversingError= e.into(); te}))
            }
            None => return Err(AzureError::HeaderNotFound("Last-Modified".to_owned())),
        };
        trace!("last_modified == {:?}", last_modified);

        let etag = match h.get::<ETag>() {
            Some(lm) => lm.to_string(),
            None => return Err(AzureError::HeaderNotFound("ETag".to_owned())),
        };
        trace!("etag == {:?}", etag);

        let x_ms_blob_sequence_number = match h.get::<XMSBlobSequenceNumber>() {
            Some(lm) => Some(*(lm as &u64)),
            None => None,
        };
        trace!("x_ms_blob_sequence_number == {:?}",
               x_ms_blob_sequence_number);

        let blob_type = match h.get::<XMSBlobType>() {
            Some(lm) => try!((&lm.to_string()).parse::<BlobType>()),
            None => return Err(AzureError::HeaderNotFound("x-ms-blob-type".to_owned())),
        };
        trace!("blob_type == {:?}", blob_type);

        let content_encoding = match h.get::<ContentEncoding>() {
            Some(ce) => Some(ce.to_string()),
            None => None,
        };
        trace!("content_encoding == {:?}", content_encoding);

        let content_language = match h.get::<ContentLanguage>() {
            Some(cl) => Some(cl.to_string()),
            None => None,
        };
        trace!("content_language == {:?}", content_language);

        let content_md5 = match h.get::<ContentMD5>() {
            Some(md5) => Some(md5.to_string()),
            None => None,
        };
        trace!("content_md5 == {:?}", content_md5);

        // TODO
        // let cache_control = match h.get::<CacheControl>() {
        //     Some(cc) => Some(cc.to_string()),
        //     None => None
        // };
        // println!("cache_control == {:?}", cache_control);

        let lease_status = match h.get::<XMSLeaseStatus>() {
            Some(ls) => try!(ls.to_string().parse::<LeaseStatus>()),
            None => return Err(AzureError::HeaderNotFound("x-ms-lease-status".to_owned())),
        };
        trace!("lease_status == {:?}", lease_status);


        let lease_state = match h.get::<XMSLeaseState>() {
            Some(ls) => try!(ls.to_string().parse::<LeaseState>()),
            None => return Err(AzureError::HeaderNotFound("x-ms-lease-state".to_owned())),
        };
        trace!("lease_state == {:?}", lease_state);


        let lease_duration = match h.get::<XMSLeaseDuration>() {
            Some(ls) => Some(try!(ls.to_string().parse::<LeaseDuration>())),
            None => None,
        };
        trace!("lease_duration == {:?}", lease_duration);

        // TODO: get the remaining headers
        // (https://msdn.microsoft.com/en-us/library/azure/dd179440.aspx)

        Ok(Blob {
               name: blob_name.to_owned(),
               container_name: container_name.to_owned(),
               snapshot_time: None,
               last_modified: last_modified,
               etag: etag,
               content_length: content_length,
               content_type: content_type,
               content_encoding: content_encoding,
               content_language: content_language,
               content_md5: content_md5,
               cache_control: None, // TODO
               x_ms_blob_sequence_number: x_ms_blob_sequence_number,
               blob_type: blob_type,
               lease_status: lease_status,
               lease_state: lease_state,
               lease_duration: lease_duration,
               copy_id: None, // TODO
               copy_status: None, // TODO
               copy_source: None, // TODO
               copy_progress: None, // TODO
               copy_completion: None, // TODO
               copy_status_description: None, // TODO
           })
    }

    pub fn list(c: &Client,
                container_name: &str,
                lbo: &ListBlobOptions)
                -> Result<IncompleteVector<Blob>, core::errors::AzureError> {

        let mut include = String::new();
        if lbo.include_snapshots {
            include = include + "snapshots";
        }
        if lbo.include_metadata {
            if include.is_empty() {
                include = include + ",";
            }
            include = include + "metadata";
        }
        if lbo.include_uncommittedblobs {
            if include.is_empty() {
                include = include + ",";
            }
            include = include + "uncommittedblobs";
        }
        if lbo.include_copy {
            if include.is_empty() {
                include = include + ",";
            }
            include = include + "copy";
        }

        let mut uri = format!("{}://{}.blob.core.windows.\
                               net/{}?restype=container&comp=list&maxresults={}",
                              c.auth_scheme(),
                              c.account(),
                              container_name,
                              lbo.max_results);

        if !include.is_empty() {
            uri = format!("{}&include={}", uri, include);
        }

        if let Some(ref nm) = lbo.next_marker {
            uri = format!("{}&marker={}", uri, nm);
        }

        if let Some(ref pref) = lbo.prefix {
            uri = format!("{}&prefix={}", uri, pref);
        }

        if let Some(ref timeout) = lbo.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Get, &Headers::new(), None));

        try!(check_status(&mut resp, StatusCode::Ok));

        let mut resp_s = String::new();
        try!(resp.read_to_string(&mut resp_s));

        trace!("resp_s == {:?}\n\n", resp_s);

        let sp = &resp_s;
        let elem: Element = try!(sp.parse());

        let next_marker = match try!(cast_optional::<String>(&elem, &["NextMarker"])) {
            Some(ref nm) if nm == "" => None,
            Some(nm) => Some(nm),
            None => None,
        };

        let mut v = Vec::new();
        for node_blob in try!(traverse(&elem, &["Blobs", "Blob"], true)) {
            // println!("{:?}", blob);
            v.push(try!(Blob::parse(node_blob, container_name)));
        }

        Ok(IncompleteVector::<Blob>::new(next_marker, v))
    }

    pub fn get(c: &Client,
               container_name: &str,
               blob_name: &str,
               snapshot: Option<&DateTime<UTC>>,
               range: Option<&Range>,
               lease_id: Option<&LeaseId>)
               -> Result<(Blob, Box<Read>), core::errors::AzureError> {
        let mut uri = format!("{}://{}.blob.core.windows.net/{}/{}",
                              c.auth_scheme(),
                              c.account(),
                              container_name,
                              blob_name);

        if let Some(snapshot) = snapshot {
            uri = format!("{}?snapshot={}", uri, snapshot.to_rfc2822());
        }

        let uri = uri;

        trace!("uri == {:?}", uri);

        let mut headers = Headers::new();

        if let Some(r) = range {
            headers.set(XMSRange(*r));

            // if range is < 4MB request md5
            if r.end - r.start <= 1024 * 1024 * 4 {
                headers.set(XMSRangeGetContentMD5(true));
            }
        }

        if let Some(l) = lease_id {
            headers.set(XMSLeaseId(*l));
        }


        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Get, &headers, None));

        // if we have requested a range the response code should be 207 (partial content)
        // otherwise 200 (ok).
        if range.is_some() {
            try!(check_status(&mut resp, StatusCode::PartialContent));
        } else {
            try!(check_status(&mut resp, StatusCode::Ok));
        }

        let blob = try!(Blob::from_headers(blob_name, container_name, &resp.headers));
        let r: Box<Read> = Box::new(resp);

        Ok((blob, r))
    }

    pub fn put(&self,
               c: &Client,
               po: &PutOptions,
               r: Option<(&mut Read, u64)>)
               -> Result<(), AzureError> {

        // parameter sanity check
        match self.blob_type {
            BlobType::BlockBlob => {
                if r.is_some() {
                    return Err(AzureError::InputParametersError("cannot use put_blob with \
                                                                 BlockBlob without a Read"
                                                                        .to_owned()));
                }
            }
            BlobType::PageBlob => {
                if r.is_some() {
                    return Err(AzureError::InputParametersError("cannot use put_blob with \
                                                                 PageBlob with a Read"
                                                                        .to_owned()));
                }

                if self.content_length % 512 != 0 {
                    return Err(AzureError::InputParametersError("PageBlob size must be aligned \
                                                                 to 512 bytes boundary"
                                                                        .to_owned()));
                }
            }
            BlobType::AppendBlob => {
                if r.is_some() {
                    return Err(AzureError::InputParametersError("cannot use put_blob with \
                                                                 AppendBlob with a Read"
                                                                        .to_owned()));
                }
            }
        }

        let mut uri = format!("{}://{}.blob.core.windows.net/{}/{}",
                              c.auth_scheme(),
                              c.account(),
                              self.container_name,
                              self.name);

        if let Some(ref timeout) = po.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let mut headers = Headers::new();

        headers.set(ContentType(self.content_type.clone()));

        if let Some(ref content_encoding) = self.content_encoding {
            use hyper::header::Encoding;
            let enc = try!(content_encoding.parse::<Encoding>());
            headers.set(ContentEncoding(vec![enc]));
        };

        // TODO Content-Language

        if let Some(ref content_md5) = self.content_md5 {
            headers.set(ContentMD5(content_md5.to_owned()));
        };

        headers.set(XMSBlobType(self.blob_type));

        if let Some(ref lease_id) = po.lease_id {
            headers.set(XMSLeaseId(*lease_id));
        }

        // TODO x-ms-blob-content-disposition

        if self.blob_type == BlobType::PageBlob {
            headers.set(XMSBlobContentLength(self.content_length));
        }

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Put, &headers, r));

        try!(core::errors::check_status(&mut resp, StatusCode::Created));

        Ok(())
    }

    pub fn lease(&self,
                 c: &Client,
                 la: LeaseAction,
                 lbo: &LeaseBlobOptions)
                 -> Result<LeaseId, AzureError> {
        let mut uri = format!("{}://{}.blob.core.windows.net/{}/{}?comp=lease",
                              c.auth_scheme(),
                              c.account(),
                              self.container_name,
                              self.name);
        if let Some(ref timeout) = lbo.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let mut headers = Headers::new();

        if let Some(ref lease_id) = lbo.lease_id {
            headers.set(XMSLeaseId(lease_id.to_owned()));
        }

        headers.set(XMSLeaseAction(la));

        if let Some(lease_break_period) = lbo.lease_break_period {
            headers.set(XMSLeaseBreakPeriod(lease_break_period));
        }
        if let Some(lease_duration) = lbo.lease_duration {
            headers.set(XMSLeaseDurationSeconds(lease_duration));
        }
        if let Some(ref proposed_lease_id) = lbo.proposed_lease_id {
            headers.set(XMSProposedLeaseId(*proposed_lease_id));
        }
        if let Some(ref request_id) = lbo.request_id {
            headers.set(XMSClientRequestId(request_id.to_owned()));
        }

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Put, &headers, None));

        let expected_result = match la {
            LeaseAction::Acquire => StatusCode::Created,
            LeaseAction::Renew | LeaseAction::Change | LeaseAction::Release => StatusCode::Ok,
            LeaseAction::Break => StatusCode::Accepted,
        };

        try!(core::errors::check_status(&mut resp, expected_result));

        let lid = match resp.headers.get::<XMSLeaseId>() {
            Some(l) => l as &Uuid,
            None => return Err(AzureError::HeaderNotFound("x-ms-lease-id".to_owned())),
        };

        Ok(*lid)
    }

    pub fn put_page(&self,
                    c: &Client,
                    range: &BA512Range,
                    ppo: &PutPageOptions,
                    content: (&mut Read, u64))
                    -> Result<(), AzureError> {

        let mut uri = format!("{}://{}.blob.core.windows.net/{}/{}?comp=page",
                              c.auth_scheme(),
                              c.account(),
                              self.container_name,
                              self.name);

        if let Some(ref timeout) = ppo.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let mut headers = Headers::new();

        headers.set(XMSRange(range.into()));
        headers.set(XMSBlobContentLength(content.1));
        if let Some(ref lease_id) = ppo.lease_id {
            headers.set(XMSLeaseId(*lease_id));
        }

        headers.set(XMSPageWrite(PageWriteType::Update));

        let mut resp =
            try!(c.perform_request(&uri, core::HTTPMethod::Put, &headers, Some(content)));
        try!(core::errors::check_status(&mut resp, StatusCode::Created));

        Ok(())
    }

    pub fn put_block(&self,
                     c: &Client,
                     block_id: &str,
                     pbo: &PutBlockOptions,
                     content: (&mut Read, u64))
                     -> Result<(), AzureError> {

        let encoded_block_id = base64::encode(block_id.as_bytes());

        let mut uri = format!("{}://{}.blob.core.windows.net/{}/{}?comp=block&blockid={}",
                              c.auth_scheme(),
                              c.account(),
                              self.container_name,
                              self.name,
                              encoded_block_id);

        if let Some(ref timeout) = pbo.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let mut headers = Headers::new();

        headers.set(XMSBlobContentLength(content.1));

        if let Some(ref lease_id) = pbo.lease_id {
            headers.set(XMSLeaseId(*lease_id));
        }
        if let Some(ref request_id) = pbo.request_id {
            headers.set(XMSClientRequestId(request_id.to_owned()));
        }

        let mut resp =
            try!(c.perform_request(&uri, core::HTTPMethod::Put, &headers, Some(content)));

        try!(core::errors::check_status(&mut resp, StatusCode::Created));

        Ok(())
    }

    pub fn clear_page(&self,
                      c: &Client,
                      range: &BA512Range,
                      lease_id: Option<LeaseId>)
                      -> Result<(), AzureError> {

        let uri = format!("{}://{}.blob.core.windows.net/{}/{}?comp=page",
                          c.auth_scheme(),
                          c.account(),
                          self.container_name,
                          self.name);
        let mut headers = Headers::new();

        headers.set(XMSRange(range.into()));
        headers.set(XMSBlobContentLength(0));
        if let Some(lease_id) = lease_id {
            headers.set(XMSLeaseId(lease_id));
        }

        headers.set(XMSPageWrite(PageWriteType::Clear));

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Put, &headers, None));
        try!(core::errors::check_status(&mut resp, StatusCode::Created));

        Ok(())
    }

    pub fn del(c: &Client,
               container_name: &str,
               blob_name: &str)
               -> Result<(), core::errors::AzureError> {
        let uri = format!("{}://{}.blob.core.windows.net/{}/{}",
                          c.auth_scheme(),
                          c.account(),
                          container_name,
                          blob_name);
        let mut resp =
            try!(c.perform_request(&uri, core::HTTPMethod::Delete, &Headers::new(), None));
        try!(core::errors::check_status(&mut resp, StatusCode::Accepted));
        Ok(())
    }
}
