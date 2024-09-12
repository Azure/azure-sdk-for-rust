use std::sync::Arc;

use crate::auth::AzureKeyCredential;
use crate::models::CreateChatCompletionsRequest;
use crate::options::AzureOpenAIClientOptions;
use crate::CreateChatCompletionsResponse;
use azure_core::{self, Method, Policy, Result};
use azure_core::{Context, Url};

pub struct AzureOpenAIClient<'a> {
    endpoint: Url,
    context: Context<'a>,
    pipeline: azure_core::Pipeline,
    options: AzureOpenAIClientOptions,
}

impl AzureOpenAIClient<'_> {
    // TODO: not sure if this should be named `with_key_credential` instead
    pub fn new(
        endpoint: impl AsRef<str>,
        secret: String,
        client_options: Option<AzureOpenAIClientOptions>,
    ) -> Result<Self> {
        let endpoint = Url::parse(endpoint.as_ref())?;
        let key_credential = AzureKeyCredential::new(secret);

        let context = Context::new();

        let options = client_options.unwrap_or_default();
        let per_call_policies: Vec<Arc<dyn Policy>> = key_credential.clone().into();

        let pipeline = Self::new_pipeline(per_call_policies, options.client_options.clone());

        Ok(AzureOpenAIClient {
            endpoint,
            context,
            pipeline,
            options,
        })
    }

    fn new_pipeline(
        per_call_policies: Vec<Arc<dyn Policy>>,
        options: azure_core::ClientOptions,
    ) -> azure_core::Pipeline {
        let crate_name = option_env!("CARGO_PKG_NAME");
        let crate_version = option_env!("CARGO_PKG_VERSION");
        let per_retry_policies = Vec::new();

        azure_core::Pipeline::new(
            crate_name,
            crate_version,
            options,
            per_call_policies,
            per_retry_policies,
        )
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub async fn create_chat_completions(
        &self,
        deployment_name: &str,
        chat_completions_request: &CreateChatCompletionsRequest,
        // Should I be using RequestContent ? All the new methods have signatures that would force me to mutate
        // the request object into &static str, Vec<u8>, etc.
        // chat_completions_request: RequestContent<CreateChatCompletionsRequest>,
    ) -> Result<CreateChatCompletionsResponse> {
        let url = Url::parse(&format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            &self.endpoint,
            deployment_name,
            &self.options.api_service_version.to_string(),
        ))?;

        let mut request = azure_core::Request::new(url, Method::Post);
        // adding the mandatory header shouldn't be necessary if the pipeline was setup correctly (?)
        // request.add_mandatory_header(&self.key_credential);

        request.set_json(chat_completions_request)?;

        let response = self
            .pipeline
            .send::<CreateChatCompletionsResponse>(&self.context, &mut request)
            .await?;
        response.into_body().json().await
    }
}
