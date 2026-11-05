// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! URL assembly helpers for [`SasUrl`](crate::SasUrl) output.
//!
//! Handles Azurite-style endpoints that have an existing path
//! (e.g. `http://127.0.0.1:10000/devstoreaccount1`) by appending to it
//! rather than replacing it. Each path segment is individually percent-encoded
//! by the `url` crate's `PathSegmentsMut`. The signed query string is set via
//! `set_query`, which preserves existing percent-encoding.

pub(crate) fn assemble(
    endpoint: &url::Url,
    path_segments: &[&str],
    query_string: &str,
) -> azure_core::Result<url::Url> {
    let mut url = endpoint.clone();
    url.path_segments_mut()
        .map_err(|_| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "SAS endpoint URL cannot be used as a base URL",
            )
        })?
        .pop_if_empty()
        .extend(path_segments);
    url.set_query(Some(query_string));
    Ok(url)
}

fn parse_endpoint(url: &str) -> azure_core::Result<url::Url> {
    url::Url::parse(url).map_err(|e| {
        azure_core::Error::with_message(azure_core::error::ErrorKind::Other, format!("{e}"))
    })
}

pub(crate) fn blob_endpoint(account: &str) -> azure_core::Result<url::Url> {
    parse_endpoint(&format!("https://{account}.blob.core.windows.net"))
}

pub(crate) fn queue_endpoint(account: &str) -> azure_core::Result<url::Url> {
    parse_endpoint(&format!("https://{account}.queue.core.windows.net"))
}

pub(crate) fn table_endpoint(account: &str) -> azure_core::Result<url::Url> {
    parse_endpoint(&format!("https://{account}.table.core.windows.net"))
}

pub(crate) fn file_endpoint(account: &str) -> azure_core::Result<url::Url> {
    parse_endpoint(&format!("https://{account}.file.core.windows.net"))
}
