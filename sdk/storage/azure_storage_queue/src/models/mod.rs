// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types for Azure Storage Queue.

pub use crate::generated::models::*;

use azure_core::error::ErrorKind;

impl TryFrom<ListOfSentMessage> for SentMessage {
    type Error = azure_core::Error;

    fn try_from(list: ListOfSentMessage) -> Result<Self, Self::Error> {
        list.items
            .unwrap_or_default()
            .into_iter()
            .next()
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    ErrorKind::DataConversion,
                    "No messages found in the response.",
                )
            })
    }
}
