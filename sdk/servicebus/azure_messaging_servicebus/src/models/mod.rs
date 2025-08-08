// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Models and types used throughout the Service Bus client.

use azure_core::fmt::SafeDebug;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Represents the state of a Service Bus entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityState {
    /// The entity is active and can send/receive messages.
    Active,
    /// The entity is disabled and cannot send/receive messages.
    Disabled,
    /// The entity is temporarily disabled due to an error.
    SendDisabled,
    /// The entity is temporarily disabled for receiving.
    ReceiveDisabled,
}

/// Statistics about a Service Bus queue.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
pub struct QueueRuntimeProperties {
    /// The name of the queue.
    pub queue_name: String,
    /// The current size of the queue in bytes.
    pub size_in_bytes: i64,
    /// The total number of messages in the queue.
    pub total_message_count: i64,
    /// The number of active messages in the queue.
    pub active_message_count: i64,
    /// The number of messages in the dead letter queue.
    pub dead_letter_message_count: i64,
    /// The number of scheduled messages.
    pub scheduled_message_count: i64,
    /// The number of messages transferred to another queue/topic.
    pub transfer_message_count: i64,
    /// The number of messages transferred to the dead letter queue.
    pub transfer_dead_letter_message_count: i64,
    /// The time when the queue was created.
    pub created_at: OffsetDateTime,
    /// The time when the queue was last updated.
    pub updated_at: OffsetDateTime,
    /// The time when the queue was last accessed.
    pub accessed_at: OffsetDateTime,
}

/// Statistics about a Service Bus topic.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
pub struct TopicRuntimeProperties {
    /// The name of the topic.
    pub topic_name: String,
    /// The current size of the topic in bytes.
    pub size_in_bytes: i64,
    /// The number of subscriptions.
    pub subscription_count: i32,
    /// The number of scheduled messages.
    pub scheduled_message_count: i64,
    /// The time when the topic was created.
    pub created_at: OffsetDateTime,
    /// The time when the topic was last updated.
    pub updated_at: OffsetDateTime,
    /// The time when the topic was last accessed.
    pub accessed_at: OffsetDateTime,
}

/// Statistics about a Service Bus subscription.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRuntimeProperties {
    /// The name of the topic.
    pub topic_name: String,
    /// The name of the subscription.
    pub subscription_name: String,
    /// The total number of messages in the subscription.
    pub total_message_count: i64,
    /// The number of active messages in the subscription.
    pub active_message_count: i64,
    /// The number of messages in the dead letter queue.
    pub dead_letter_message_count: i64,
    /// The number of messages transferred to another queue/topic.
    pub transfer_message_count: i64,
    /// The number of messages transferred to the dead letter queue.
    pub transfer_dead_letter_message_count: i64,
    /// The time when the subscription was created.
    pub created_at: OffsetDateTime,
    /// The time when the subscription was last updated.
    pub updated_at: OffsetDateTime,
    /// The time when the subscription was last accessed.
    pub accessed_at: OffsetDateTime,
}

/// Represents the status of a Service Bus entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityStatus {
    /// The entity is active.
    Active,
    /// The entity is creating.
    Creating,
    /// The entity is deleting.
    Deleting,
    /// The entity is disabled.
    Disabled,
    /// The entity is receiving disabled.
    ReceiveDisabled,
    /// The entity is renaming.
    Renaming,
    /// The entity is restoring.
    Restoring,
    /// The entity is sending disabled.
    SendDisabled,
    /// The entity status is unknown.
    Unknown,
}

/// Represents access rights for a Service Bus entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessRights {
    /// Permission to manage the entity.
    Manage,
    /// Permission to send messages to the entity.
    Send,
    /// Permission to receive messages from the entity.
    Listen,
}

/// Information about a Service Bus namespace.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
pub struct NamespaceProperties {
    /// The name of the namespace.
    pub namespace_name: String,
    /// The type of the namespace.
    pub namespace_type: NamespaceType,
    /// The time when the namespace was created.
    pub created_at: OffsetDateTime,
    /// The time when the namespace was last modified.
    pub modified_at: OffsetDateTime,
    /// The messaging SKU of the namespace.
    pub messaging_sku: MessagingSku,
    /// The number of messaging units for premium namespaces.
    pub messaging_units: Option<i32>,
}

/// The type of Service Bus namespace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NamespaceType {
    /// Messaging namespace.
    Messaging,
    /// Mixed namespace (deprecated).
    Mixed,
    /// Notification Hub namespace.
    NotificationHub,
    /// Relay namespace.
    Relay,
}

/// The messaging SKU (pricing tier) of a Service Bus namespace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagingSku {
    /// Basic tier.
    Basic,
    /// Standard tier.
    Standard,
    /// Premium tier.
    Premium,
}

impl std::fmt::Display for EntityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityState::Active => write!(f, "Active"),
            EntityState::Disabled => write!(f, "Disabled"),
            EntityState::SendDisabled => write!(f, "SendDisabled"),
            EntityState::ReceiveDisabled => write!(f, "ReceiveDisabled"),
        }
    }
}

impl std::fmt::Display for EntityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityStatus::Active => write!(f, "Active"),
            EntityStatus::Creating => write!(f, "Creating"),
            EntityStatus::Deleting => write!(f, "Deleting"),
            EntityStatus::Disabled => write!(f, "Disabled"),
            EntityStatus::ReceiveDisabled => write!(f, "ReceiveDisabled"),
            EntityStatus::Renaming => write!(f, "Renaming"),
            EntityStatus::Restoring => write!(f, "Restoring"),
            EntityStatus::SendDisabled => write!(f, "SendDisabled"),
            EntityStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::fmt::Display for AccessRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessRights::Manage => write!(f, "Manage"),
            AccessRights::Send => write!(f, "Send"),
            AccessRights::Listen => write!(f, "Listen"),
        }
    }
}

impl std::fmt::Display for MessagingSku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagingSku::Basic => write!(f, "Basic"),
            MessagingSku::Standard => write!(f, "Standard"),
            MessagingSku::Premium => write!(f, "Premium"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_state_display() {
        assert_eq!(EntityState::Active.to_string(), "Active");
        assert_eq!(EntityState::Disabled.to_string(), "Disabled");
        assert_eq!(EntityState::SendDisabled.to_string(), "SendDisabled");
        assert_eq!(EntityState::ReceiveDisabled.to_string(), "ReceiveDisabled");
    }

    #[test]
    fn entity_status_display() {
        assert_eq!(EntityStatus::Active.to_string(), "Active");
        assert_eq!(EntityStatus::Creating.to_string(), "Creating");
        assert_eq!(EntityStatus::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn access_rights_display() {
        assert_eq!(AccessRights::Manage.to_string(), "Manage");
        assert_eq!(AccessRights::Send.to_string(), "Send");
        assert_eq!(AccessRights::Listen.to_string(), "Listen");
    }

    #[test]
    fn messaging_sku_display() {
        assert_eq!(MessagingSku::Basic.to_string(), "Basic");
        assert_eq!(MessagingSku::Standard.to_string(), "Standard");
        assert_eq!(MessagingSku::Premium.to_string(), "Premium");
    }
}
