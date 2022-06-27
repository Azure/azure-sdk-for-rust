use crate::{clients::QueueClient, prelude::*, PopReceipt};
use azure_core::{
    collect_pinned_stream,
    error::{ErrorKind, ResultExt},
    headers::utc_date_from_rfc2822,
    prelude::*,
    Context, Response as AzureResponse,
};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use chrono::{DateTime, Utc};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetMessagesBuilder {
    queue_client: QueueClient,
    number_of_messages: Option<NumberOfMessages>,
    visibility_timeout: Option<VisibilityTimeout>,
    timeout: Option<Timeout>,
    context: Context,
}

impl GetMessagesBuilder {
    pub(crate) fn new(queue_client: QueueClient) -> Self {
        GetMessagesBuilder {
            queue_client,
            number_of_messages: None,
            visibility_timeout: None,

            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        number_of_messages: NumberOfMessages => Some(number_of_messages),
        visibility_timeout: VisibilityTimeout => Some(visibility_timeout),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.queue_client.url_with_segments(Some("messages"))?;

            self.visibility_timeout.append_to_url_query(&mut url);
            self.number_of_messages.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request = self.queue_client.storage_client().prepare_request(
                url.as_str(),
                http::method::Method::GET,
                None,
            )?;

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            GetMessagesResponse::try_from(response).await
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetMessagesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetMessagesBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct GetMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub pop_receipt: PopReceipt,
    pub insertion_time: DateTime<Utc>,
    pub expiration_time: DateTime<Utc>,
    pub time_next_visible: DateTime<Utc>,
    pub dequeue_count: u64,
    pub message_text: String,
}

impl From<Message> for PopReceipt {
    fn from(message: Message) -> Self {
        message.pop_receipt
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageInternal {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "InsertionTime")]
    pub insertion_time: String,
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
    #[serde(rename = "PopReceipt")]
    pub pop_receipt: String,
    #[serde(rename = "TimeNextVisible")]
    pub time_next_visible: String,
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: u64,
    #[serde(rename = "MessageText")]
    pub message_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessagesInternal {
    #[serde(rename = "QueueMessage")]
    pub messages: Option<Vec<MessageInternal>>,
}

impl GetMessagesResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let response: MessagesInternal = read_xml(&body).map_kind(ErrorKind::DataConversion)?;

        let mut messages = Vec::new();
        for message in response.messages.unwrap_or_default().into_iter() {
            messages.push(Message {
                pop_receipt: PopReceipt::new(message.message_id, message.pop_receipt),
                insertion_time: utc_date_from_rfc2822(&message.insertion_time)?,
                expiration_time: utc_date_from_rfc2822(&message.expiration_time)?,
                time_next_visible: utc_date_from_rfc2822(&message.time_next_visible)?,
                dequeue_count: message.dequeue_count,
                message_text: message.message_text,
            })
        }

        Ok(GetMessagesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            messages,
        })
    }
}
