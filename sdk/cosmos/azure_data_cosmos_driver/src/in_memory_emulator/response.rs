// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response construction and header generation.

use azure_core::http::headers::{HeaderName, HeaderValue, Headers};
use azure_core::http::{AsyncRawResponse, StatusCode};

use std::time::Instant;

// Header-name constants used by both the emulator's response builder and the
// integration tests that assert on emulator responses. Re-exported from a
// `#[doc(hidden)] pub` module so tests can use the same values without drifting.
// cspell:ignore activityid acked llsn gatewayversion serviceversion fixdate
#[doc(hidden)]
pub mod headers {
    use azure_core::http::headers::HeaderName;
    pub static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
    pub static REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");
    pub static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
    pub static ETAG: HeaderName = HeaderName::from_static("etag");
    pub static CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
    pub static CONTENT_TYPE: HeaderName = HeaderName::from_static("content-type");
    pub static DATE: HeaderName = HeaderName::from_static("date");
    pub static VERSION: HeaderName = HeaderName::from_static("x-ms-version");
    pub static SUBSTATUS: HeaderName = HeaderName::from_static("x-ms-substatus");
    pub static ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
    pub static RETRY_AFTER: HeaderName = HeaderName::from_static("x-ms-retry-after-ms");
    pub static LSN: HeaderName = HeaderName::from_static("lsn");
    pub static ITEM_LSN: HeaderName = HeaderName::from_static("x-ms-item-lsn");
    pub static SERVER_DURATION_MS: HeaderName = HeaderName::from_static("x-ms-request-duration-ms");
    pub static TRANSPORT_REQUEST_ID: HeaderName =
        HeaderName::from_static("x-ms-transport-request-id");
    pub static GLOBAL_COMMITTED_LSN: HeaderName =
        HeaderName::from_static("x-ms-global-committed-lsn");
    pub static QUORUM_ACKED_LSN: HeaderName = HeaderName::from_static("x-ms-quorum-acked-lsn");
    pub static QUORUM_ACKED_LOCAL_LSN: HeaderName =
        HeaderName::from_static("x-ms-cosmos-quorum-acked-llsn");
    pub static LOCAL_LSN: HeaderName = HeaderName::from_static("x-ms-cosmos-llsn");
    pub static ITEM_LOCAL_LSN: HeaderName = HeaderName::from_static("x-ms-cosmos-item-llsn");
    pub static NUMBER_OF_READ_REGIONS: HeaderName =
        HeaderName::from_static("x-ms-number-of-read-regions");
    pub static LAST_STATE_CHANGE_UTC: HeaderName =
        HeaderName::from_static("x-ms-last-state-change-utc");
    pub static GATEWAY_VERSION: HeaderName = HeaderName::from_static("x-ms-gatewayversion");
    pub static SERVICE_VERSION: HeaderName = HeaderName::from_static("x-ms-serviceversion");
    pub static RESOURCE_QUOTA: HeaderName = HeaderName::from_static("x-ms-resource-quota");
    pub static RESOURCE_USAGE: HeaderName = HeaderName::from_static("x-ms-resource-usage");
    pub static PARTITION_KEY_RANGE_ID: HeaderName =
        HeaderName::from_static("x-ms-documentdb-partitionkeyrangeid");
    pub static INTERNAL_PARTITION_ID: HeaderName =
        HeaderName::from_static("x-ms-cosmos-internal-partition-id");
}
use headers::{
    ACTIVITY_ID, CONTENT_TYPE, DATE, ETAG, GATEWAY_VERSION, GLOBAL_COMMITTED_LSN, ITEM_COUNT,
    LAST_STATE_CHANGE_UTC, LOCAL_LSN, LSN, NUMBER_OF_READ_REGIONS, QUORUM_ACKED_LOCAL_LSN,
    QUORUM_ACKED_LSN, REQUEST_CHARGE, RESOURCE_QUOTA, RESOURCE_USAGE, RETRY_AFTER,
    SERVER_DURATION_MS, SERVICE_VERSION, SESSION_TOKEN, SUBSTATUS, VERSION,
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
        // Real Cosmos DB emits these headers on *every* response, including
        // control-plane and error responses. We pre-populate them here so
        // dual-backend response comparisons work without per-test
        // allowlists. For point operations the handler overwrites the
        // partition-derived values via `decorate_point_response` so the
        // emitted values reflect actual partition state instead of the
        // synthetic defaults used for control-plane responses.
        //
        // `partition_key_range_id` and `internal_partition_id` are
        // intentionally NOT pre-seeded with garbage values (the prior
        // implementation set a random UUID and `"0"` respectively, which
        // surfaced meaningless values on control-plane responses and could
        // mask correlation bugs). Handlers that target a physical partition
        // attach the real values via `decorate_point_response`.
        //
        // `x-ms-transport-request-id` is intentionally NOT pre-seeded here
        // either. Point-op responses set it explicitly via
        // `decorate_point_response` from the store's monotonic counter, and
        // any response that reaches the dispatch post-processor without one
        // gets stamped there from the same counter. Pre-seeding with a
        // random UUID-derived `u32` was masking the case where a handler
        // forgot to thread the counter through, and produced
        // non-monotonic values that broke retry-correlation assertions.
        headers.insert(GLOBAL_COMMITTED_LSN.clone(), HeaderValue::from_static("0"));
        headers.insert(QUORUM_ACKED_LSN.clone(), HeaderValue::from_static("0"));
        headers.insert(
            QUORUM_ACKED_LOCAL_LSN.clone(),
            HeaderValue::from_static("0"),
        );
        headers.insert(LOCAL_LSN.clone(), HeaderValue::from_static("0"));
        headers.insert(
            NUMBER_OF_READ_REGIONS.clone(),
            HeaderValue::from_static("0"),
        );
        headers.insert(
            LAST_STATE_CHANGE_UTC.clone(),
            HeaderValue::from_static("Thu, 01 Jan 1970 00:00:00 GMT"),
        );
        headers.insert(
            GATEWAY_VERSION.clone(),
            HeaderValue::from_static("version=emulator"),
        );
        headers.insert(
            SERVICE_VERSION.clone(),
            HeaderValue::from_static("version=emulator"),
        );
        headers.insert(
            RESOURCE_QUOTA.clone(),
            HeaderValue::from_static(
                "documentSize=10240;documentsSize=10485760;documentsCount=-1;collectionSize=10485760;",
            ),
        );
        headers.insert(
            RESOURCE_USAGE.clone(),
            HeaderValue::from_static(
                "documentSize=0;documentsSize=0;documentsCount=0;collectionSize=0;",
            ),
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

    /// Sets the `x-ms-session-token` response header.
    ///
    /// An empty string is a no-op rather than emitting `x-ms-session-token: `,
    /// because real Cosmos DB only sends a session token on operations that
    /// actually progress the session vector. Emitting an empty header makes
    /// drivers see a no-progress token from a control-plane response and
    /// retry incorrectly on session-not-available errors.
    pub fn with_session_token(mut self, token: &str) -> Self {
        if token.is_empty() {
            self.headers.remove(SESSION_TOKEN.clone());
            return self;
        }
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

    pub fn with_header_value(mut self, name: HeaderName, value: impl ToString) -> Self {
        self.headers
            .insert(name, HeaderValue::from(value.to_string()));
        self
    }

    /// Adds the Retry-After header (in milliseconds) for throttling responses.
    pub fn with_retry_after_ms(mut self, ms: u64) -> Self {
        self.headers
            .insert(RETRY_AFTER.clone(), HeaderValue::from(ms.to_string()));
        self
    }

    pub fn with_json_body(mut self, body: &serde_json::Value) -> Self {
        self.body = serde_json::to_vec(body).unwrap_or_default();
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

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::StatusCode;
    use std::time::Instant;

    #[test]
    fn with_session_token_skips_empty_token() {
        // An empty session token must NOT produce an `x-ms-session-token: `
        // header. Real Cosmos only emits the header on operations that
        // advance the session vector; emitting an empty header makes
        // drivers treat control-plane responses as no-progress tokens and
        // mis-handle session-not-available retries.
        let builder = ResponseBuilder::new(StatusCode::Ok, Instant::now()).with_session_token("");
        let resp = builder.build();
        assert!(
            resp.headers().get_optional_str(&SESSION_TOKEN).is_none(),
            "empty session token must not emit the header"
        );
    }

    #[test]
    fn with_session_token_emits_non_empty_token() {
        let builder =
            ResponseBuilder::new(StatusCode::Ok, Instant::now()).with_session_token("0:1#5#0=5");
        let resp = builder.build();
        assert_eq!(
            resp.headers().get_optional_str(&SESSION_TOKEN),
            Some("0:1#5#0=5")
        );
    }

    #[test]
    fn with_session_token_clears_previously_set_value_when_called_with_empty() {
        let builder = ResponseBuilder::new(StatusCode::Ok, Instant::now())
            .with_session_token("0:1#5#0=5")
            .with_session_token("");
        let resp = builder.build();
        assert!(resp.headers().get_optional_str(&SESSION_TOKEN).is_none());
    }
}
