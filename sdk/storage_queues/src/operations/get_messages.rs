use crate::{clients::QueueClient, prelude::*, PopReceipt};
use azure_core::{collect_pinned_stream, prelude::*, Context, Method, Response as AzureResponse};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use chrono::{DateTime, Utc};
use serde::Deserialize;
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

            let mut request =
                self.queue_client
                    .storage_client()
                    .prepare_request(url, Method::GET, None)?;

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

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    #[serde(rename = "MessageId")]
    message_id: String,
    #[serde(rename = "PopReceipt")]
    pop_receipt: String,
    #[serde(rename = "InsertionTime", deserialize_with = "deserialize_utc")]
    pub insertion_time: DateTime<Utc>,
    #[serde(rename = "ExpirationTime", deserialize_with = "deserialize_utc")]
    pub expiration_time: DateTime<Utc>,
    #[serde(rename = "TimeNextVisible", deserialize_with = "deserialize_utc")]
    pub time_next_visible: DateTime<Utc>,
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: u64,
    #[serde(rename = "MessageText")]
    pub message_text: String,
}

impl Message {
    pub fn pop_receipt(&self) -> PopReceipt {
        PopReceipt::new(self.message_id.clone(), self.pop_receipt.clone())
    }
}

impl From<Message> for PopReceipt {
    fn from(message: Message) -> Self {
        PopReceipt::new(message.message_id, message.pop_receipt)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct MessageList {
    #[serde(rename = "QueueMessage")]
    pub messages: Option<Vec<Message>>,
}

impl GetMessagesResponse {
    fn parse_messages(body: &[u8]) -> azure_core::Result<Vec<Message>> {
        let response: MessageList = read_xml(body)?;
        Ok(response.messages.unwrap_or_default())
    }

    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let messages = Self::parse_messages(&body)?;

        Ok(GetMessagesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            messages,
        })
    }
}

fn deserialize_utc<'de, D>(deserializer: D) -> std::result::Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let date = DateTime::parse_from_rfc2822(&s).map_err(serde::de::Error::custom)?;
    Ok(DateTime::from_utc(date.naive_utc(), Utc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_messages() -> azure_core::Result<()> {
        let message = b"\xef\xbb\xbf\
             <?xml version=\"1.0\" encoding=\"utf-8\"?>\
            <QueueMessagesList><QueueMessage>\
            <MessageId>00000000-0000-0000-0000-000000000000</MessageId>\
            <InsertionTime>Mon, 27 Jun 2022 13:38:48 GMT</InsertionTime>\
            <ExpirationTime>Mon, 04 Jul 2022 13:38:48 GMT</ExpirationTime>\
            <PopReceipt>REDACTED1</PopReceipt>\
            <TimeNextVisible>Mon, 27 Jun 2022 13:38:53 GMT</TimeNextVisible>\
            <DequeueCount>1</DequeueCount>\
            <MessageText>test1</MessageText>\
            </QueueMessage>\
            <QueueMessage>\
            <MessageId>11111111-1111-1111-1111-111111111111</MessageId>\
            <InsertionTime>Mon, 27 Jun 2022 13:38:48 GMT</InsertionTime>\
            <ExpirationTime>Mon, 04 Jul 2022 13:38:48 GMT</ExpirationTime>\
            <PopReceipt>REDACTED2</PopReceipt>\
            <TimeNextVisible>Mon, 27 Jun 2022 13:38:53 GMT</TimeNextVisible>\
            <DequeueCount>1</DequeueCount>\
            <MessageText>test2</MessageText>\
            </QueueMessage>\
            </QueueMessagesList>\
            ";

        let messages = GetMessagesResponse::parse_messages(message)?;

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].message_text, "test1");
        assert_eq!(messages[1].message_text, "test2");

        Ok(())
    }
}
