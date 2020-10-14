use azure_core::errors::AzureError;
use azure_core::{utc_date_from_rfc2822, CommonStorageResponseHeaders};
use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;
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

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for PutMessageResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("headers == {:?}", headers);

        let received = &std::str::from_utf8(body)?[3..];
        debug!("receieved == {:#?}", received);
        let response: PutMessageResponseInternal = serde_xml_rs::from_reader(&body[3..])?;

        let queue_message = QueueMessage {
            message_id: response.queue_message.message_id,
            insertion_time: utc_date_from_rfc2822(&response.queue_message.insertion_time)?,
            expiration_time: utc_date_from_rfc2822(&response.queue_message.expiration_time)?,
            pop_receipt: response.queue_message.pop_receipt,
            time_next_visible: utc_date_from_rfc2822(&response.queue_message.time_next_visible)?,
        };

        Ok(Self {
            common_storage_response_headers: headers.try_into()?,
            queue_message,
        })
    }
}
