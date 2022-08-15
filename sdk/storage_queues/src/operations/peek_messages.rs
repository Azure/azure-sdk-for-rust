use crate::{clients::QueueClient, prelude::*};
use azure_core::{headers::Headers, prelude::*, Method, Response as AzureResponse};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    PeekMessages,
    client: QueueClient,
    ?number_of_messages: NumberOfMessages
}

impl PeekMessagesBuilder {
    pub fn into_future(mut self) -> PeekMessages {
        Box::pin(async move {
            let mut url = self.client.url_with_segments(Some("messages"))?;

            url.query_pairs_mut().append_pair("peekonly", "true");
            self.number_of_messages.append_to_url_query(&mut url);

            let mut request = self.client.storage_client().finalize_request(
                url,
                Method::Get,
                Headers::new(),
                None,
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            PeekMessagesResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct PeekMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub messages: Vec<PeekMessage>,
}

#[derive(Debug, Deserialize)]
pub struct PeekMessagesBody {
    #[serde(rename = "QueueMessage", default)]
    pub messages: Vec<PeekMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PeekMessage {
    pub message_id: String,
    #[serde(with = "azure_core::date::rfc1123")]
    pub insertion_time: OffsetDateTime,
    #[serde(with = "azure_core::date::rfc1123")]
    pub expiration_time: OffsetDateTime,
    pub dequeue_count: u64,
    pub message_text: String,
}

impl PeekMessagesResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        let messages = read_xml::<PeekMessagesBody>(&body)?.messages;

        Ok(PeekMessagesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            messages,
        })
    }
}
