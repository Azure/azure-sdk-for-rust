// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared Key request signing for the `Session` authorization scheme.
//!
//! A session returns a symmetric session key (base64) that is used to sign
//! eligible requests with the Storage Shared Key protocol. This module builds
//! the canonical string-to-sign and computes the HMAC-SHA256 signature that is
//! placed in the `Authorization: Session {token}:{signature}` header.
//!
//! This is an internal signing primitive: it is keyed by the service-minted
//! session key, never by a customer-supplied account key, and it is only ever
//! invoked for the narrow set of GET blob download requests that are eligible
//! for session authentication.

use azure_core::{
    error::ErrorKind,
    http::{
        headers::{HeaderName, CONTENT_LENGTH, CONTENT_TYPE, IF_MATCH},
        Request,
    },
    Error, Result,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;

const CONTENT_ENCODING: HeaderName = HeaderName::from_static("content-encoding");
const CONTENT_LANGUAGE: HeaderName = HeaderName::from_static("content-language");
const CONTENT_MD5: HeaderName = HeaderName::from_static("content-md5");
const IF_MODIFIED_SINCE: HeaderName = HeaderName::from_static("if-modified-since");
const IF_NONE_MATCH: HeaderName = HeaderName::from_static("if-none-match");
const IF_UNMODIFIED_SINCE: HeaderName = HeaderName::from_static("if-unmodified-since");
const RANGE: HeaderName = HeaderName::from_static("range");

/// Prefix identifying the Storage custom headers included in the canonicalized headers.
const MS_HEADER_PREFIX: &str = "x-ms-";

/// Signs `request` with the Shared Key protocol using the base64 `session_key`
/// and returns the base64 signature for the `Session` authorization scheme.
///
/// The caller is responsible for having already set the `x-ms-date` and
/// `x-ms-version` headers, since they participate in the string-to-sign.
pub(crate) fn sign(request: &Request, account: &str, session_key: &str) -> Result<String> {
    let string_to_sign = string_to_sign(request, account);
    compute_signature(session_key, &string_to_sign)
}

/// Computes `base64(HMAC-SHA256(base64_decode(session_key), string_to_sign))`.
///
/// The session key is a base64-encoded symmetric key, mirroring how a Storage
/// account key is decoded before use as the HMAC key.
fn compute_signature(session_key: &str, string_to_sign: &str) -> Result<String> {
    let key = STANDARD.decode(session_key).map_err(|e| {
        Error::with_message(
            ErrorKind::DataConversion,
            format!("session key is not valid base64: {e}"),
        )
    })?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&key).map_err(|e| {
        Error::with_message(ErrorKind::Other, format!("invalid session key length: {e}"))
    })?;
    mac.update(string_to_sign.as_bytes());
    Ok(STANDARD.encode(mac.finalize().into_bytes()))
}

/// Builds the Storage Shared Key string-to-sign for `request`.
///
/// The `Date` field is intentionally empty because signing relies on the
/// `x-ms-date` header, which is carried in the canonicalized headers instead.
fn string_to_sign(request: &Request, account: &str) -> String {
    let headers = request.headers();
    let header = |name: &HeaderName| headers.get_optional_str(name).unwrap_or_default();

    // Content-Length is emitted as an empty string when zero or absent.
    let content_length = match headers.get_optional_str(&CONTENT_LENGTH) {
        Some("0") | None => "",
        Some(value) => value,
    };

    format!(
        "{verb}\n{content_encoding}\n{content_language}\n{content_length}\n{content_md5}\n{content_type}\n{date}\n{if_modified_since}\n{if_match}\n{if_none_match}\n{if_unmodified_since}\n{range}\n{canonicalized_headers}{canonicalized_resource}",
        verb = request.method().as_str(),
        content_encoding = header(&CONTENT_ENCODING),
        content_language = header(&CONTENT_LANGUAGE),
        content_length = content_length,
        content_md5 = header(&CONTENT_MD5),
        content_type = header(&CONTENT_TYPE),
        date = "",
        if_modified_since = header(&IF_MODIFIED_SINCE),
        if_match = header(&IF_MATCH),
        if_none_match = header(&IF_NONE_MATCH),
        if_unmodified_since = header(&IF_UNMODIFIED_SINCE),
        range = header(&RANGE),
        canonicalized_headers = canonicalized_headers(request),
        canonicalized_resource = canonicalized_resource(request, account),
    )
}

/// Builds the canonicalized headers: every `x-ms-*` header, lowercased, sorted
/// by name, each formatted as `name:value\n`.
fn canonicalized_headers(request: &Request) -> String {
    let mut entries: Vec<(String, String)> = request
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            let name = name.as_str().to_ascii_lowercase();
            name.starts_with(MS_HEADER_PREFIX)
                .then(|| (name, unfold(value.as_str())))
        })
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result = String::new();
    for (name, value) in entries {
        result.push_str(&name);
        result.push(':');
        result.push_str(&value);
        result.push('\n');
    }
    result
}

/// Builds the canonicalized resource: `/{account}{path}` followed by each query
/// parameter (lowercased name, decoded values sorted and comma-joined) as
/// `\nname:value`.
fn canonicalized_resource(request: &Request, account: &str) -> String {
    let url = request.url();

    let mut resource = String::with_capacity(account.len() + url.path().len() + 1);
    resource.push('/');
    resource.push_str(account);
    resource.push_str(url.path());

    let mut params: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, value) in url.query_pairs() {
        params
            .entry(name.to_ascii_lowercase())
            .or_default()
            .push(value.into_owned());
    }
    for (name, mut values) in params {
        values.sort();
        resource.push('\n');
        resource.push_str(&name);
        resource.push(':');
        resource.push_str(&values.join(","));
    }
    resource
}

/// Collapses folding whitespace in a header value to a single space and trims
/// the ends, matching the Shared Key canonicalization rules.
fn unfold(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{Method, Request, Url};

    fn request(url: &str, method: Method) -> Request {
        Request::new(Url::parse(url).unwrap(), method)
    }

    #[test]
    fn string_to_sign_get_blob() {
        let mut req = request(
            "https://myaccount.blob.core.windows.net/mycontainer/myblob",
            Method::Get,
        );
        req.insert_header("x-ms-date", "Fri, 01 Jan 2027 00:00:00 GMT");
        req.insert_header("x-ms-version", "2026-02-06");

        let expected = concat!(
            "GET\n\n\n\n\n\n\n\n\n\n\n\n",
            "x-ms-date:Fri, 01 Jan 2027 00:00:00 GMT\n",
            "x-ms-version:2026-02-06\n",
            "/myaccount/mycontainer/myblob",
        );
        assert_eq!(string_to_sign(&req, "myaccount"), expected);
    }

    #[test]
    fn string_to_sign_includes_range_and_content_headers() {
        let mut req = request(
            "https://myaccount.blob.core.windows.net/mycontainer/myblob",
            Method::Get,
        );
        req.insert_header("x-ms-date", "Fri, 01 Jan 2027 00:00:00 GMT");
        req.insert_header("range", "bytes=0-1023");

        let expected = concat!(
            "GET\n\n\n\n\n\n\n\n\n\n\n",
            "bytes=0-1023\n",
            "x-ms-date:Fri, 01 Jan 2027 00:00:00 GMT\n",
            "/myaccount/mycontainer/myblob",
        );
        assert_eq!(string_to_sign(&req, "myaccount"), expected);
    }

    #[test]
    fn canonicalized_headers_are_sorted_and_lowercased() {
        let mut req = request(
            "https://myaccount.blob.core.windows.net/mycontainer/myblob",
            Method::Get,
        );
        req.insert_header("x-ms-version", "2026-02-06");
        req.insert_header("x-ms-date", "Fri, 01 Jan 2027 00:00:00 GMT");
        req.insert_header("x-ms-client-request-id", "abc");
        // Non x-ms header must be excluded.
        req.insert_header("accept", "application/xml");

        let expected = concat!(
            "x-ms-client-request-id:abc\n",
            "x-ms-date:Fri, 01 Jan 2027 00:00:00 GMT\n",
            "x-ms-version:2026-02-06\n",
        );
        assert_eq!(canonicalized_headers(&req), expected);
    }

    #[test]
    fn canonicalized_resource_sorts_query_params() {
        let req = request(
            "https://myaccount.blob.core.windows.net/mycontainer/myblob?timeout=30&snapshot=2027-01-01",
            Method::Get,
        );
        let expected = "/myaccount/mycontainer/myblob\nsnapshot:2027-01-01\ntimeout:30";
        assert_eq!(canonicalized_resource(&req, "myaccount"), expected);
    }

    #[test]
    fn compute_signature_matches_known_answer() {
        // Independently computed with .NET HMAC-SHA256 over the same inputs.
        let signature = compute_signature("c2Vzc2lvbi1rZXk=", "test-string-to-sign").unwrap();
        assert_eq!(signature, "I4BGEfas+uyRVJfNiy9KsuLp3XnhEw7x1yOgv0y3RTM=");
    }

    #[test]
    fn compute_signature_rejects_invalid_base64_key() {
        assert!(compute_signature("not valid base64!", "message").is_err());
    }
}
