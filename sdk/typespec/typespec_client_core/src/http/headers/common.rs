// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

// HTTP headers are case-insensitive.
// We use lowercase below for simple comparisons downstream.

pub const ACCEPT: HeaderName = HeaderName::from_static("accept");
pub const AUTHORIZATION: HeaderName = HeaderName::from_static("authorization");
pub const CONTENT_LENGTH: HeaderName = HeaderName::from_static("content-length");
pub const CONTENT_TYPE: HeaderName = HeaderName::from_static("content-type");
pub const ETAG: HeaderName = HeaderName::from_static("etag");
pub const IF_MATCH: HeaderName = HeaderName::from_static("if-match");
pub const LAST_MODIFIED: HeaderName = HeaderName::from_static("last-modified");
pub const PREFER: HeaderName = HeaderName::from_static("prefer");
pub const RETRY_AFTER: HeaderName = HeaderName::from_static("retry-after");
pub const USER_AGENT: HeaderName = HeaderName::from_static("user-agent");
