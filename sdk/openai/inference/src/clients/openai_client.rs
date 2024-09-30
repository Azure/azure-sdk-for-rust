// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use std::sync::Arc;

use azure_core::{Policy, Result, Url};

use crate::{credentials::OpenAIKeyCredential, OpenAIClientOptions};

use super::{BaseOpenAIClientMethods, ChatCompletionsClient};

/// Defines the methods provided by a [`OpenAIClient`] and can be used for mocking.
pub trait OpenAIClientMethods {
    fn chat_completions_client(&self) -> ChatCompletionsClient;
}

/// An OpenAI client.
#[derive(Debug, Clone)]
pub struct OpenAIClient {
    base_url: Url,
    pipeline: azure_core::Pipeline,
}

impl OpenAIClient {
    /// Creates a new [`OpenAIClient`] using a secret key.
    ///
    /// # Parameters
    /// * `secret` - The key credential used for authentication.
    /// * `client_options` - Optional configuration for the client. Reserved for future used, currently can always be `None`.
    ///
    /// # Example
    /// ```no_run
    /// use azure_openai_inference::clients::{OpenAIClient, OpenAIClientMethods};
    ///
    /// let secret = std::env::var("OPENAI_KEY").expect("Set OPENAI_KEY env variable");
    /// let client = OpenAIClient::with_key_credential(secret, None).unwrap();
    /// ```
    pub fn with_key_credential(
        secret: impl Into<String>,
        client_options: Option<OpenAIClientOptions>,
    ) -> Result<Self> {
        let base_url = Url::parse("https://api.openai.com/v1/")?;
        let options = client_options.unwrap_or_default();
        let auth_policy: Arc<dyn Policy> = OpenAIKeyCredential::new(secret).into();

        let pipeline = super::new_pipeline(vec![auth_policy], options.client_options.clone());

        Ok(OpenAIClient { base_url, pipeline })
    }
}

impl OpenAIClientMethods for OpenAIClient {
    fn chat_completions_client(&self) -> ChatCompletionsClient {
        ChatCompletionsClient::new(Box::new(self.clone()))
    }
}

impl BaseOpenAIClientMethods for OpenAIClient {
    fn pipeline(&self) -> &azure_core::Pipeline {
        &self.pipeline
    }

    fn base_url(&self, _deployment_name: Option<&str>) -> Result<Url> {
        Ok(self.base_url.clone())
    }
}
