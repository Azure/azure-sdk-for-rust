// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Executes requests and transport-level retries against a fixed endpoint.
//!
//! The operation pipeline owns routing, failover, and session recovery.

pub(crate) mod adaptive_transport;
mod authorization;
pub(crate) mod connectivity_probe;
pub(crate) mod cosmos_headers;
mod cosmos_transport;
pub(crate) mod cosmos_transport_client;
mod deadline;
mod diagnostics;
mod dispatch;
pub(crate) mod execution;
#[cfg(feature = "fault_injection")]
mod fault_injection;
mod gateway_v2;
pub(crate) mod http_client_factory;
pub(crate) mod recovery;
mod request_preparation;
pub(crate) mod request_signing;
#[cfg(feature = "reqwest")]
pub(crate) mod reqwest_transport_client;
mod response;
mod sent_status;
mod sharded_transport;
pub(crate) mod types;

pub(crate) use authorization::AuthorizationContext;
pub(crate) use cosmos_transport::{uses_dataplane_pipeline, CosmosTransport};
pub(crate) use gateway_v2::eligibility::is_operation_supported_by_gateway_v2;
#[cfg(feature = "fault_injection")]
pub(crate) use gateway_v2::unwrap_response_for_gateway_v2;
#[cfg(test)]
pub(crate) use gateway_v2::{wrap_request_for_gateway_v2, WrapInputs};
pub(crate) use sharded_transport::EndpointKey;
