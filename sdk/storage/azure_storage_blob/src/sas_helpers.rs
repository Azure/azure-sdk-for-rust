// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal helpers for SAS URL generation.

use azure_core::{
    error::{Error, ErrorKind},
    http::Url,
};
use percent_encoding::percent_decode_str;

/// Appends `query` to `endpoint`, preserving existing query parameters.
pub(crate) fn append_query(endpoint: &Url, query: &str) -> Url {
    let mut url = endpoint.clone();
    match url.query() {
        Some(existing) if !existing.is_empty() => {
            url.set_query(Some(&format!("{existing}&{query}")));
        }
        _ => {
            url.set_query(Some(query));
        }
    }
    url
}

/// Appends `query` to `endpoint`, preserving existing query parameters except
/// those whose key appears in `exclude`.
pub(crate) fn append_query_excluding(endpoint: &Url, query: &str, exclude: &[&str]) -> Url {
    let mut url = endpoint.clone();
    let kept: Vec<&str> = endpoint
        .query()
        .unwrap_or("")
        .split('&')
        .filter(|s| !s.is_empty())
        .filter(|s| {
            let raw_key = s.split('=').next().unwrap_or("");
            let decoded = percent_decode_str(raw_key).decode_utf8_lossy();
            !exclude.contains(&decoded.as_ref())
        })
        .collect();
    let merged = if kept.is_empty() {
        query.to_string()
    } else {
        format!("{}&{}", kept.join("&"), query)
    };
    url.set_query(Some(&merged));
    url
}

/// Extracts `snapshot` and `versionid` query parameters from a blob endpoint.
/// Returns an error if both are present (the service does not accept them together).
pub(crate) fn extract_blob_qualifiers(
    endpoint: &Url,
) -> azure_core::Result<(Option<String>, Option<String>)> {
    let mut snapshot = None;
    let mut version_id = None;
    for (k, v) in endpoint.query_pairs() {
        match k.as_ref() {
            "snapshot" => snapshot = Some(v.into_owned()),
            "versionid" => version_id = Some(v.into_owned()),
            _ => {}
        }
    }
    if snapshot.is_some() && version_id.is_some() {
        return Err(Error::with_message(
            ErrorKind::Other,
            "endpoint URL has both `snapshot=` and `versionid=` query parameters; specify at most one",
        ));
    }
    Ok((snapshot, version_id))
}
