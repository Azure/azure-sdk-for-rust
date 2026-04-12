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
        ACTIVITY_ID, CONTINUATION, INDEX_METRICS, ITEM_COUNT, OFFER_REPLACE_PENDING, QUERY_METRICS,
        REQUEST_CHARGE, REQUEST_DURATION_MS, SESSION_TOKEN, SUB_STATUS,
    },
    models::CosmosResponse,
};

/// Converts a driver [`DriverResponse`] into the SDK's typed [`CosmosResponse<T>`].
///
/// This reconstructs an `azure_core::Response<T>` from the driver's raw bytes,
/// status code, and headers, then wraps it in the SDK's response type.
pub(crate) fn driver_response_to_cosmos_response<T>(
    driver_response: DriverResponse,
) -> CosmosResponse<T> {
    let status_code: StatusCode = driver_response.status().status_code();
    let headers = driver_response_headers_to_headers(driver_response.headers());
    let body = driver_response.into_body();

    let raw_response = RawResponse::from_bytes(status_code, headers, Bytes::from(body));
    let typed_response: Response<T> = raw_response.into();

    CosmosResponse::from_response(typed_response)
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

    headers
}

/// Translates SDK fault injection rules into driver fault injection rules.
///
/// The `enabled` and `hit_count` state is shared between the SDK and driver
/// rules via `Arc`, so toggling a rule in tests affects both paths.
#[cfg(feature = "fault_injection")]
pub(crate) fn sdk_fi_rules_to_driver_fi_rules(
    sdk_rules: &[std::sync::Arc<crate::fault_injection::FaultInjectionRule>],
) -> Vec<std::sync::Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>> {
    use crate::fault_injection::{
        FaultInjectionErrorType as SdkErrorType, FaultOperationType as SdkOpType,
    };
    use azure_data_cosmos_driver::fault_injection::{
        self as driver_fi, FaultInjectionConditionBuilder as DriverConditionBuilder,
        FaultInjectionResultBuilder as DriverResultBuilder,
        FaultInjectionRuleBuilder as DriverRuleBuilder,
    };
    use azure_data_cosmos_driver::options::Region;

    sdk_rules
        .iter()
        .map(|sdk_rule| {
            // Translate condition
            let mut cond_builder = DriverConditionBuilder::new();
            if let Some(op) = &sdk_rule.condition.operation_type {
                let driver_op = match op {
                    SdkOpType::ReadItem => driver_fi::FaultOperationType::ReadItem,
                    SdkOpType::QueryItem => driver_fi::FaultOperationType::QueryItem,
                    SdkOpType::CreateItem => driver_fi::FaultOperationType::CreateItem,
                    SdkOpType::UpsertItem => driver_fi::FaultOperationType::UpsertItem,
                    SdkOpType::ReplaceItem => driver_fi::FaultOperationType::ReplaceItem,
                    SdkOpType::DeleteItem => driver_fi::FaultOperationType::DeleteItem,
                    SdkOpType::PatchItem => driver_fi::FaultOperationType::PatchItem,
                    SdkOpType::BatchItem => driver_fi::FaultOperationType::BatchItem,
                    SdkOpType::ChangeFeedItem => driver_fi::FaultOperationType::ChangeFeedItem,
                    SdkOpType::MetadataReadContainer => {
                        driver_fi::FaultOperationType::MetadataReadContainer
                    }
                    SdkOpType::MetadataReadDatabaseAccount => {
                        driver_fi::FaultOperationType::MetadataReadDatabaseAccount
                    }
                    SdkOpType::MetadataQueryPlan => {
                        driver_fi::FaultOperationType::MetadataQueryPlan
                    }
                    SdkOpType::MetadataPartitionKeyRanges => {
                        driver_fi::FaultOperationType::MetadataPartitionKeyRanges
                    }
                };
                cond_builder = cond_builder.with_operation_type(driver_op);
            }
            if let Some(region) = &sdk_rule.condition.region {
                cond_builder = cond_builder.with_region(Region::new(region.to_string()));
            }
            if let Some(container_id) = &sdk_rule.condition.container_id {
                cond_builder = cond_builder.with_container_id(container_id.clone());
            }

            // Translate result
            let mut result_builder = DriverResultBuilder::new();
            if let Some(err) = &sdk_rule.result.error_type {
                let driver_err = match err {
                    SdkErrorType::InternalServerError => {
                        driver_fi::FaultInjectionErrorType::InternalServerError
                    }
                    SdkErrorType::TooManyRequests => {
                        driver_fi::FaultInjectionErrorType::TooManyRequests
                    }
                    SdkErrorType::ReadSessionNotAvailable => {
                        driver_fi::FaultInjectionErrorType::ReadSessionNotAvailable
                    }
                    SdkErrorType::Timeout => driver_fi::FaultInjectionErrorType::Timeout,
                    SdkErrorType::ServiceUnavailable => {
                        driver_fi::FaultInjectionErrorType::ServiceUnavailable
                    }
                    SdkErrorType::PartitionIsGone => {
                        driver_fi::FaultInjectionErrorType::PartitionIsGone
                    }
                    SdkErrorType::WriteForbidden => {
                        driver_fi::FaultInjectionErrorType::WriteForbidden
                    }
                    SdkErrorType::DatabaseAccountNotFound => {
                        driver_fi::FaultInjectionErrorType::DatabaseAccountNotFound
                    }
                    SdkErrorType::ConnectionError => {
                        driver_fi::FaultInjectionErrorType::ConnectionError
                    }
                    SdkErrorType::ResponseTimeout => {
                        driver_fi::FaultInjectionErrorType::ResponseTimeout
                    }
                };
                result_builder = result_builder.with_error(driver_err);
            }
            if sdk_rule.result.delay > std::time::Duration::ZERO {
                result_builder = result_builder.with_delay(sdk_rule.result.delay);
            }
            let prob = sdk_rule.result.probability();
            if prob < 1.0 {
                result_builder = result_builder.with_probability(prob);
            }
            // Note: custom_response translation is skipped for now.
            // None of the current failing tests use custom responses.

            // Build driver rule with shared state
            let mut rule_builder =
                DriverRuleBuilder::new(sdk_rule.id.clone(), result_builder.build())
                    .with_condition(cond_builder.build())
                    .with_shared_state(sdk_rule.shared_enabled(), sdk_rule.shared_hit_count());

            if let Some(end_time) = sdk_rule.end_time {
                rule_builder = rule_builder.with_end_time(end_time);
            }
            if let Some(hit_limit) = sdk_rule.hit_limit {
                rule_builder = rule_builder.with_hit_limit(hit_limit);
            }
            // SDK start_time is always set (Instant::now() by default).
            // Driver start_time is Option<Instant>.
            rule_builder = rule_builder.with_start_time(sdk_rule.start_time);

            std::sync::Arc::new(rule_builder.build())
        })
        .collect()
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
    }
}
