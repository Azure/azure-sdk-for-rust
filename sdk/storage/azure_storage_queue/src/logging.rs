// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Logging configuration for Azure Storage Queue clients.
//!
//! These defaults are automatically applied to all Storage Queue clients and are additive with any user-specified logging options.

use azure_core::http::ClientOptions;
use std::borrow::Cow;

/// Default allowed header names for Azure Storage Queue logging.
pub static STORAGE_ALLOWED_HEADERS: &[&str] = &[
    // CORS
    "access-control-allow-origin",
    // General Azure headers
    "x-ms-date",
    "x-ms-error-code",
    "x-ms-version",
    // Content headers
    "content-encoding",
    "content-language",
    "vary",
    // Queue-specific response headers
    "x-ms-approximate-messages-count",
    "x-ms-popreceipt",
    "x-ms-time-next-visible",
    // Account and SKU info
    "x-ms-account-kind",
    "x-ms-sku-name",
];

/// Default allowed query parameters for Azure Storage Queue logging.
pub static STORAGE_ALLOWED_QUERY_PARAMETERS: &[&str] = &[
    // SAS token parameters (values are time-limited or non-sensitive identifiers)
    "se",
    "si",
    "sip",
    "sp",
    "spr",
    "ss",
    "srt",
    "st",
    "sv",
    // User delegation key parameters
    "ske",
    "skoid",
    "sks",
    "skt",
    "sktid",
    "skv",
    // Operation parameters
    "comp",
    "restype",
    // Listing parameters
    "maxresults",
    "include",
    "marker",
    "prefix",
    // Message operation parameters
    "numofmessages",
    "visibilitytimeout",
    "messagettl",
    "popreceipt",
    "peekonly",
];

/// Applies the default Azure Storage Queue logging configuration to client options.
///
/// This function adds the storage-specific allowed headers and query parameters
/// to the user's existing logging options. User-specified options are preserved and
/// take effect in addition to the storage defaults.
pub(crate) fn apply_storage_logging_defaults(options: &mut ClientOptions) {
    options
        .logging
        .additional_allowed_header_names
        .extend(STORAGE_ALLOWED_HEADERS.iter().map(|s| Cow::Borrowed(*s)));

    options.logging.additional_allowed_query_params.extend(
        STORAGE_ALLOWED_QUERY_PARAMETERS
            .iter()
            .map(|s| Cow::Borrowed(*s)),
    );
}
