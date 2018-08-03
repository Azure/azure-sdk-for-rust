extern crate hyper;
extern crate ring;
extern crate url;

#[macro_use]
pub mod errors;
pub mod parsing;
#[macro_use]
pub mod enumerations;
pub mod incompletevector;
pub mod lease;
use azure::core::util::HeaderMapExt;
use azure::storage::client::Client;
use std::fmt::Debug;
pub mod ba512_range;
use base64::encode;
pub mod range;
use url::percent_encoding;
define_encode_set! {
    pub COMPLETE_ENCODE_SET = [percent_encoding::USERINFO_ENCODE_SET] | {
        '+', '-', '&'
    }
}
pub mod headers;
use self::headers::{CONTENT_MD5, CLIENT_REQUEST_ID, LEASE_BREAK_PERIOD, LEASE_DURATION, LEASE_ID, PROPOSED_LEASE_ID, REQUEST_ID};
use hyper::header::{CACHE_CONTROL, CONTENT_ENCODING, CONTENT_LANGUAGE, CONTENT_TYPE, RANGE};
use uuid::Uuid;
pub type RequestId = Uuid;
use azure::core::errors::AzureError;
use azure::core::lease::LeaseId;
use http::request::Builder;
use http::HeaderMap;
use std::collections::HashMap;
mod stored_access_policy;
pub(crate) mod util;
pub use self::stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Yes;
#[derive(Debug)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

pub trait ClientRequired<'a> {
    fn client(&self) -> &'a Client;
}

pub trait TimeoutSupport {
    type O;
    fn with_timeout(self, timeout: u64) -> Self::O;
}

pub trait TimeoutOption {
    fn timeout(&self) -> Option<u64>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(nm) = self.timeout() {
            Some(format!("timeout={}", nm))
        } else {
            None
        }
    }
}

pub trait ClientRequestIdSupport<'a> {
    type O;
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O;
}

pub trait ClientRequestIdOption<'a> {
    fn client_request_id(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(client_request_id) = self.client_request_id() {
            builder.header(CLIENT_REQUEST_ID, client_request_id);
        }
    }
}

pub trait ContentDispositionSupport<'a> {
    type O;
    fn with_content_disposition(self, content_disposition: &'a str) -> Self::O;
}

pub trait ContentDispositionOption<'a> {
    fn content_disposition(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(content_disposition) = self.content_disposition() {
            builder.header(CACHE_CONTROL, content_disposition);
        }
    }
}

pub trait MetadataSupport<'a> {
    type O;
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O;
}

pub trait MetadataOption<'a> {
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(metadata) = self.metadata() {
            for (key, val) in metadata.iter() {
                builder.header(&format!("x-ms-meta-{}", key) as &str, val as &str);
            }
        }
    }
}

pub trait CacheControlSupport<'a> {
    type O;
    fn with_cache_control(self, cache_control: &'a str) -> Self::O;
}

pub trait CacheControlOption<'a> {
    fn cache_control(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(cache_control) = self.cache_control() {
            builder.header(CACHE_CONTROL, cache_control);
        }
    }
}

pub trait ContentEncodingSupport<'a> {
    type O;
    fn with_content_encoding(self, content_encoding: &'a str) -> Self::O;
}

pub trait ContentEncodingOption<'a> {
    fn content_encoding(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(content_encoding) = self.content_encoding() {
            builder.header(CONTENT_ENCODING, content_encoding);
        }
    }
}

pub trait ContentTypeSupport<'a> {
    type O;
    fn with_content_type(self, content_type: &'a str) -> Self::O;
}

pub trait ContentTypeOption<'a> {
    fn content_type(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(content_type) = self.content_type() {
            builder.header(CONTENT_TYPE, content_type);
        }
    }
}

pub trait ContentLanguageSupport<'a> {
    type O;
    fn with_content_language(self, content_language: &'a str) -> Self::O;
}

pub trait ContentLanguageOption<'a> {
    fn content_language(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(content_language) = self.content_language() {
            builder.header(CONTENT_LANGUAGE, content_language);
        }
    }
}

pub trait NextMarkerSupport<'a> {
    type O;
    fn with_next_marker(self, next_marker: &'a str) -> Self::O;
}

pub trait NextMarkerOption<'a> {
    fn next_marker(&self) -> Option<&'a str>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(ref nm) = self.next_marker() {
            Some(format!("marker={}", nm))
        } else {
            None
        }
    }
}

pub trait SnapshotSupport {
    type O;
    fn with_snapshot(self, snapshot: DateTime<Utc>) -> Self::O;
}

pub trait SnapshotOption {
    fn snapshot(&self) -> Option<DateTime<Utc>>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(ref nm) = self.snapshot() {
            Some(format!("snapshot={}", nm.to_rfc2822()))
        } else {
            None
        }
    }
}

pub trait DelimiterSupport<'a> {
    type O;
    fn with_delimiter(self, delimiter: &'a str) -> Self::O;
}

pub trait DelimiterOption<'a> {
    fn delimiter(&self) -> Option<&'a str>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(_) = self.delimiter() {
            Some("delimiter".to_owned())
        } else {
            None
        }
    }
}

pub trait MaxResultsSupport {
    type O;
    fn with_max_results(self, max_results: u32) -> Self::O;
}

pub trait MaxResultsOption {
    fn max_results(&self) -> Option<u32>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(ref nm) = self.max_results() {
            Some(format!("maxresults={}", nm))
        } else {
            None
        }
    }
}

pub trait IncludeSnapshotsSupport {
    type O;
    fn with_include_snapshots(self) -> Self::O;
}

pub trait IncludeSnapshotsOption {
    fn include_snapshots(&self) -> bool;
}

pub trait IncludeUncommittedBlobsSupport {
    type O;
    fn with_include_uncommitted_blobs(self) -> Self::O;
}

pub trait IncludeUncommittedBlobsOption {
    fn include_uncommitted_blobs(&self) -> bool;
}

pub trait IncludeMetadataSupport {
    type O;
    fn with_include_metadata(self) -> Self::O;
}

pub trait IncludeMetadataOption {
    fn include_metadata(&self) -> bool;
}

pub trait IncludeCopySupport {
    type O;
    fn with_include_copy(self) -> Self::O;
}

pub trait IncludeCopyOption {
    fn include_copy(&self) -> bool;
}

pub trait IncludeDeletedSupport {
    type O;
    fn with_include_deleted(self) -> Self::O;
}

pub trait IncludeDeletedOption {
    fn include_deleted(&self) -> bool;
}

pub trait IncludeListOptions:
    IncludeSnapshotsOption + IncludeMetadataOption + IncludeUncommittedBlobsOption + IncludeCopyOption + IncludeDeletedOption
{
    fn to_uri_parameter(&self) -> Option<String> {
        let mut s = String::new();
        let mut f_first = true;

        if self.include_snapshots() {
            s.push_str("snapshots");
            f_first = false;
        }

        if self.include_metadata() {
            if !f_first {
                s.push_str(",");
            }
            s.push_str("metadata");
            f_first = false;
        }

        if self.include_uncommitted_blobs() {
            if !f_first {
                s.push_str(",");
            }
            s.push_str("uncommittedblobs");
            f_first = false;
        }

        if self.include_copy() {
            if !f_first {
                s.push_str(",");
            }
            s.push_str("copy");
            f_first = false;
        }

        if self.include_deleted() {
            if !f_first {
                s.push_str(",");
            }
            s.push_str("deleted");
        }

        if s.len() > 0 {
            Some(format!("include={}", s))
        } else {
            None
        }
    }
}

pub trait PrefixSupport<'a> {
    type O;
    fn with_prefix(self, prefix: &'a str) -> Self::O;
}

pub trait PrefixOption<'a> {
    fn prefix(&self) -> Option<&'a str>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(ref nm) = self.prefix() {
            Some(format!("prefix={}", nm))
        } else {
            None
        }
    }
}

pub trait LeaseIdSupport<'a> {
    type O;
    fn with_lease_id(self, &'a LeaseId) -> Self::O;
}

pub trait LeaseIdOption<'a> {
    fn lease_id(&self) -> Option<&'a LeaseId>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(lease_id) = self.lease_id() {
            builder.header(LEASE_ID, &lease_id.to_string() as &str);
        }
    }
}

pub trait LeaseIdRequired<'a> {
    fn lease_id(&self) -> &'a LeaseId;

    fn add_header(&self, builder: &mut Builder) {
        builder.header(LEASE_ID, &self.lease_id().to_string() as &str);
    }
}

pub trait BodySupport<'a> {
    type O;
    fn with_body(self, &'a [u8]) -> Self::O;
}

pub trait BodyRequired<'a> {
    fn body(&self) -> &'a [u8];
}

pub trait ContentMD5Support<'a> {
    type O;
    fn with_content_md5(self, &'a [u8]) -> Self::O;
}

pub trait ContentMD5Option<'a> {
    fn content_md5(&self) -> Option<&'a [u8]>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(content_md5) = self.content_md5() {
            let s = encode(content_md5);
            builder.header(CONTENT_MD5, &s as &str);
        }
    }
}

pub trait RangeSupport<'a> {
    type O;
    fn with_range(self, &'a range::Range) -> Self::O;
}

pub trait RangeOption<'a> {
    fn range(&self) -> Option<&'a range::Range>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(range) = self.range() {
            builder.header(RANGE, &range.to_string() as &str);
        }
    }
}

pub trait LeaseDurationSupport {
    type O;
    fn with_lease_duration(self, i8) -> Self::O;
}

pub trait LeaseDurationRequired {
    fn lease_duration(&self) -> i8;

    fn add_header(&self, builder: &mut Builder) {
        builder.header(LEASE_DURATION, &self.lease_duration().to_string() as &str);
    }
}

pub trait ProposedLeaseIdSupport<'a> {
    type O;
    fn with_proposed_lease_id(self, &'a LeaseId) -> Self::O;
}

pub trait ProposedLeaseIdOption<'a> {
    fn proposed_lease_id(&self) -> Option<&'a LeaseId>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(pld) = self.proposed_lease_id() {
            builder.header(PROPOSED_LEASE_ID, &pld.to_string() as &str);
        }
    }
}

pub trait ProposedLeaseIdRequired<'a> {
    fn proposed_lease_id(&self) -> &'a LeaseId;

    fn add_header(&self, builder: &mut Builder) {
        builder.header(PROPOSED_LEASE_ID, &self.proposed_lease_id().to_string() as &str);
    }
}

pub trait LeaseBreakPeriodSupport {
    type O;
    fn with_lease_break_period(self, lease_break_period: u8) -> Self::O;
}

pub trait LeaseBreakPeriodOption {
    fn lease_break_period(&self) -> Option<u8>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(lease_break_period) = self.lease_break_period() {
            builder.header(LEASE_BREAK_PERIOD, &lease_break_period.to_string() as &str);
        }
    }
}

pub trait ContainerNameSupport<'a> {
    type O;
    fn with_container_name(self, container_name: &'a str) -> Self::O;
}

pub trait ContainerNameRequired<'a> {
    fn container_name(&self) -> &'a str;
}

pub trait BlobNameSupport<'a> {
    type O;
    fn with_blob_name(self, blob_name: &'a str) -> Self::O;
}

pub trait BlobNameRequired<'a> {
    fn blob_name(&self) -> &'a str;
}

pub(crate) fn request_id_from_headers(headers: &HeaderMap) -> Result<RequestId, AzureError> {
    let request_id = headers
        .get_as_str(REQUEST_ID)
        .ok_or_else(|| AzureError::HeaderNotFound(REQUEST_ID.to_owned()))?;
    Ok(Uuid::parse_str(request_id)?)
}
