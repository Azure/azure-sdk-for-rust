use azure_openai_inference::{
    clients::{AzureOpenAIClient, AzureOpenAIClientMethods, ChatCompletionsClientMethods},
    request::CreateChatCompletionsRequest,
    AzureOpenAIClientOptions, AzureServiceVersion,
};
use futures::stream::StreamExt;
use std::io::{self, Write};

/// This example illustrates how to use Azure OpenAI with key credential authentication to stream chat completions.
#[tokio::main]
async fn main() {
    let endpoint =
        std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let chat_completions_client = AzureOpenAIClient::with_key_credential(
        endpoint,
        secret,
        Some(
            AzureOpenAIClientOptions::builder()
                .with_api_version(AzureServiceVersion::V2023_12_01Preview)
                .build(),
        ),
    )
    .unwrap()
    .chat_completions_client();

    let chat_completions_request = CreateChatCompletionsRequest::with_user_message_and_stream(
        "gpt-4-1106-preview",
        "Write me an essay that is at least 200 words long on the nutritional values (or lack thereof) of fast food.
        Start the essay by stating 'this essay will be x many words long' where x is the number of words in the essay.",);

    let response = chat_completions_client
        .stream_chat_completions(&chat_completions_request.model, &chat_completions_request)
        .await
        .unwrap();

    // this pins the stream to the stack so it is safe to poll it (namely, it won't be dealloacted or moved)
    futures::pin_mut!(response);

    while let Some(result) = response.next().await {
        match result {
            Ok(delta) => {
                if let Some(choice) = delta.choices.get(0) {
                    choice.delta.as_ref().map(|d| {
                        d.content.as_ref().map(|c| {
                            print!("{}", c);
                            let _ = io::stdout().flush();
                        });
                    });
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
