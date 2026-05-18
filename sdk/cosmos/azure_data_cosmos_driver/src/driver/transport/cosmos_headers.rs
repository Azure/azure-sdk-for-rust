// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bare function: apply standard Cosmos DB headers to an outgoing HTTP request.
//!
//! This replaces `CosmosHeadersPolicy` from the old policy-chain pipeline.

use azure_core::http::headers::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};

use super::{cosmos_transport_client::HttpRequest, COSMOS_API_VERSION};

const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
const VERSION: HeaderName = HeaderName::from_static("x-ms-version");
const SDK_SUPPORTED_CAPABILITIES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-sdk-supportedcapabilities");
const IGNORE_UNKNOWN_RNTBD_TOKENS_BIT: u32 = 8;
pub(crate) const SUPPORTED_CAPABILITIES_BITS: u32 = IGNORE_UNKNOWN_RNTBD_TOKENS_BIT;
const _: () = assert!(SUPPORTED_CAPABILITIES_BITS == 8);
/// String-encoded SDK capabilities bitmask.
///
/// Derived from `IgnoreUnknownRntbdTokens` (8), which advertises Gateway 2.0
/// forward compatibility with unknown RNTBD tokens.
const SUPPORTED_CAPABILITIES_VALUE: &str = "8";
const CACHE_CONTROL: HeaderName = HeaderName::from_static("cache-control");
const NO_CACHE: HeaderValue = HeaderValue::from_static("no-cache");

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
            IGNORE_UNKNOWN_RNTBD_TOKENS_BIT
        );
        assert_eq!(
            request
                .headers
                .get_optional_str(&SDK_SUPPORTED_CAPABILITIES),
            Some(SUPPORTED_CAPABILITIES_VALUE)
        );
    }
}
