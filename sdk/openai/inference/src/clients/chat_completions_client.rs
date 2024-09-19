use super::{new_json_request, BaseOpenAIClientMethods};
use crate::{
    helpers::streaming::{string_chunks, EventStreamer},
    request::CreateChatCompletionsRequest,
    response::{CreateChatCompletionsResponse, CreateChatCompletionsStreamResponse},
};
use azure_core::{Context, Method, Response, Result};
use futures::{Stream, StreamExt};

pub trait ChatCompletionsClientMethods {
    #[allow(async_fn_in_trait)]
    async fn create_chat_completions(
        &self,
        deployment_name: impl AsRef<str>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<Response<CreateChatCompletionsResponse>>;

    #[allow(async_fn_in_trait)]
    async fn stream_chat_completions(
        &self,
        deployment_name: impl AsRef<str>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<impl Stream<Item = Result<CreateChatCompletionsStreamResponse>>>;
}

pub struct ChatCompletionsClient {
    base_client: Box<dyn BaseOpenAIClientMethods>,
}

impl ChatCompletionsClient {
    pub(super) fn new(base_client: Box<dyn BaseOpenAIClientMethods>) -> Self {
        Self { base_client }
    }
}

impl ChatCompletionsClientMethods for ChatCompletionsClient {
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
