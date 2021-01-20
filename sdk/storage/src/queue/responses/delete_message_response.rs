use azure_core::errors::AzureError;
use azure_core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
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

impl<'a> std::fmt::Debug for &'a dyn PopReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "PopReceipt {{ message_id: {}, pop_receipt: {} }}",
            self.message_id(),
            self.pop_receipt()
        )
    }
}

#[derive(Debug, Clone)]
pub struct DeleteMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&Response<Bytes>> for DeleteMessageResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("response == {:?}", response);

        Ok(DeleteMessageResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
