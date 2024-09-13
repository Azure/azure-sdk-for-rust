use std::pin::Pin;

use super::BaseOpenAIClientMethods;
use crate::{
    helpers::streaming::{string_chunks, EventStreamer},
    request::CreateChatCompletionsRequest,
    response::{CreateChatCompletionsResponse, CreateChatCompletionsStreamResponse},
};
use azure_core::{
    headers::{ACCEPT, CONTENT_TYPE},
    Context, Method, Response, Result,
};
use futures::{Stream, TryStreamExt};

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
    pub fn new(base_client: Box<dyn BaseOpenAIClientMethods>) -> Self {
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

        let context = Context::new();

        let mut request = azure_core::Request::new(request_url, Method::Post);
        // this was replaced by the AzureServiceVersion policy, not sure what is the right approach
        // adding the mandatory header shouldn't be necessary if the pipeline was setup correctly (?)
        // request.add_mandatory_header(&self.key_credential);

        // For some reason non-Azure OpenAI's API is strict about these headers being present
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_header(ACCEPT, "application/json");
        request.set_json(chat_completions_request)?;

        self.base_client
            .pipeline()
            .send::<CreateChatCompletionsResponse>(&context, &mut request)
            .await
    }

    async fn stream_chat_completions(
        &self,
        deployment_name: impl AsRef<str>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<impl Stream<Item = Result<CreateChatCompletionsStreamResponse>>> {
        let base_url = self.base_client.base_url(Some(deployment_name.as_ref()))?;
        let request_url = base_url.join("chat/completions")?;

        let context = Context::new();

        let mut chat_completions_request = chat_completions_request;
        chat_completions_request.stream = Some(true);

        let mut request = azure_core::Request::new(request_url, Method::Post);
        // this was replaced by the AzureServiceVersion policy, not sure what is the right approach
        // adding the mandatory header shouldn't be necessary if the pipeline was setup correctly (?)
        // request.add_mandatory_header(&self.key_credential);

        // For some reason non-Azure OpenAI's API is strict about these headers being present
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_header(ACCEPT, "application/json");
        request.set_json(chat_completions_request)?;

        let response = self.base_client
            .pipeline()
            .send(&context, &mut request)
            .await?
            .into_body();

        let stream_handler = ChatCompletionsStreamHandler::new("\n\n");

        let stream = stream_handler.event_stream(response).await;
        Ok(stream)
    }

}

struct ChatCompletionsStreamHandler {
    stream_event_delimiter: String,
}

impl ChatCompletionsStreamHandler {
    pub fn new(stream_event_delimiter: impl Into<String>) -> Self {
        let stream_event_delimiter = stream_event_delimiter.into();
        Self {
            stream_event_delimiter,
        }
    }
}

impl EventStreamer<Result<CreateChatCompletionsStreamResponse>> for ChatCompletionsStreamHandler {
    fn delimiter(&self) -> impl AsRef<str> {
        self.stream_event_delimiter.as_str()
    }

    async fn event_stream(
        &self,
        response_body: azure_core::ResponseBody,
    ) -> Pin<Box<impl Stream<Item = Result<CreateChatCompletionsStreamResponse>>>> {
        let response_body = response_body;

        let stream =
            string_chunks(response_body, self.stream_event_delimiter.as_str()).map_ok(|event| {
                // println!("EVENT AS A STRING: {:?}", &event);
                serde_json::from_str::<CreateChatCompletionsStreamResponse>(&event)
                    .expect("Deserialization failed")
                // CreateChatCompletionsStreamResponse { choices: vec![] }
            });
        Box::pin(stream)
    }
}
