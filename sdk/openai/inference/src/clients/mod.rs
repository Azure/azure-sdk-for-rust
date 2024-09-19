mod azure_openai_client;
mod chat_completions_client;
mod openai_client;

use std::sync::Arc;

pub use azure_openai_client::{AzureOpenAIClient, AzureOpenAIClientMethods};
pub use chat_completions_client::{ChatCompletionsClient, ChatCompletionsClientMethods};
pub use openai_client::{OpenAIClient, OpenAIClientMethods};

pub(crate) trait BaseOpenAIClientMethods {
    fn base_url(&self, deployment_name: Option<&str>) -> azure_core::Result<azure_core::Url>;

    fn pipeline(&self) -> &azure_core::Pipeline;
}

fn new_pipeline(
    per_call_policies: Vec<Arc<dyn azure_core::Policy>>,
    options: azure_core::ClientOptions,
) -> azure_core::Pipeline {
    let crate_name = option_env!("CARGO_PKG_NAME");
    let crate_version = option_env!("CARGO_PKG_VERSION");
    // should I be using per_call_policies here too or are they used by default on retries too?
    let per_retry_policies = Vec::new();

    azure_core::Pipeline::new(
        crate_name,
        crate_version,
        options,
        per_call_policies,
        per_retry_policies,
    )
}

fn new_json_request<T>(
    url: azure_core::Url,
    method: azure_core::Method,
    json_body: &T,
) -> azure_core::Request
where
    T: serde::Serialize,
{
    let mut request = azure_core::Request::new(url, method);

    // For some reason non-Azure OpenAI's API is strict about these headers being present
    request.insert_header(azure_core::headers::CONTENT_TYPE, "application/json");
    request.insert_header(azure_core::headers::ACCEPT, "application/json");

    request.set_json(json_body).unwrap();
    request
}
