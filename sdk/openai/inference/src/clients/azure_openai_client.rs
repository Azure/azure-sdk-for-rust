use std::sync::Arc;

use crate::auth::{AzureKeyCredential, DEFAULT_SCOPE};

use crate::options::AzureOpenAIClientOptions;
use azure_core::auth::TokenCredential;
use azure_core::{self, Policy, Result};
use azure_core::{BearerTokenCredentialPolicy, Url};

use super::chat_completions_client::ChatCompletionsClient;
use super::BaseOpenAIClientMethods;

pub trait AzureOpenAIClientMethods: BaseOpenAIClientMethods {
    fn new(
        endpoint: impl AsRef<str>,
        credentials: Arc<dyn TokenCredential>,
        client_options: Option<AzureOpenAIClientOptions>,
    ) -> Result<Self>
    where
        Self: Sized;

    fn with_key_credential(
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
    #[allow(dead_code)]
    options: AzureOpenAIClientOptions,
}

impl AzureOpenAIClientMethods for AzureOpenAIClient {
    fn new(
        endpoint: impl AsRef<str>,
        credential: Arc<dyn TokenCredential>,
        client_options: Option<AzureOpenAIClientOptions>,
    ) -> Result<Self> {
        let endpoint = Url::parse(endpoint.as_ref())?;

        let options = client_options.unwrap_or_default();

        let auth_policy = Arc::new(BearerTokenCredentialPolicy::new(credential, DEFAULT_SCOPE));
        let version_policy: Arc<dyn Policy> = options.api_service_version.clone().into();
        let per_call_policies: Vec<Arc<dyn Policy>> = vec![auth_policy, version_policy];

        let pipeline = super::new_pipeline(per_call_policies, options.client_options.clone());

        Ok(AzureOpenAIClient {
            endpoint,
            pipeline,
            options,
        })
    }

    fn with_key_credential(
        endpoint: impl AsRef<str>,
        secret: impl Into<String>,
        client_options: Option<AzureOpenAIClientOptions>,
    ) -> Result<Self> {
        let endpoint = Url::parse(endpoint.as_ref())?;

        let options = client_options.unwrap_or_default();

        let auth_policy: Arc<dyn Policy> = AzureKeyCredential::new(secret).into();
        let version_policy: Arc<dyn Policy> = options.api_service_version.clone().into();
        let per_call_policies: Vec<Arc<dyn Policy>> = vec![auth_policy, version_policy];

        let pipeline = super::new_pipeline(per_call_policies, options.client_options.clone());

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
        // TODO gracefully handle this, if it makes sense. A panic seems appropriate IMO.
        Ok(self
            .endpoint()
            .join("openai/")?
            .join("deployments/")?
            .join(&format!(
                "{}/",
                deployment_name.expect("Deployment name is required.")
            ))?)
    }

    fn pipeline(&self) -> &azure_core::Pipeline {
        &self.pipeline
    }
}
