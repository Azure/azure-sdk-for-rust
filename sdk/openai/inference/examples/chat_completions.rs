// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use azure_openai_inference::{
    clients::{ChatCompletionsClientMethods, OpenAIClient, OpenAIClientMethods},
    CreateChatCompletionsRequest,
};

/// This example illustrates how to use OpenAI to generate a chat completion.
#[tokio::main]
pub async fn main() {
    let secret = std::env::var("OPENAI_KEY").expect("Set OPENAI_KEY env variable");

    let chat_completions_client = OpenAIClient::with_key_credential(secret, None)
        .unwrap()
        .chat_completions_client();

    let chat_completions_request = CreateChatCompletionsRequest::with_user_message(
        "gpt-3.5-turbo-1106",
        "Tell me a joke about pineapples",
    );

    let response = chat_completions_client
        .create_chat_completions(&chat_completions_request.model, &chat_completions_request)
        .await;

    match response {
        Ok(chat_completions_response) => {
            let chat_completions = chat_completions_response
                .deserialize_body()
                .await
                .expect("Failed to deserialize response");
            println!("{:#?}", &chat_completions);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
}
