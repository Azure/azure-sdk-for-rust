// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use super::{new_json_request, BaseOpenAIClientMethods};
use crate::{
    helpers::streaming::{string_chunks, EventStreamer},
    response::{CreateChatCompletionsResponse, CreateChatCompletionsStreamResponse},
    CreateChatCompletionsRequest,
};
use azure_core::{Context, Method, Response, Result};
use futures::{Stream, StreamExt};

/// A [`ChatCompletionsClient`]'s methods. This trait can be used for mocking.
pub trait ChatCompletionsClientMethods {
    /// Creates a new chat completion.
    ///
    /// # Arguments
    /// * `deployment_name` - The name of the deployment in Azure. In OpenAI it is the model name to be used.
    /// * `chat_completions_request` - The request specifying the chat completion to be created.
    #[allow(async_fn_in_trait)]
    async fn create_chat_completions(
        &self,
        deployment_name: impl AsRef<str>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<Response<CreateChatCompletionsResponse>>;

    /// Creates a new chat completion and returns a streamed response.
    ///
    /// # Arguments
    /// * `deployment_name` - The name of the deployment in Azure. In OpenAI it is the model name to be used.
    /// * `chat_completions_request` - The request specifying the chat completion to be created.
    #[allow(async_fn_in_trait)]
    async fn stream_chat_completions(
        &self,
        deployment_name: impl AsRef<str>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<impl Stream<Item = Result<CreateChatCompletionsStreamResponse>>>;
}

/// A client for Chat Completions related operations.
pub struct ChatCompletionsClient {
    /// The underlying HTTP client with an associated pipeline.
    base_client: Box<dyn BaseOpenAIClientMethods>,
}

impl ChatCompletionsClient {
    pub(super) fn new(base_client: Box<dyn BaseOpenAIClientMethods>) -> Self {
        Self { base_client }
    }
}

impl ChatCompletionsClientMethods for ChatCompletionsClient {
    /// Creates a new chat completion.
    ///
    /// # Arguments
    /// * `deployment_name` - The name of the deployment in Azure. In OpenAI it is the model name to be used.
    /// * `chat_completions_request` - The request specifying the chat completion to be created.
    async fn create_chat_completions(
        &self,
        deployment_name: impl AsRef<str>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<Response<CreateChatCompletionsResponse>> {
        let base_url = self.base_client.base_url(Some(deployment_name.as_ref()))?;
        let request_url = base_url.join("chat/completions")?;

        let mut request = new_json_request(request_url, Method::Post, &chat_completions_request);

        self.base_client
            .pipeline()
            .send::<CreateChatCompletionsResponse>(&Context::new(), &mut request)
            .await
    }

    /// Creates a new chat completion and returns a streamed response.
    ///
    /// # Arguments
    /// * `deployment_name` - The name of the deployment in Azure. In OpenAI it is the model name to be used.
    /// * `chat_completions_request` - The request specifying the chat completion to be created.
    async fn stream_chat_completions(
        &self,
        deployment_name: impl AsRef<str>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<impl Stream<Item = Result<CreateChatCompletionsStreamResponse>>> {
        let base_url = self.base_client.base_url(Some(deployment_name.as_ref()))?;
        let request_url = base_url.join("chat/completions")?;

        let mut request = new_json_request(request_url, Method::Post, &chat_completions_request);

        let response_body = self
            .base_client
            .pipeline()
            .send::<()>(&Context::new(), &mut request)
            .await?
            .into_body();

        Ok(ChatCompletionsStreamHandler::event_stream(response_body))
    }
}

/// A placeholder type to provide an implementation for the [`EventStreamer`] trait specifically for chat completions.
struct ChatCompletionsStreamHandler;

impl EventStreamer<CreateChatCompletionsStreamResponse> for ChatCompletionsStreamHandler {
    fn event_stream(
        response_body: azure_core::ResponseBody,
    ) -> impl Stream<Item = Result<CreateChatCompletionsStreamResponse>> {
        let stream_event_delimiter = "\n\n";

        string_chunks(response_body, stream_event_delimiter).map(|event| match event {
            Ok(event) => serde_json::from_str::<CreateChatCompletionsStreamResponse>(&event)
                .map_err(|e| e.into()),
            Err(e) => Err(e),
        })
    }
}
