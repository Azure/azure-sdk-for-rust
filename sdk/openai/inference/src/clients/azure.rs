use std::sync::Arc;

use crate::auth::AzureKeyCredential;
use crate::models::CreateChatCompletionsRequest;
use crate::CreateChatCompletionsResponse;
use azure_core::{
    self, builders::ClientOptionsBuilder, AppendToUrlQuery, ClientOptions, HttpClient, Method,
    Policy, Result,
};
use azure_core::{Context, Url};

// TODO: Implement using this instead
// typespec_client_core::json_model!(CreateChatCompletionsResponse);

// TODO: I was not  able to  find ClientOptions  as  a derive macros
#[derive(Clone, Debug, Default)]
pub struct AzureOpenAIClientOptions {
    client_options: ClientOptions,
    api_service_version: AzureServiceVersion,
}

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

        let pipeline = Self::new_pipeline(per_call_policies);

        Ok(AzureOpenAIClient {
            endpoint,
            context,
            pipeline,
            options,
        })
    }

    fn new_pipeline(per_call_policies: Vec<Arc<dyn Policy>>) -> azure_core::Pipeline {
        let crate_name = option_env!("CARGO_PKG_NAME");
        let crate_version = option_env!("CARGO_PKG_VERSION");
        let options = azure_core::ClientOptions::default();
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

impl AzureOpenAIClientOptions {
    pub fn builder() -> builders::AzureOpenAIClientOptionsBuilder {
        builders::AzureOpenAIClientOptionsBuilder::new()
    }
}

pub mod builders {
    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct AzureOpenAIClientOptionsBuilder {
        options: AzureOpenAIClientOptions,
    }

    impl AzureOpenAIClientOptionsBuilder {
        pub(super) fn new() -> Self {
            Self::default()
        }
        pub fn with_api_version(mut self, api_service_version: AzureServiceVersion) -> Self {
            self.options.api_service_version = api_service_version;
            self
        }

        pub fn build(&self) -> AzureOpenAIClientOptions {
            self.options.clone()
        }
    }
}

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
