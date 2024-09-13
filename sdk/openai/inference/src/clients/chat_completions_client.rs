use super::BaseOpenAIClientMethods;
use crate::{request::CreateChatCompletionsRequest, response::CreateChatCompletionsResponse};
use azure_core::{Context, Method, Result};

pub trait ChatCompletionsClientMethods {
    #[allow(async_fn_in_trait)]
    async fn create_chat_completions(
        &self,
        deployment_name: impl AsRef<str>,
        chat_completions_request: &CreateChatCompletionsRequest,
    ) -> Result<CreateChatCompletionsResponse>;
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
    ) -> Result<CreateChatCompletionsResponse> {
        let base_url = self.base_client.base_url(Some(deployment_name.as_ref()))?;
        let request_url = base_url.join("chat/completions")?;

        let context = Context::new();

        let mut request = azure_core::Request::new(request_url, Method::Post);
        // this was replaced by the AzureServiceVersion policy, not sure what is the right approach
        // adding the mandatory header shouldn't be necessary if the pipeline was setup correctly (?)
        // request.add_mandatory_header(&self.key_credential);

        request.set_json(chat_completions_request)?;

        let response = self
            .base_client
            .pipeline()
            .send::<CreateChatCompletionsResponse>(&context, &mut request)
            .await?;
        response.into_body().json().await

        // todo!()
    }
}
