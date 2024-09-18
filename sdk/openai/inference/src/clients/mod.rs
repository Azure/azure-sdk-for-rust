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
