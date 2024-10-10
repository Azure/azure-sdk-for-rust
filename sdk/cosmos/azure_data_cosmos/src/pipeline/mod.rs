// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod authorization_policy;

use std::sync::Arc;

pub(crate) use authorization_policy::{AuthorizationPolicy, ResourceType};
use azure_core::{Context, Pager, Request};
use serde::de::DeserializeOwned;
use typespec_client_core::http::PagerResult;

use crate::{constants, models::QueryResults, Query};

/// Newtype that wraps an Azure Core pipeline to provide a Cosmos-specific pipeline which configures our authorization policy and enforces that a [`ResourceType`] is set on the context.
#[derive(Debug, Clone)]
pub struct CosmosPipeline(azure_core::Pipeline);

impl CosmosPipeline {
    pub fn new(
        auth_policy: AuthorizationPolicy,
        client_options: azure_core::ClientOptions,
    ) -> Self {
        CosmosPipeline(azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::new(),
            vec![Arc::new(auth_policy)],
        ))
    }

    pub async fn send<T>(
        &self,
        ctx: azure_core::Context<'_>,
        request: &mut azure_core::Request,
        resource_type: ResourceType,
    ) -> azure_core::Result<azure_core::Response<T>> {
        let ctx = ctx.with_value(resource_type);
        self.0.send(&ctx, request).await
    }

    pub fn send_query_request<T: DeserializeOwned>(
        &self,
        query: Query,
        mut base_request: Request,
        resource_type: ResourceType,
    ) -> azure_core::Result<Pager<QueryResults<T>>> {
        base_request.insert_header(constants::QUERY, "True");
        base_request.add_mandatory_header(&constants::QUERY_CONTENT_TYPE);
        base_request.set_json(&query)?;

        // We have to double-clone here.
        // First we clone the pipeline to pass it in to the closure
        let pipeline = self.0.clone();
        let context = Context::new().with_value(resource_type);
        Ok(Pager::from_callback(move |continuation| {
            // Then we have to clone it again to pass it in to the async block.
            // This is because Pageable can't borrow any data, it has to own it all.
            // That's probably good, because it means a Pageable can outlive the client that produced it, but it requires some extra cloning.
            let pipeline = pipeline.clone();
            let mut req = base_request.clone();
            let context = context.clone();
            async move {
                if let Some(continuation) = continuation {
                    req.insert_header(constants::CONTINUATION, continuation);
                }

                let resp = pipeline.send(&context, &mut req).await?;

                Ok(PagerResult::from_response_header(
                    resp,
                    &constants::CONTINUATION,
                ))
            }
        }))
    }
}
