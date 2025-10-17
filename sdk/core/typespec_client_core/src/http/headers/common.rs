// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

// HTTP headers are case-insensitive.
// We use lowercase below for simple comparisons downstream.

/// "accept" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-12.5.1>.
pub const ACCEPT: HeaderName = HeaderName::from_static_standard("accept");
/// "accept-charset" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-12.5.2>.
pub const ACCEPT_CHARSET: HeaderName = HeaderName::from_static_standard("accept-charset");
/// "authorization" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-11.6.2>.
pub const AUTHORIZATION: HeaderName = HeaderName::from_static_standard("authorization");
/// "content-length" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-8.6>.
pub const CONTENT_LENGTH: HeaderName = HeaderName::from_static_standard("content-length");
/// "content-type" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-8.3>
pub const CONTENT_TYPE: HeaderName = HeaderName::from_static_standard("content-type");
/// "etag" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-11.7>.
pub const ETAG: HeaderName = HeaderName::from_static_standard("etag");
/// "if-match" HTTP header. See <https://www.rfc-editor.org/rfc/rfc7232#section-3.1>.
pub const IF_MATCH: HeaderName = HeaderName::from_static_standard("if-match");
/// "last-modified" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-13.1.1>.
pub const LAST_MODIFIED: HeaderName = HeaderName::from_static_standard("last-modified");
/// "prefer" HTTP header. See <https://www.rfc-editor.org/rfc/rfc7240>.
pub const PREFER: HeaderName = HeaderName::from_static_standard("prefer");
/// "retry-after" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-10.2.3>.
pub const RETRY_AFTER: HeaderName = HeaderName::from_static_standard("retry-after");
/// "user-agent" HTTP header. See <https://www.rfc-editor.org/rfc/rfc9110#section-10.1.5>.
pub const USER_AGENT: HeaderName = HeaderName::from_static_standard("user-agent");
