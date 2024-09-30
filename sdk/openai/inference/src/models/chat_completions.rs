// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
pub mod request {

    use serde::Serialize;

    /// The configuration information for a chat completions request.
    /// Completions support a wide variety of tasks and generate text that continues from or "completes"
    /// provided prompt data.
    #[derive(Serialize, Debug, Clone, Default)]
    #[non_exhaustive]
    pub struct CreateChatCompletionsRequest {
        pub messages: Vec<ChatCompletionRequestMessage>,
        pub model: String,
        pub stream: Option<bool>,
    }

    /// An abstract representation of  a chat message as provided in a request.
    #[derive(Serialize, Debug, Clone, Default)]
    #[non_exhaustive]
    pub struct ChatCompletionRequestMessageBase {
        /// An optional name for the participant.
        #[serde(skip)]
        pub name: Option<String>,
        /// The contents of the message.
        pub content: String, // TODO this should be either a string or ChatCompletionRequestMessageContentPart (a polymorphic type)
    }

    /// A description of the intended purpose of a message within a chat completions interaction.
    #[derive(Serialize, Debug, Clone)]
    #[non_exhaustive]
    #[serde(tag = "role")]
    pub enum ChatCompletionRequestMessage {
        /// The role that instructs or sets the behavior of the assistant."
        #[serde(rename = "system")]
        System(ChatCompletionRequestMessageBase),

        /// The role that provides input for chat completions.
        #[serde(rename = "user")]
        User(ChatCompletionRequestMessageBase),
    }

    impl ChatCompletionRequestMessage {
        /// Creates a new [`ChatCompletionRequestMessage`] with a single `user` message.
        pub fn with_user_role(content: impl Into<String>) -> Self {
            Self::User(ChatCompletionRequestMessageBase {
                content: content.into(),
                name: None,
            })
        }

        /// Creates a new [`ChatCompletionRequestMessage`] with a single `system` message.
        pub fn with_system_role(content: impl Into<String>) -> Self {
            Self::System(ChatCompletionRequestMessageBase {
                content: content.into(),
                name: None,
            })
        }
    }

    impl CreateChatCompletionsRequest {
        /// Creates a new [`CreateChatCompletionsRequest`] with a single `user` message.
        ///
        /// # Example
        ///
        /// ```rust
        /// let request = azure_openai_inference::request::CreateChatCompletionsRequest::with_user_message(
        ///     "gpt-3.5-turbo-1106",
        ///     "Why couldn't the eagles take Frodo directly to mount doom?");
        /// ```
        pub fn with_user_message(model: &str, prompt: &str) -> Self {
            Self {
                model: model.to_string(),
                messages: vec![ChatCompletionRequestMessage::with_user_role(prompt)],
                ..Default::default()
            }
        }

        /// Creates a new [`CreateChatCompletionsRequest`] with a single `system` message whose response will be streamed.
        ///
        /// # Example
        ///
        /// ```rust
        /// let request = azure_openai_inference::request::CreateChatCompletionsRequest::with_user_message_and_stream(
        ///     "gpt-3.5-turbo-1106",
        ///     "Why couldn't the eagles take Frodo directly to Mount Doom?");
        /// ```
        pub fn with_user_message_and_stream(
            model: impl Into<String>,
            prompt: impl Into<String>,
        ) -> Self {
            Self {
                model: model.into(),
                messages: vec![ChatCompletionRequestMessage::with_user_role(prompt)],
                stream: Some(true),
                ..Default::default()
            }
        }

        /// Creates a new [`CreateChatCompletionsRequest`] with a list of messages.
        ///
        /// # Example
        /// ```rust
        /// let request = azure_openai_inference::request::CreateChatCompletionsRequest::with_messages(
        ///     "gpt-3.5-turbo-1106",
        ///     vec![
        ///         azure_openai_inference::request::ChatCompletionRequestMessage::with_system_role("You are a good math tutor who explains things briefly."),
        ///         azure_openai_inference::request::ChatCompletionRequestMessage::with_user_role("What is the value of 'x' in the equation: '2x + 3 = 11'?"),
        ///    ]);
        pub fn with_messages(
            model: impl Into<String>,
            messages: Vec<ChatCompletionRequestMessage>,
        ) -> Self {
            Self {
                model: model.into(),
                messages,
                ..Default::default()
            }
        }
    }
}

pub mod response {

    use azure_core::Model;
    use serde::Deserialize;

    /// Representation of the response data from a chat completions request.
    /// Completions support a wide variety of tasks and generate text that continues from or "completes"
    /// provided prompt data.
    #[derive(Debug, Clone, Deserialize, Model)]
    #[non_exhaustive]
    pub struct CreateChatCompletionsResponse {
        /// The collection of completions choices associated with this completions response.
        /// Generally, `n` choices are generated per provided prompt with a default value of 1.
        /// Token limits and other settings may limit the number of choices generated.
        pub choices: Vec<ChatCompletionChoice>,
    }

    /// The representation of a single prompt completion as part of an overall chat completions request.
    /// Generally, `n` choices are generated per provided prompt with a default value of 1.
    /// Token limits and other settings may limit the number of choices generated.
    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct ChatCompletionChoice {
        /// The chat message for a given chat completions prompt.
        pub message: ChatCompletionResponseMessage,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct ChatCompletionResponseMessage {
        /// The content of the message.
        pub content: Option<String>,

        /// The chat role associated with the message.
        pub role: String,
    }

    // region: --- Streaming
    /// Represents a streamed chunk of a chat completion response returned by model, based on the provided input.
    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct CreateChatCompletionsStreamResponse {
        /// A list of chat completion choices. Can contain more than one elements if `n` is greater than 1.
        pub choices: Vec<ChatCompletionStreamChoice>,
    }

    /// A chat completion delta generated by streamed model responses.
    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct ChatCompletionStreamChoice {
        /// The delta message content for a streaming response.
        pub delta: Option<ChatCompletionStreamResponseMessage>,
    }

    /// A chat completion delta generated by streamed model responses.
    ///
    /// Note: all fields are optional because in a streaming scenario there is no guarantee of what is present in the model.
    #[derive(Debug, Clone, Deserialize)]
    #[non_exhaustive]
    pub struct ChatCompletionStreamResponseMessage {
        /// The content of the chunk message.
        pub content: Option<String>,

        /// The chat role associated with the message.
        pub role: Option<String>,
    }
    // endregion: Streaming
}
