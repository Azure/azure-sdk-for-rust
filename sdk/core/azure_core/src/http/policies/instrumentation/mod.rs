// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Instrumentation pipeline policies.

mod public_api_instrumentation;
mod request_instrumentation;

// Distributed tracing span attribute names. Defined in
// [OpenTelemetrySpans](https://github.com/open-telemetry/semantic-conventions/blob/main/docs/http/http-spans.md)
// and [Azure conventions for open telemetry spans](https://github.com/Azure/azure-sdk/blob/main/docs/tracing/distributed-tracing-conventions.md)
const AZ_NAMESPACE_ATTRIBUTE: &str = "az.namespace";
const AZ_CLIENT_REQUEST_ID_ATTRIBUTE: &str = "az.client_request_id";
pub(crate) const ERROR_TYPE_ATTRIBUTE: &str = "error.type";
const AZ_SERVICE_REQUEST_ID_ATTRIBUTE: &str = "az.service_request.id";
const HTTP_REQUEST_RESEND_COUNT_ATTRIBUTE: &str = "http.request.resend_count";
const HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE: &str = "http.response.status_code";
const HTTP_REQUEST_METHOD_ATTRIBUTE: &str = "http.request.method";
const SERVER_ADDRESS_ATTRIBUTE: &str = "server.address";
const SERVER_PORT_ATTRIBUTE: &str = "server.port";
const URL_FULL_ATTRIBUTE: &str = "url.full";

pub(crate) use public_api_instrumentation::PublicApiInstrumentationPolicy;
pub use public_api_instrumentation::{create_public_api_span, PublicApiInstrumentationInformation};
pub(crate) use request_instrumentation::*;
