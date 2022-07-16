use crate::{clients::QueueClient, prelude::*};
use azure_core::{
    collect_pinned_stream, headers::Headers, prelude::*, Method, Response as AzureResponse,
};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use chrono::{DateTime, Utc};
use std::convert::TryInto;

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
    #[serde(deserialize_with = "deserialize_utc_date_from_rfc2822")]
    pub insertion_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_utc_date_from_rfc2822")]
    pub expiration_time: DateTime<Utc>,
    pub dequeue_count: u64,
    pub message_text: String,
}

impl PeekMessagesResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let messages = read_xml::<PeekMessagesBody>(&body)?.messages;

        Ok(PeekMessagesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            messages,
        })
    }
}
