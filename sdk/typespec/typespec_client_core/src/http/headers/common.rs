// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(missing_docs)]

use super::*;

// HTTP headers are case-insensitive.
// We use lowercase below for simple comparisons downstream.

pub const ACCEPT: HeaderName = HeaderName::from_static_standard("accept");
pub const AUTHORIZATION: HeaderName = HeaderName::from_static_standard("authorization");
pub const CONTENT_LENGTH: HeaderName = HeaderName::from_static_standard("content-length");
pub const CONTENT_TYPE: HeaderName = HeaderName::from_static_standard("content-type");
pub const ETAG: HeaderName = HeaderName::from_static_standard("etag");
pub const IF_MATCH: HeaderName = HeaderName::from_static_standard("if-match");
pub const LAST_MODIFIED: HeaderName = HeaderName::from_static_standard("last-modified");
pub const PREFER: HeaderName = HeaderName::from_static_standard("prefer");
pub const RETRY_AFTER: HeaderName = HeaderName::from_static_standard("retry-after");
pub const USER_AGENT: HeaderName = HeaderName::from_static_standard("user-agent");
