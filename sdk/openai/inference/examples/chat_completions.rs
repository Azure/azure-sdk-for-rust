use azure_openai_inference::{
    clients::{ChatCompletionsClientMethods, OpenAIClient, OpenAIClientMethods},
    request::CreateChatCompletionsRequest,
};

#[tokio::main]
pub async fn main() -> azure_core::Result<()> {
    let secret = std::env::var("OPENAI_KEY").expect("Set OPENAI_KEY env variable");

    let chat_completions_client = OpenAIClient::with_key(secret, None)?.chat_completions_client();

    let chat_completions_request = CreateChatCompletionsRequest::new_with_user_message(
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
                .await?;
            println!("{:#?}", &chat_completions);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
    Ok(())
}
