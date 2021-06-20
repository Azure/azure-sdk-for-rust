use crate::xml::read_xml;
use azure_core::headers::{utc_date_from_rfc2822, CommonStorageResponseHeaders};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::response::Response;
use std::convert::TryInto;

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

impl std::convert::TryFrom<&Response<Bytes>> for PutMessageResponse {
    type Error = crate::Error;
    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = response.body();

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);
        let response: PutMessageResponseInternal = read_xml(body)?;
        let queue_message = response.queue_message;

        let queue_message = QueueMessage {
            message_id: queue_message.message_id,
            insertion_time: utc_date_from_rfc2822(&queue_message.insertion_time)?,
            expiration_time: utc_date_from_rfc2822(&queue_message.expiration_time)?,
            pop_receipt: queue_message.pop_receipt,
            time_next_visible: utc_date_from_rfc2822(&queue_message.time_next_visible)?,
        };

        Ok(Self {
            common_storage_response_headers: headers.try_into()?,
            queue_message,
        })
    }
}
