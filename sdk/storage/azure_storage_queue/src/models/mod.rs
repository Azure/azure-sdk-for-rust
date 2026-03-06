// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::models::*;

use azure_core::fmt::SafeDebug;
use serde::Deserialize;
use std::ops::Deref;

#[derive(Clone, Default, SafeDebug)]
pub struct SentMessage(SentMessageInternal);

impl Deref for SentMessage {
    type Target = SentMessageInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SentMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "QueueMessagesList")]
        struct ListOfSentMessage {
            #[serde(rename = "QueueMessage", skip_serializing_if = "Option::is_none")]
            pub items: Option<Vec<SentMessageInternal>>,
        }

        let list = ListOfSentMessage::deserialize(deserializer)?;
        let message = list
            .items
            .unwrap_or_default()
            .into_iter()
            .next()
            .ok_or_else(|| serde::de::Error::custom("No messages found in the response."))?;

        Ok(Self(message))
    }
}
