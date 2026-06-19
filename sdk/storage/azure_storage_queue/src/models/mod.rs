// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types for Azure Storage Queue.

#[cfg(feature = "sas")]
#[cfg_attr(docsrs, doc(cfg(feature = "sas")))]
pub mod sas;

pub use crate::generated::models::*;

use azure_core::{error::ErrorKind, Result};

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
