// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response construction and header generation.

use azure_core::http::headers::{HeaderName, HeaderValue, Headers};
use azure_core::http::{AsyncRawResponse, StatusCode};

static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
static REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");
static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
static ETAG: HeaderName = HeaderName::from_static("etag");
static CONTENT_TYPE: HeaderName = HeaderName::from_static("content-type");
static DATE: HeaderName = HeaderName::from_static("date");
static VERSION: HeaderName = HeaderName::from_static("x-ms-version");
static SUBSTATUS: HeaderName = HeaderName::from_static("x-ms-substatus");
static ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
static RETRY_AFTER: HeaderName = HeaderName::from_static("x-ms-retry-after-ms");
#[allow(dead_code)]
static CONTENT_PATH: HeaderName = HeaderName::from_static("x-ms-content-path");
#[allow(dead_code)]
static ALT_CONTENT_PATH: HeaderName = HeaderName::from_static("x-ms-alt-content-path");

const COSMOS_VERSION: &str = "2020-07-15";

/// Builds an emulator HTTP response.
pub(crate) struct ResponseBuilder {
    status: StatusCode,
    headers: Headers,
    body: Vec<u8>,
}

impl ResponseBuilder {
    pub fn new(status: StatusCode) -> Self {
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
        AsyncRawResponse::from_bytes(self.status, self.headers, self.body)
    }
}

/// Creates a success response with a JSON body.
pub(crate) fn success_response(
    status: StatusCode,
    body: &serde_json::Value,
    charge: f64,
    session_token: &str,
) -> ResponseBuilder {
    ResponseBuilder::new(status)
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
) -> ResponseBuilder {
    let body = serde_json::json!({
        "code": code,
        "message": message
    });

    let mut builder = ResponseBuilder::new(status)
        .with_request_charge(charge)
        .with_session_token(session_token)
        .with_json_body(&body);

    if let Some(ss) = substatus {
        builder = builder.with_substatus(ss);
    }

    builder
}

/// Formats the current UTC time in RFC 1123 format.
fn format_rfc1123_date() -> String {
    use std::time::SystemTime;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Simple RFC 1123 approximation - sufficient for emulator
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    // Calculate date components from Unix timestamp
    let secs_per_day: u64 = 86400;
    let total_days = now / secs_per_day;
    let day_of_week = ((total_days + 4) % 7) as usize; // Jan 1 1970 was Thursday (4)

    let time_of_day = now % secs_per_day;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Calculate year/month/day from total days (simplified Gregorian)
    let mut y = 1970i64;
    let mut remaining = total_days as i64;
    loop {
        let days_in_year = if is_leap_year(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }

    let days_in_months: [i64; 12] = if is_leap_year(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut m = 0usize;
    for (i, &d) in days_in_months.iter().enumerate() {
        if remaining < d {
            m = i;
            break;
        }
        remaining -= d;
    }
    let day = remaining + 1;

    format!(
        "{}, {:02} {} {} {:02}:{:02}:{:02} GMT",
        days[day_of_week], day, months[m], y, hours, minutes, seconds
    )
}

fn is_leap_year(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
