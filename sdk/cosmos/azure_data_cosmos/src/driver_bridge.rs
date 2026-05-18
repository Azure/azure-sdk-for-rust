// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bridge between driver types and SDK types.
//!
//! This module provides conversion functions for translating between the driver's
//! operation/response types and the SDK's public-facing types. It is the shared
//! foundation for routing SDK operations through the driver.

use azure_core::{
    http::{headers::Headers, response::Response, RawResponse, StatusCode},
    Bytes,
};
use azure_data_cosmos_driver::models::{CosmosResponse as DriverResponse, CosmosResponseHeaders};

use crate::{
    constants::{
        ACTIVITY_ID, CONTINUATION, COSMOS_INTERNAL_PARTITION_ID, INDEX_METRICS, ITEM_COUNT,
        OFFER_REPLACE_PENDING, PARTITION_KEY_RANGE_ID, QUERY_METRICS, REQUEST_CHARGE,
        REQUEST_DURATION_MS, SESSION_TOKEN, SUB_STATUS,
    },
    models::CosmosResponse,
};

/// Converts a driver [`DriverResponse`] into the SDK's typed [`CosmosResponse<T>`].
///
/// This reconstructs an `azure_core::Response<T>` from the driver's raw bytes,
/// status code, and headers, then wraps it in the SDK's response type.
///
/// The driver's pre-parsed [`CosmosResponseHeaders`] are passed directly to
/// avoid double-parsing. Some headers (e.g., `index_metrics`) are base64-decoded
/// by the driver; re-parsing from raw headers would fail on already-decoded values.
/// The driver's [`DiagnosticsContext`](azure_data_cosmos_driver::diagnostics::DiagnosticsContext)
/// is plumbed through unchanged so that all SDK response wrappers expose the
/// rich per-operation diagnostics produced by the driver pipeline.
pub(crate) fn driver_response_to_cosmos_response<T>(
    driver_response: DriverResponse,
) -> CosmosResponse<T> {
    let status_code: StatusCode = driver_response.status().status_code();
    let cosmos_headers = driver_response.headers().clone();
    let diagnostics = driver_response.diagnostics();
    let headers = driver_response_headers_to_headers(&cosmos_headers);
    let body = driver_response.into_body();

    let raw_response = RawResponse::from_bytes(status_code, headers, Bytes::from(body));
    let typed_response: Response<T> = raw_response.into();

    CosmosResponse::from_driver_response(typed_response, cosmos_headers, diagnostics)
}

/// Converts driver [`CosmosResponseHeaders`] into raw [`Headers`] for the SDK response.
///
/// Only headers that were parsed by the driver are included. Any "extra" headers
/// from the server that the driver did not capture are lost.
fn driver_response_headers_to_headers(cosmos_headers: &CosmosResponseHeaders) -> Headers {
    let mut headers = Headers::new();

    if let Some(activity_id) = &cosmos_headers.activity_id {
        headers.insert(ACTIVITY_ID, activity_id.as_str().to_owned());
    }
    if let Some(charge) = &cosmos_headers.request_charge {
        headers.insert(REQUEST_CHARGE, charge.value().to_string());
    }
    if let Some(session_token) = &cosmos_headers.session_token {
        headers.insert(SESSION_TOKEN, session_token.as_str().to_owned());
    }
    if let Some(etag) = &cosmos_headers.etag {
        headers.insert(azure_core::http::headers::ETAG, etag.as_str().to_owned());
    }
    if let Some(continuation) = &cosmos_headers.continuation {
        headers.insert(CONTINUATION, continuation.clone());
    }
    if let Some(item_count) = cosmos_headers.item_count {
        headers.insert(ITEM_COUNT, item_count.to_string());
    }
    if let Some(substatus) = &cosmos_headers.substatus {
        headers.insert(SUB_STATUS, substatus.value().to_string());
    }
    if let Some(server_duration) = cosmos_headers.server_duration_ms {
        headers.insert(REQUEST_DURATION_MS, server_duration.to_string());
    }
    if let Some(index_metrics) = &cosmos_headers.index_metrics {
        headers.insert(INDEX_METRICS, index_metrics.clone());
    }
    if let Some(query_metrics) = &cosmos_headers.query_metrics {
        headers.insert(QUERY_METRICS, query_metrics.clone());
    }
    if let Some(pending) = cosmos_headers.offer_replace_pending {
        headers.insert(OFFER_REPLACE_PENDING, pending.to_string());
    }
    if let Some(pk_range_id) = &cosmos_headers.partition_key_range_id {
        headers.insert(PARTITION_KEY_RANGE_ID, pk_range_id.clone());
    }
    if let Some(internal_id) = &cosmos_headers.internal_partition_id {
        headers.insert(COSMOS_INTERNAL_PARTITION_ID, internal_id.clone());
    }

    headers
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos_driver::models::{
        ActivityId, CosmosResponseHeaders, ETag, RequestCharge, SessionToken as DriverSessionToken,
        SubStatusCode,
    };

    fn make_headers_all_some() -> CosmosResponseHeaders {
        let mut h = CosmosResponseHeaders::new();
        h.activity_id = Some(ActivityId::from_string("act-123".to_string()));
        h.request_charge = Some(RequestCharge::new(4.2));
        h.session_token = Some(DriverSessionToken::new("session-token".to_string()));
        h.etag = Some(ETag::new("\"etag-value\"".to_string()));
        h.continuation = Some("cont-token".to_string());
        h.item_count = Some(42);
        h.substatus = Some(SubStatusCode::new(0));
        h.offer_replace_pending = Some(true);
        h.partition_key_range_id = Some("5".to_string());
        h.internal_partition_id = Some("int-part-99".to_string());
        h
    }

    #[test]
    fn headers_all_some() {
        let headers = driver_response_headers_to_headers(&make_headers_all_some());

        assert_eq!(headers.get_optional_str(&ACTIVITY_ID), Some("act-123"));
        assert_eq!(headers.get_optional_str(&REQUEST_CHARGE), Some("4.2"));
        assert_eq!(
            headers.get_optional_str(&SESSION_TOKEN),
            Some("session-token")
        );
        assert_eq!(
            headers.get_optional_str(&azure_core::http::headers::ETAG),
            Some("\"etag-value\""),
        );
        assert_eq!(headers.get_optional_str(&CONTINUATION), Some("cont-token"));
        assert_eq!(headers.get_optional_str(&ITEM_COUNT), Some("42"));
        assert_eq!(headers.get_optional_str(&SUB_STATUS), Some("0"));
        assert_eq!(
            headers.get_optional_str(&OFFER_REPLACE_PENDING),
            Some("true")
        );
        assert_eq!(headers.get_optional_str(&PARTITION_KEY_RANGE_ID), Some("5"));
        assert_eq!(
            headers.get_optional_str(&COSMOS_INTERNAL_PARTITION_ID),
            Some("int-part-99")
        );
    }

    #[test]
    fn headers_all_none() {
        let headers = driver_response_headers_to_headers(&CosmosResponseHeaders::new());

        assert_eq!(headers.get_optional_str(&ACTIVITY_ID), None);
        assert_eq!(headers.get_optional_str(&REQUEST_CHARGE), None);
        assert_eq!(headers.get_optional_str(&SESSION_TOKEN), None);
        assert_eq!(
            headers.get_optional_str(&azure_core::http::headers::ETAG),
            None
        );
        assert_eq!(headers.get_optional_str(&CONTINUATION), None);
        assert_eq!(headers.get_optional_str(&ITEM_COUNT), None);
        assert_eq!(headers.get_optional_str(&SUB_STATUS), None);
        assert_eq!(headers.get_optional_str(&OFFER_REPLACE_PENDING), None);
        assert_eq!(headers.get_optional_str(&PARTITION_KEY_RANGE_ID), None);
        assert_eq!(
            headers.get_optional_str(&COSMOS_INTERNAL_PARTITION_ID),
            None
        );
    }

    /// Regression test: index_metrics (base64-decoded by the driver) must survive
    /// the driver→SDK bridge without double-decoding.
    ///
    /// Exercises `CosmosResponse::from_driver_response` which accepts pre-parsed
    /// headers, ensuring that already-decoded index_metrics are preserved rather
    /// than being base64-decoded a second time (which would silently return None).
    #[test]
    fn driver_response_preserves_index_metrics() {
        use crate::feed::{FeedBody, QueryFeedPage};
        use crate::models::CosmosResponse;
        use azure_data_cosmos_driver::diagnostics::DiagnosticsContext;
        use azure_data_cosmos_driver::models::ActivityId;
        use std::sync::Arc;

        let mut cosmos_headers = CosmosResponseHeaders::new();
        cosmos_headers.index_metrics = Some(r#"{"UtilizedSingleIndexes":[]}"#.to_string());
        cosmos_headers.query_metrics =
            Some("totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01".to_string());

        // Build a minimal raw response with synthesized headers (as the bridge does).
        let raw_headers = driver_response_headers_to_headers(&cosmos_headers);
        let raw_response = azure_core::http::RawResponse::from_bytes(
            StatusCode::Ok,
            raw_headers,
            Bytes::from_static(br#"{"Documents":[]}"#),
        );
        let typed_response: azure_core::http::response::Response<FeedBody<serde_json::Value>> =
            raw_response.into();

        // This is the code path used by driver_response_to_cosmos_response:
        // pre-parsed headers are passed directly, skipping re-parsing.
        let diagnostics = Arc::new(DiagnosticsContext::for_testing(ActivityId::new_uuid()));
        let cosmos_response =
            CosmosResponse::from_driver_response(typed_response, cosmos_headers, diagnostics);

        assert_eq!(
            cosmos_response.cosmos_headers().index_metrics.as_deref(),
            Some(r#"{"UtilizedSingleIndexes":[]}"#),
            "index_metrics should survive the driver bridge without double base64-decoding"
        );
        assert_eq!(
            cosmos_response.cosmos_headers().query_metrics.as_deref(),
            Some("totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01"),
        );

        let rt = tokio::runtime::Runtime::new().unwrap();
        let page = rt
            .block_on(QueryFeedPage::<serde_json::Value>::from_response(
                cosmos_response,
            ))
            .unwrap();
        assert_eq!(
            page.index_metrics(),
            Some(r#"{"UtilizedSingleIndexes":[]}"#)
        );
        assert_eq!(
            page.query_metrics(),
            Some("totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01")
        );
    }
}
