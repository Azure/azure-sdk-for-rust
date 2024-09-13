use std::sync::Arc;

use crate::auth::AzureKeyCredential;

use crate::options::AzureOpenAIClientOptions;
use azure_core::Url;
use azure_core::{self, Policy, Result};

use super::chat_completions_client::ChatCompletionsClient;
use super::BaseOpenAIClientMethods;

pub trait AzureOpenAIClientMethods: BaseOpenAIClientMethods {
    fn with_key(
        endpoint: impl AsRef<str>,
        secret: impl Into<String>,
        client_options: Option<AzureOpenAIClientOptions>,
    ) -> Result<Self>
    where
        Self: Sized;

    fn endpoint(&self) -> &Url;

    fn chat_completions_client(&self) -> ChatCompletionsClient;
}

#[derive(Debug, Clone)]
pub struct AzureOpenAIClient {
    endpoint: Url,
    pipeline: azure_core::Pipeline,
    options: AzureOpenAIClientOptions,
}

impl AzureOpenAIClientMethods for AzureOpenAIClient {
    fn with_key(
        endpoint: impl AsRef<str>,
        secret: impl Into<String>,
        client_options: Option<AzureOpenAIClientOptions>,
    ) -> Result<Self> {
        let endpoint = Url::parse(endpoint.as_ref())?;

        let options = client_options.unwrap_or_default();

        let auth_policy: Arc<dyn Policy> = AzureKeyCredential::new(secret).into();
        let version_policy: Arc<dyn Policy> = options.api_service_version.clone().into();
        let per_call_policies: Vec<Arc<dyn Policy>> = vec![auth_policy, version_policy];

        let pipeline = new_pipeline(per_call_policies, options.client_options.clone());

        Ok(AzureOpenAIClient {
            endpoint,
            pipeline,
            options,
        })
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn chat_completions_client(&self) -> ChatCompletionsClient {
        ChatCompletionsClient::new(Box::new(self.clone()))
    }
}

impl BaseOpenAIClientMethods for AzureOpenAIClient {
    fn base_url(&self, deployment_name: Option<&str>) -> azure_core::Result<Url> {
        // TODO gracefully handle this
        Ok(self
            .endpoint()
            .join("openai/")?
            .join("deployments/")?
            .join(&format!(
                "{}/",
                deployment_name.expect("Deployment name is required")
            ))?)
    }

    fn pipeline(&self) -> &azure_core::Pipeline {
        &self.pipeline
    }
}

fn new_pipeline(
    per_call_policies: Vec<Arc<dyn Policy>>,
    options: azure_core::ClientOptions,
) -> azure_core::Pipeline {
    let crate_name = option_env!("CARGO_PKG_NAME");
    let crate_version = option_env!("CARGO_PKG_VERSION");
    // should I be using per_call_policies here too or are they used by default on retries too?
    let per_retry_policies = Vec::new();

    azure_core::Pipeline::new(
        crate_name,
        crate_version,
        options,
        per_call_policies,
        per_retry_policies,
    )
}
