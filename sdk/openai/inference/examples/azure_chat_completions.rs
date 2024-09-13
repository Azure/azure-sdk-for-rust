use azure_core::Result;
use azure_openai_inference::{
    clients::{AzureOpenAIClient, AzureOpenAIClientMethods, ChatCompletionsClientMethods},
    request::CreateChatCompletionsRequest,
    AzureOpenAIClientOptions, AzureServiceVersion,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let endpoint =
        std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let chat_completions_client = AzureOpenAIClient::with_key(
        endpoint,
        secret,
        Some(
            AzureOpenAIClientOptions::builder()
                .with_api_version(AzureServiceVersion::V2023_12_01Preview)
                .build(),
        ),
    )?
    .chat_completions_client();

    let chat_completions_request = CreateChatCompletionsRequest::new_with_user_message(
        "gpt-4-1106-preview",
        "Tell me a joke about pineapples",
    );

    let response = chat_completions_client
        .create_chat_completions(&chat_completions_request.model, &chat_completions_request)
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
