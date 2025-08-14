// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Models and types used throughout the Service Bus client.

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Represents the state of a Service Bus entity.
#[derive(Clone, Copy, SafeDebug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EntityState {
    /// The entity is active and can send/receive messages.
    #[serde(rename = "Active")]
    Active,
    /// The entity is disabled and cannot send/receive messages.
    #[serde(rename = "Disabled")]
    Disabled,
    /// The entity is temporarily disabled due to an error.
    #[serde(rename = "SendDisabled")]
    SendDisabled,
    /// The entity is temporarily disabled for receiving.
    #[serde(rename = "ReceiveDisabled")]
    ReceiveDisabled,
}

/// Statistics about a Service Bus queue.
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct QueueRuntimeProperties {
    /// The name of the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    /// The current size of the queue in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    /// The total number of messages in the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_message_count: Option<i64>,
    /// The number of active messages in the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_message_count: Option<i64>,
    /// The number of messages in the dead letter queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_message_count: Option<i64>,
    /// The number of scheduled messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_message_count: Option<i64>,
    /// The number of messages transferred to another queue/topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message_count: Option<i64>,
    /// The number of messages transferred to the dead letter queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_dead_letter_message_count: Option<i64>,
    /// The time when the queue was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<OffsetDateTime>,
    /// The time when the queue was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<OffsetDateTime>,
    /// The time when the queue was last accessed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed_at: Option<OffsetDateTime>,
}

/// Statistics about a Service Bus topic.
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TopicRuntimeProperties {
    /// The name of the topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// The current size of the topic in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    /// The number of subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_count: Option<i32>,
    /// The number of scheduled messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_message_count: Option<i64>,
    /// The time when the topic was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<OffsetDateTime>,
    /// The time when the topic was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<OffsetDateTime>,
    /// The time when the topic was last accessed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed_at: Option<OffsetDateTime>,
}

/// Statistics about a Service Bus subscription.
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRuntimeProperties {
    /// The name of the topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// The name of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    /// The total number of messages in the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_message_count: Option<i64>,
    /// The number of active messages in the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_message_count: Option<i64>,
    /// The number of messages in the dead letter queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_message_count: Option<i64>,
    /// The number of messages transferred to another queue/topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message_count: Option<i64>,
    /// The number of messages transferred to the dead letter queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_dead_letter_message_count: Option<i64>,
    /// The time when the subscription was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<OffsetDateTime>,
    /// The time when the subscription was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<OffsetDateTime>,
    /// The time when the subscription was last accessed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed_at: Option<OffsetDateTime>,
}

/// Represents the status of a Service Bus entity.
#[derive(Clone, Copy, SafeDebug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EntityStatus {
    /// The entity is active.
    #[serde(rename = "Active")]
    Active,
    /// The entity is creating.
    #[serde(rename = "Creating")]
    Creating,
    /// The entity is deleting.
    #[serde(rename = "Deleting")]
    Deleting,
    /// The entity is disabled.
    #[serde(rename = "Disabled")]
    Disabled,
    /// The entity is receiving disabled.
    #[serde(rename = "ReceiveDisabled")]
    ReceiveDisabled,
    /// The entity is renaming.
    #[serde(rename = "Renaming")]
    Renaming,
    /// The entity is restoring.
    #[serde(rename = "Restoring")]
    Restoring,
    /// The entity is sending disabled.
    #[serde(rename = "SendDisabled")]
    SendDisabled,
    /// The entity status is unknown.
    #[serde(rename = "Unknown")]
    Unknown,
}

/// Represents access rights for a Service Bus entity.
#[derive(Clone, Copy, SafeDebug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AccessRights {
    /// Permission to manage the entity.
    #[serde(rename = "Manage")]
    Manage,
    /// Permission to send messages to the entity.
    #[serde(rename = "Send")]
    Send,
    /// Permission to receive messages from the entity.
    #[serde(rename = "Listen")]
    Listen,
}

/// Information about a Service Bus namespace.
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct NamespaceProperties {
    /// The name of the namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// The type of the namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<NamespaceType>,
    /// The time when the namespace was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<OffsetDateTime>,
    /// The time when the namespace was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<OffsetDateTime>,
    /// The messaging SKU of the namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messaging_sku: Option<MessagingSku>,
    /// The number of messaging units for premium namespaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messaging_units: Option<i32>,
}

/// The type of Service Bus namespace.
#[derive(Clone, Copy, SafeDebug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum NamespaceType {
    /// Messaging namespace.
    #[serde(rename = "Messaging")]
    Messaging,
    /// Mixed namespace (deprecated).
    #[serde(rename = "Mixed")]
    Mixed,
    /// Notification Hub namespace.
    #[serde(rename = "NotificationHub")]
    NotificationHub,
    /// Relay namespace.
    #[serde(rename = "Relay")]
    Relay,
}

/// The messaging SKU (pricing tier) of a Service Bus namespace.
#[derive(Clone, Copy, SafeDebug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MessagingSku {
    /// Basic tier.
    #[serde(rename = "Basic")]
    Basic,
    /// Standard tier.
    #[serde(rename = "Standard")]
    Standard,
    /// Premium tier.
    #[serde(rename = "Premium")]
    Premium,
}
