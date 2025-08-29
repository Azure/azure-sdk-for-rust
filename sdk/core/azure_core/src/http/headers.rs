// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP headers.

// Re-export headers to flatten out this module with typespec_client_core.
pub use typespec_client_core::http::headers::*;

// HTTP headers are case-insensitive.
// We use lowercase below for simple comparisons downstream.

pub const ACCOUNT_KIND: HeaderName = HeaderName::from_static("x-ms-account-kind");
pub(crate) const ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
pub(crate) const APP: HeaderName = HeaderName::from_static("x-ms-app");
pub const APPEND_POSITION: HeaderName = HeaderName::from_static("x-ms-blob-condition-appendpos"); // cspell:ignore appendpos
pub const AZURE_ASYNCOPERATION: HeaderName = HeaderName::from_static("azure-asyncoperation");
pub(crate) const BLOB_SEQUENCE_NUMBER: HeaderName =
    HeaderName::from_static("x-ms-blob-sequence-number");
pub const CLIENT_REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-client-request-id");
pub(crate) const CLIENT_VERSION: HeaderName = HeaderName::from_static("x-ms-client-version");
pub(crate) const CONTENT_DISPOSITION: HeaderName =
    HeaderName::from_static("x-ms-blob-content-disposition");
pub(crate) const CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
pub(crate) const IF_SEQUENCE_NUMBER_EQ: HeaderName =
    HeaderName::from_static("x-ms-if-sequence-number-eq");
pub(crate) const IF_SEQUENCE_NUMBER_LE: HeaderName =
    HeaderName::from_static("x-ms-if-sequence-number-le");
pub(crate) const IF_SEQUENCE_NUMBER_LT: HeaderName =
    HeaderName::from_static("x-ms-if-sequence-number-lt");
pub(crate) const IF_TAGS: HeaderName = HeaderName::from_static("x-ms-if-tags");
pub(crate) const LEASE_BREAK_PERIOD: HeaderName =
    HeaderName::from_static("x-ms-lease-break-period");
pub(crate) const LEASE_DURATION: HeaderName = HeaderName::from_static("x-ms-lease-duration");
pub(crate) const LEASE_ID: HeaderName = HeaderName::from_static("x-ms-lease-id");
pub(crate) const MAX_ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-max-item-count");
pub const MS_DATE: HeaderName = HeaderName::from_static("x-ms-date");
pub(crate) const MS_RANGE: HeaderName = HeaderName::from_static("x-ms-range");
pub(crate) const PROPOSED_LEASE_ID: HeaderName = HeaderName::from_static("x-ms-proposed-lease-id");
pub(crate) const RANGE_GET_CONTENT_CRC64: HeaderName =
    HeaderName::from_static("x-ms-range-get-content-crc64");
pub(crate) const REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-request-id");
pub(crate) const SOURCE_IF_MATCH: HeaderName = HeaderName::from_static("x-ms-source-if-match");
pub(crate) const SOURCE_IF_MODIFIED_SINCE: HeaderName =
    HeaderName::from_static("x-ms-source-if-modified-since");
pub(crate) const SOURCE_IF_NONE_MATCH: HeaderName =
    HeaderName::from_static("x-ms-source-if-none-match");
pub(crate) const SOURCE_IF_UNMODIFIED_SINCE: HeaderName =
    HeaderName::from_static("x-ms-source-if-unmodified-since");
pub(crate) const SOURCE_LEASE_ID: HeaderName = HeaderName::from_static("x-ms-source-lease-id");
pub(crate) const USER: HeaderName = HeaderName::from_static("x-ms-user");
pub const VERSION: HeaderName = HeaderName::from_static("x-ms-version");
pub const ERROR_CODE: HeaderName = HeaderName::from_static("x-ms-error-code");
pub const RETRY_AFTER_MS: HeaderName = HeaderName::from_static("retry-after-ms");
pub const X_MS_RETRY_AFTER_MS: HeaderName = HeaderName::from_static("x-ms-retry-after-ms");

/// Constants related to the Content-Type header
///
/// <https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type>
pub mod content_type {
    use crate::http::headers::HeaderValue;

    // Form content types
    // https://www.w3.org/TR/html401/interact/forms.html#h-17.13.4

    pub const APPLICATION_X_WWW_FORM_URLENCODED: HeaderValue =
        HeaderValue::from_static("application/x-www-form-urlencoded");
}

/// Constants related to query parameters
pub mod query_param {
    pub const API_VERSION: &str = "api-version";
}
