// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bridge between driver types and SDK types.
//!
//! This module provides conversion functions for translating between the driver's
//! operation/response types and the SDK's public-facing types. It is the shared
//! foundation for routing SDK operations through the driver.

use azure_data_cosmos_driver::models::CosmosResponse as DriverResponse;

use crate::models::CosmosResponse;

/// Converts a driver [`DriverResponse`] into the SDK's [`CosmosResponse`].
///
/// Thin passthrough over [`CosmosResponse::from_driver_response`]; kept as the
/// single import point so call sites stay short and the bridge module remains
/// the conventional place to look when chasing driver↔SDK conversions.
pub(crate) fn driver_response_to_cosmos_response(
    driver_response: DriverResponse,
) -> CosmosResponse {
    CosmosResponse::from_driver_response(driver_response)
}

#[cfg(test)]
mod tests {
    use azure_data_cosmos_driver::models::CosmosResponseHeaders;

    /// Regression test: index_metrics (base64-decoded by the driver) must survive
    /// the driver→SDK bridge without double-decoding.
    #[test]
    fn driver_response_preserves_index_metrics() {
        use crate::feed::QueryFeedPage;
        use crate::models::CosmosResponse;
        use azure_core::http::StatusCode;
        use azure_data_cosmos_driver::diagnostics::DiagnosticsContext;
        use azure_data_cosmos_driver::models::{ActivityId, CosmosStatus, ResponseBody};
        use std::sync::Arc;

        let mut cosmos_headers = CosmosResponseHeaders::new();
        cosmos_headers.index_metrics = Some(r#"{"UtilizedSingleIndexes":[]}"#.to_string());
        cosmos_headers.query_metrics =
            Some("totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01".to_string());

        let body = ResponseBody::Bytes(azure_core::Bytes::from_static(br#"{"Documents":[]}"#));
        let status = CosmosStatus::new(StatusCode::Ok);
        let diagnostics = Arc::new(DiagnosticsContext::for_testing(ActivityId::new_uuid()));
        let cosmos_response = CosmosResponse::from_driver_parts(
            body.into(),
            cosmos_headers.into(),
            status,
            diagnostics,
        );

        assert_eq!(
            cosmos_response.cosmos_headers().index_metrics(),
            Some(r#"{"UtilizedSingleIndexes":[]}"#),
            "index_metrics should survive the driver bridge without double base64-decoding"
        );
        assert_eq!(
            cosmos_response.cosmos_headers().query_metrics(),
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

    /// Regression test: a SDK fault-injection rule built with `with_custom_response`
    /// must surface that custom response on the translated driver-side rule.
    ///
    /// Without this, the driver's `FaultClient::apply_fault` falls through into
    /// `ApplyResult::NoEffect` (rule matches and bumps `hit_count`, but the
    /// request still goes to the real transport), which is exactly the failure
    /// mode that broke the SDK-level patch-412 fault-injection emulator tests:
    /// the diagnostic showed `Applied { rule_id: "sdk-patch-412-always" }` on a
    /// real 200 from the emulator, because the synthetic 412 never reached the
    /// `FaultInjectionResult` actually evaluated by the driver.
    #[cfg(feature = "fault_injection")]
    #[test]
    fn sdk_fi_rule_custom_response_survives_translation() {
        use super::sdk_fi_rules_to_driver_fi_rules;
        use crate::fault_injection::{
            CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
            FaultInjectionRuleBuilder, FaultOperationType,
        };
        use azure_core::http::headers::HeaderName;
        use azure_core::http::StatusCode;
        use std::sync::Arc;

        let custom_412 = CustomResponseBuilder::new(StatusCode::PreconditionFailed)
            .with_header(HeaderName::from_static("x-ms-injected-marker"), "patch-412")
            .with_body(br#"{"code":"PreconditionFailed","message":"injected 412"}"#.to_vec())
            .build();
        let result = FaultInjectionResultBuilder::new()
            .with_custom_response(custom_412)
            .build();
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::ReplaceItem)
            .build();
        let sdk_rule = Arc::new(
            FaultInjectionRuleBuilder::new("custom-response-bridge", result)
                .with_condition(condition)
                .build(),
        );

        let driver_rules = sdk_fi_rules_to_driver_fi_rules(std::slice::from_ref(&sdk_rule));
        assert_eq!(driver_rules.len(), 1);
        let driver_rule = &driver_rules[0];
        let driver_custom = driver_rule
            .result()
            .custom_response()
            .expect("custom_response must be translated onto the driver-side rule");
        assert_eq!(driver_custom.status_code(), StatusCode::PreconditionFailed);
        assert_eq!(
            driver_custom.body(),
            br#"{"code":"PreconditionFailed","message":"injected 412"}"#
        );
        assert_eq!(
            driver_custom
                .headers()
                .get_optional_str(&HeaderName::from_static("x-ms-injected-marker")),
            Some("patch-412"),
            "custom-response headers must also survive translation"
        );
    }
}
