use azure_core::Result;
use azure_openai_inference::{
    AzureOpenAIClient, AzureServiceVersion, CreateChatCompletionsRequest,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let endpoint =
        std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let azure_openai_client = AzureOpenAIClient::new(endpoint, secret)?;

    let chat_completions_request = CreateChatCompletionsRequest::new_with_user_message(
        "gpt-4-1106-preview",
        "Tell me a joke about pineapples",
    );

    let response = azure_openai_client
        .create_chat_completions_through_pipeline(
            &chat_completions_request.model,
            AzureServiceVersion::V2023_12_01Preview,
            &chat_completions_request,
        )
        .await;

    match response {
        Ok(chat_completions) => {
            println!("{:#?}", &chat_completions);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
    Ok(())
}
