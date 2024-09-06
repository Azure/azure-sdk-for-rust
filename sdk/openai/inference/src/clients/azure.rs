use std::sync::Arc;

use crate::auth::AzureKeyCredential;
use crate::models::CreateChatCompletionsRequest;
use crate::CreateChatCompletionsResponse;
use azure_core::{self, ClientOptions, HttpClient, Method, Policy, Result};
use azure_core::{Context, Url};

// TODO: Implement using this instead
// typespec_client_core::json_model!(CreateChatCompletionsResponse);

#[derive(Clone, Debug, Default)]
pub struct AzureOpenAIClientOptions {
    client_options: ClientOptions,
}

pub struct AzureOpenAIClient <'a> {
    http_client: Arc<dyn HttpClient>,
    endpoint: Url,
    key_credential: AzureKeyCredential,
    context: Context<'a>,
    pipeline: azure_core::Pipeline,
    azure_openai_client_options: AzureOpenAIClientOptions
}

impl AzureOpenAIClient <'_> {
    // TODO: not sure if this should be named `with_key_credential` instead
    pub fn new(endpoint: impl AsRef<str>, secret: String, client_options: Option<AzureOpenAIClientOptions>) -> Result<Self> {
        let endpoint = Url::parse(endpoint.as_ref())?;
        let key_credential = AzureKeyCredential::new(secret);

        let context = Context::new();

        let pipeline = Self::new_pipeline();
        let mut azure_openai_client_options = client_options.unwrap_or_default();
        let per_call_policies: Vec<Arc<dyn Policy>> = key_credential.clone().into();
        azure_openai_client_options.client_options.set_per_call_policies(per_call_policies);

        Ok(AzureOpenAIClient {
            http_client: azure_core::new_http_client(),
            endpoint,
            key_credential,
            context,
            pipeline,
            azure_openai_client_options
        })
    }

    fn new_pipeline() -> azure_core::Pipeline {
        let crate_name = option_env!("CARGO_PKG_NAME");
        let crate_version = option_env!("CARGO_PKG_VERSION");
        let options = azure_core::ClientOptions::default();
        let per_call_policies = Vec::new();
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

    pub async fn create_chat_completions_through_pipeline(
        &self,
        deployment_name: &str,
        api_version: impl Into<String>,
        chat_completions_request: &CreateChatCompletionsRequest,
        // Should I be using RequestContent ? All the new methods have signatures that would force me to mutate
        // the request object into &static str, Vec<u8>, etc.
        // chat_completions_request: RequestContent<CreateChatCompletionsRequest>,
    ) -> Result<CreateChatCompletionsResponse> {
        let url = Url::parse(&format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            &self.endpoint,
            deployment_name,
            api_version.into()
        ))?;

        let mut request = azure_core::Request::new(url, Method::Post);
        // adding the mandatory header shouldn't be necessary if the pipeline was setup correctly (?)
        request.add_mandatory_header(&self.key_credential);

        request.set_json(chat_completions_request)?;

        let response = self
            .pipeline
            .send::<CreateChatCompletionsResponse>(&self.context, &mut request)
            .await?;
        response.into_body().json().await
    }

    pub async fn create_chat_completions(
        &self,
        deployment_name: &str,
        api_version: impl Into<String>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<CreateChatCompletionsResponse> {
        let url = Url::parse(&format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            &self.endpoint,
            deployment_name,
            api_version.into()
        ))?;
        let request = super::build_request(
            &self.key_credential,
            url,
            Method::Post,
            chat_completions_request,
        )?;
        let response = self.http_client.execute_request(&request).await?;
        let (status_code, headers, body) = response.deconstruct();

        println!("Status code: {:?}", status_code);
        println!("Headers: {:?}", headers);
        Ok(body.json().await?)
    }
}

pub enum AzureServiceVersion {
    V2023_09_01Preview,
    V2023_12_01Preview,
    V2024_07_01Preview,
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
