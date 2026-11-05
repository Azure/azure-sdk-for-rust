// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types for Azure Storage Queue.

pub use crate::generated::models::*;

use azure_core::{
    error::ErrorKind,
    http::{
        headers::{HeaderName, Headers},
        Response, XmlFormat,
    },
    Result,
};
use azure_storage_common::models::UserDelegationKey;

const REQUEST_PRIORITY: HeaderName = HeaderName::from_static("x-ms-request-priority");

/// Provides typed response headers for
/// [`QueueServiceClient::get_user_delegation_key()`](crate::clients::QueueServiceClient::get_user_delegation_key).
///
/// # Examples
///
/// ```
/// use azure_core::http::{Response, XmlFormat};
/// use azure_storage_queue::models::UserDelegationKeyHeaders;
/// use azure_storage_common::models::UserDelegationKey;
///
/// fn priority(response: &Response<UserDelegationKey, XmlFormat>) -> azure_core::Result<Option<i32>> {
///     response.effective_request_priority()
/// }
/// ```
pub trait UserDelegationKeyHeaders: private::Sealed {
    /// Returns the effective request priority reported by the service.
    ///
    /// Returns `None` if the header is absent, or an error if its value cannot be parsed as an `i32`.
    fn effective_request_priority(&self) -> Result<Option<i32>>;
}

impl UserDelegationKeyHeaders for Response<UserDelegationKey, XmlFormat> {
    fn effective_request_priority(&self) -> Result<Option<i32>> {
        Headers::get_optional_as(self.headers(), &REQUEST_PRIORITY)
    }
}

mod private {
    use super::{Response, UserDelegationKey, XmlFormat};

    pub trait Sealed {}

    impl Sealed for Response<UserDelegationKey, XmlFormat> {}
}

impl ListOfSentMessage {
    /// Consumes the response and returns the single [`SentMessage`].
    pub fn into_message(self) -> Result<SentMessage> {
        self.items
            .and_then(|v| v.into_iter().next())
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    ErrorKind::DataConversion,
                    "expected a sent message in the response",
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{RawResponse, StatusCode};

    fn response(priority: Option<&str>) -> Response<UserDelegationKey, XmlFormat> {
        let mut headers = Headers::new();
        if let Some(priority) = priority {
            headers.insert(REQUEST_PRIORITY, priority.to_owned());
        }
        RawResponse::from_bytes(StatusCode::Ok, headers, Vec::<u8>::new()).into()
    }

    #[test]
    fn effective_request_priority_reads_supported_values() -> Result<()> {
        for priority in 0..=7 {
            let response = response(Some(&priority.to_string()));
            assert_eq!(response.effective_request_priority()?, Some(priority));
        }
        Ok(())
    }

    #[test]
    fn effective_request_priority_missing() -> Result<()> {
        assert_eq!(response(None).effective_request_priority()?, None);
        Ok(())
    }

    #[test]
    fn effective_request_priority_rejects_malformed_values() {
        for value in ["not-an-integer", "1.5", "2147483648"] {
            assert!(
                response(Some(value)).effective_request_priority().is_err(),
                "expected a parsing error for {value}"
            );
        }
    }

    #[test]
    fn into_message_returns_first() {
        let list = ListOfSentMessage {
            items: Some(vec![SentMessage {
                message_id: Some("abc".into()),
                ..Default::default()
            }]),
        };
        let msg = list.into_message().expect("expected a message");
        assert_eq!(msg.message_id.as_deref(), Some("abc"));
    }

    #[test]
    fn into_message_errors_when_empty() {
        let list = ListOfSentMessage { items: None };
        let err = list.into_message().expect_err("expected an error");
        assert_eq!(err.kind(), &ErrorKind::DataConversion);

        let list = ListOfSentMessage {
            items: Some(vec![]),
        };
        let err = list.into_message().expect_err("expected an error");
        assert_eq!(err.kind(), &ErrorKind::DataConversion);
    }
}
