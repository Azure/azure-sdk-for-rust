// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bare function: apply standard Cosmos DB headers to an outgoing HTTP request.
//!
//! This replaces `CosmosHeadersPolicy` from the old policy-chain pipeline.

use azure_core::http::headers::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};

use super::{cosmos_transport_client::HttpRequest, COSMOS_API_VERSION};
use crate::options::ReadConsistencyStrategy;

const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
const VERSION: HeaderName = HeaderName::from_static("x-ms-version");
const SDK_SUPPORTED_CAPABILITIES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-sdk-supportedcapabilities");
const IGNORE_UNKNOWN_RNTBD_TOKENS_BIT: u32 = 8;
pub(crate) const SUPPORTED_CAPABILITIES_BITS: u32 = IGNORE_UNKNOWN_RNTBD_TOKENS_BIT;
const _: () = assert!(SUPPORTED_CAPABILITIES_BITS == 8);
/// String-encoded SDK capabilities bitmask. Scoped to `IGNORE_UNKNOWN_RNTBD_TOKENS`,
/// which Gateway 2.0 needs to forward-compat unrecognized RNTBD tokens.
/// Additional bits (`PARTITION_MERGE`, `CHANGE_FEED_WITH_START_TIME_POST_MERGE`)
/// will be added in a follow-up once their handling is wired through.
const SUPPORTED_CAPABILITIES_VALUE: &str = "8";
const CACHE_CONTROL: HeaderName = HeaderName::from_static("cache-control");
const NO_CACHE: HeaderValue = HeaderValue::from_static("no-cache");
pub(crate) const CONSISTENCY_LEVEL: HeaderName = HeaderName::from_static("x-ms-consistency-level");
pub(crate) const READ_CONSISTENCY_STRATEGY: HeaderName =
    HeaderName::from_static("x-ms-cosmos-read-consistency-strategy");
/// Disables ComputeGateway's server-side 449 RetryWith retry on Gateway V1
/// requests so RetryWith is always handled client-side by the SDK, matching
/// Java's `RxGatewayStoreModel.applyGatewayRetryWithHeaders`. Not sent on the
/// thin-client (Gateway 2.0) path, where 449 is never produced server-side.
pub(crate) const NO_RETRY_449: HeaderName = HeaderName::from_static("x-ms-noretry-449");

/// Applies standard Cosmos DB headers to an outgoing HTTP request.
///
/// Sets `x-ms-version`, `x-ms-cosmos-sdk-supportedcapabilities`, `Content-Type`,
/// `Accept`, `Cache-Control`, and `User-Agent`.
pub(crate) fn apply_cosmos_headers(request: &mut HttpRequest, user_agent: &HeaderValue) {
    request
        .headers
        .insert(VERSION, HeaderValue::from_static(COSMOS_API_VERSION));
    request.headers.insert(
        SDK_SUPPORTED_CAPABILITIES,
        HeaderValue::from_static(SUPPORTED_CAPABILITIES_VALUE),
    );

    if request.headers.get_optional_str(&CONTENT_TYPE).is_none() {
        request
            .headers
            .insert(CONTENT_TYPE, APPLICATION_JSON.clone());
    }

    request.headers.insert(ACCEPT, APPLICATION_JSON.clone());
    request.headers.insert(CACHE_CONTROL, NO_CACHE.clone());
    request.headers.insert(USER_AGENT, user_agent.clone());
}

/// Apply the `ReadConsistencyStrategy` to an outgoing V1 HTTP request.
///
/// When `strategy` is non-`Default`:
///   - sets `x-ms-cosmos-read-consistency-strategy: <strategy>` (string form), and
///   - removes any `x-ms-consistency-level` header (RCS wins over
///     CL, including caller-injected custom headers).
///
/// When `strategy` is `Default` or `is_read = false`, this is a no-op so the
/// pre-RCS behavior of forwarding any custom `x-ms-consistency-level` header is
/// preserved verbatim. Callers MUST invoke this only on the V1 Gateway path
/// (GatewayV2 emits the equivalent metadata via the RNTBD
/// `ReadConsistencyStrategy` token in `wrap_request_for_gateway_v2`).
pub(crate) fn apply_read_consistency_strategy(
    request: &mut HttpRequest,
    strategy: ReadConsistencyStrategy,
    is_read: bool,
) {
    if !is_read || !strategy.is_non_default() {
        return;
    }
    request.headers.insert(
        READ_CONSISTENCY_STRATEGY,
        HeaderValue::from(strategy.as_str()),
    );
    request.headers.remove(CONSISTENCY_LEVEL);
}

/// Tags a request with `x-ms-fault-injection-operation` so
/// `FaultInjectingHttpClient` can match operation-typed fault rules against it.
///
/// Single source of truth for header name + value formatting; called from both
/// the data-plane operation pipeline and off-pipeline bootstrap fetches.
#[cfg(feature = "fault_injection")]
pub(crate) fn apply_fault_injection_operation_tag(
    headers: &mut azure_core::http::headers::Headers,
    operation_type: crate::fault_injection::FaultOperationType,
) {
    use crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_OPERATION;
    headers.insert(FAULT_INJECTION_OPERATION, operation_type.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{headers::Headers, Method};
    use url::Url;

    #[test]
    fn applies_supported_capabilities_bitmask() {
        let mut request = HttpRequest {
            url: Url::parse("https://example.documents.azure.com/").unwrap(),
            method: Method::Get,
            headers: Headers::new(),
            body: None,
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        };
        let user_agent = HeaderValue::from_static("test-agent");

        apply_cosmos_headers(&mut request, &user_agent);

        assert_eq!(
            SUPPORTED_CAPABILITIES_VALUE.parse::<u32>().unwrap(),
            SUPPORTED_CAPABILITIES_BITS
        );
        assert_eq!(
            request
                .headers
                .get_optional_str(&SDK_SUPPORTED_CAPABILITIES),
            Some(SUPPORTED_CAPABILITIES_VALUE)
        );
    }

    fn make_request_with_cl_header(cl: Option<&'static str>) -> HttpRequest {
        let mut headers = Headers::new();
        if let Some(cl) = cl {
            headers.insert(CONSISTENCY_LEVEL, HeaderValue::from_static(cl));
        }
        HttpRequest {
            url: Url::parse("https://example.documents.azure.com/").unwrap(),
            method: Method::Get,
            headers,
            body: None,
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        }
    }

    #[test]
    fn rcs_default_is_transparent_on_v1_path() {
        // Rule 3: Default RCS leaves any caller-injected CL header alone.
        let mut request = make_request_with_cl_header(Some("Session"));
        apply_read_consistency_strategy(&mut request, ReadConsistencyStrategy::Default, true);
        assert_eq!(
            request.headers.get_optional_str(&CONSISTENCY_LEVEL),
            Some("Session"),
            "Default RCS must not strip caller-injected CL header"
        );
        assert!(request
            .headers
            .get_optional_str(&READ_CONSISTENCY_STRATEGY)
            .is_none());
    }

    #[test]
    fn rcs_non_default_on_read_emits_header_and_strips_cl() {
        // Rule 1: RCS wins over CL — including caller-injected ones.
        for (strategy, expected) in [
            (ReadConsistencyStrategy::Eventual, "Eventual"),
            (ReadConsistencyStrategy::Session, "Session"),
            (ReadConsistencyStrategy::LatestCommitted, "LatestCommitted"),
            (ReadConsistencyStrategy::GlobalStrong, "GlobalStrong"),
        ] {
            let mut request = make_request_with_cl_header(Some("Strong"));
            apply_read_consistency_strategy(&mut request, strategy, true);
            assert_eq!(
                request.headers.get_optional_str(&READ_CONSISTENCY_STRATEGY),
                Some(expected),
                "{strategy:?} should emit `{expected}`"
            );
            assert!(
                request
                    .headers
                    .get_optional_str(&CONSISTENCY_LEVEL)
                    .is_none(),
                "{strategy:?} must strip caller-injected CL header"
            );
        }
    }

    #[test]
    fn rcs_non_default_on_write_is_noop() {
        // Reads-only: RCS must not leak onto write operations.
        let mut request = make_request_with_cl_header(Some("Session"));
        apply_read_consistency_strategy(
            &mut request,
            ReadConsistencyStrategy::LatestCommitted,
            false,
        );
        assert!(request
            .headers
            .get_optional_str(&READ_CONSISTENCY_STRATEGY)
            .is_none());
        assert_eq!(
            request.headers.get_optional_str(&CONSISTENCY_LEVEL),
            Some("Session"),
            "non-read operation must leave CL header untouched"
        );
    }
}
