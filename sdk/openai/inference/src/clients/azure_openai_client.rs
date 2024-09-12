use std::sync::Arc;

use crate::auth::AzureKeyCredential;

use crate::options::AzureOpenAIClientOptions;
use crate::request::CreateChatCompletionsRequest;
use crate::response::CreateChatCompletionsResponse;
use azure_core::{self, Method, Policy, Result};
use azure_core::{Context, Url};

pub struct AzureOpenAIClient {
    endpoint: Url,
    pipeline: azure_core::Pipeline,
    options: AzureOpenAIClientOptions,
}

impl AzureOpenAIClient {
    pub fn with_key(
        endpoint: impl AsRef<str>,
        secret: impl Into<String>,
        client_options: Option<AzureOpenAIClientOptions>,
    ) -> Result<Self> {
        let endpoint = Url::parse(endpoint.as_ref())?;
        let key_credential = AzureKeyCredential::new(secret.into());

        let options = client_options.unwrap_or_default();
        let auth_policy: Arc<dyn Policy> = key_credential.clone().into();
        let version_policy: Arc<dyn Policy> = options.api_service_version.clone().into();
        let per_call_policies: Vec<Arc<dyn Policy>> = vec![auth_policy, version_policy];

        let pipeline = Self::new_pipeline(per_call_policies, options.client_options.clone());

        Ok(AzureOpenAIClient {
            endpoint,
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
            "{}/openai/deployments/{}/chat/completions",
            &self.endpoint,
            deployment_name
        ))?;

        let context = Context::new();

        let mut request = azure_core::Request::new(url, Method::Post);
        // adding the mandatory header shouldn't be necessary if the pipeline was setup correctly (?)
        // request.add_mandatory_header(&self.key_credential);

        request.set_json(chat_completions_request)?;

        let response = self
            .pipeline
            .send::<CreateChatCompletionsResponse>(&context, &mut request)
            .await?;
        response.into_body().json().await
    }
}
