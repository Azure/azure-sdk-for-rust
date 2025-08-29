// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP headers.

// Re-export headers to flatten out this module with typespec_client_core.
pub use typespec_client_core::http::headers::*;

// HTTP headers are case-insensitive.
// We use lowercase below for simple comparisons downstream.

pub const ACCOUNT_KIND: HeaderName = HeaderName::from_static("x-ms-account-kind");
pub const ACL: HeaderName = HeaderName::from_static("x-ms-acl");
pub const ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
pub const APP: HeaderName = HeaderName::from_static("x-ms-app");
pub const APPEND_POSITION: HeaderName = HeaderName::from_static("x-ms-blob-condition-appendpos"); // cspell:ignore appendpos
pub const AZURE_ASYNCOPERATION: HeaderName = HeaderName::from_static("azure-asyncoperation");
pub const BLOB_ACCESS_TIER: HeaderName = HeaderName::from_static("x-ms-access-tier");
pub const BLOB_CACHE_CONTROL: HeaderName = HeaderName::from_static("x-ms-blob-cache-control");
pub const BLOB_COMMITTED_BLOCK_COUNT: HeaderName =
    HeaderName::from_static("x-ms-blob-committed-block-count");
pub const BLOB_CONTENT_LENGTH: HeaderName = HeaderName::from_static("x-ms-blob-content-length");
pub const BLOB_PUBLIC_ACCESS: HeaderName = HeaderName::from_static("x-ms-blob-public-access");
pub const BLOB_SEQUENCE_NUMBER: HeaderName = HeaderName::from_static("x-ms-blob-sequence-number");
pub const BLOB_TYPE: HeaderName = HeaderName::from_static("x-ms-blob-type");
pub const CLIENT_REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-client-request-id");
pub const CLIENT_VERSION: HeaderName = HeaderName::from_static("x-ms-client-version");
pub const CONTENT_DISPOSITION: HeaderName =
    HeaderName::from_static("x-ms-blob-content-disposition");
pub const CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
pub const COPY_COMPLETION_TIME: HeaderName = HeaderName::from_static("x-ms-copy-completion-time");
pub const COPY_PROGRESS: HeaderName = HeaderName::from_static("x-ms-copy-progress");
pub const COPY_SOURCE: HeaderName = HeaderName::from_static("x-ms-copy-source");
pub const COPY_STATUS_DESCRIPTION: HeaderName =
    HeaderName::from_static("x-ms-copy-status-description");
pub const COPY_STATUS: HeaderName = HeaderName::from_static("x-ms-copy-status");
pub const CREATION_TIME: HeaderName = HeaderName::from_static("x-ms-creation-time");
pub const DELETE_SNAPSHOTS: HeaderName = HeaderName::from_static("x-ms-delete-snapshots");
pub const DELETE_TYPE_PERMANENT: HeaderName = HeaderName::from_static("x-ms-delete-type-permanent");
pub const ENCRYPTION_ALGORITHM: HeaderName = HeaderName::from_static("x-ms-encryption-algorithm");
pub const ENCRYPTION_KEY_SHA256: HeaderName = HeaderName::from_static("x-ms-encryption-key-sha256");
pub const ENCRYPTION_KEY: HeaderName = HeaderName::from_static("x-ms-encryption-key");
pub const HAS_IMMUTABILITY_POLICY: HeaderName =
    HeaderName::from_static("x-ms-has-immutability-policy");
pub const HAS_LEGAL_HOLD: HeaderName = HeaderName::from_static("x-ms-has-legal-hold");
pub const IF_SEQUENCE_NUMBER_EQ: HeaderName = HeaderName::from_static("x-ms-if-sequence-number-eq");
pub const IF_SEQUENCE_NUMBER_LE: HeaderName = HeaderName::from_static("x-ms-if-sequence-number-le");
pub const IF_SEQUENCE_NUMBER_LT: HeaderName = HeaderName::from_static("x-ms-if-sequence-number-lt");
pub const IF_TAGS: HeaderName = HeaderName::from_static("x-ms-if-tags");
pub const ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
pub const ITEM_TYPE: HeaderName = HeaderName::from_static("x-ms-item-type");
pub const LEASE_ACTION: HeaderName = HeaderName::from_static("x-ms-lease-action");
pub const LEASE_BREAK_PERIOD: HeaderName = HeaderName::from_static("x-ms-lease-break-period");
pub const LEASE_DURATION: HeaderName = HeaderName::from_static("x-ms-lease-duration");
pub const LEASE_ID: HeaderName = HeaderName::from_static("x-ms-lease-id");
pub const LEASE_STATE: HeaderName = HeaderName::from_static("x-ms-lease-state");
pub const LEASE_STATUS: HeaderName = HeaderName::from_static("x-ms-lease-status");
pub const LEASE_TIME: HeaderName = HeaderName::from_static("x-ms-lease-time");
pub const MAX_ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-max-item-count");
pub const META_PREFIX: HeaderName = HeaderName::from_static("x-ms-meta-");
pub const MS_DATE: HeaderName = HeaderName::from_static("x-ms-date");
pub const MS_RANGE: HeaderName = HeaderName::from_static("x-ms-range");
pub const NAMESPACE_ENABLED: HeaderName = HeaderName::from_static("x-ms-namespace-enabled");
pub const PAGE_WRITE: HeaderName = HeaderName::from_static("x-ms-page-write");
pub const PROPERTIES: HeaderName = HeaderName::from_static("x-ms-properties");
pub const PROPOSED_LEASE_ID: HeaderName = HeaderName::from_static("x-ms-proposed-lease-id");
pub const RANGE_GET_CONTENT_CRC64: HeaderName =
    HeaderName::from_static("x-ms-range-get-content-crc64");
pub const RANGE_GET_CONTENT_MD5: HeaderName = HeaderName::from_static("x-ms-range-get-content-md5");
pub const REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-request-id");
pub const REQUEST_SERVER_ENCRYPTED: HeaderName =
    HeaderName::from_static("x-ms-request-server-encrypted");
pub const REQUIRES_SYNC: HeaderName = HeaderName::from_static("x-ms-requires-sync");
pub const SERVER_ENCRYPTED: HeaderName = HeaderName::from_static("x-ms-server-encrypted");
pub const SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
pub const SKU_NAME: HeaderName = HeaderName::from_static("x-ms-sku-name");
pub const SOURCE_IF_MATCH: HeaderName = HeaderName::from_static("x-ms-source-if-match");
pub const SOURCE_IF_MODIFIED_SINCE: HeaderName =
    HeaderName::from_static("x-ms-source-if-modified-since");
pub const SOURCE_IF_NONE_MATCH: HeaderName = HeaderName::from_static("x-ms-source-if-none-match");
pub const SOURCE_IF_UNMODIFIED_SINCE: HeaderName =
    HeaderName::from_static("x-ms-source-if-unmodified-since");
pub const SOURCE_LEASE_ID: HeaderName = HeaderName::from_static("x-ms-source-lease-id");
pub const SOURCE_RANGE: HeaderName = HeaderName::from_static("x-ms-source-range");
pub const TAGS: HeaderName = HeaderName::from_static("x-ms-tags");
pub const USER: HeaderName = HeaderName::from_static("x-ms-user");
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

    pub const MULTIPART_FORM_DATA: HeaderValue = HeaderValue::from_static("multipart/form-data");
    pub const APPLICATION_X_WWW_FORM_URLENCODED: HeaderValue =
        HeaderValue::from_static("application/x-www-form-urlencoded");

    pub const APPLICATION_XML: HeaderValue = HeaderValue::from_static("application/xml");
    pub const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
    pub const APPLICATION_OCTET_STREAM: HeaderValue =
        HeaderValue::from_static("application/octet-stream");
    pub const TEXT_PLAIN: HeaderValue = HeaderValue::from_static("text/plain");
}

/// Constants related to query parameters
pub mod query_param {
    pub const API_VERSION: &str = "api-version";
}
