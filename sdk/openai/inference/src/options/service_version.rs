// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use async_trait::async_trait;
use std::sync::Arc;

use azure_core::{Context, Policy, PolicyResult, Request};

/// The version of the Azure service to use.
/// This enum is passed to the [`AzureOpenAIClientOptionsBuilder`](crate::builders::AzureOpenAIClientOptionsBuilder) to configure an [`AzureOpenAIClient`](crate::clients::AzureOpenAIClient) to specify the version of the service to use.
///
/// If no version is specified, the latest version will be used. See [`AzureServiceVersion::get_latest()`](AzureServiceVersion::get_latest).
#[derive(Debug, Clone)]
pub enum AzureServiceVersion {
    V2023_09_01Preview,
    V2023_12_01Preview,
    V2024_07_01Preview,
}

impl Default for AzureServiceVersion {
    fn default() -> AzureServiceVersion {
        AzureServiceVersion::get_latest()
    }
}

impl AzureServiceVersion {
    /// Returns the latest supported version of the Azure OpenAI service.
    pub fn get_latest() -> AzureServiceVersion {
        AzureServiceVersion::V2024_07_01Preview
    }
}

impl From<AzureServiceVersion> for String {
    fn from(version: AzureServiceVersion) -> String {
        let as_str = match version {
            AzureServiceVersion::V2023_09_01Preview => "2023-09-01-preview",
            AzureServiceVersion::V2023_12_01Preview => "2023-12-01-preview",
            AzureServiceVersion::V2024_07_01Preview => "2024-07-01-preview",
        };
        return String::from(as_str);
    }
}

impl ToString for AzureServiceVersion {
    fn to_string(&self) -> String {
        String::from(self.clone())
    }
}

// code lifted from BearerTokenCredentialPolicy
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for AzureServiceVersion {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        request
            .url_mut()
            .query_pairs_mut()
            .append_pair("api-version", &self.to_string());
        next[0].send(ctx, request, &next[1..]).await
    }
}

impl Into<Arc<dyn Policy>> for AzureServiceVersion {
    fn into(self) -> Arc<dyn Policy> {
        Arc::new(self)
    }
}
