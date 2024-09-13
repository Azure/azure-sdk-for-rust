mod azure_openai_client;
mod chat_completions_client;
mod openai_client;

pub use azure_openai_client::{AzureOpenAIClient, AzureOpenAIClientMethods};
pub use chat_completions_client::{ChatCompletionsClient, ChatCompletionsClientMethods};

pub trait BaseOpenAIClientMethods {
    fn base_url(&self, deployment_name: Option<&str>) -> azure_core::Result<azure_core::Url>;

    fn pipeline(&self) -> &azure_core::Pipeline;
}
