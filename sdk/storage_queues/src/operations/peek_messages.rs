use crate::{clients::QueueClient, prelude::*};
use azure_core::{
    collect_pinned_stream, headers::utc_date_from_rfc2822, headers::Headers, prelude::*, Context,
    Method, Response as AzureResponse,
};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use chrono::{DateTime, Utc};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct PeekMessagesBuilder {
    queue_client: QueueClient,
    number_of_messages: Option<NumberOfMessages>,
    context: Context,
}

impl PeekMessagesBuilder {
    pub(crate) fn new(queue_client: QueueClient) -> Self {
        PeekMessagesBuilder {
            queue_client,
            number_of_messages: None,
            context: Context::new(),
        }
    }

    setters! {
        number_of_messages: NumberOfMessages => Some(number_of_messages),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.queue_client.url_with_segments(Some("messages"))?;

            url.query_pairs_mut().append_pair("peekonly", "true");
            self.number_of_messages.append_to_url_query(&mut url);

            let mut request = self.queue_client.storage_client().finalize_request(
                url,
                Method::Get,
                Headers::new(),
                None,
            )?;

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            PeekMessagesResponse::try_from(response).await
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PeekMessagesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateQueueBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct PeekMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub messages: Vec<PeekMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeekMessageInternal {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "InsertionTime")]
    pub insertion_time: String,
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: u64,
    #[serde(rename = "MessageText")]
    pub message_text: String,
}

#[derive(Debug, Clone)]
pub struct PeekMessage {
    pub message_id: String,
    pub insertion_time: DateTime<Utc>,
    pub expiration_time: DateTime<Utc>,
    pub dequeue_count: u64,
    pub message_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeekMessagesInternal {
    #[serde(rename = "QueueMessage")]
    pub messages: Option<Vec<PeekMessageInternal>>,
}

impl PeekMessagesResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let response: PeekMessagesInternal = read_xml(&body)?;

        let mut messages = Vec::new();
        for message in response.messages.unwrap_or_default().into_iter() {
            messages.push(PeekMessage {
                message_id: message.message_id,
                insertion_time: utc_date_from_rfc2822(&message.insertion_time)?,
                expiration_time: utc_date_from_rfc2822(&message.expiration_time)?,
                dequeue_count: message.dequeue_count,
                message_text: message.message_text,
            })
        }

        Ok(PeekMessagesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            messages,
        })
    }
}
