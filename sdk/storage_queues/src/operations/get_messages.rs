use crate::{clients::QueueClient, prelude::*, PopReceipt};
use azure_core::{headers::Headers, prelude::*, Method, Response as AzureResponse};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use serde::Deserialize;
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    GetMessages,
    client: QueueClient,
    ?number_of_messages: NumberOfMessages,
    ?visibility_timeout: VisibilityTimeout
}

impl GetMessagesBuilder {
    pub fn into_future(mut self) -> GetMessages {
        Box::pin(async move {
            let mut url = self.client.url_with_segments(Some("messages"))?;

            self.visibility_timeout.append_to_url_query(&mut url);
            self.number_of_messages.append_to_url_query(&mut url);

            let mut request = self.client.storage_client().finalize_request(
                url,
                Method::Get,
                Headers::new(),
                None,
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            GetMessagesResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Message {
    message_id: String,
    pop_receipt: String,
    #[serde(with = "azure_core::date::rfc1123")]
    pub insertion_time: OffsetDateTime,
    #[serde(with = "azure_core::date::rfc1123")]
    pub expiration_time: OffsetDateTime,
    #[serde(with = "azure_core::date::rfc1123")]
    pub time_next_visible: OffsetDateTime,
    pub dequeue_count: u64,
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
    #[serde(rename = "QueueMessage", default)]
    pub messages: Vec<Message>,
}

impl GetMessagesResponse {
    fn parse_messages(body: &[u8]) -> azure_core::Result<Vec<Message>> {
        let response: MessageList = read_xml(body)?;
        Ok(response.messages)
    }

    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        let messages = Self::parse_messages(&body)?;

        Ok(GetMessagesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            messages,
        })
    }
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
