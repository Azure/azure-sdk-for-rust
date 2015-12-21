use chrono::datetime::DateTime;
use chrono::UTC;

use azure::storage::{LeaseStatus, LeaseState, LeaseDuration};
use azure::core;
use azure::core::parsing::{traverse_single_must, traverse_single_optional, traverse,
                           from_azure_time, inner_text, traverse_inner_text_must,
                           traverse_inner_text_optional, traverse_inner_date_optional,
                           traverse_inner_date_must, traverse_single_cast_must,
                           traverse_single_cast_optional, traverse_inner_u64_optional};

use xml::Element;

use std::str::FromStr;
use azure::core::enumerations;
use std::io::Read;
use std::fmt;

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CopyProgress {
    pub completed: u64,
    pub total: u64,
}

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
    pub copy_progress: Option<CopyProgress>,
    pub copy_completion: Option<DateTime<UTC>>,
    pub copy_status_description: Option<String>,
}

pub fn parse(elem: &Element) -> Result<Blob, core::errors::AzureError> {
    let name = try!(traverse_inner_text_must(elem, &["Name"]));
    let snapshot_time = try!(traverse_inner_date_optional(elem, &["Snapshot"]));
    let last_modified = try!(traverse_inner_date_must(elem, &["Properties", "Last-Modified"]));
    let etag = try!(traverse_inner_text_must(elem, &["Properties", "Etag"]));
    let content_length = try!(try!(traverse_inner_text_must(elem,
                                                            &["Properties", "Content-Length"]))
                                  .parse::<u64>());
    let content_type = try!(traverse_inner_text_must(elem, &["Properties", "Content-Type"]));
    let content_encoding = try!(traverse_inner_text_optional(elem,
                                                             &["Properties", "Content-Encoding"]));
    let content_language = try!(traverse_inner_text_optional(elem,
                                                             &["Properties", "Content-Language"]));
    let content_md5 = try!(traverse_inner_text_optional(elem, &["Properties", "Content-MD5"]));
    let cache_control = try!(traverse_inner_text_optional(elem, &["Properties", "Cache-Control"]));
    let x_ms_blob_sequence_number = try!(traverse_inner_text_optional(elem,
                                                                      &["Properties",
                                                                        "x-ms-blob-sequence-num\
                                                                         ber"]));
    let blob_type = try!(traverse_single_cast_must::<BlobType>(elem, &["Properties", "BlobType"]));
    let lease_status = try!(traverse_single_cast_must::<LeaseStatus>(elem,
                                                                     &["Properties",
                                                                       "LeaseStatus"]));
    let lease_state = try!(traverse_single_cast_must::<LeaseState>(elem,
                                                                   &["Properties", "LeaseState"]));
    let lease_duration = try!(traverse_single_cast_optional::<LeaseDuration>(elem,
                                                                             &["Properties",
                                                                               "LeaseDuration"]));
    let copy_id = try!(traverse_inner_text_optional(elem, &["Properties", "CopyId"]));
    let copy_status = try!(traverse_single_cast_optional::<CopyStatus>(elem,
                                                                       &["Properties",
                                                                         "CopyStatus"]));
    let copy_source = try!(traverse_inner_text_optional(elem, &["Properties", "CopySource"]));
    let copy_progress = try!(traverse_inner_text_optional(elem, &["Properties", "CopyProgress"]));
    let copy_completion = try!(traverse_inner_date_optional(elem,
                                                            &["Properties", "CopyCompletionTime"]));
    let copy_status_description = try!(traverse_inner_text_optional(elem,
                                                                    &["Properties",
                                                                      "CopyStatusDescription"]));

    let mut cp_bytes: Option<CopyProgress> = None;
    if let Some(txt) = copy_progress {
        let v = txt.split("/").collect::<Vec<&str>>();
        if v.len() < 2 {
            return Err(core::errors::AzureError::ResponseParsingError(core::errors::TraversingError::GenericParseError("not enough information in CopyProgress field".to_owned())));
        }

        let cp_bytes_completed = try!(v[0].parse::<u64>());
        let cp_bytes_total = try!(v[1].parse::<u64>());

        cp_bytes = Some(CopyProgress {
            completed: cp_bytes_completed,
            total: cp_bytes_total,
        });
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

    // panic!("dd");
}

impl Blob {}
