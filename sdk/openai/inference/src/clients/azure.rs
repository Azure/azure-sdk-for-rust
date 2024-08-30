use std::sync::Arc;

use crate::auth::AzureKeyCredential;
use crate::models::CreateChatCompletionsRequest;
use crate::CreateChatCompletionsResponse;
use azure_core::Url;
use azure_core::{self, HttpClient, Method, Result};

// TODO: Implement using this instead
// typespec_client_core::json_model!(CreateChatCompletionsResponse);

pub struct AzureOpenAIClient {
    http_client: Arc<dyn HttpClient>,
    endpoint: Url,
    key_credential: AzureKeyCredential,
}

impl AzureOpenAIClient {
    pub fn new(endpoint: impl AsRef<str>, secret: String) -> Result<Self> {
        let endpoint = Url::parse(endpoint.as_ref())?;
        let key_credential = AzureKeyCredential::new(secret);

        Ok(AzureOpenAIClient {
            http_client: azure_core::new_http_client(),
            endpoint,
            key_credential,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
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
        Ok(response.into_body().json().await?)
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
