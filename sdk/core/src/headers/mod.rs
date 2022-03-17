//! Azure HTTP headers.
mod utilities;

use http::{request::Builder, HeaderValue};

pub use utilities::*;

/// View a type as an HTTP header.
///
/// Ad interim we require two functions: `add_as_header` and `add_as_header2`. Make sure
/// your implementations are functionally equivalent between the two. In other words, the
/// effect should be the same regardless of which function the SDK calls.
///
/// While not restricted by the type system, please add HTTP headers only. In particular, do not
/// interact with the body of the request.
///
/// As soon as the migration to the pipeline architecture will be complete we will phase out
/// `add_as_header`.
pub trait Header {
    fn name(&self) -> &'static str;
    fn value(&self) -> String;

    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(self.name(), self.value())
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HttpHeaderError> {
        request
            .headers_mut()
            .insert(self.name(), HeaderValue::from_str(&self.value())?);
        Ok(())
    }
}

impl<T> Header for Option<T>
where
    T: Header,
{
    fn name(&self) -> &'static str {
        self.as_ref()
            .map(|h| h.name())
            .expect("tried to get optional header when None")
    }

    fn value(&self) -> String {
        self.as_ref()
            .map(|h| h.value())
            .expect("tried to get optional header when None")
    }

    fn add_as_header(&self, builder: Builder) -> Builder {
        if let Some(h) = self {
            builder.header(h.name(), h.value())
        } else {
            builder
        }
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HttpHeaderError> {
        if let Some(h) = self {
            request
                .headers_mut()
                .insert(h.name(), HeaderValue::from_str(&h.value())?);
        }
        Ok(())
    }
}

#[must_use]
pub fn add_optional_header_ref<T: Header>(item: &Option<&T>, mut builder: Builder) -> Builder {
    if let Some(item) = item {
        builder = item.add_as_header(builder);
    }
    builder
}

#[must_use]
pub fn add_optional_header<T: Header>(item: &Option<T>, mut builder: Builder) -> Builder {
    if let Some(item) = item {
        builder = item.add_as_header(builder);
    }
    builder
}

pub fn add_optional_header2<T: Header>(
    item: &Option<T>,
    request: &mut crate::Request,
) -> Result<(), crate::errors::HttpHeaderError> {
    if let Some(item) = item {
        item.add_as_header2(request)?
    }
    Ok(())
}

#[must_use]
pub fn add_mandatory_header<T: Header>(item: &T, builder: Builder) -> Builder {
    item.add_as_header(builder)
}

pub fn add_mandatory_header2<T: Header>(
    item: &T,
    request: &mut crate::Request,
) -> Result<(), crate::errors::HttpHeaderError> {
    item.add_as_header2(request)
}

pub const ACCOUNT_KIND: &str = "x-ms-account-kind";
pub const ACTIVITY_ID: &str = "x-ms-activity-id";
pub const APPEND_POSITION: &str = "x-ms-blob-condition-appendpos";
pub const BLOB_ACCESS_TIER: &str = "x-ms-access-tier";
pub const BLOB_CONTENT_LENGTH: &str = "x-ms-blob-content-length";
pub const BLOB_PUBLIC_ACCESS: &str = "x-ms-blob-public-access";
pub const BLOB_SEQUENCE_NUMBER: &str = "x-ms-blob-sequence-number";
pub const BLOB_TYPE: &str = "x-ms-blob-type";
pub const CACHE_CONTROL: &str = "x-ms-blob-cache-control";
pub const CLIENT_REQUEST_ID: &str = "x-ms-client-request-id";
pub const CONTENT_DISPOSITION: &str = "x-ms-blob-content-disposition";
pub const CONTINUATION: &str = "x-ms-continuation";
pub const COPY_COMPLETION_TIME: &str = "x-ms-copy-completion-time";
pub const COPY_PROGRESS: &str = "x-ms-copy-progress";
pub const COPY_SOURCE: &str = "x-ms-copy-source";
pub const COPY_STATUS: &str = "x-ms-copy-status";
pub const COPY_STATUS_DESCRIPTION: &str = "x-ms-copy-status-description";
pub const CREATION_TIME: &str = "x-ms-creation-time";
pub const DELETE_SNAPSHOTS: &str = "x-ms-delete-snapshots";
pub const DELETE_TYPE_PERMANENT: &str = "x-ms-delete-type-permanent";
pub const HAS_IMMUTABILITY_POLICY: &str = "x-ms-has-immutability-policy";
pub const HAS_LEGAL_HOLD: &str = "x-ms-has-legal-hold";
pub const IF_SEQUENCE_NUMBER_EQ: &str = "x-ms-if-sequence-number-eq";
pub const IF_SEQUENCE_NUMBER_LE: &str = "x-ms-if-sequence-number-le";
pub const IF_SEQUENCE_NUMBER_LT: &str = "x-ms-if-sequence-number-lt";
pub const ITEM_COUNT: &str = "x-ms-item-count";
pub const ITEM_TYPE: &str = "x-ms-item-type";
pub const LEASE_ACTION: &str = "x-ms-lease-action";
pub const LEASE_BREAK_PERIOD: &str = "x-ms-lease-break-period";
pub const LEASE_DURATION: &str = "x-ms-lease-duration";
pub const LEASE_ID: &str = "x-ms-lease-id";
pub const LEASE_STATE: &str = "x-ms-lease-state";
pub const LEASE_STATUS: &str = "x-ms-lease-status";
pub const LEASE_TIME: &str = "x-ms-lease-time";
pub const MAX_ITEM_COUNT: &str = "x-ms-max-item-count";
pub const META_PREFIX: &str = "x-ms-meta-";
pub const MS_DATE: &str = "x-ms-date";
pub const NAMESPACE_ENABLED: &str = "x-ms-namespace-enabled";
pub const PAGE_WRITE: &str = "x-ms-page-write";
pub const PROPERTIES: &str = "x-ms-properties";
pub const PROPOSED_LEASE_ID: &str = "x-ms-proposed-lease-id";
pub const RANGE_GET_CONTENT_MD5: &str = "x-ms-range-get-content-md5";
pub const REQUEST_ID: &str = "x-ms-request-id";
pub const REQUEST_SERVER_ENCRYPTED: &str = "x-ms-request-server-encrypted";
pub const REQUIRES_SYNC: &str = "x-ms-requires-sync";
pub const SERVER_ENCRYPTED: &str = "x-ms-server-encrypted";
pub const SESSION_TOKEN: &str = "x-ms-session-token";
pub const SKU_NAME: &str = "x-ms-sku-name";
pub const SOURCE_IF_MATCH: &str = "x-ms-source-if-match";
pub const SOURCE_IF_MODIFIED_SINCE: &str = "x-ms-source-if-modified-since";
pub const SOURCE_IF_NONE_MATCH: &str = "x-ms-source-if-none-match";
pub const SOURCE_IF_UNMODIFIED_SINCE: &str = "x-ms-source-if-unmodified-since";
pub const SOURCE_LEASE_ID: &str = "x-ms-source-lease-id";
pub const VERSION: &str = "x-ms-version";
