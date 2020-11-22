use azure_core::errors::AzureError;
use azure_core::headers::CommonStorageResponseHeaders;
use hyper::header::HeaderMap;
use std::convert::TryInto;

use super::get_messages_response::Message;

pub trait PopReceipt {
    fn message_id(&self) -> &str;
    fn pop_receipt(&self) -> &str;
}

impl PopReceipt for Message {
    fn message_id(&self) -> &str {
        &self.message_id
    }

    fn pop_receipt(&self) -> &str {
        &self.pop_receipt
    }
}

impl From<Message> for Box<dyn PopReceipt> {
    fn from(message: Message) -> Box<dyn PopReceipt> {
        Box::new(message)
    }
}

impl std::fmt::Debug for dyn PopReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PopReceipt {{ message_id: {}, pop_receipt: {} }}", self.message_id(), self.pop_receipt())
    }
}

#[derive(Debug, Clone)]
pub struct DeleteMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&HeaderMap> for DeleteMessageResponse {
    type Error = AzureError;
    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:?}", headers);

        Ok(DeleteMessageResponse {
            common_storage_response_headers: headers.try_into()?,
        })
    }
}
