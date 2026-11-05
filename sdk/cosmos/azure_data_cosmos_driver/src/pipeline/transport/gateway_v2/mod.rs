// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway V2 HTTP envelopes and their RNTBD adaptation.

use azure_core::{error::ErrorKind, http::headers::HeaderName};

use crate::{
    models::{
        cosmos_headers::{request_header_names, response_header_names},
        effective_partition_key::EffectivePartitionKey,
        DefaultConsistencyLevel, OperationType, ResourceType,
    },
    options::ReadConsistencyStrategy,
};

use super::AuthorizationContext;

const GLOBAL_DATABASE_ACCOUNT_NAME: HeaderName =
    HeaderName::from_static(request_header_names::GLOBAL_DATABASE_ACCOUNT_NAME);
const X_MS_DOCUMENTDB_COLLECTION_RID: HeaderName =
    HeaderName::from_static(request_header_names::COLLECTION_RID);
const X_MS_ACTIVITY_ID: HeaderName = HeaderName::from_static(request_header_names::ACTIVITY_ID);
const X_MS_DATE: HeaderName = HeaderName::from_static(request_header_names::DATE);
const X_MS_LSN: HeaderName = HeaderName::from_static(response_header_names::MS_LSN);
const X_MS_GLOBAL_COMMITTED_LSN: HeaderName =
    HeaderName::from_static(response_header_names::GLOBAL_COMMITTED_LSN);
const X_MS_CONTINUATION: HeaderName = HeaderName::from_static(request_header_names::CONTINUATION);
const X_MS_SESSION_TOKEN: HeaderName = HeaderName::from_static(request_header_names::SESSION_TOKEN);
const X_MS_MAX_ITEM_COUNT: HeaderName =
    HeaderName::from_static(request_header_names::MAX_ITEM_COUNT);
const IF_MATCH: HeaderName = HeaderName::from_static(request_header_names::IF_MATCH);
const IF_NONE_MATCH: HeaderName = HeaderName::from_static(request_header_names::IF_NONE_MATCH);
const IF_MODIFIED_SINCE: HeaderName =
    HeaderName::from_static(request_header_names::IF_MODIFIED_SINCE);
const A_IM: HeaderName = HeaderName::from_static(request_header_names::A_IM);
const CHANGE_FEED_WIRE_FORMAT_VERSION: HeaderName =
    HeaderName::from_static(request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION);
const X_MS_VERSION: HeaderName = HeaderName::from_static(request_header_names::VERSION);
const CACHE_CONTROL: HeaderName = HeaderName::from_static(request_header_names::CACHE_CONTROL);

// Gateway 2.0 (thin-client) outer header aliases, sharing the canonical wire
// strings in `models::cosmos_headers` rather than re-declaring the literals.
const GATEWAY_V2_OPERATION_TYPE: HeaderName =
    HeaderName::from_static(request_header_names::THINCLIENT_PROXY_OPERATION_TYPE);
const GATEWAY_V2_RESOURCE_TYPE: HeaderName =
    HeaderName::from_static(request_header_names::THINCLIENT_PROXY_RESOURCE_TYPE);
const GATEWAY_V2_RANGE_MIN: HeaderName =
    HeaderName::from_static(request_header_names::THINCLIENT_RANGE_MIN);
const GATEWAY_V2_RANGE_MAX: HeaderName =
    HeaderName::from_static(request_header_names::THINCLIENT_RANGE_MAX);
const GATEWAY_V2_DISCOVERY_OPT_IN: HeaderName =
    HeaderName::from_static(request_header_names::USE_THINCLIENT);

/// Inputs resolved by the operation pipeline before a Gateway 2.0 dispatch.
pub(crate) struct WrapInputs<'a> {
    pub(crate) auth_context: &'a AuthorizationContext,
    pub(crate) operation_type: OperationType,
    pub(crate) resource_type: ResourceType,
    /// Effective partition key precomputed in the operation pipeline (point or
    /// prefix EPK). `None` for cross-partition or partition-key-less requests.
    pub(crate) effective_partition_key: Option<&'a EffectivePartitionKey>,
    pub(crate) effective_consistency: DefaultConsistencyLevel,
    /// When non-`Default`, emit the `ReadConsistencyStrategy` RNTBD token (`0x00FE`)
    /// and SUPPRESS the `ConsistencyLevel` token. Callers must already have gated
    /// this to read operations.
    pub(crate) read_consistency_strategy: ReadConsistencyStrategy,
    pub(crate) account_name: Option<&'a str>,
    pub(crate) collection_rid: Option<&'a str>,
}

/// Creates the conversion error returned for invalid Gateway V2 request or response data.
fn data_conversion_error(message: impl Into<String>) -> azure_core::Error {
    azure_core::Error::with_message(ErrorKind::DataConversion, message.into())
}

pub(crate) mod eligibility;
mod request;
mod request_tokens;
mod response;
pub(crate) use request::wrap_request_for_gateway_v2;
pub(crate) use response::unwrap_response_for_gateway_v2;
#[cfg(test)]
mod tests;
