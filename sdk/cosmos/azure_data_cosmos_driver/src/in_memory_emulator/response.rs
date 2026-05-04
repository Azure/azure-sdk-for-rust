// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response construction and header generation.

use azure_core::http::headers::{HeaderValue, Headers};
use azure_core::http::{AsyncRawResponse, StatusCode};

use std::time::Instant;

// Header-name constants used by both the emulator's response builder and the
// integration tests that assert on emulator responses. Re-exported from a
// `#[doc(hidden)] pub` module so tests can use the same values without drifting.
#[doc(hidden)]
pub mod headers {
    use azure_core::http::headers::HeaderName;
    pub static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
    pub static REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");
    pub static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
    pub static ETAG: HeaderName = HeaderName::from_static("etag");
    pub static CONTENT_TYPE: HeaderName = HeaderName::from_static("content-type");
    pub static DATE: HeaderName = HeaderName::from_static("date");
    pub static VERSION: HeaderName = HeaderName::from_static("x-ms-version");
    pub static SUBSTATUS: HeaderName = HeaderName::from_static("x-ms-substatus");
    pub static ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
    pub static RETRY_AFTER: HeaderName = HeaderName::from_static("x-ms-retry-after-ms");
    pub static LSN: HeaderName = HeaderName::from_static("lsn");
    pub static SERVER_DURATION_MS: HeaderName = HeaderName::from_static("x-ms-request-duration-ms");
    #[allow(dead_code)]
    pub static CONTENT_PATH: HeaderName = HeaderName::from_static("x-ms-content-path");
    #[allow(dead_code)]
    pub static ALT_CONTENT_PATH: HeaderName = HeaderName::from_static("x-ms-alt-content-path");
}
use headers::{
    ACTIVITY_ID, ALT_CONTENT_PATH, CONTENT_PATH, CONTENT_TYPE, DATE, ETAG, ITEM_COUNT, LSN,
    REQUEST_CHARGE, RETRY_AFTER, SERVER_DURATION_MS, SESSION_TOKEN, SUBSTATUS, VERSION,
};

const COSMOS_VERSION: &str = "2020-07-15";

/// Builds an emulator HTTP response.
pub(crate) struct ResponseBuilder {
    status: StatusCode,
    headers: Headers,
    body: Vec<u8>,
    start: Instant,
}

impl ResponseBuilder {
    pub fn new(status: StatusCode, start: Instant) -> Self {
        let mut headers = Headers::new();
        headers.insert(
            CONTENT_TYPE.clone(),
            HeaderValue::from_static("application/json"),
        );
        headers.insert(VERSION.clone(), HeaderValue::from_static(COSMOS_VERSION));
        headers.insert(DATE.clone(), HeaderValue::from(format_rfc1123_date()));
        headers.insert(
            ACTIVITY_ID.clone(),
            HeaderValue::from(uuid::Uuid::new_v4().to_string()),
        );

        Self {
            status,
            headers,
            body: Vec::new(),
            start,
        }
    }

    pub fn with_request_charge(mut self, charge: f64) -> Self {
        self.headers.insert(
            REQUEST_CHARGE.clone(),
            HeaderValue::from(format!("{:.2}", charge)),
        );
        self
    }

    pub fn with_session_token(mut self, token: &str) -> Self {
        self.headers
            .insert(SESSION_TOKEN.clone(), HeaderValue::from(token.to_string()));
        self
    }

    pub fn with_etag(mut self, etag: &str) -> Self {
        self.headers
            .insert(ETAG.clone(), HeaderValue::from(etag.to_string()));
        self
    }

    pub fn with_substatus(mut self, code: u32) -> Self {
        self.headers
            .insert(SUBSTATUS.clone(), HeaderValue::from(code.to_string()));
        self
    }

    pub fn with_item_count(mut self, count: u32) -> Self {
        self.headers
            .insert(ITEM_COUNT.clone(), HeaderValue::from(count.to_string()));
        self
    }

    pub fn with_lsn(mut self, lsn: u64) -> Self {
        self.headers
            .insert(LSN.clone(), HeaderValue::from(lsn.to_string()));
        self
    }

    /// Adds the Retry-After header (in milliseconds) for throttling responses.
    pub fn with_retry_after_ms(mut self, ms: u64) -> Self {
        self.headers
            .insert(RETRY_AFTER.clone(), HeaderValue::from(ms.to_string()));
        self
    }

    #[allow(dead_code)]
    pub fn with_content_path(mut self, rid: &str) -> Self {
        self.headers
            .insert(CONTENT_PATH.clone(), HeaderValue::from(rid.to_string()));
        self
    }

    #[allow(dead_code)]
    pub fn with_alt_content_path(mut self, path: &str) -> Self {
        self.headers.insert(
            ALT_CONTENT_PATH.clone(),
            HeaderValue::from(path.to_string()),
        );
        self
    }

    pub fn with_json_body(mut self, body: &serde_json::Value) -> Self {
        self.body = serde_json::to_vec(body).unwrap_or_default();
        self
    }

    #[allow(dead_code)]
    pub fn with_raw_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn build(self) -> AsyncRawResponse {
        let mut headers = self.headers;
        let elapsed_ms = self.start.elapsed().as_secs_f64() * 1000.0;
        headers.insert(
            SERVER_DURATION_MS.clone(),
            HeaderValue::from(format!("{:.2}", elapsed_ms)),
        );
        AsyncRawResponse::from_bytes(self.status, headers, self.body)
    }
}

/// Creates a success response with a JSON body.
pub(crate) fn success_response(
    status: StatusCode,
    body: &serde_json::Value,
    charge: f64,
    session_token: &str,
    start: Instant,
) -> ResponseBuilder {
    ResponseBuilder::new(status, start)
        .with_request_charge(charge)
        .with_session_token(session_token)
        .with_json_body(body)
}

/// Creates an error response.
pub(crate) fn error_response(
    status: StatusCode,
    substatus: Option<u32>,
    code: &str,
    message: &str,
    charge: f64,
    session_token: &str,
    start: Instant,
) -> ResponseBuilder {
    let body = serde_json::json!({
        "code": code,
        "message": message
    });

    let mut builder = ResponseBuilder::new(status, start)
        .with_request_charge(charge)
        .with_session_token(session_token)
        .with_json_body(&body);

    if let Some(ss) = substatus {
        builder = builder.with_substatus(ss);
    }

    builder
}

/// Formats the current UTC time in RFC 1123 format (the form used by HTTP `Date`
/// headers, e.g. `"Sun, 06 Nov 1994 08:49:37 GMT"`).
///
/// Backed by the `time` crate to avoid hand-rolled Gregorian arithmetic.
fn format_rfc1123_date() -> String {
    use time::format_description::FormatItem;
    use time::macros::format_description;
    use time::OffsetDateTime;

    // RFC 1123 / RFC 7231 IMF-fixdate format.
    const FORMAT: &[FormatItem<'_>] = format_description!(
        "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
    );

    OffsetDateTime::now_utc()
        .format(FORMAT)
        .unwrap_or_else(|_| String::from("Thu, 01 Jan 1970 00:00:00 GMT"))
}
