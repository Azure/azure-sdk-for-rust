use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateChatCompletionsResponse {
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChoice {
    pub message: ChatCompletionResponseMessage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponseMessage {
    pub content: Option<String>,
    pub role: String,
}

// region: --- Streaming
#[derive(Debug, Clone, Deserialize)]
pub struct CreateChatCompletionsStreamResponse {
    pub choices: Vec<ChatCompletionStreamChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionStreamChoice {
    pub delta: Option<ChatCompletionStreamResponseMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionStreamResponseMessage {
    pub content: Option<String>,
    pub role: Option<String>,
}

// endregion: Streaming
