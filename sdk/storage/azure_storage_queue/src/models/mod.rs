// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types for Azure Storage Queue.

pub use crate::generated::models::*;

use azure_core::{fmt::SafeDebug, time::OffsetDateTime};
use serde::Deserialize;

#[derive(Clone, Default, SafeDebug)]
pub struct SentMessage(SentMessageInternal);

impl SentMessage {
    /// The ID of the message.
    pub fn message_id(&self) -> Option<&str> {
        self.0.message_id.as_deref()
    }

    /// An opaque value required to delete the message. If deletion fails using this
    /// `PopReceipt` then the message has been dequeued by another client.
    pub fn pop_receipt(&self) -> Option<&str> {
        self.0.pop_receipt.as_deref()
    }

    /// The time the message was inserted into the queue.
    pub fn insertion_time(&self) -> Option<OffsetDateTime> {
        self.0.insertion_time
    }

    /// The time that the message will expire and be automatically deleted.
    pub fn expiration_time(&self) -> Option<OffsetDateTime> {
        self.0.expiration_time
    }

    /// The time that the message will again become visible in the queue.
    pub fn time_next_visible(&self) -> Option<OffsetDateTime> {
        self.0.time_next_visible
    }
}

/// XML envelope used to deserialize the `QueueMessagesList` response for a put-message operation.
#[derive(Deserialize)]
#[serde(rename = "QueueMessagesList")]
struct SentMessageEnvelope {
    #[serde(rename = "QueueMessage", skip_serializing_if = "Option::is_none")]
    items: Option<Vec<SentMessageInternal>>,
}

impl<'de> Deserialize<'de> for SentMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let list = SentMessageEnvelope::deserialize(deserializer)?;
        let message = list
            .items
            .unwrap_or_default()
            .into_iter()
            .next()
            .ok_or_else(|| serde::de::Error::custom("No messages found in the response."))?;

        Ok(Self(message))
    }
}
