use chrono::datetime::DateTime;
use chrono::UTC;

use azure::storage::{LeaseStatus, LeaseState, LeaseDuration};
use azure::core::{ContentMD5, XMSLeaseStatus, XMSLeaseDuration, XMSLeaseState};
use azure::core::parsing::{cast_must, cast_optional, from_azure_time};

use xml::Element;

use std::str::FromStr;
use azure::core::enumerations;
use std::fmt;

use azure::core::ETag;

use std::io::Read;

use azure::core::errors::{TraversingError, AzureError};
use azure::core::parsing::FromStringOptional;

use azure::core::range::Range;
use mime::Mime;

use hyper::header::{Headers, ContentType, ContentLength, LastModified, ContentEncoding,
                    ContentLanguage};

create_enum!(BlobType,
                            (BlockBlob,        "BlockBlob"),
                            (PageBlob,         "PageBlob"),
                            (AppendBlob,       "AppendBlob")
);

create_enum!(CopyStatus,
                            (Pending,          "pending"),
                            (Success,          "success"),
                            (Aborted,          "aborted"),
                            (Failed,           "failed")
);

header! { (XMSBlobSequenceNumber, "x-ms-blob-sequence-number") => [u64] }
header! { (XMSBlobType, "x-ms-blob-type") => [BlobType] }

#[derive(Debug)]
pub struct Blob {
    pub name: String,
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

pub fn parse(elem: &Element) -> Result<Blob, AzureError> {
    let name = try!(cast_must::<String>(elem, &["Name"]));
    let snapshot_time = try!(cast_optional::<DateTime<UTC>>(elem, &["Snapshot"]));
    let last_modified = try!(cast_must::<DateTime<UTC>>(elem, &["Properties", "Last-Modified"]));
    let etag = try!(cast_must::<String>(elem, &["Properties", "Etag"]));

    let content_length = try!(cast_must::<u64>(elem, &["Properties", "Content-Length"]));

    let content_type = try!(cast_must::<String>(elem, &["Properties", "Content-Type"]));
    let content_encoding = try!(cast_optional::<String>(elem, &["Properties", "Content-Encoding"]));
    let content_language = try!(cast_optional::<String>(elem, &["Properties", "Content-Language"]));
    let content_md5 = try!(cast_optional::<String>(elem, &["Properties", "Content-MD5"]));
    let cache_control = try!(cast_optional::<String>(elem, &["Properties", "Cache-Control"]));
    let x_ms_blob_sequence_number = try!(cast_optional::<u64>(elem,
                                                              &["Properties",
                                                                "x-ms-blob-sequence-number"]));

    let blob_type = try!(cast_must::<BlobType>(elem, &["Properties", "BlobType"]));

    let lease_status = try!(cast_must::<LeaseStatus>(elem, &["Properties", "LeaseStatus"]));
    let lease_state = try!(cast_must::<LeaseState>(elem, &["Properties", "LeaseState"]));
    let lease_duration = try!(cast_optional::<LeaseDuration>(elem,
                                                             &["Properties", "LeaseDuration"]));
    let copy_id = try!(cast_optional::<String>(elem, &["Properties", "CopyId"]));
    let copy_status = try!(cast_optional::<CopyStatus>(elem, &["Properties", "CopyStatus"]));
    let copy_source = try!(cast_optional::<String>(elem, &["Properties", "CopySource"]));
    let copy_progress = try!(cast_optional::<String>(elem, &["Properties", "CopyProgress"]));
    let copy_completion = try!(cast_optional::<DateTime<UTC>>(elem,
                                                              &["Properties",
                                                                "CopyCompletionTime"]));
    let copy_status_description = try!(cast_optional::<String>(elem,
                                                               &["Properties",
                                                                 "CopyStatusDescription"]));

    let mut cp_bytes: Option<Range> = None;
    if let Some(txt) = copy_progress {
        cp_bytes = Some(try!(txt.parse::<Range>()));
    }

    let ctype = try!(content_type.parse::<Mime>());

    Ok(Blob {
        name: name,
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

pub fn from_headers(blob_name: &str, h: &Headers) -> Result<Blob, AzureError> {
    let content_type = match h.get::<ContentType>() {
        Some(ct) => (ct as &Mime).clone(),
        None => try!("application/octet-stream".parse::<Mime>()),
    };
    println!("content_type == {:?}", content_type);

    let content_length = match h.get::<ContentLength>() {
        Some(cl) => (cl as &u64).clone(),
        None => return Err(AzureError::HeaderNotFound("Content-Length".to_owned())),
    };
    println!("content_length == {:?}", content_length);

    let last_modified = match h.get::<LastModified>() {
        Some(lm) => try!(from_azure_time(&lm.to_string())),
        None => return Err(AzureError::HeaderNotFound("Last-Modified".to_owned())),
    };
    println!("last_modified == {:?}", last_modified);

    let etag = match h.get::<ETag>() {
        Some(lm) => lm.to_string(),
        None => return Err(AzureError::HeaderNotFound("ETag".to_owned())),
    };
    println!("etag == {:?}", etag);

    let x_ms_blob_sequence_number = match h.get::<XMSBlobSequenceNumber>() {
        Some(lm) => Some((&lm as &u64).clone()),
        None => None,
    };
    println!("x_ms_blob_sequence_number == {:?}",
             x_ms_blob_sequence_number);

    let blob_type = match h.get::<XMSBlobType>() {
        Some(lm) => try!((&lm.to_string()).parse::<BlobType>()),
        None => return Err(AzureError::HeaderNotFound("x-ms-blob-type".to_owned())),
    };
    println!("blob_type == {:?}", blob_type);

    let content_encoding = match h.get::<ContentEncoding>() {
        Some(ce) => Some(ce.to_string()),
        None => None,
    };
    println!("content_encoding == {:?}", content_encoding);

    let content_language = match h.get::<ContentLanguage>() {
        Some(cl) => Some(cl.to_string()),
        None => None,
    };
    println!("content_language == {:?}", content_language);

    let content_md5 = match h.get::<ContentMD5>() {
        Some(md5) => Some(md5.to_string()),
        None => None,
    };
    println!("content_md5 == {:?}", content_md5);

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
    println!("lease_status == {:?}", lease_status);


    let lease_state = match h.get::<XMSLeaseState>() {
        Some(ls) => try!(ls.to_string().parse::<LeaseState>()),
        None => return Err(AzureError::HeaderNotFound("x-ms-lease-state".to_owned())),
    };
    println!("lease_state == {:?}", lease_state);


    let lease_duration = match h.get::<XMSLeaseDuration>() {
        Some(ls) => Some(try!(ls.to_string().parse::<LeaseDuration>())),
        None => None,
    };
    println!("lease_duration == {:?}", lease_duration);

    // TODO: get the remaining headers (https://msdn.microsoft.com/en-us/library/azure/dd179440.aspx)

    Ok(Blob {
        name: blob_name.to_owned(),
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

impl Blob {
    pub fn put_block_blob(&self, container_name: &str, r: &Read) {}
}
