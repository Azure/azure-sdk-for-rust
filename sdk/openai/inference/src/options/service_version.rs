use std::sync::Arc;
use async_trait::async_trait;

use azure_core::{Context, Policy, PolicyResult, Request};

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

// Not entirely sure this is a good idea
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
        request.url_mut().query_pairs_mut().append_pair("api-version", &self.to_string());
        next[0].send(ctx, request, &next[1..]).await
    }
}

impl Into<Arc<dyn Policy>> for AzureServiceVersion {
    fn into(self) -> Arc<dyn Policy> {
        Arc::new(self)
    }
}
