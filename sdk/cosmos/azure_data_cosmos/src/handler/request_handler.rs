// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ptr::null;
use crate::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
use crate::retry_policies::{RetryPolicy, RetryResult};
use async_trait::async_trait;
use serde::Serialize;
use azure_core::{
    async_runtime::get_async_runtime,
    http::{request::Request, RawResponse},
};
use typespec_client_core::http::{Method, Response};
use typespec_client_core::http::request::options::ContentType;
use crate::{ItemOptions, PartitionKey};
use crate::cosmos_request::{AuthorizationTokenType, CosmosRequest};
use crate::operation_context::OperationType;
use crate::pipeline::CosmosPipeline;
use crate::resource_context::{ResourceLink, ResourceType};

// Helper trait to conditionally require Send on non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
pub trait ConditionalSend: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> ConditionalSend for T {}

/// Concrete retry handler implementation with exponential back off.
/// This handler provides automatic retry capabilities for Cosmos DB operations using
/// a pluggable retry policy system. It wraps HTTP requests with intelligent retry logic
/// that handles both transient network errors and HTTP error responses.
#[derive(Debug, Clone)]
pub struct RequestHandler {

    pipeline: CosmosPipeline
}

impl RequestHandler {
    /// Creates a new `RequestHandler` wrapping the provided `CosmosPipeline`.
    ///
    /// This is a lightweight constructor; the handler itself is stateless aside
    /// from holding a cloneable reference to the underlying `CosmosPipeline`.
    ///
    /// # Arguments
    /// * `pipeline` - An initialized `CosmosPipeline` used to send requests.
    ///
    /// # Examples
    /// ```ignore
    /// let handler = RequestHandler::new(pipeline.clone());
    /// ```
    pub fn new(pipeline: CosmosPipeline) -> Self {
        Self { pipeline }
    }

    pub async fn send<T>(
        &self,
        partition_key: PartitionKey,
        body: Option<Vec<u8>>,
        operation_type: OperationType,
        resource_type: ResourceType,
        options: Option<ItemOptions<'_>>,
        resource_link: ResourceLink
    ) -> azure_core::Result<Response<T>> {

        let mut cosmos_request = CosmosRequest::new(operation_type, resource_type, Some("abv".parse()?), partition_key, body, false, AuthorizationTokenType::Primary, options);
        cosmos_request.request_context.location_endpoint_to_route = Option::from(resource_link.url(&self.pipeline.endpoint));
        // let mut request = cosmos_request.to_raw_request();
        let item_options = cosmos_request.clone().options.unwrap_or_default();

        self.pipeline
            .send_doc(item_options.method_options.context, &mut cosmos_request, resource_link.clone())
            .await
    }
}