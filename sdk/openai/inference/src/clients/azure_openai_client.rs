// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use std::sync::Arc;

use crate::credentials::{AzureKeyCredential, DEFAULT_SCOPE};

use crate::options::AzureOpenAIClientOptions;
use azure_core::auth::TokenCredential;
use azure_core::{self, Policy, Result};
use azure_core::{BearerTokenCredentialPolicy, Url};

use super::chat_completions_client::ChatCompletionsClient;
use super::BaseOpenAIClientMethods;

/// Defines the methods provided by a [`AzureOpenAIClient`] and can be used for mocking.
pub trait AzureOpenAIClientMethods {
    /// Returns the endpoint [`Url`] of the client.
    fn endpoint(&self) -> &Url;

    /// Returns a new instance of the [`ChatCompletionsClient`].
    fn chat_completions_client(&self) -> ChatCompletionsClient;
}

/// An Azure OpenAI client.
#[derive(Debug, Clone)]
pub struct AzureOpenAIClient {
    /// The Azure resource endpoint
    endpoint: Url,

    /// The pipeline for sending requests to the service.
    pipeline: azure_core::Pipeline,

    /// The options for the client.
    #[allow(dead_code)]
    options: AzureOpenAIClientOptions,
}

impl AzureOpenAIClient {
    /// Creates a new [`AzureOpenAIClient`] using a [`TokenCredential`].
    /// See the following example for Azure Active Directory authentication:
    ///
    /// # Parameters
    /// * `endpoint` - The full URL of the Azure OpenAI resource endpoint.
    /// * `credential` - An implementation of [`TokenCredential`] used for authentication.
    /// * `client_options` - Optional configuration for the client. The [`AzureServiceVersion`](crate::options::AzureServiceVersion) can be provided here.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_openai_inference::clients::{AzureOpenAIClient, AzureOpenAIClientMethods};
    /// use azure_identity::DefaultAzureCredentialBuilder;
    /// use std::sync::Arc;
    ///
    /// let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT environment variable");
    /// let client = AzureOpenAIClient::new(
    ///     endpoint,
    ///     Arc::new(DefaultAzureCredentialBuilder::new().build().unwrap()),
    ///     None,
    /// ).unwrap();
    /// ```
    pub fn new(
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

    /// Creates a new [`AzureOpenAIClient`] using a key credential
    ///
    /// # Parameters
    /// * `endpoint` - The full URL of the Azure OpenAI resource endpoint.
    /// * `secret` - The key creadential used for authentication. Passed as header parameter in the request.
    /// * `client_options` - Optional configuration for the client. The [`AzureServiceVersion`](crate::options::AzureServiceVersion) can be provided here.
    ///
    /// # Example
    /// ```no_run
    /// use azure_openai_inference::clients::{AzureOpenAIClient, AzureOpenAIClientMethods};
    ///
    /// let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT environment variable");
    /// let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY environment variable");
    /// let client = AzureOpenAIClient::with_key_credential(
    ///     endpoint,
    ///     secret,
    ///     None,
    /// ).unwrap();
    /// ```
    pub fn with_key_credential(
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
}

impl AzureOpenAIClientMethods for AzureOpenAIClient {
    /// Returns the endpoint [`Url`] of the client.
    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Returns a new instance of the [`ChatCompletionsClient`] using an [`AzureOpenAIClient`] underneath.
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
