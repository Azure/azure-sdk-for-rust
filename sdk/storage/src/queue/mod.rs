mod clients;
pub mod prelude;
pub mod requests;
pub mod responses;

use crate::core::Client;
use crate::responses::PopReceipt;
pub use clients::*;
use std::borrow::Cow;
use std::fmt::Debug;
use std::time::Duration;

//********* Request traits
pub trait VisibilityTimeoutSupport {
    type O;
    fn with_visibility_timeout(self, timeout: Duration) -> Self::O;
}

pub trait VisibilityTimeoutOption {
    fn visibility_timeout(&self) -> Option<Duration>;

    fn to_uri_parameter(&self) -> Option<String> {
        self.visibility_timeout()
            .map(|visibility_timeout| format!("visibilitytimeout={}", visibility_timeout.as_secs()))
    }

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(visibility_timeout) = self.visibility_timeout() {
            url.query_pairs_mut().append_pair(
                "visibilitytimeout",
                &format!("{}", visibility_timeout.as_secs()),
            );
        }
    }
}

pub trait VisibilityTimeoutRequired {
    fn visibility_timeout(&self) -> Duration;

    fn to_uri_parameter(&self) -> String {
        format!("visibilitytimeout={}", self.visibility_timeout().as_secs())
    }
}

pub trait MessageTTLSupport {
    type O;
    fn with_message_ttl_seconds(self, timeout: u64) -> Self::O;
}

pub trait MessageTTLRequired {
    fn message_ttl_seconds(&self) -> u64;

    fn to_uri_parameter(&self) -> String {
        format!("messagettl={}", self.message_ttl_seconds())
    }

    fn append_to_url(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("messagettl", &format!("{}", self.message_ttl_seconds()));
    }
}

pub trait NumberOfMessagesSupport {
    type O;
    fn with_number_of_messages(self, number_of_messages: u32) -> Self::O;
}

pub trait NumberOfMessagesOption {
    fn number_of_messages(&self) -> Option<u32>;

    fn to_uri_parameter(&self) -> Option<String> {
        self.number_of_messages()
            .map(|number_of_messages| format!("numofmessages={}", number_of_messages))
    }

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(number_of_messages) = self.number_of_messages() {
            url.query_pairs_mut()
                .append_pair("numofmessages", &format!("{}", number_of_messages));
        }
    }
}

/// Wraps the message like: '\<QueueMessage>\<MessageText>{}\</MessageText>\</QueueMessage>'
/// as per Azure specification.
/// See
/// [https://docs.microsoft.com/en-us/rest/api/storageservices/put-message](https://docs.microsoft.com/en-us/rest/api/storageservices/put-message)
pub trait MessageBodySupport<'b> {
    type O;

    /// Wraps the message like: '\<QueueMessage>\<MessageText>{}\</MessageText>\</QueueMessage>'
    /// as per Azure specification.
    /// See
    /// [https://docs.microsoft.com/en-us/rest/api/storageservices/put-message](https://docs.microsoft.com/en-us/rest/api/storageservices/put-message)
    fn with_message_body<BODY: Into<Cow<'b, str>>>(self, body: BODY) -> Self::O;
}

pub trait MessageBodyRequired {
    fn message_body(&self) -> &str;
}

/// Sets both the message id and the pop receipt for deleting a message as per Azure specification.
/// See
/// [https://docs.microsoft.com/en-us/rest/api/storageservices/delete-message2](https://docs.microsoft.com/en-us/rest/api/storageservices/delete-message2)
pub trait PopReceiptSupport {
    type O;
    fn with_pop_receipt(self, pop_receipt: Box<dyn PopReceipt>) -> Self::O;
}

pub trait PopReceiptRequired {
    fn pop_receipt(&self) -> &dyn PopReceipt;
}

//********* Queue service traits
pub trait HasStorageClient: Debug + Send + Sync {
    type StorageClient: Client;
    fn storage_client(&self) -> &Self::StorageClient;
}
