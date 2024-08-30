use std::sync::Arc;

use azure_core::auth::{Secret, TokenCredential};
use azure_core::{self, HttpClient, Result};
use reqwest::Url;

pub struct AzureOpenAIClient {
    http_client: Arc<dyn HttpClient>,
    endpoint: Url,
    secret: Secret,
}

impl AzureOpenAIClient {
    pub fn new(endpoint: impl AsRef<str>, secret: String) -> Result<Self> {
        let endpoint = Url::parse(endpoint.as_ref())?;
        let secret = Secret::from(secret);

        Ok(AzureOpenAIClient {
            http_client: azure_core::new_http_client(),
            endpoint,
            secret,
        })
    }
}
