use std::sync::Arc;

use azure_core::{Policy, Result, Url};

use crate::{auth::OpenAIKeyCredential, OpenAIClientOptions};

use super::{BaseOpenAIClientMethods, ChatCompletionsClient};

pub trait OpenAIClientMethods {
    fn with_key(
        secret: impl Into<String>,
        client_options: Option<OpenAIClientOptions>,
    ) -> Result<Self>
    where
        Self: Sized;

    fn chat_completions_client(&self) -> ChatCompletionsClient;
}

#[derive(Debug, Clone)]
pub struct OpenAIClient {
    base_url: Url,
    pipeline: azure_core::Pipeline,
    #[allow(dead_code)]
    options: OpenAIClientOptions,
}

impl OpenAIClientMethods for OpenAIClient {
    fn with_key(
        secret: impl Into<String>,
        client_options: Option<OpenAIClientOptions>,
    ) -> Result<Self> {
        let base_url = Url::parse("https://api.openai.com/v1/")?;
        let options = client_options.unwrap_or_default();
        let auth_policy: Arc<dyn Policy> = OpenAIKeyCredential::new(secret).into();

        let pipeline = super::new_pipeline(vec![auth_policy], options.client_options.clone());

        Ok(OpenAIClient {
            base_url,
            pipeline,
            options,
        })
    }

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
