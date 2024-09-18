pub mod request {

    use serde::Serialize;

    #[derive(Serialize, Debug, Clone, Default)]
    #[non_exhaustive]
    pub struct CreateChatCompletionsRequest {
        pub messages: Vec<ChatCompletionRequestMessage>,
        pub model: String,
        pub stream: Option<bool>,
        // pub frequency_penalty: f64,
        // pub logit_bias: Option<HashMap<String, f64>>,
        // pub logprobs: Option<bool>,
        // pub top_logprobs: Option<i32>,
        // pub max_tokens: Option<i32>,
    }

    #[derive(Serialize, Debug, Clone, Default)]
    #[non_exhaustive]
    pub struct ChatCompletionRequestMessageBase {
        #[serde(skip)]
        pub name: Option<String>,
        pub content: String, // TODO this should be either a string or ChatCompletionRequestMessageContentPart (a polymorphic type)
    }

    #[derive(Serialize, Debug, Clone)]
    #[non_exhaustive]
    #[serde(tag = "role")]
    pub enum ChatCompletionRequestMessage {
        #[serde(rename = "system")]
        System(ChatCompletionRequestMessageBase),
        #[serde(rename = "user")]
        User(ChatCompletionRequestMessageBase),
    }

    impl ChatCompletionRequestMessage {
        pub fn new_user(content: impl Into<String>) -> Self {
            Self::User(ChatCompletionRequestMessageBase {
                content: content.into(),
                name: None,
            })
        }

        pub fn new_system(content: impl Into<String>) -> Self {
            Self::System(ChatCompletionRequestMessageBase {
                content: content.into(),
                name: None,
            })
        }
    }
    impl CreateChatCompletionsRequest {
        pub fn new_with_user_message(model: &str, prompt: &str) -> Self {
            Self {
                model: model.to_string(),
                messages: vec![ChatCompletionRequestMessage::new_user(prompt)],
                ..Default::default()
            }
        }

        pub fn new_stream_with_user_message(
            model: impl Into<String>,
            prompt: impl Into<String>,
        ) -> Self {
            Self {
                model: model.into(),
                messages: vec![ChatCompletionRequestMessage::new_user(prompt)],
                stream: Some(true),
                ..Default::default()
            }
        }
    }
}

pub mod response {

    use azure_core::Model;
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize, Model)]
    #[non_exhaustive]
    pub struct CreateChatCompletionsResponse {
        pub choices: Vec<ChatCompletionChoice>,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct ChatCompletionChoice {
        pub message: ChatCompletionResponseMessage,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct ChatCompletionResponseMessage {
        pub content: Option<String>,
        pub role: String,
    }

    // region: --- Streaming
    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct CreateChatCompletionsStreamResponse {
        pub choices: Vec<ChatCompletionStreamChoice>,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct ChatCompletionStreamChoice {
        pub delta: Option<ChatCompletionStreamResponseMessage>,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct ChatCompletionStreamResponseMessage {
        pub content: Option<String>,
        pub role: Option<String>,
    }

    // endregion: Streaming
}
