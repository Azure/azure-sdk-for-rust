// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod authorization_policy;

use std::sync::Arc;

pub(crate) use authorization_policy::{AuthorizationPolicy, ResourceType};

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
}
