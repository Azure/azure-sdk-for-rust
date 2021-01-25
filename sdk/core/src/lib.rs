#![recursion_limit = "256"]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

pub mod ba512_range;
mod client_request_id;
mod content_disposition;
mod content_encoding;
mod content_language;
mod content_type;
mod delimiter;
pub mod errors;
pub mod headers;
mod http_client;
mod if_match_condition;
mod if_modified_since_condition;
mod if_source_match_condition;
mod if_source_modified_since_condition;
pub mod incompletevector;
pub mod lease;
mod lease_break_period;
mod lease_duration;
mod max_results;
mod metadata;
mod next_marker;
pub mod parsing;
mod prefix;
pub mod prelude;
mod proposed_lease_id;
pub mod range;
mod sequence_number;
mod sequence_number_condition;
mod source_content_md5;
mod source_lease_id;
mod stored_access_policy;
mod timeout;
pub mod util;

pub use self::stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};
use crate::errors::AzureError;
use crate::lease::LeaseId;
use base64::encode;
use chrono::{DateTime, Utc};
pub use client_request_id::ClientRequestId;
pub use content_disposition::ContentDisposition;
pub use content_encoding::ContentEncoding;
pub use content_language::ContentLanguage;
pub use content_type::ContentType;
pub use delimiter::Delimiter;
use headers::*;
use http::request::Builder;
pub use http_client::*;
use hyper::header::{
    CONTENT_ENCODING, CONTENT_LANGUAGE, CONTENT_LENGTH, CONTENT_TYPE, IF_MODIFIED_SINCE, RANGE,
    USER_AGENT,
};
pub use if_match_condition::IfMatchCondition;
pub use if_modified_since_condition::IfModifiedSinceCondition;
pub use if_source_match_condition::IfSourceMatchCondition;
pub use if_source_modified_since_condition::IfSourceModifiedSinceCondition;
pub use lease_break_period::LeaseBreakPeriod;
pub use lease_duration::LeaseDuration;
pub use max_results::MaxResults;
pub use metadata::Metadata;
pub use next_marker::NextMarker;
use oauth2::AccessToken;
pub use prefix::Prefix;
pub use proposed_lease_id::ProposedLeaseId;
pub use sequence_number::SequenceNumber;
pub use sequence_number_condition::SequenceNumberCondition;
pub use source_content_md5::SourceContentMD5;
pub use source_lease_id::SourceLeaseId;
use std::collections::HashMap;
use std::fmt::Debug;
pub use timeout::Timeout;
use uuid::Uuid;

pub type RequestId = Uuid;
pub type SessionToken = String;

/// Represents an Azure service bearer access token with expiry information.
#[derive(Debug, Clone)]
pub struct TokenResponse {
    /// Get the access token value.
    pub token: AccessToken,
    /// Gets the time when the provided token expires.
    pub expires_on: DateTime<Utc>,
}

impl TokenResponse {
    /// Create a new `TokenResponse`
    pub fn new(token: AccessToken, expires_on: DateTime<Utc>) -> Self {
        Self { token, expires_on }
    }
}

/// Represents a credential capable of providing an OAuth token.
#[async_trait::async_trait]
pub trait TokenCredential: Send + Sync {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError>;
}

#[macro_export]
macro_rules! response_from_headers {
    ($cn:ident, $($fh:path => $na:ident: $typ:ty),+) => {
        use http::HeaderMap;

        #[derive(Debug, Clone, PartialEq)]
        pub struct $cn {
             $(pub $na: $typ),+,
        }

        impl $cn {
            pub(crate) fn from_headers(headers: &HeaderMap) -> Result<$cn, $crate::errors::AzureError> {
               $(
                    let $na = $fh(headers)?;
                )+

                Ok($cn {
                    $($na,)+
                })
            }

        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct Yes;
#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

#[derive(Debug, Clone, PartialEq)]
pub enum Consistency {
    Md5([u8; 16]),
    Crc64([u8; 8]),
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

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(timeout) = self.timeout() {
            url.query_pairs_mut()
                .append_pair("timeout", &format!("{}", timeout));
        }
    }
}

pub trait ClientRequestIdSupport<'a> {
    type O;
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O;
}

pub trait ClientRequestIdOption<'a> {
    fn client_request_id(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(client_request_id) = self.client_request_id() {
            builder = builder.header(CLIENT_REQUEST_ID, client_request_id);
        }
        builder
    }
}

pub trait AppendPositionSupport {
    type O;
    fn with_append_position(self, append_position: u32) -> Self::O;
}

pub trait AppendPositionOption {
    fn append_position(&self) -> Option<u32>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(append_position) = self.append_position() {
            builder = builder.header(APPEND_POSITION, append_position);
        }
        builder
    }
}

pub trait ContentDispositionSupport<'a> {
    type O;
    fn with_content_disposition(self, content_disposition: &'a str) -> Self::O;
}

pub trait ContentDispositionOption<'a> {
    fn content_disposition(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(content_disposition) = self.content_disposition() {
            builder = builder.header(CONTENT_DISPOSITION, content_disposition);
        }
        builder
    }
}

pub trait MetadataSupport<'a> {
    type O;
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O;
}

pub trait MetadataOption<'a> {
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(metadata) = self.metadata() {
            for (key, val) in metadata.iter() {
                builder = builder.header(&format!("x-ms-meta-{}", key) as &str, val as &str);
            }
        }
        builder
    }
}

pub trait CacheControlSupport<'a> {
    type O;
    fn with_cache_control(self, cache_control: &'a str) -> Self::O;
}

pub trait CacheControlOption<'a> {
    fn cache_control(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(cache_control) = self.cache_control() {
            builder = builder.header(CACHE_CONTROL, cache_control);
        }
        builder
    }
}

pub trait IsSynchronousSupport {
    type O;
    fn with_is_synchronous(self, is_synchronous: bool) -> Self::O;
}

pub trait IsSynchronousOption {
    fn is_synchronous(&self) -> bool;

    #[must_use]
    fn add_optional_header(&self, builder: Builder) -> Builder {
        builder.header(REQUIRES_SYNC, format!("{}", self.is_synchronous()))
    }
}

pub trait ContentEncodingSupport<'a> {
    type O;
    fn with_content_encoding(self, content_encoding: &'a str) -> Self::O;
}

pub trait ContentEncodingOption<'a> {
    fn content_encoding(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(content_encoding) = self.content_encoding() {
            builder = builder.header(CONTENT_ENCODING, content_encoding);
        }
        builder
    }
}

pub trait ContentTypeSupport<'a> {
    type O;
    fn with_content_type(self, content_type: &'a str) -> Self::O;
}

pub trait ContentTypeOption<'a> {
    fn content_type(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(content_type) = self.content_type() {
            builder = builder.header(CONTENT_TYPE, content_type);
        }
        builder
    }
}

pub trait ContentTypeRequired<'a> {
    fn content_type(&self) -> &'a str;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(CONTENT_TYPE, self.content_type())
    }
}

pub trait SourceUrlSupport<'a> {
    type O;
    fn with_source_url(self, source_url: &'a str) -> Self::O;
}

pub trait SourceUrlRequired<'a> {
    fn source_url(&self) -> &'a str;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(COPY_SOURCE, self.source_url())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IfModifiedSince<'a>(&'a DateTime<Utc>);

impl<'a> IfModifiedSince<'a> {
    pub fn new(time: &'a DateTime<Utc>) -> Self {
        Self(time)
    }
}

impl AddAsHeader for IfModifiedSince<'_> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(IF_MODIFIED_SINCE, self.0.to_rfc2822())
    }
}

pub trait IfModifiedSinceSupport<'a> {
    type O;
    fn with_if_modified_since(self, if_modified_since: &'a DateTime<Utc>) -> Self::O;
}

pub trait IfModifiedSinceOption<'a> {
    fn if_modified_since(&self) -> Option<&'a DateTime<Utc>>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(if_modified_since) = self.if_modified_since() {
            builder = builder.header(IF_MODIFIED_SINCE, if_modified_since.to_rfc2822());
        }
        builder
    }
}

pub trait UserAgentSupport<'a> {
    type O;
    fn with_user_agent(self, user_agent: &'a str) -> Self::O;
}

#[derive(Debug, Clone, Copy)]
pub struct UserAgent<'a>(&'a str);

impl<'a> UserAgent<'a> {
    pub fn new(agent: &'a str) -> Self {
        Self(agent)
    }
}

impl<'a> AddAsHeader for UserAgent<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(USER_AGENT, self.0)
    }
}

pub trait ActivityIdSupport<'a> {
    type O;
    fn with_activity_id(self, activity_id: &'a str) -> Self::O;
}

#[derive(Debug, Clone, Copy)]
pub struct ActivityId<'a>(&'a str);

impl<'a> ActivityId<'a> {
    pub fn new(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> AddAsHeader for ActivityId<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(ACTIVITY_ID, self.0)
    }
}

pub trait ContentLanguageSupport<'a> {
    type O;
    fn with_content_language(self, content_language: &'a str) -> Self::O;
}

pub trait ContentLanguageOption<'a> {
    fn content_language(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(content_language) = self.content_language() {
            builder = builder.header(CONTENT_LANGUAGE, content_language);
        }
        builder
    }
}

pub trait BlockIdSupport<'a> {
    type O;
    fn with_block_id(self, block_id: &'a [u8]) -> Self::O;
}

pub trait BlockIdRequired<'a> {
    fn block_id(&self) -> &'a [u8];

    fn to_uri_parameter(&self) -> String {
        format!("blockid={}", base64::encode(self.block_id()))
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

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(next_marker) = self.next_marker() {
            url.query_pairs_mut().append_pair("marker", next_marker);
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

pub trait SnapshotRequired {
    fn snapshot(&self) -> DateTime<Utc>;

    fn to_uri_parameter(&self) -> String {
        format!("snapshot={}", self.snapshot().to_rfc2822())
    }
}

pub trait DelimiterSupport<'a> {
    type O;
    fn with_delimiter(self, delimiter: &'a str) -> Self::O;
}

pub trait DelimiterOption<'a> {
    fn delimiter(&self) -> Option<&'a str>;

    fn to_uri_parameter(&self) -> Option<String> {
        if self.delimiter().is_some() {
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

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(max_results) = self.max_results() {
            url.query_pairs_mut()
                .append_pair("maxresults", &format!("{}", max_results));
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

    fn to_uri_parameter(&self) -> Option<&'static str> {
        if self.include_metadata() {
            Some("include=metadata")
        } else {
            None
        }
    }

    fn append_to_url(&self, url: &mut url::Url) {
        if self.include_metadata() {
            url.query_pairs_mut().append_pair("include", "metadata");
        }
    }
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
    IncludeSnapshotsOption
    + IncludeMetadataOption
    + IncludeUncommittedBlobsOption
    + IncludeCopyOption
    + IncludeDeletedOption
{
    fn to_uri_parameter(&self) -> Option<String> {
        let mut s = String::new();
        let mut f_first = if self.include_snapshots() {
            s.push_str("snapshots");
            false
        } else {
            true
        };

        if self.include_metadata() {
            if !f_first {
                s.push(',');
            }
            s.push_str("metadata");
            f_first = false;
        }

        if self.include_uncommitted_blobs() {
            if !f_first {
                s.push(',');
            }
            s.push_str("uncommittedblobs");
            f_first = false;
        }

        if self.include_copy() {
            if !f_first {
                s.push(',');
            }
            s.push_str("copy");
            f_first = false;
        }

        if self.include_deleted() {
            if !f_first {
                s.push(',');
            }
            s.push_str("deleted");
        }

        if !s.is_empty() {
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

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(prefix) = self.prefix() {
            url.query_pairs_mut().append_pair("prefix", prefix);
        }
    }
}

pub trait SequenceNumberSupport {
    type O;
    fn with_sequence_number(self, sequence_number: u64) -> Self::O;
}

pub trait SequenceNumberOption {
    fn sequence_number(&self) -> u64;

    #[must_use]
    fn add_optional_header(&self, builder: Builder) -> Builder {
        builder.header(
            BLOB_SEQUENCE_NUMBER,
            &self.sequence_number().to_string() as &str,
        )
    }
}

pub trait SequenceNumberConditionSupport {
    type O;
    fn with_sequence_number_condition(
        self,
        sequence_number_condition: SequenceNumberCondition,
    ) -> Self::O;
}

pub trait PageBlobLengthSupport {
    type O;
    fn with_content_length(self, content_length: u64) -> Self::O;
}

pub trait PageBlobLengthRequired {
    fn content_length(&self) -> u64;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(
            BLOB_CONTENT_LENGTH,
            &self.content_length().to_string() as &str,
        )
    }
}

pub trait ContentLengthSupport {
    type O;
    fn with_content_length(self, content_length: u64) -> Self::O;
}

pub trait ContentLengthOption {
    fn content_length(&self) -> Option<u64>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(content_length) = self.content_length() {
            builder = builder.header(CONTENT_LENGTH, &content_length.to_string() as &str);
        }
        builder
    }
}

pub trait ContentLengthRequired {
    fn content_length(&self) -> u64;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(CONTENT_LENGTH, &self.content_length().to_string() as &str)
    }
}

pub trait LeaseIdSupport<'a> {
    type O;
    fn with_lease_id(self, _: &'a LeaseId) -> Self::O;
}

pub trait LeaseIdOption<'a> {
    fn lease_id(&self) -> Option<&'a LeaseId>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(lease_id) = self.lease_id() {
            builder = builder.header(LEASE_ID, &lease_id.to_string() as &str);
        }
        builder
    }
}

pub trait SourceLeaseIdSupport<'a> {
    type O;
    fn with_source_lease_id(self, _: &'a LeaseId) -> Self::O;
}

pub trait SourceLeaseIdOption<'a> {
    fn source_lease_id(&self) -> Option<&'a LeaseId>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(lease_id) = self.source_lease_id() {
            builder = builder.header(SOURCE_LEASE_ID, &lease_id.to_string() as &str);
        }
        builder
    }
}

pub trait LeaseIdRequired<'a> {
    fn lease_id(&self) -> &'a LeaseId;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(LEASE_ID, &self.lease_id().to_string() as &str)
    }
}

pub trait BodySupport<'a> {
    type O;
    fn with_body(self, _: &'a [u8]) -> Self::O;
}

pub trait BodyRequired<'a> {
    fn body(&self) -> &'a [u8];
}

pub trait ContentMD5Support<'a> {
    type O;
    fn with_content_md5(self, _: &'a [u8]) -> Self::O;
}

pub trait ContentMD5Option<'a> {
    fn content_md5(&self) -> Option<&'a [u8]>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(content_md5) = self.content_md5() {
            builder = add_content_md5_header(content_md5, builder);
        }
        builder
    }
}

pub trait SourceContentMD5Support<'a> {
    type O;
    fn with_source_content_md5(self, _: &'a [u8]) -> Self::O;
}

pub trait SourceContentMD5Option<'a> {
    fn source_content_md5(&self) -> Option<&'a [u8]>;

    #[must_use]
    fn add_optional_header(&self, builder: Builder) -> Builder {
        if let Some(source_content_md5) = self.source_content_md5() {
            let s = encode(source_content_md5);
            builder.header(SOURCE_CONTENT_MD5, &s as &str)
        } else {
            builder
        }
    }
}

#[inline]
#[must_use]
pub fn add_content_md5_header(content_md5: &[u8], builder: Builder) -> Builder {
    let s = encode(content_md5);
    builder.header(CONTENT_MD5, &s as &str)
}

pub trait ChunkSizeSupport {
    type O;
    fn with_chunk_size(self, chunk_size: u64) -> Self::O;
}

pub trait ChunkSizeOption {
    fn chunk_size(&self) -> u64;
}

pub trait RangeSupport<'a> {
    type O;
    fn with_range(self, _: &'a range::Range) -> Self::O;
}

pub trait RangeOption<'a> {
    fn range(&self) -> Option<&'a range::Range>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(range) = self.range() {
            builder = builder.header(RANGE, &range.to_string() as &str);
        }
        builder
    }
}

pub trait RangeRequired<'a> {
    fn range(&self) -> &'a range::Range;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(RANGE, &self.range().to_string() as &str)
    }
}

pub trait BA512RangeSupport<'a> {
    type O;
    fn with_ba512_range(self, _: &'a ba512_range::BA512Range) -> Self::O;
}

pub trait BA512RangeOption<'a> {
    fn ba512_range(&self) -> Option<&'a ba512_range::BA512Range>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(ba512_range) = self.ba512_range() {
            builder = builder.header(RANGE, &ba512_range.to_string() as &str);
        }
        builder
    }
}

pub trait BA512RangeRequired<'a> {
    fn ba512_range(&self) -> &'a ba512_range::BA512Range;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(RANGE, &self.ba512_range().to_string() as &str)
    }
}

pub trait LeaseDurationSupport {
    type O;
    fn with_lease_duration(self, _: i8) -> Self::O;
}

pub trait LeaseDurationRequired {
    fn lease_duration(&self) -> i8;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(LEASE_DURATION, &self.lease_duration().to_string() as &str)
    }
}

pub trait ProposedLeaseIdSupport<'a> {
    type O;
    fn with_proposed_lease_id(self, _: &'a LeaseId) -> Self::O;
}

pub trait ProposedLeaseIdOption<'a> {
    fn proposed_lease_id(&self) -> Option<&'a LeaseId>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(pld) = self.proposed_lease_id() {
            builder = builder.header(PROPOSED_LEASE_ID, &pld.to_string() as &str);
        }
        builder
    }
}

pub trait ProposedLeaseIdRequired<'a> {
    fn proposed_lease_id(&self) -> &'a LeaseId;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(
            PROPOSED_LEASE_ID,
            &self.proposed_lease_id().to_string() as &str,
        )
    }
}

pub trait LeaseBreakPeriodSupport {
    type O;
    fn with_lease_break_period(self, lease_break_period: u8) -> Self::O;
}

pub trait LeaseBreakPeriodRequired {
    fn lease_break_period(&self) -> u8;

    #[must_use]
    fn add_mandatory_header(&self, builder: Builder) -> Builder {
        builder.header(
            LEASE_BREAK_PERIOD,
            &self.lease_break_period().to_string() as &str,
        )
    }
}

pub trait LeaseBreakPeriodOption {
    fn lease_break_period(&self) -> Option<u8>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(lease_break_period) = self.lease_break_period() {
            builder = builder.header(LEASE_BREAK_PERIOD, &lease_break_period.to_string() as &str);
        }
        builder
    }
}

pub trait ContinuationSupport<'a> {
    type O;
    fn with_continuation(self, continuation: &'a str) -> Self::O;
}

#[derive(Debug, Clone, Copy)]
pub struct Continuation<'a>(&'a str);
impl<'a> Continuation<'a> {
    pub fn new(c: &'a str) -> Self {
        Self(c)
    }
}

impl AddAsHeader for Continuation<'_> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(CONTINUATION, self.0)
    }
}

pub trait ContinuationOption<'a> {
    fn continuation(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, builder: Builder) -> Builder {
        if let Some(continuation) = self.continuation() {
            builder.header(CONTINUATION, continuation)
        } else {
            builder
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

pub trait AddAsHeader {
    fn add_as_header(&self, builder: Builder) -> Builder;
}

pub trait AppendToUrlQuery {
    fn append_to_url_query(&self, url: &mut url::Url);
}

impl<T> AppendToUrlQuery for Option<T>
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut url::Url) {
        if let Some(i) = self {
            i.append_to_url_query(url);
        }
    }
}
