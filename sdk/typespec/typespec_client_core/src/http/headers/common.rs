// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

// HTTP headers are case-insensitive.
// We use lowercase below for simple comparisons downstream.

pub const ACCEPT_ENCODING: HeaderName = HeaderName::from_static("accept-encoding");
pub const ACCEPT: HeaderName = HeaderName::from_static("accept");
pub const AUTHORIZATION: HeaderName = HeaderName::from_static("authorization");
pub const CACHE_CONTROL: HeaderName = HeaderName::from_static("cache-control");
pub const CONTENT_ENCODING: HeaderName = HeaderName::from_static("content-encoding");
pub const CONTENT_LANGUAGE: HeaderName = HeaderName::from_static("content-language");
pub const CONTENT_LENGTH: HeaderName = HeaderName::from_static("content-length");
pub const CONTENT_LOCATION: HeaderName = HeaderName::from_static("content-location");
pub const CONTENT_MD5: HeaderName = HeaderName::from_static("content-md5");
pub const CONTENT_RANGE: HeaderName = HeaderName::from_static("content-range");
pub const CONTENT_SECURITY_POLICY: HeaderName = HeaderName::from_static("content-security-policy");
pub const CONTENT_TYPE: HeaderName = HeaderName::from_static("content-type");
pub const DATE: HeaderName = HeaderName::from_static("date");
pub const ETAG: HeaderName = HeaderName::from_static("etag");
pub const IF_MATCH: HeaderName = HeaderName::from_static("if-match");
pub const IF_MODIFIED_SINCE: HeaderName = HeaderName::from_static("if-modified-since");
pub const IF_NONE_MATCH: HeaderName = HeaderName::from_static("if-none-match");
pub const IF_RANGE: HeaderName = HeaderName::from_static("if-range");
pub const IF_UNMODIFIED_SINCE: HeaderName = HeaderName::from_static("if-unmodified-since");
pub const KEEP_ALIVE: HeaderName = HeaderName::from_static("keep-alive");
pub const LAST_MODIFIED: HeaderName = HeaderName::from_static("last-modified");
pub const LINK: HeaderName = HeaderName::from_static("link");
pub const LOCATION: HeaderName = HeaderName::from_static("location");
pub const OPERATION_LOCATION: HeaderName = HeaderName::from_static("operation-location");
pub const PREFER: HeaderName = HeaderName::from_static("prefer");
pub const RANGE: HeaderName = HeaderName::from_static("range");
pub const RETRY_AFTER: HeaderName = HeaderName::from_static("retry-after");
pub const SERVER: HeaderName = HeaderName::from_static("server");
pub const USER_AGENT: HeaderName = HeaderName::from_static("user-agent");
pub const WWW_AUTHENTICATE: HeaderName = HeaderName::from_static("www-authenticate");
