// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bare function: apply standard Cosmos DB headers to an outgoing HTTP request.
//!
//! This replaces `CosmosHeadersPolicy` from the old policy-chain pipeline.

use azure_core::http::headers::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};

use super::{cosmos_transport_client::CosmosHttpRequest, COSMOS_API_VERSION};

const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
const VERSION: HeaderName = HeaderName::from_static("x-ms-version");
const SDK_SUPPORTED_CAPABILITIES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-sdk-supportedcapabilities");
const SUPPORTED_CAPABILITIES_VALUE: &str = "0";
const CACHE_CONTROL: HeaderName = HeaderName::from_static("cache-control");
const NO_CACHE: HeaderValue = HeaderValue::from_static("no-cache");

/// Applies standard Cosmos DB headers to an outgoing HTTP request.
///
/// Sets `x-ms-version`, `x-ms-cosmos-sdk-supportedcapabilities`, `Content-Type`,
/// `Accept`, `Cache-Control`, and `User-Agent`.
pub(crate) fn apply_cosmos_headers(request: &mut CosmosHttpRequest, user_agent: &HeaderValue) {
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
