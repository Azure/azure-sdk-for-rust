use chrono::datetime::DateTime;
use chrono::UTC;

use azure::storage::{LeaseStatus, LeaseState, LeaseDuration};
use azure::core;
use azure::core::parsing::{cast_must, cast_optional};

use xml::Element;

use std::str::FromStr;
use azure::core::enumerations;
use std::fmt;

use azure::core::errors::TraversingError;
use azure::core::parsing::FromStringOptional;

use azure::core::range::Range;

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

#[derive(Debug)]
pub struct Blob {
    pub name: String,
    pub snapshot_time: Option<DateTime<UTC>>,
    pub last_modified: DateTime<UTC>,
    pub etag: String,
    pub content_length: u64,
    pub content_type: String,
    pub content_encoding: Option<String>,
    pub content_language: Option<String>,
    pub content_md5: Option<String>,
    pub cache_control: Option<String>,
    pub x_ms_blob_sequence_number: Option<String>,
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

pub fn parse(elem: &Element) -> Result<Blob, core::errors::AzureError> {
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
    let x_ms_blob_sequence_number = try!(cast_optional::<String>(elem,
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

    Ok(Blob {
        name: name,
        snapshot_time: snapshot_time,
        last_modified: last_modified,
        etag: etag,
        content_length: content_length,
        content_type: content_type,
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

impl Blob {}
