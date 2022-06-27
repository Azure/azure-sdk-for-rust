use crate::prelude::*;
use azure_core::{
    collect_pinned_stream, headers::utc_date_from_rfc2822, prelude::*, Context, Method,
    Response as AzureResponse,
};
use azure_storage::{core::headers::CommonStorageResponseHeaders, xml::read_xml};
use chrono::{DateTime, Utc};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct PutMessageBuilder {
    body: String,
    queue_client: QueueClient,
    visibility_timeout: Option<VisibilityTimeout>,
    ttl: Option<MessageTTL>,
    timeout: Option<Timeout>,
    context: Context,
}

impl PutMessageBuilder {
    pub(crate) fn new(queue_client: QueueClient, body: String) -> Self {
        PutMessageBuilder {
            body,
            queue_client,
            visibility_timeout: None,
            ttl: None,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        visibility_timeout: VisibilityTimeout => Some(visibility_timeout),
        ttl: MessageTTL => Some(ttl),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.queue_client.url_with_segments(Some("messages"))?;

            self.visibility_timeout.append_to_url_query(&mut url);
            self.ttl.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            // since the format is fixed we just decorate the message with the tags.
            // This could be made optional in the future and/or more
            // stringent.
            let message = format!(
                "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
                self.body
            );

            let mut request = self.queue_client.storage_client().prepare_request(
                url.as_str(),
                Method::Post,
                Some(message.into()),
            )?;

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            PutMessageResponse::try_from(response).await
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PutMessageResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for PutMessageBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct PutMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub queue_message: QueueMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PutMessageResponseInternal {
    #[serde(rename = "QueueMessage")]
    pub queue_message: QueueMessageInternal,
}

#[derive(Debug, Clone)]
pub struct QueueMessage {
    pub message_id: String,
    pub insertion_time: DateTime<Utc>,
    pub expiration_time: DateTime<Utc>,
    pub pop_receipt: String,
    pub time_next_visible: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QueueMessageInternal {
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
}

impl PutMessageResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let response: PutMessageResponseInternal = read_xml(&body)?;
        let queue_message = response.queue_message;

        let queue_message = QueueMessage {
            message_id: queue_message.message_id,
            insertion_time: utc_date_from_rfc2822(&queue_message.insertion_time)?,
            expiration_time: utc_date_from_rfc2822(&queue_message.expiration_time)?,
            pop_receipt: queue_message.pop_receipt,
            time_next_visible: utc_date_from_rfc2822(&queue_message.time_next_visible)?,
        };

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            queue_message,
        })
    }
}
