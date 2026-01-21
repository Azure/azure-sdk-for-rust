// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Logging configuration for Azure Storage Blob clients.
//!
//! These defaults are automatically applied to all Storage Blob clients and are additive with any user-specified logging options.

use azure_core::http::ClientOptions;
use std::borrow::Cow;

/// Default allowed header names for Azure Storage Blob logging.
pub static STORAGE_ALLOWED_HEADERS: &[&str] = &[
    // CORS
    "access-control-allow-origin",
    // General Azure headers
    "x-ms-date",
    "x-ms-error-code",
    "x-ms-version",
    // Content headers
    "accept-ranges",
    "content-disposition",
    "content-encoding",
    "content-language",
    "content-md5",
    "content-range",
    "vary",
    // CRC
    "x-ms-content-crc64",
    // Copy operations
    "x-ms-copy-action",
    "x-ms-copy-completion-time",
    "x-ms-copy-id",
    "x-ms-copy-progress",
    "x-ms-copy-status",
    "x-ms-copy-destination-snapshot",
    "x-ms-copy-source-error-code",
    "x-ms-copy-source-status-code",
    // Lease headers
    "x-ms-lease-state",
    "x-ms-lease-status",
    "x-ms-lease-action",
    "x-ms-lease-break-period",
    "x-ms-lease-duration",
    "x-ms-lease-id",
    "x-ms-lease-time",
    // Legal/Policy
    "x-ms-has-immutability-policy",
    "x-ms-has-legal-hold",
    // Range headers
    "x-ms-range",
    "x-ms-source-range",
    // Encryption
    "x-ms-request-server-encrypted",
    "x-ms-server-encrypted",
    "x-ms-encryption-algorithm",
    "x-ms-encryption-key-sha256",
    "x-ms-default-encryption-scope",
    "x-ms-deny-encryption-scope-override",
    // Access tier
    "x-ms-access-tier",
    "x-ms-access-tier-change-time",
    "x-ms-access-tier-inferred",
    "x-ms-archive-status",
    "x-ms-rehydrate-priority",
    // Blob metadata
    "x-ms-blob-type",
    "x-ms-blob-sequence-number",
    "x-ms-blob-content-type",
    "x-ms-blob-content-md5",
    "x-ms-blob-content-encoding",
    "x-ms-blob-content-language",
    "x-ms-blob-content-disposition",
    "x-ms-blob-content-length",
    "x-ms-blob-cache-control",
    "x-ms-blob-public-access",
    // Append blob
    "x-ms-blob-append-offset",
    "x-ms-blob-committed-block-count",
    "x-ms-blob-condition-appendpos",
    "x-ms-blob-condition-maxsize",
    // Page blob
    "x-ms-page-write",
    "x-ms-if-sequence-number-eq",
    "x-ms-if-sequence-number-le",
    "x-ms-if-sequence-number-lt",
    "x-ms-sequence-number-action",
    // Miscellaneous
    "x-ms-account-kind",
    "x-ms-sku-name",
    "x-ms-snapshot",
    "x-ms-creation-time",
    "x-ms-delete-snapshots",
    "x-ms-delete-type-permanent",
    "x-ms-incremental-copy",
    "x-ms-proposed-lease-id",
    "x-ms-range-get-content-md5",
    "x-ms-tag-count",
    // Source conditionals
    "x-ms-source-if-match",
    "x-ms-source-if-modified-since",
    "x-ms-source-if-none-match",
    "x-ms-source-if-unmodified-since",
    "x-ms-source-content-md5",
];

/// Default allowed query parameters for Azure Storage Blob logging.
pub static STORAGE_ALLOWED_QUERY_PARAMETERS: &[&str] = &[
    // SAS token parameters (values are time-limited or non-sensitive identifiers)
    "se",
    "si",
    "sip",
    "sp",
    "spr",
    "sr",
    "srt",
    "ss",
    "st",
    "sv",
    "ske",
    "skoid",
    "sks",
    "skt",
    "sktid",
    "skv",
    // Operation parameters
    "comp",
    "restype",
    "copyid",
    // Listing parameters
    "maxresults",
    "include",
    "marker",
    "prefix",
    "delimiter",
    // Cache control parameters
    "rscc",
    "rscd",
    "rsce",
    "rscl",
    "rsct",
    // Block blob parameters
    "blockid",
    "blocklisttype",
    // Snapshot parameters
    "snapshot",
    "prevsnapshot",
];

/// Applies the default Azure Storage Blob logging configuration to client options.
///
/// This function prepends the storage-specific allowed headers and query parameters
/// to the user's existing logging options. User-specified options are preserved and
/// take effect in addition to the storage defaults.
pub(crate) fn apply_storage_logging_defaults(options: &mut ClientOptions) {
    // Prepend storage-specific headers to any user-specified headers
    let user_headers = std::mem::take(&mut options.logging.additional_allowed_header_names);
    options.logging.additional_allowed_header_names = STORAGE_ALLOWED_HEADERS
        .iter()
        .map(|s| Cow::Borrowed(*s))
        .chain(user_headers)
        .collect();

    // Prepend storage-specific query params to any user-specified query params
    let user_query_params = std::mem::take(&mut options.logging.additional_allowed_query_params);
    options.logging.additional_allowed_query_params = STORAGE_ALLOWED_QUERY_PARAMETERS
        .iter()
        .map(|s| Cow::Borrowed(*s))
        .chain(user_query_params)
        .collect();
}
