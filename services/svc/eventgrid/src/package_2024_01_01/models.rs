#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Schema of common properties of all chat events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatEventBaseProperties {
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "recipientCommunicationIdentifier")]
    pub recipient_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The transaction id will be used as co-relation vector"]
    #[serde(rename = "transactionId", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[doc = "The chat thread id"]
    #[serde(rename = "threadId", default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}
impl AcsChatEventBaseProperties {
    pub fn new(recipient_communication_identifier: CommunicationIdentifierModel) -> Self {
        Self {
            recipient_communication_identifier,
            transaction_id: None,
            thread_id: None,
        }
    }
}
#[doc = "Schema of common properties of all thread-level chat events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatEventInThreadBaseProperties {
    #[doc = "The transaction id will be used as co-relation vector"]
    #[serde(rename = "transactionId", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[doc = "The chat thread id"]
    #[serde(rename = "threadId", default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}
impl AcsChatEventInThreadBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatMessageDeletedEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties,
    #[doc = "The time at which the message was deleted"]
    #[serde(rename = "deleteTime", with = "azure_core::date::rfc3339")]
    pub delete_time: ::time::OffsetDateTime,
}
impl AcsChatMessageDeletedEventData {
    pub fn new(acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties, delete_time: ::time::OffsetDateTime) -> Self {
        Self {
            acs_chat_message_event_base_properties,
            delete_time,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageDeletedInThread event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatMessageDeletedInThreadEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
    #[doc = "The time at which the message was deleted"]
    #[serde(rename = "deleteTime", with = "azure_core::date::rfc3339")]
    pub delete_time: ::time::OffsetDateTime,
}
impl AcsChatMessageDeletedInThreadEventData {
    pub fn new(
        acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
        delete_time: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            acs_chat_message_event_in_thread_base_properties,
            delete_time,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageEdited event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatMessageEditedEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties,
    #[doc = "The body of the chat message"]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[doc = "The chat message metadata"]
    pub metadata: serde_json::Value,
    #[doc = "The time at which the message was edited"]
    #[serde(rename = "editTime", with = "azure_core::date::rfc3339")]
    pub edit_time: ::time::OffsetDateTime,
}
impl AcsChatMessageEditedEventData {
    pub fn new(
        acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties,
        metadata: serde_json::Value,
        edit_time: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            acs_chat_message_event_base_properties,
            message_body: None,
            metadata,
            edit_time,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageEditedInThread event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatMessageEditedInThreadEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
    #[doc = "The body of the chat message"]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[doc = "The chat message metadata"]
    pub metadata: serde_json::Value,
    #[doc = "The time at which the message was edited"]
    #[serde(rename = "editTime", with = "azure_core::date::rfc3339")]
    pub edit_time: ::time::OffsetDateTime,
}
impl AcsChatMessageEditedInThreadEventData {
    pub fn new(
        acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
        metadata: serde_json::Value,
        edit_time: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            acs_chat_message_event_in_thread_base_properties,
            message_body: None,
            metadata,
            edit_time,
        }
    }
}
#[doc = "Schema of common properties of all chat message events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatMessageEventBaseProperties {
    #[serde(flatten)]
    pub acs_chat_event_base_properties: AcsChatEventBaseProperties,
    #[doc = "The chat message id"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "senderCommunicationIdentifier")]
    pub sender_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The display name of the sender"]
    #[serde(rename = "senderDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub sender_display_name: Option<String>,
    #[doc = "The original compose time of the message"]
    #[serde(rename = "composeTime", with = "azure_core::date::rfc3339")]
    pub compose_time: ::time::OffsetDateTime,
    #[doc = "The type of the message"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The version of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatMessageEventBaseProperties {
    pub fn new(
        acs_chat_event_base_properties: AcsChatEventBaseProperties,
        sender_communication_identifier: CommunicationIdentifierModel,
        compose_time: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            acs_chat_event_base_properties,
            message_id: None,
            sender_communication_identifier,
            sender_display_name: None,
            compose_time,
            type_: None,
            version: None,
        }
    }
}
#[doc = "Schema of common properties of all thread-level chat message events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatMessageEventInThreadBaseProperties {
    #[serde(flatten)]
    pub acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties,
    #[doc = "The chat message id"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "senderCommunicationIdentifier")]
    pub sender_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The display name of the sender"]
    #[serde(rename = "senderDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub sender_display_name: Option<String>,
    #[doc = "The original compose time of the message"]
    #[serde(rename = "composeTime", with = "azure_core::date::rfc3339")]
    pub compose_time: ::time::OffsetDateTime,
    #[doc = "The type of the message"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The version of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatMessageEventInThreadBaseProperties {
    pub fn new(sender_communication_identifier: CommunicationIdentifierModel, compose_time: ::time::OffsetDateTime) -> Self {
        Self {
            acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties::default(),
            message_id: None,
            sender_communication_identifier,
            sender_display_name: None,
            compose_time,
            type_: None,
            version: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatMessageReceivedEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties,
    #[doc = "The body of the chat message"]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[doc = "The chat message metadata"]
    pub metadata: serde_json::Value,
}
impl AcsChatMessageReceivedEventData {
    pub fn new(acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties, metadata: serde_json::Value) -> Self {
        Self {
            acs_chat_message_event_base_properties,
            message_body: None,
            metadata,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageReceivedInThread event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatMessageReceivedInThreadEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
    #[doc = "The body of the chat message"]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[doc = "The chat message metadata"]
    pub metadata: serde_json::Value,
}
impl AcsChatMessageReceivedInThreadEventData {
    pub fn new(
        acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            acs_chat_message_event_in_thread_base_properties,
            message_body: None,
            metadata,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadParticipantAdded event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatParticipantAddedToThreadEventData {
    #[serde(flatten)]
    pub acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties,
    #[doc = "The time at which the user was added to the thread"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub time: ::time::OffsetDateTime,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "addedByCommunicationIdentifier")]
    pub added_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "Schema of the chat thread participant"]
    #[serde(rename = "participantAdded")]
    pub participant_added: AcsChatThreadParticipantProperties,
    #[doc = "The version of the thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatParticipantAddedToThreadEventData {
    pub fn new(
        time: ::time::OffsetDateTime,
        added_by_communication_identifier: CommunicationIdentifierModel,
        participant_added: AcsChatThreadParticipantProperties,
    ) -> Self {
        Self {
            acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties::default(),
            time,
            added_by_communication_identifier,
            participant_added,
            version: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatParticipantAddedToThreadWithUser event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatParticipantAddedToThreadWithUserEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "The time at which the user was added to the thread"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub time: ::time::OffsetDateTime,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "addedByCommunicationIdentifier")]
    pub added_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "Schema of the chat thread participant"]
    #[serde(rename = "participantAdded")]
    pub participant_added: AcsChatThreadParticipantProperties,
}
impl AcsChatParticipantAddedToThreadWithUserEventData {
    pub fn new(
        acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
        time: ::time::OffsetDateTime,
        added_by_communication_identifier: CommunicationIdentifierModel,
        participant_added: AcsChatThreadParticipantProperties,
    ) -> Self {
        Self {
            acs_chat_thread_event_base_properties,
            time,
            added_by_communication_identifier,
            participant_added,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadParticipantRemoved event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatParticipantRemovedFromThreadEventData {
    #[serde(flatten)]
    pub acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties,
    #[doc = "The time at which the user was removed to the thread"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub time: ::time::OffsetDateTime,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "removedByCommunicationIdentifier")]
    pub removed_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "Schema of the chat thread participant"]
    #[serde(rename = "participantRemoved")]
    pub participant_removed: AcsChatThreadParticipantProperties,
    #[doc = "The version of the thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatParticipantRemovedFromThreadEventData {
    pub fn new(
        time: ::time::OffsetDateTime,
        removed_by_communication_identifier: CommunicationIdentifierModel,
        participant_removed: AcsChatThreadParticipantProperties,
    ) -> Self {
        Self {
            acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties::default(),
            time,
            removed_by_communication_identifier,
            participant_removed,
            version: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatParticipantRemovedFromThreadWithUser event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatParticipantRemovedFromThreadWithUserEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "The time at which the user was removed to the thread"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub time: ::time::OffsetDateTime,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "removedByCommunicationIdentifier")]
    pub removed_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "Schema of the chat thread participant"]
    #[serde(rename = "participantRemoved")]
    pub participant_removed: AcsChatThreadParticipantProperties,
}
impl AcsChatParticipantRemovedFromThreadWithUserEventData {
    pub fn new(
        acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
        time: ::time::OffsetDateTime,
        removed_by_communication_identifier: CommunicationIdentifierModel,
        participant_removed: AcsChatThreadParticipantProperties,
    ) -> Self {
        Self {
            acs_chat_thread_event_base_properties,
            time,
            removed_by_communication_identifier,
            participant_removed,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadCreatedEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "createdByCommunicationIdentifier")]
    pub created_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The thread properties"]
    pub properties: serde_json::Value,
    #[doc = "The thread metadata"]
    pub metadata: serde_json::Value,
    #[doc = "The list of properties of participants who are part of the thread"]
    pub participants: Vec<AcsChatThreadParticipantProperties>,
}
impl AcsChatThreadCreatedEventData {
    pub fn new(
        acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
        created_by_communication_identifier: CommunicationIdentifierModel,
        properties: serde_json::Value,
        metadata: serde_json::Value,
        participants: Vec<AcsChatThreadParticipantProperties>,
    ) -> Self {
        Self {
            acs_chat_thread_event_in_thread_base_properties,
            created_by_communication_identifier,
            properties,
            metadata,
            participants,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadCreatedWithUser event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadCreatedWithUserEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "createdByCommunicationIdentifier")]
    pub created_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The thread properties"]
    pub properties: serde_json::Value,
    #[doc = "The thread metadata"]
    pub metadata: serde_json::Value,
    #[doc = "The list of properties of participants who are part of the thread"]
    pub participants: Vec<AcsChatThreadParticipantProperties>,
}
impl AcsChatThreadCreatedWithUserEventData {
    pub fn new(
        acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
        created_by_communication_identifier: CommunicationIdentifierModel,
        properties: serde_json::Value,
        metadata: serde_json::Value,
        participants: Vec<AcsChatThreadParticipantProperties>,
    ) -> Self {
        Self {
            acs_chat_thread_event_base_properties,
            created_by_communication_identifier,
            properties,
            metadata,
            participants,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadDeletedEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "deletedByCommunicationIdentifier")]
    pub deleted_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The deletion time of the thread"]
    #[serde(rename = "deleteTime", with = "azure_core::date::rfc3339")]
    pub delete_time: ::time::OffsetDateTime,
}
impl AcsChatThreadDeletedEventData {
    pub fn new(
        acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
        deleted_by_communication_identifier: CommunicationIdentifierModel,
        delete_time: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            acs_chat_thread_event_in_thread_base_properties,
            deleted_by_communication_identifier,
            delete_time,
        }
    }
}
#[doc = "Schema of common properties of all chat thread events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadEventBaseProperties {
    #[serde(flatten)]
    pub acs_chat_event_base_properties: AcsChatEventBaseProperties,
    #[doc = "The original creation time of the thread"]
    #[serde(rename = "createTime", with = "azure_core::date::rfc3339")]
    pub create_time: ::time::OffsetDateTime,
    #[doc = "The version of the thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatThreadEventBaseProperties {
    pub fn new(acs_chat_event_base_properties: AcsChatEventBaseProperties, create_time: ::time::OffsetDateTime) -> Self {
        Self {
            acs_chat_event_base_properties,
            create_time,
            version: None,
        }
    }
}
#[doc = "Schema of common properties of all chat thread events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadEventInThreadBaseProperties {
    #[serde(flatten)]
    pub acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties,
    #[doc = "The original creation time of the thread"]
    #[serde(rename = "createTime", with = "azure_core::date::rfc3339")]
    pub create_time: ::time::OffsetDateTime,
    #[doc = "The version of the thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatThreadEventInThreadBaseProperties {
    pub fn new(create_time: ::time::OffsetDateTime) -> Self {
        Self {
            acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties::default(),
            create_time,
            version: None,
        }
    }
}
#[doc = "Schema of the chat thread participant"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadParticipantProperties {
    #[doc = "The name of the user"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "participantCommunicationIdentifier")]
    pub participant_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The metadata of the user"]
    pub metadata: serde_json::Value,
}
impl AcsChatThreadParticipantProperties {
    pub fn new(participant_communication_identifier: CommunicationIdentifierModel, metadata: serde_json::Value) -> Self {
        Self {
            display_name: None,
            participant_communication_identifier,
            metadata,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadPropertiesUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadPropertiesUpdatedEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "editedByCommunicationIdentifier")]
    pub edited_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The time at which the properties of the thread were updated"]
    #[serde(rename = "editTime", with = "azure_core::date::rfc3339")]
    pub edit_time: ::time::OffsetDateTime,
    #[doc = "The updated thread properties"]
    pub properties: serde_json::Value,
    #[doc = "The thread metadata"]
    pub metadata: serde_json::Value,
}
impl AcsChatThreadPropertiesUpdatedEventData {
    pub fn new(
        acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
        edited_by_communication_identifier: CommunicationIdentifierModel,
        edit_time: ::time::OffsetDateTime,
        properties: serde_json::Value,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            acs_chat_thread_event_in_thread_base_properties,
            edited_by_communication_identifier,
            edit_time,
            properties,
            metadata,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadPropertiesUpdatedPerUser event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadPropertiesUpdatedPerUserEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "editedByCommunicationIdentifier")]
    pub edited_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The time at which the properties of the thread were updated"]
    #[serde(rename = "editTime", with = "azure_core::date::rfc3339")]
    pub edit_time: ::time::OffsetDateTime,
    #[doc = "The thread metadata"]
    pub metadata: serde_json::Value,
    #[doc = "The updated thread properties"]
    pub properties: serde_json::Value,
}
impl AcsChatThreadPropertiesUpdatedPerUserEventData {
    pub fn new(
        acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
        edited_by_communication_identifier: CommunicationIdentifierModel,
        edit_time: ::time::OffsetDateTime,
        metadata: serde_json::Value,
        properties: serde_json::Value,
    ) -> Self {
        Self {
            acs_chat_thread_event_base_properties,
            edited_by_communication_identifier,
            edit_time,
            metadata,
            properties,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadWithUserDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatThreadWithUserDeletedEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "deletedByCommunicationIdentifier")]
    pub deleted_by_communication_identifier: CommunicationIdentifierModel,
    #[doc = "The deletion time of the thread"]
    #[serde(rename = "deleteTime", with = "azure_core::date::rfc3339")]
    pub delete_time: ::time::OffsetDateTime,
}
impl AcsChatThreadWithUserDeletedEventData {
    pub fn new(
        acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
        deleted_by_communication_identifier: CommunicationIdentifierModel,
        delete_time: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            acs_chat_thread_event_base_properties,
            deleted_by_communication_identifier,
            delete_time,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.EmailDeliveryReportReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsEmailDeliveryReportReceivedEventData {
    #[doc = "The Sender Email Address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[doc = "The recipient Email Address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[doc = "The Id of the email been sent"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "The status of the email. Any value other than Delivered is considered failed."]
    pub status: AcsEmailDeliveryReportStatus,
    #[doc = "Detailed information about the status if any"]
    #[serde(rename = "deliveryStatusDetails")]
    pub delivery_status_details: AcsEmailDeliveryReportStatusDetails,
    #[doc = "The time at which the email delivery report received timestamp"]
    #[serde(rename = "deliveryAttemptTimestamp", with = "azure_core::date::rfc3339")]
    pub delivery_attempt_timestamp: ::time::OffsetDateTime,
}
impl AcsEmailDeliveryReportReceivedEventData {
    pub fn new(
        status: AcsEmailDeliveryReportStatus,
        delivery_status_details: AcsEmailDeliveryReportStatusDetails,
        delivery_attempt_timestamp: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            sender: None,
            recipient: None,
            message_id: None,
            status,
            delivery_status_details,
            delivery_attempt_timestamp,
        }
    }
}
#[doc = "The status of the email. Any value other than Delivered is considered failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsEmailDeliveryReportStatus")]
pub enum AcsEmailDeliveryReportStatus {
    Bounced,
    Delivered,
    Failed,
    FilteredSpam,
    Quarantined,
    Suppressed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsEmailDeliveryReportStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsEmailDeliveryReportStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsEmailDeliveryReportStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bounced => serializer.serialize_unit_variant("AcsEmailDeliveryReportStatus", 0u32, "Bounced"),
            Self::Delivered => serializer.serialize_unit_variant("AcsEmailDeliveryReportStatus", 1u32, "Delivered"),
            Self::Failed => serializer.serialize_unit_variant("AcsEmailDeliveryReportStatus", 2u32, "Failed"),
            Self::FilteredSpam => serializer.serialize_unit_variant("AcsEmailDeliveryReportStatus", 3u32, "FilteredSpam"),
            Self::Quarantined => serializer.serialize_unit_variant("AcsEmailDeliveryReportStatus", 4u32, "Quarantined"),
            Self::Suppressed => serializer.serialize_unit_variant("AcsEmailDeliveryReportStatus", 5u32, "Suppressed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Detailed information about the status if any"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsEmailDeliveryReportStatusDetails {
    #[doc = "Detailed status message"]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}
impl AcsEmailDeliveryReportStatusDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.EmailEngagementTrackingReportReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsEmailEngagementTrackingReportReceivedEventData {
    #[doc = "The Sender Email Address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[doc = "The Recipient Email Address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[doc = "The Id of the email that has been sent"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "The time at which the user interacted with the email"]
    #[serde(rename = "userActionTimestamp", with = "azure_core::date::rfc3339")]
    pub user_action_timestamp: ::time::OffsetDateTime,
    #[doc = "The context of the type of engagement user had with email"]
    #[serde(rename = "engagementContext", default, skip_serializing_if = "Option::is_none")]
    pub engagement_context: Option<String>,
    #[doc = "The user agent interacting with the email"]
    #[serde(rename = "userAgent", default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[doc = "The type of engagement user have with email."]
    #[serde(rename = "engagementType")]
    pub engagement_type: AcsUserEngagement,
}
impl AcsEmailEngagementTrackingReportReceivedEventData {
    pub fn new(user_action_timestamp: ::time::OffsetDateTime, engagement_type: AcsUserEngagement) -> Self {
        Self {
            sender: None,
            recipient: None,
            message_id: None,
            user_action_timestamp,
            engagement_context: None,
            user_agent: None,
            engagement_type,
        }
    }
}
#[doc = "Custom Context of Incoming Call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsIncomingCallCustomContext {
    #[doc = "Sip Headers for incoming call"]
    #[serde(rename = "sipHeaders")]
    pub sip_headers: serde_json::Value,
    #[doc = "Voip Headers for incoming call"]
    #[serde(rename = "voipHeaders")]
    pub voip_headers: serde_json::Value,
}
impl AcsIncomingCallCustomContext {
    pub fn new(sip_headers: serde_json::Value, voip_headers: serde_json::Value) -> Self {
        Self { sip_headers, voip_headers }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Communication.IncomingCall event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsIncomingCallEventData {
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    pub to: CommunicationIdentifierModel,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    pub from: CommunicationIdentifierModel,
    #[doc = "The Id of the server call"]
    #[serde(rename = "serverCallId", default, skip_serializing_if = "Option::is_none")]
    pub server_call_id: Option<String>,
    #[doc = "Display name of caller."]
    #[serde(rename = "callerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub caller_display_name: Option<String>,
    #[doc = "Custom Context of Incoming Call"]
    #[serde(rename = "customContext")]
    pub custom_context: AcsIncomingCallCustomContext,
    #[doc = "Signed incoming call context."]
    #[serde(rename = "incomingCallContext", default, skip_serializing_if = "Option::is_none")]
    pub incoming_call_context: Option<String>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "onBehalfOfCallee", default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_callee: Option<CommunicationIdentifierModel>,
    #[doc = "CorrelationId (CallId)."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
}
impl AcsIncomingCallEventData {
    pub fn new(to: CommunicationIdentifierModel, from: CommunicationIdentifierModel, custom_context: AcsIncomingCallCustomContext) -> Self {
        Self {
            to,
            from,
            server_call_id: None,
            caller_display_name: None,
            custom_context,
            incoming_call_context: None,
            on_behalf_of_callee: None,
            correlation_id: None,
        }
    }
}
#[doc = "Interactive reply kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsInteractiveReplyKind")]
pub enum AcsInteractiveReplyKind {
    #[serde(rename = "buttonReply")]
    ButtonReply,
    #[serde(rename = "listReply")]
    ListReply,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsInteractiveReplyKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsInteractiveReplyKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsInteractiveReplyKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ButtonReply => serializer.serialize_unit_variant("AcsInteractiveReplyKind", 0u32, "buttonReply"),
            Self::ListReply => serializer.serialize_unit_variant("AcsInteractiveReplyKind", 1u32, "listReply"),
            Self::Unknown => serializer.serialize_unit_variant("AcsInteractiveReplyKind", 2u32, "unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Message Button Content"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsMessageButtonContent {
    #[doc = "The Text of the button"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The Payload of the button which was clicked by the user, setup by the business"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}
impl AcsMessageButtonContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Message Channel Event Error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsMessageChannelEventError {
    #[doc = "The channel error code"]
    #[serde(rename = "channelCode", default, skip_serializing_if = "Option::is_none")]
    pub channel_code: Option<String>,
    #[doc = "The channel error message"]
    #[serde(rename = "channelMessage", default, skip_serializing_if = "Option::is_none")]
    pub channel_message: Option<String>,
}
impl AcsMessageChannelEventError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Message channel kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsMessageChannelKind")]
pub enum AcsMessageChannelKind {
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsMessageChannelKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsMessageChannelKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsMessageChannelKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Whatsapp => serializer.serialize_unit_variant("AcsMessageChannelKind", 0u32, "whatsapp"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Message Context"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsMessageContext {
    #[doc = "The WhatsApp ID for the customer who replied to an inbound message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "The message ID for the sent message for an inbound reply"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl AcsMessageContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Message delivery status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsMessageDeliveryStatus")]
pub enum AcsMessageDeliveryStatus {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsMessageDeliveryStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsMessageDeliveryStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsMessageDeliveryStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Read => serializer.serialize_unit_variant("AcsMessageDeliveryStatus", 0u32, "read"),
            Self::Delivered => serializer.serialize_unit_variant("AcsMessageDeliveryStatus", 1u32, "delivered"),
            Self::Failed => serializer.serialize_unit_variant("AcsMessageDeliveryStatus", 2u32, "failed"),
            Self::Sent => serializer.serialize_unit_variant("AcsMessageDeliveryStatus", 3u32, "sent"),
            Self::Warning => serializer.serialize_unit_variant("AcsMessageDeliveryStatus", 4u32, "warning"),
            Self::Unknown => serializer.serialize_unit_variant("AcsMessageDeliveryStatus", 5u32, "unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.AdvancedMessageDeliveryStatusUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsMessageDeliveryStatusUpdatedEventData {
    #[serde(flatten)]
    pub acs_message_event_data: AcsMessageEventData,
    #[doc = "The message id"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "Message delivery status"]
    pub status: AcsMessageDeliveryStatus,
    #[doc = "Message channel kind"]
    #[serde(rename = "channelType")]
    pub channel_type: AcsMessageChannelKind,
}
impl AcsMessageDeliveryStatusUpdatedEventData {
    pub fn new(acs_message_event_data: AcsMessageEventData, status: AcsMessageDeliveryStatus, channel_type: AcsMessageChannelKind) -> Self {
        Self {
            acs_message_event_data,
            message_id: None,
            status,
            channel_type,
        }
    }
}
#[doc = "Schema of common properties of all chat thread events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsMessageEventData {
    #[doc = "The message sender"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "The message recipient"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[doc = "The time message was received"]
    #[serde(rename = "receivedTimeStamp", with = "azure_core::date::rfc3339")]
    pub received_time_stamp: ::time::OffsetDateTime,
    #[doc = "Message Channel Event Error"]
    pub error: AcsMessageChannelEventError,
}
impl AcsMessageEventData {
    pub fn new(received_time_stamp: ::time::OffsetDateTime, error: AcsMessageChannelEventError) -> Self {
        Self {
            from: None,
            to: None,
            received_time_stamp,
            error,
        }
    }
}
#[doc = "Message Interactive button reply content for a user to business message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsMessageInteractiveButtonReplyContent {
    #[doc = "The ID of the button"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The title of the button"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl AcsMessageInteractiveButtonReplyContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Message Interactive Content"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsMessageInteractiveContent {
    #[doc = "Interactive reply kind"]
    #[serde(rename = "type")]
    pub type_: AcsInteractiveReplyKind,
    #[doc = "Message Interactive button reply content for a user to business message"]
    #[serde(rename = "buttonReply")]
    pub button_reply: AcsMessageInteractiveButtonReplyContent,
    #[doc = "Message Interactive list reply content for a user to business message"]
    #[serde(rename = "listReply")]
    pub list_reply: AcsMessageInteractiveListReplyContent,
}
impl AcsMessageInteractiveContent {
    pub fn new(
        type_: AcsInteractiveReplyKind,
        button_reply: AcsMessageInteractiveButtonReplyContent,
        list_reply: AcsMessageInteractiveListReplyContent,
    ) -> Self {
        Self {
            type_,
            button_reply,
            list_reply,
        }
    }
}
#[doc = "Message Interactive list reply content for a user to business message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsMessageInteractiveListReplyContent {
    #[doc = "The ID of the selected list item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The title of the selected list item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The description of the selected row"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AcsMessageInteractiveListReplyContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Message Media Content"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsMessageMediaContent {
    #[doc = "The MIME type of the file this media represents"]
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[doc = "The media identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The filename of the underlying media file as specified when uploaded"]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "The caption for the media object, if supported and provided"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}
impl AcsMessageMediaContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.AdvancedMessageReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsMessageReceivedEventData {
    #[serde(flatten)]
    pub acs_message_event_data: AcsMessageEventData,
    #[doc = "The message content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "Message channel kind"]
    #[serde(rename = "channelType")]
    pub channel_type: AcsMessageChannelKind,
    #[doc = "Message Media Content"]
    pub media: AcsMessageMediaContent,
    #[doc = "Message Context"]
    pub context: AcsMessageContext,
    #[doc = "Message Button Content"]
    pub button: AcsMessageButtonContent,
    #[doc = "Message Interactive Content"]
    pub interactive: AcsMessageInteractiveContent,
}
impl AcsMessageReceivedEventData {
    pub fn new(
        acs_message_event_data: AcsMessageEventData,
        channel_type: AcsMessageChannelKind,
        media: AcsMessageMediaContent,
        context: AcsMessageContext,
        button: AcsMessageButtonContent,
        interactive: AcsMessageInteractiveContent,
    ) -> Self {
        Self {
            acs_message_event_data,
            content: None,
            channel_type,
            media,
            context,
            button,
            interactive,
        }
    }
}
#[doc = "Schema for all properties of  Recording Chunk Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRecordingChunkInfoProperties {
    #[doc = "The documentId of the recording chunk"]
    #[serde(rename = "documentId", default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[doc = "The index of the recording chunk"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[doc = "The reason for ending the recording chunk"]
    #[serde(rename = "endReason", default, skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<String>,
    #[doc = "The location of the metadata for this chunk"]
    #[serde(rename = "metadataLocation", default, skip_serializing_if = "Option::is_none")]
    pub metadata_location: Option<String>,
    #[doc = "The location of the content for this chunk"]
    #[serde(rename = "contentLocation", default, skip_serializing_if = "Option::is_none")]
    pub content_location: Option<String>,
    #[doc = "The location to delete all chunk storage"]
    #[serde(rename = "deleteLocation", default, skip_serializing_if = "Option::is_none")]
    pub delete_location: Option<String>,
}
impl AcsRecordingChunkInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RecordingFileStatusUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRecordingFileStatusUpdatedEventData {
    #[doc = "Schema for all properties of Recording Storage Information."]
    #[serde(rename = "recordingStorageInfo")]
    pub recording_storage_info: AcsRecordingStorageInfoProperties,
    #[doc = "The time at which the recording started"]
    #[serde(rename = "recordingStartTime", with = "azure_core::date::rfc3339")]
    pub recording_start_time: ::time::OffsetDateTime,
    #[doc = "The recording duration in milliseconds"]
    #[serde(rename = "recordingDurationMs", default, skip_serializing_if = "Option::is_none")]
    pub recording_duration_ms: Option<i64>,
    #[doc = "Recording content type"]
    #[serde(rename = "recordingContentType")]
    pub recording_content_type: RecordingContentType,
    #[doc = "Recording channel type"]
    #[serde(rename = "recordingChannelType")]
    pub recording_channel_type: RecordingChannelType,
    #[doc = "Recording format type"]
    #[serde(rename = "recordingFormatType")]
    pub recording_format_type: RecordingFormatType,
    #[doc = "The reason for ending recording session"]
    #[serde(rename = "sessionEndReason", default, skip_serializing_if = "Option::is_none")]
    pub session_end_reason: Option<String>,
}
impl AcsRecordingFileStatusUpdatedEventData {
    pub fn new(
        recording_storage_info: AcsRecordingStorageInfoProperties,
        recording_start_time: ::time::OffsetDateTime,
        recording_content_type: RecordingContentType,
        recording_channel_type: RecordingChannelType,
        recording_format_type: RecordingFormatType,
    ) -> Self {
        Self {
            recording_storage_info,
            recording_start_time,
            recording_duration_ms: None,
            recording_content_type,
            recording_channel_type,
            recording_format_type,
            session_end_reason: None,
        }
    }
}
#[doc = "Schema for all properties of Recording Storage Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRecordingStorageInfoProperties {
    #[doc = "List of details of recording chunks information"]
    #[serde(rename = "recordingChunks")]
    pub recording_chunks: Vec<AcsRecordingChunkInfoProperties>,
}
impl AcsRecordingStorageInfoProperties {
    pub fn new(recording_chunks: Vec<AcsRecordingChunkInfoProperties>) -> Self {
        Self { recording_chunks }
    }
}
#[doc = "Router Channel Configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterChannelConfiguration {
    #[doc = "Channel ID for Router Job"]
    #[serde(rename = "channelId", default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[doc = "Capacity Cost Per Job for Router Job"]
    #[serde(rename = "capacityCostPerJob", default, skip_serializing_if = "Option::is_none")]
    pub capacity_cost_per_job: Option<i32>,
    #[doc = "Max Number of Jobs for Router Job"]
    #[serde(rename = "maxNumberOfJobs", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_jobs: Option<i32>,
}
impl AcsRouterChannelConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Router Communication Error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterCommunicationError {
    #[doc = "Router Communication Error Code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Router Communication Error Message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Router Communication Error Target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Router Communication Error"]
    pub innererror: Box<AcsRouterCommunicationError>,
    #[doc = "List of Router Communication Errors"]
    pub details: Vec<AcsRouterCommunicationError>,
}
impl AcsRouterCommunicationError {
    pub fn new(innererror: Box<AcsRouterCommunicationError>, details: Vec<AcsRouterCommunicationError>) -> Self {
        Self {
            code: None,
            message: None,
            target: None,
            innererror,
            details,
        }
    }
}
#[doc = "Schema of common properties of all Router events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterEventData {
    #[doc = "Router Event Job ID"]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Router Event Channel Reference"]
    #[serde(rename = "channelReference", default, skip_serializing_if = "Option::is_none")]
    pub channel_reference: Option<String>,
    #[doc = "Router Event Channel ID"]
    #[serde(rename = "channelId", default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}
impl AcsRouterEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobCancelled event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobCancelledEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Note"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[doc = "Router Job Disposition Code"]
    #[serde(rename = "dispositionCode", default, skip_serializing_if = "Option::is_none")]
    pub disposition_code: Option<String>,
}
impl AcsRouterJobCancelledEventData {
    pub fn new(acs_router_job_event_data: AcsRouterJobEventData) -> Self {
        Self {
            acs_router_job_event_data,
            note: None,
            disposition_code: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobClassificationFailed event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobClassificationFailedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Classification Policy Id"]
    #[serde(rename = "classificationPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub classification_policy_id: Option<String>,
    #[doc = "Router Job Classification Failed Errors"]
    pub errors: Vec<AcsRouterCommunicationError>,
}
impl AcsRouterJobClassificationFailedEventData {
    pub fn new(acs_router_job_event_data: AcsRouterJobEventData, errors: Vec<AcsRouterCommunicationError>) -> Self {
        Self {
            acs_router_job_event_data,
            classification_policy_id: None,
            errors,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobClassified event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobClassifiedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Queue Details"]
    #[serde(rename = "queueDetails")]
    pub queue_details: AcsRouterQueueDetails,
    #[doc = "Router Job Classification Policy Id"]
    #[serde(rename = "classificationPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub classification_policy_id: Option<String>,
    #[doc = "Router Job Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Attached Worker Selector"]
    #[serde(rename = "attachedWorkerSelectors")]
    pub attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
}
impl AcsRouterJobClassifiedEventData {
    pub fn new(
        acs_router_job_event_data: AcsRouterJobEventData,
        queue_details: AcsRouterQueueDetails,
        attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
    ) -> Self {
        Self {
            acs_router_job_event_data,
            queue_details,
            classification_policy_id: None,
            priority: None,
            attached_worker_selectors,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobClosed event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobClosedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Closed Assignment Id"]
    #[serde(rename = "assignmentId", default, skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    #[doc = "Router Job Closed Worker Id"]
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[doc = "Router Job Closed Disposition Code"]
    #[serde(rename = "dispositionCode", default, skip_serializing_if = "Option::is_none")]
    pub disposition_code: Option<String>,
}
impl AcsRouterJobClosedEventData {
    pub fn new(acs_router_job_event_data: AcsRouterJobEventData) -> Self {
        Self {
            acs_router_job_event_data,
            assignment_id: None,
            worker_id: None,
            disposition_code: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobCompleted event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobCompletedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Completed Assignment Id"]
    #[serde(rename = "assignmentId", default, skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    #[doc = "Router Job Completed Worker Id"]
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}
impl AcsRouterJobCompletedEventData {
    pub fn new(acs_router_job_event_data: AcsRouterJobEventData) -> Self {
        Self {
            acs_router_job_event_data,
            assignment_id: None,
            worker_id: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobDeleted event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobDeletedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
}
impl AcsRouterJobDeletedEventData {
    pub fn new(acs_router_job_event_data: AcsRouterJobEventData) -> Self {
        Self { acs_router_job_event_data }
    }
}
#[doc = "Schema of common properties of all Router Job events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobEventData {
    #[serde(flatten)]
    pub acs_router_event_data: AcsRouterEventData,
    #[doc = "Router Job events Queue Id"]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[doc = "Router Job events Labels"]
    pub labels: serde_json::Value,
    #[doc = "Router Jobs events Tags"]
    pub tags: serde_json::Value,
}
impl AcsRouterJobEventData {
    pub fn new(labels: serde_json::Value, tags: serde_json::Value) -> Self {
        Self {
            acs_router_event_data: AcsRouterEventData::default(),
            queue_id: None,
            labels,
            tags,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobExceptionTriggered event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobExceptionTriggeredEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Exception Triggered Rule Key"]
    #[serde(rename = "ruleKey", default, skip_serializing_if = "Option::is_none")]
    pub rule_key: Option<String>,
    #[doc = "Router Job Exception Triggered Rule Id"]
    #[serde(rename = "exceptionRuleId", default, skip_serializing_if = "Option::is_none")]
    pub exception_rule_id: Option<String>,
}
impl AcsRouterJobExceptionTriggeredEventData {
    pub fn new(acs_router_job_event_data: AcsRouterJobEventData) -> Self {
        Self {
            acs_router_job_event_data,
            rule_key: None,
            exception_rule_id: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobQueued event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobQueuedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Queued Attached Worker Selector"]
    #[serde(rename = "attachedWorkerSelectors")]
    pub attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Queued Requested Worker Selector"]
    #[serde(rename = "requestedWorkerSelectors")]
    pub requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
}
impl AcsRouterJobQueuedEventData {
    pub fn new(
        acs_router_job_event_data: AcsRouterJobEventData,
        attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
        requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    ) -> Self {
        Self {
            acs_router_job_event_data,
            priority: None,
            attached_worker_selectors,
            requested_worker_selectors,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobReceived event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobReceivedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Acs Router Job Status"]
    #[serde(rename = "jobStatus")]
    pub job_status: AcsRouterJobStatus,
    #[doc = "Router Job Classification Policy Id"]
    #[serde(rename = "classificationPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub classification_policy_id: Option<String>,
    #[doc = "Router Job Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Received Requested Worker Selectors"]
    #[serde(rename = "requestedWorkerSelectors")]
    pub requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Received Scheduled Time in UTC"]
    #[serde(rename = "scheduledOn", with = "azure_core::date::rfc3339")]
    pub scheduled_on: ::time::OffsetDateTime,
    #[doc = "Unavailable For Matching for Router Job Received"]
    #[serde(rename = "unavailableForMatching")]
    pub unavailable_for_matching: bool,
}
impl AcsRouterJobReceivedEventData {
    pub fn new(
        acs_router_job_event_data: AcsRouterJobEventData,
        job_status: AcsRouterJobStatus,
        requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
        scheduled_on: ::time::OffsetDateTime,
        unavailable_for_matching: bool,
    ) -> Self {
        Self {
            acs_router_job_event_data,
            job_status,
            classification_policy_id: None,
            priority: None,
            requested_worker_selectors,
            scheduled_on,
            unavailable_for_matching,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobSchedulingFailed event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobSchedulingFailedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Scheduling Failed Attached Worker Selector Expired"]
    #[serde(rename = "expiredAttachedWorkerSelectors")]
    pub expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Scheduling Failed Requested Worker Selector Expired"]
    #[serde(rename = "expiredRequestedWorkerSelectors")]
    pub expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Scheduling Failed Scheduled Time in UTC"]
    #[serde(rename = "scheduledOn", with = "azure_core::date::rfc3339")]
    pub scheduled_on: ::time::OffsetDateTime,
    #[doc = "Router Job Scheduling Failed Reason"]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}
impl AcsRouterJobSchedulingFailedEventData {
    pub fn new(
        acs_router_job_event_data: AcsRouterJobEventData,
        expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
        expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
        scheduled_on: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            acs_router_job_event_data,
            priority: None,
            expired_attached_worker_selectors,
            expired_requested_worker_selectors,
            scheduled_on,
            failure_reason: None,
        }
    }
}
#[doc = "Acs Router Job Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsRouterJobStatus")]
pub enum AcsRouterJobStatus {
    PendingClassification,
    Queued,
    Assigned,
    Completed,
    Closed,
    Cancelled,
    ClassificationFailed,
    Created,
    PendingSchedule,
    Scheduled,
    ScheduleFailed,
    WaitingForActivation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsRouterJobStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsRouterJobStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsRouterJobStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PendingClassification => serializer.serialize_unit_variant("AcsRouterJobStatus", 0u32, "PendingClassification"),
            Self::Queued => serializer.serialize_unit_variant("AcsRouterJobStatus", 1u32, "Queued"),
            Self::Assigned => serializer.serialize_unit_variant("AcsRouterJobStatus", 2u32, "Assigned"),
            Self::Completed => serializer.serialize_unit_variant("AcsRouterJobStatus", 3u32, "Completed"),
            Self::Closed => serializer.serialize_unit_variant("AcsRouterJobStatus", 4u32, "Closed"),
            Self::Cancelled => serializer.serialize_unit_variant("AcsRouterJobStatus", 5u32, "Cancelled"),
            Self::ClassificationFailed => serializer.serialize_unit_variant("AcsRouterJobStatus", 6u32, "ClassificationFailed"),
            Self::Created => serializer.serialize_unit_variant("AcsRouterJobStatus", 7u32, "Created"),
            Self::PendingSchedule => serializer.serialize_unit_variant("AcsRouterJobStatus", 8u32, "PendingSchedule"),
            Self::Scheduled => serializer.serialize_unit_variant("AcsRouterJobStatus", 9u32, "Scheduled"),
            Self::ScheduleFailed => serializer.serialize_unit_variant("AcsRouterJobStatus", 10u32, "ScheduleFailed"),
            Self::WaitingForActivation => serializer.serialize_unit_variant("AcsRouterJobStatus", 11u32, "WaitingForActivation"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobUnassigned event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobUnassignedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Unassigned Assignment Id"]
    #[serde(rename = "assignmentId", default, skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    #[doc = "Router Job Unassigned Worker Id"]
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}
impl AcsRouterJobUnassignedEventData {
    pub fn new(acs_router_job_event_data: AcsRouterJobEventData) -> Self {
        Self {
            acs_router_job_event_data,
            assignment_id: None,
            worker_id: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobWaitingForActivation event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobWaitingForActivationEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Waiting For Activation Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Waiting For Activation Worker Selector Expired"]
    #[serde(rename = "expiredAttachedWorkerSelectors")]
    pub expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Waiting For Activation Requested Worker Selector Expired"]
    #[serde(rename = "expiredRequestedWorkerSelectors")]
    pub expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Waiting For Activation Scheduled Time in UTC"]
    #[serde(rename = "scheduledOn", with = "azure_core::date::rfc3339")]
    pub scheduled_on: ::time::OffsetDateTime,
    #[doc = "Router Job Waiting For Activation Unavailable For Matching"]
    #[serde(rename = "unavailableForMatching")]
    pub unavailable_for_matching: bool,
}
impl AcsRouterJobWaitingForActivationEventData {
    pub fn new(
        acs_router_job_event_data: AcsRouterJobEventData,
        expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
        expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
        scheduled_on: ::time::OffsetDateTime,
        unavailable_for_matching: bool,
    ) -> Self {
        Self {
            acs_router_job_event_data,
            priority: None,
            expired_attached_worker_selectors,
            expired_requested_worker_selectors,
            scheduled_on,
            unavailable_for_matching,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobWorkerSelectorsExpired event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobWorkerSelectorsExpiredEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Worker Selectors Expired Requested Worker Selectors"]
    #[serde(rename = "expiredRequestedWorkerSelectors")]
    pub expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Worker Selectors Expired Attached Worker Selectors"]
    #[serde(rename = "expiredAttachedWorkerSelectors")]
    pub expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
}
impl AcsRouterJobWorkerSelectorsExpiredEventData {
    pub fn new(
        acs_router_job_event_data: AcsRouterJobEventData,
        expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
        expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
    ) -> Self {
        Self {
            acs_router_job_event_data,
            expired_requested_worker_selectors,
            expired_attached_worker_selectors,
        }
    }
}
#[doc = "Router Job Worker Selector Label Operator"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsRouterLabelOperator")]
pub enum AcsRouterLabelOperator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterThanOrEqual,
    LessThanOrEqual,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsRouterLabelOperator {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsRouterLabelOperator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsRouterLabelOperator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Equal => serializer.serialize_unit_variant("AcsRouterLabelOperator", 0u32, "Equal"),
            Self::NotEqual => serializer.serialize_unit_variant("AcsRouterLabelOperator", 1u32, "NotEqual"),
            Self::Greater => serializer.serialize_unit_variant("AcsRouterLabelOperator", 2u32, "Greater"),
            Self::Less => serializer.serialize_unit_variant("AcsRouterLabelOperator", 3u32, "Less"),
            Self::GreaterThanOrEqual => serializer.serialize_unit_variant("AcsRouterLabelOperator", 4u32, "GreaterThanOrEqual"),
            Self::LessThanOrEqual => serializer.serialize_unit_variant("AcsRouterLabelOperator", 5u32, "LessThanOrEqual"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Router Queue Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterQueueDetails {
    #[doc = "Router Queue Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Router Queue Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Router Queue Labels"]
    pub labels: serde_json::Value,
}
impl AcsRouterQueueDetails {
    pub fn new(labels: serde_json::Value) -> Self {
        Self {
            id: None,
            name: None,
            labels,
        }
    }
}
#[doc = "Worker properties that can be updated"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsRouterUpdatedWorkerProperty")]
pub enum AcsRouterUpdatedWorkerProperty {
    AvailableForOffers,
    TotalCapacity,
    QueueAssignments,
    Labels,
    Tags,
    ChannelConfigurations,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsRouterUpdatedWorkerProperty {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsRouterUpdatedWorkerProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsRouterUpdatedWorkerProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AvailableForOffers => serializer.serialize_unit_variant("AcsRouterUpdatedWorkerProperty", 0u32, "AvailableForOffers"),
            Self::TotalCapacity => serializer.serialize_unit_variant("AcsRouterUpdatedWorkerProperty", 1u32, "TotalCapacity"),
            Self::QueueAssignments => serializer.serialize_unit_variant("AcsRouterUpdatedWorkerProperty", 2u32, "QueueAssignments"),
            Self::Labels => serializer.serialize_unit_variant("AcsRouterUpdatedWorkerProperty", 3u32, "Labels"),
            Self::Tags => serializer.serialize_unit_variant("AcsRouterUpdatedWorkerProperty", 4u32, "Tags"),
            Self::ChannelConfigurations => {
                serializer.serialize_unit_variant("AcsRouterUpdatedWorkerProperty", 5u32, "ChannelConfigurations")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerDeleted event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterWorkerDeletedEventData {
    #[serde(flatten)]
    pub acs_router_worker_event_data: AcsRouterWorkerEventData,
}
impl AcsRouterWorkerDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerDeregistered event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterWorkerDeregisteredEventData {
    #[doc = "Router Worker Deregistered Worker Id"]
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}
impl AcsRouterWorkerDeregisteredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of all Router Worker events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterWorkerEventData {
    #[serde(flatten)]
    pub acs_router_event_data: AcsRouterEventData,
    #[doc = "Router Worker events Worker Id"]
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}
impl AcsRouterWorkerEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerOfferAccepted event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterWorkerOfferAcceptedEventData {
    #[serde(flatten)]
    pub acs_router_worker_event_data: AcsRouterWorkerEventData,
    #[doc = "Router Worker Offer Accepted Queue Id"]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[doc = "Router Worker Offer Accepted Offer Id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Router Worker Offer Accepted Assignment Id"]
    #[serde(rename = "assignmentId", default, skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    #[doc = "Router Worker Offer Accepted Job Priority"]
    #[serde(rename = "jobPriority", default, skip_serializing_if = "Option::is_none")]
    pub job_priority: Option<i32>,
    #[doc = "Router Worker Offer Accepted Worker Labels"]
    #[serde(rename = "workerLabels")]
    pub worker_labels: serde_json::Value,
    #[doc = "Router Worker Offer Accepted Worker Tags"]
    #[serde(rename = "workerTags")]
    pub worker_tags: serde_json::Value,
    #[doc = "Router Worker Offer Accepted Job Labels"]
    #[serde(rename = "jobLabels")]
    pub job_labels: serde_json::Value,
    #[doc = "Router Worker Offer Accepted Job Tags"]
    #[serde(rename = "jobTags")]
    pub job_tags: serde_json::Value,
}
impl AcsRouterWorkerOfferAcceptedEventData {
    pub fn new(
        worker_labels: serde_json::Value,
        worker_tags: serde_json::Value,
        job_labels: serde_json::Value,
        job_tags: serde_json::Value,
    ) -> Self {
        Self {
            acs_router_worker_event_data: AcsRouterWorkerEventData::default(),
            queue_id: None,
            offer_id: None,
            assignment_id: None,
            job_priority: None,
            worker_labels,
            worker_tags,
            job_labels,
            job_tags,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerOfferDeclined event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterWorkerOfferDeclinedEventData {
    #[serde(flatten)]
    pub acs_router_worker_event_data: AcsRouterWorkerEventData,
    #[doc = "Router Worker Offer Declined Queue Id"]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[doc = "Router Worker Offer Declined Offer Id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
}
impl AcsRouterWorkerOfferDeclinedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerOfferExpired event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterWorkerOfferExpiredEventData {
    #[serde(flatten)]
    pub acs_router_worker_event_data: AcsRouterWorkerEventData,
    #[doc = "Router Worker Offer Expired Queue Id"]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[doc = "Router Worker Offer Expired Offer Id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
}
impl AcsRouterWorkerOfferExpiredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerOfferIssued event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterWorkerOfferIssuedEventData {
    #[serde(flatten)]
    pub acs_router_worker_event_data: AcsRouterWorkerEventData,
    #[doc = "Router Worker Offer Issued Queue Id"]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[doc = "Router Worker Offer Issued Offer Id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Router Worker Offer Issued Job Priority"]
    #[serde(rename = "jobPriority", default, skip_serializing_if = "Option::is_none")]
    pub job_priority: Option<i32>,
    #[doc = "Router Worker Offer Issued Worker Labels"]
    #[serde(rename = "workerLabels")]
    pub worker_labels: serde_json::Value,
    #[doc = "Router Worker Offer Issued Time in UTC"]
    #[serde(rename = "offeredOn", with = "azure_core::date::rfc3339")]
    pub offered_on: ::time::OffsetDateTime,
    #[doc = "Router Worker Offer Issued Expiration Time in UTC"]
    #[serde(rename = "expiresOn", with = "azure_core::date::rfc3339")]
    pub expires_on: ::time::OffsetDateTime,
    #[doc = "Router Worker Offer Issued Worker Tags"]
    #[serde(rename = "workerTags")]
    pub worker_tags: serde_json::Value,
    #[doc = "Router Worker Offer Issued Job Labels"]
    #[serde(rename = "jobLabels")]
    pub job_labels: serde_json::Value,
    #[doc = "Router Worker Offer Issued Job Tags"]
    #[serde(rename = "jobTags")]
    pub job_tags: serde_json::Value,
}
impl AcsRouterWorkerOfferIssuedEventData {
    pub fn new(
        worker_labels: serde_json::Value,
        offered_on: ::time::OffsetDateTime,
        expires_on: ::time::OffsetDateTime,
        worker_tags: serde_json::Value,
        job_labels: serde_json::Value,
        job_tags: serde_json::Value,
    ) -> Self {
        Self {
            acs_router_worker_event_data: AcsRouterWorkerEventData::default(),
            queue_id: None,
            offer_id: None,
            job_priority: None,
            worker_labels,
            offered_on,
            expires_on,
            worker_tags,
            job_labels,
            job_tags,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerOfferRevoked event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterWorkerOfferRevokedEventData {
    #[serde(flatten)]
    pub acs_router_worker_event_data: AcsRouterWorkerEventData,
    #[doc = "Router Worker Offer Revoked Queue Id"]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[doc = "Router Worker Offer Revoked Offer Id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
}
impl AcsRouterWorkerOfferRevokedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerRegistered event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterWorkerRegisteredEventData {
    #[doc = "Router Worker Registered Worker Id"]
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[doc = "Router Worker Registered Queue Info"]
    #[serde(rename = "queueAssignments")]
    pub queue_assignments: Vec<AcsRouterQueueDetails>,
    #[doc = "Router Worker Registered Channel Configuration"]
    #[serde(rename = "channelConfigurations")]
    pub channel_configurations: Vec<AcsRouterChannelConfiguration>,
    #[doc = "Router Worker Register Total Capacity"]
    #[serde(rename = "totalCapacity", default, skip_serializing_if = "Option::is_none")]
    pub total_capacity: Option<i32>,
    #[doc = "Router Worker Registered Labels"]
    pub labels: serde_json::Value,
    #[doc = "Router Worker Registered Tags"]
    pub tags: serde_json::Value,
}
impl AcsRouterWorkerRegisteredEventData {
    pub fn new(
        queue_assignments: Vec<AcsRouterQueueDetails>,
        channel_configurations: Vec<AcsRouterChannelConfiguration>,
        labels: serde_json::Value,
        tags: serde_json::Value,
    ) -> Self {
        Self {
            worker_id: None,
            queue_assignments,
            channel_configurations,
            total_capacity: None,
            labels,
            tags,
        }
    }
}
#[doc = "Router Job Worker Selector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterWorkerSelector {
    #[doc = "Router Job Worker Selector Key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Router Job Worker Selector Label Operator"]
    #[serde(rename = "labelOperator")]
    pub label_operator: AcsRouterLabelOperator,
    #[doc = "Router Job Worker Selector Value"]
    pub value: serde_json::Value,
    #[doc = "Router Job Worker Selector Time to Live in Seconds"]
    #[serde(rename = "ttlSeconds")]
    pub ttl_seconds: f64,
    #[doc = "Router Worker Selector State"]
    pub state: AcsRouterWorkerSelectorState,
    #[doc = "Router Job Worker Selector Expiration Time"]
    #[serde(rename = "expirationTime", with = "azure_core::date::rfc3339")]
    pub expiration_time: ::time::OffsetDateTime,
}
impl AcsRouterWorkerSelector {
    pub fn new(
        label_operator: AcsRouterLabelOperator,
        value: serde_json::Value,
        ttl_seconds: f64,
        state: AcsRouterWorkerSelectorState,
        expiration_time: ::time::OffsetDateTime,
    ) -> Self {
        Self {
            key: None,
            label_operator,
            value,
            ttl_seconds,
            state,
            expiration_time,
        }
    }
}
#[doc = "Router Worker Selector State"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsRouterWorkerSelectorState")]
pub enum AcsRouterWorkerSelectorState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsRouterWorkerSelectorState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsRouterWorkerSelectorState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsRouterWorkerSelectorState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("AcsRouterWorkerSelectorState", 0u32, "active"),
            Self::Expired => serializer.serialize_unit_variant("AcsRouterWorkerSelectorState", 1u32, "expired"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterWorkerUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterWorkerUpdatedEventData {
    #[doc = "Router Worker Updated Worker Id"]
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[doc = "Router Worker Updated Queue Info"]
    #[serde(rename = "queueAssignments")]
    pub queue_assignments: Vec<AcsRouterQueueDetails>,
    #[doc = "Router Worker Updated Channel Configuration"]
    #[serde(rename = "channelConfigurations")]
    pub channel_configurations: Vec<AcsRouterChannelConfiguration>,
    #[doc = "Router Worker Updated Total Capacity"]
    #[serde(rename = "totalCapacity", default, skip_serializing_if = "Option::is_none")]
    pub total_capacity: Option<i32>,
    #[doc = "Router Worker Updated Labels"]
    pub labels: serde_json::Value,
    #[doc = "Router Worker Updated Tags"]
    pub tags: serde_json::Value,
    #[doc = "Router Worker Properties Updated"]
    #[serde(rename = "updatedWorkerProperties")]
    pub updated_worker_properties: Vec<AcsRouterUpdatedWorkerProperty>,
}
impl AcsRouterWorkerUpdatedEventData {
    pub fn new(
        queue_assignments: Vec<AcsRouterQueueDetails>,
        channel_configurations: Vec<AcsRouterChannelConfiguration>,
        labels: serde_json::Value,
        tags: serde_json::Value,
        updated_worker_properties: Vec<AcsRouterUpdatedWorkerProperty>,
    ) -> Self {
        Self {
            worker_id: None,
            queue_assignments,
            channel_configurations,
            total_capacity: None,
            labels,
            tags,
            updated_worker_properties,
        }
    }
}
#[doc = "Schema for details of a delivery attempt"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsSmsDeliveryAttemptProperties {
    #[doc = "TimeStamp when delivery was attempted"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "Number of segments that were successfully delivered"]
    #[serde(rename = "segmentsSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub segments_succeeded: Option<i32>,
    #[doc = "Number of segments whose delivery failed"]
    #[serde(rename = "segmentsFailed", default, skip_serializing_if = "Option::is_none")]
    pub segments_failed: Option<i32>,
}
impl AcsSmsDeliveryAttemptProperties {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            segments_succeeded: None,
            segments_failed: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.SMSDeliveryReportReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsSmsDeliveryReportReceivedEventData {
    #[serde(flatten)]
    pub acs_sms_event_base_properties: AcsSmsEventBaseProperties,
    #[doc = "Status of Delivery"]
    #[serde(rename = "deliveryStatus", default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    #[doc = "Details about Delivery Status"]
    #[serde(rename = "deliveryStatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub delivery_status_details: Option<String>,
    #[doc = "List of details of delivery attempts made"]
    #[serde(rename = "deliveryAttempts")]
    pub delivery_attempts: Vec<AcsSmsDeliveryAttemptProperties>,
    #[doc = "The time at which the SMS delivery report was received"]
    #[serde(rename = "receivedTimestamp", with = "azure_core::date::rfc3339")]
    pub received_timestamp: ::time::OffsetDateTime,
    #[doc = "Customer Content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl AcsSmsDeliveryReportReceivedEventData {
    pub fn new(delivery_attempts: Vec<AcsSmsDeliveryAttemptProperties>, received_timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            acs_sms_event_base_properties: AcsSmsEventBaseProperties::default(),
            delivery_status: None,
            delivery_status_details: None,
            delivery_attempts,
            received_timestamp,
            tag: None,
        }
    }
}
#[doc = "Schema of common properties of all SMS events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsSmsEventBaseProperties {
    #[doc = "The identity of the SMS message"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "The identity of SMS message sender"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "The identity of SMS message receiver"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
impl AcsSmsEventBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.SMSReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsSmsReceivedEventData {
    #[serde(flatten)]
    pub acs_sms_event_base_properties: AcsSmsEventBaseProperties,
    #[doc = "The SMS content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The time at which the SMS was received"]
    #[serde(rename = "receivedTimestamp", with = "azure_core::date::rfc3339")]
    pub received_timestamp: ::time::OffsetDateTime,
}
impl AcsSmsReceivedEventData {
    pub fn new(received_timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            acs_sms_event_base_properties: AcsSmsEventBaseProperties::default(),
            message: None,
            received_timestamp,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Communication.UserDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsUserDisconnectedEventData {
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "userCommunicationIdentifier")]
    pub user_communication_identifier: CommunicationIdentifierModel,
}
impl AcsUserDisconnectedEventData {
    pub fn new(user_communication_identifier: CommunicationIdentifierModel) -> Self {
        Self {
            user_communication_identifier,
        }
    }
}
#[doc = "The type of engagement user have with email."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcsUserEngagement")]
pub enum AcsUserEngagement {
    #[serde(rename = "view")]
    View,
    #[serde(rename = "click")]
    Click,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcsUserEngagement {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcsUserEngagement {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcsUserEngagement {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::View => serializer.serialize_unit_variant("AcsUserEngagement", 0u32, "view"),
            Self::Click => serializer.serialize_unit_variant("AcsUserEngagement", 1u32, "click"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the data property of an EventGridEvent for a Microsoft.ApiCenter.ApiDefinitionAdded event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiCenterApiDefinitionAddedEventData {
    #[doc = "API definition title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "API definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "API specification details."]
    pub specification: ApiCenterApiSpecification,
}
impl ApiCenterApiDefinitionAddedEventData {
    pub fn new(specification: ApiCenterApiSpecification) -> Self {
        Self {
            title: None,
            description: None,
            specification,
        }
    }
}
#[doc = "Schema of the data property of an EventGridEvent for a Microsoft.ApiCenter.ApiDefinitionUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiCenterApiDefinitionUpdatedEventData {
    #[doc = "API definition title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "API definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "API specification details."]
    pub specification: ApiCenterApiSpecification,
}
impl ApiCenterApiDefinitionUpdatedEventData {
    pub fn new(specification: ApiCenterApiSpecification) -> Self {
        Self {
            title: None,
            description: None,
            specification,
        }
    }
}
#[doc = "API specification details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCenterApiSpecification {
    #[doc = "Specification name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specification version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ApiCenterApiSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.APICreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementApiCreatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementApiCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.APIDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementApiDeletedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementApiDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.APIReleaseCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementApiReleaseCreatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementApiReleaseCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.APIReleaseDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementApiReleaseDeletedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementApiReleaseDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.APIReleaseUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementApiReleaseUpdatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementApiReleaseUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.APIUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementApiUpdatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementApiUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayAPIAdded event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayApiAddedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<GatewayName>/apis/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayApiAddedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayAPIRemoved event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayApiRemovedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<GatewayName>/apis/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayApiRemovedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayCertificateAuthorityCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayCertificateAuthorityCreatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<GatewayName>/certificateAuthorities/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayCertificateAuthorityCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayCertificateAuthorityDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayCertificateAuthorityDeletedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<GatewayName>/certificateAuthorities/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayCertificateAuthorityDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayCertificateAuthorityUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayCertificateAuthorityUpdatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<GatewayName>/certificateAuthorities/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayCertificateAuthorityUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayCreatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayDeletedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayHostnameConfigurationCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayHostnameConfigurationCreatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<GatewayName>/hostnameConfigurations/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayHostnameConfigurationCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayHostnameConfigurationDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayHostnameConfigurationDeletedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<GatewayName>/hostnameConfigurations/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayHostnameConfigurationDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayHostnameConfigurationUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayHostnameConfigurationUpdatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<GatewayName>/hostnameConfigurations/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayHostnameConfigurationUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.GatewayUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementGatewayUpdatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/gateways/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementGatewayUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.ProductCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementProductCreatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementProductCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.ProductDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementProductDeletedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementProductDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.ProductUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementProductUpdatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementProductUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.SubscriptionCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSubscriptionCreatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementSubscriptionCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.SubscriptionDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSubscriptionDeletedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementSubscriptionDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.SubscriptionUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSubscriptionUpdatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementSubscriptionUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.UserCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementUserCreatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementUserCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.UserDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementUserDeletedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementUserDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ApiManagement.UserUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementUserUpdatedEventData {
    #[doc = "The fully qualified ID of the resource that the compliance state change is for, including the resource name and resource type. Uses the format, `/subscriptions/<SubscriptionID>/resourceGroups/<ResourceGroup>/Microsoft.ApiManagement/service/<ServiceName>/<ResourceType>/<ResourceName>`"]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}
impl ApiManagementUserUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of action of the operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AppAction")]
pub enum AppAction {
    Restarted,
    Stopped,
    ChangedAppSettings,
    Started,
    Completed,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AppAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AppAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AppAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Restarted => serializer.serialize_unit_variant("AppAction", 0u32, "Restarted"),
            Self::Stopped => serializer.serialize_unit_variant("AppAction", 1u32, "Stopped"),
            Self::ChangedAppSettings => serializer.serialize_unit_variant("AppAction", 2u32, "ChangedAppSettings"),
            Self::Started => serializer.serialize_unit_variant("AppAction", 3u32, "Started"),
            Self::Completed => serializer.serialize_unit_variant("AppAction", 4u32, "Completed"),
            Self::Failed => serializer.serialize_unit_variant("AppAction", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AppConfiguration.KeyValueDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfigurationKeyValueDeletedEventData {
    #[doc = "The key used to identify the key-value that was deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The label, if any, used to identify the key-value that was deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The etag representing the key-value that was deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The sync token representing the server state after the event."]
    #[serde(rename = "syncToken", default, skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
}
impl AppConfigurationKeyValueDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AppConfiguration.KeyValueModified event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfigurationKeyValueModifiedEventData {
    #[doc = "The key used to identify the key-value that was modified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The label, if any, used to identify the key-value that was modified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The etag representing the new state of the key-value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The sync token representing the server state after the event."]
    #[serde(rename = "syncToken", default, skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
}
impl AppConfigurationKeyValueModifiedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AppConfiguration.SnapshotCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfigurationSnapshotCreatedEventData {
    #[serde(flatten)]
    pub app_configuration_snapshot_event_data: AppConfigurationSnapshotEventData,
}
impl AppConfigurationSnapshotCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of snapshot events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfigurationSnapshotEventData {
    #[doc = "The name of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The etag representing the new state of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The sync token representing the server state after the event."]
    #[serde(rename = "syncToken", default, skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
}
impl AppConfigurationSnapshotEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AppConfiguration.SnapshotModified event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfigurationSnapshotModifiedEventData {
    #[serde(flatten)]
    pub app_configuration_snapshot_event_data: AppConfigurationSnapshotEventData,
}
impl AppConfigurationSnapshotModifiedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detail of action on the app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppEventTypeDetail {
    #[doc = "Type of action of the operation"]
    pub action: AppAction,
}
impl AppEventTypeDetail {
    pub fn new(action: AppAction) -> Self {
        Self { action }
    }
}
#[doc = "Type of action on the app service plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AppServicePlanAction")]
pub enum AppServicePlanAction {
    Updated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AppServicePlanAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AppServicePlanAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AppServicePlanAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Updated => serializer.serialize_unit_variant("AppServicePlanAction", 0u32, "Updated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Detail of action on the app service plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServicePlanEventTypeDetail {
    #[doc = "Kind of environment where app service plan is."]
    #[serde(rename = "stampKind")]
    pub stamp_kind: StampKind,
    #[doc = "Type of action on the app service plan."]
    pub action: AppServicePlanAction,
    #[doc = "Asynchronous operation status of the operation on the app service plan."]
    pub status: AsyncStatus,
}
impl AppServicePlanEventTypeDetail {
    pub fn new(stamp_kind: StampKind, action: AppServicePlanAction, status: AsyncStatus) -> Self {
        Self {
            stamp_kind,
            action,
            status,
        }
    }
}
#[doc = "Asynchronous operation status of the operation on the app service plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AsyncStatus")]
pub enum AsyncStatus {
    Started,
    Completed,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AsyncStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AsyncStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AsyncStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Started => serializer.serialize_unit_variant("AsyncStatus", 0u32, "Started"),
            Self::Completed => serializer.serialize_unit_variant("AsyncStatus", 1u32, "Completed"),
            Self::Failed => serializer.serialize_unit_variant("AsyncStatus", 2u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ClusterCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsClusterCreatedEventData {
    #[serde(flatten)]
    pub avs_cluster_event_data: AvsClusterEventData,
}
impl AvsClusterCreatedEventData {
    pub fn new(avs_cluster_event_data: AvsClusterEventData) -> Self {
        Self { avs_cluster_event_data }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ClusterDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsClusterDeletedEventData {
    #[serde(flatten)]
    pub avs_cluster_event_data: AvsClusterEventData,
}
impl AvsClusterDeletedEventData {
    pub fn new(avs_cluster_event_data: AvsClusterEventData) -> Self {
        Self { avs_cluster_event_data }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for Microsoft.AVS/clusters events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsClusterEventData {
    #[doc = "Id of the operation that caused this event."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[doc = "Hosts added to the cluster in this event, if any."]
    #[serde(
        rename = "addedHostNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub added_host_names: Vec<String>,
    #[doc = "Hosts removed from the cluster in this event, if any."]
    #[serde(
        rename = "removedHostNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub removed_host_names: Vec<String>,
    #[doc = "Hosts in Maintenance mode in the cluster, if any."]
    #[serde(
        rename = "inMaintenanceHostNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub in_maintenance_host_names: Vec<String>,
}
impl AvsClusterEventData {
    pub fn new(operation_id: String) -> Self {
        Self {
            operation_id,
            added_host_names: Vec::new(),
            removed_host_names: Vec::new(),
            in_maintenance_host_names: Vec::new(),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ClusterFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsClusterFailedEventData {
    #[serde(flatten)]
    pub avs_cluster_event_data: AvsClusterEventData,
    #[doc = "Failure reason of an event."]
    #[serde(rename = "failureMessage", default, skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}
impl AvsClusterFailedEventData {
    pub fn new(avs_cluster_event_data: AvsClusterEventData) -> Self {
        Self {
            avs_cluster_event_data,
            failure_message: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ClusterUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsClusterUpdatedEventData {
    #[serde(flatten)]
    pub avs_cluster_event_data: AvsClusterEventData,
}
impl AvsClusterUpdatedEventData {
    pub fn new(avs_cluster_event_data: AvsClusterEventData) -> Self {
        Self { avs_cluster_event_data }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ClusterUpdating event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsClusterUpdatingEventData {
    #[serde(flatten)]
    pub avs_cluster_event_data: AvsClusterEventData,
}
impl AvsClusterUpdatingEventData {
    pub fn new(avs_cluster_event_data: AvsClusterEventData) -> Self {
        Self { avs_cluster_event_data }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for Microsoft.AVS/privateClouds events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsPrivateCloudEventData {
    #[doc = "Id of the operation that caused this event."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
impl AvsPrivateCloudEventData {
    pub fn new(operation_id: String) -> Self {
        Self { operation_id }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.PrivateCloudFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsPrivateCloudFailedEventData {
    #[serde(flatten)]
    pub avs_private_cloud_event_data: AvsPrivateCloudEventData,
    #[doc = "Failure reason of an event."]
    #[serde(rename = "failureMessage", default, skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}
impl AvsPrivateCloudFailedEventData {
    pub fn new(avs_private_cloud_event_data: AvsPrivateCloudEventData) -> Self {
        Self {
            avs_private_cloud_event_data,
            failure_message: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.PrivateCloudUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsPrivateCloudUpdatedEventData {
    #[serde(flatten)]
    pub avs_private_cloud_event_data: AvsPrivateCloudEventData,
}
impl AvsPrivateCloudUpdatedEventData {
    pub fn new(avs_private_cloud_event_data: AvsPrivateCloudEventData) -> Self {
        Self {
            avs_private_cloud_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.PrivateCloudUpdating event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsPrivateCloudUpdatingEventData {
    #[serde(flatten)]
    pub avs_private_cloud_event_data: AvsPrivateCloudEventData,
}
impl AvsPrivateCloudUpdatingEventData {
    pub fn new(avs_private_cloud_event_data: AvsPrivateCloudEventData) -> Self {
        Self {
            avs_private_cloud_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ScriptExecutionCancelled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsScriptExecutionCancelledEventData {
    #[serde(flatten)]
    pub avs_script_execution_event_data: AvsScriptExecutionEventData,
}
impl AvsScriptExecutionCancelledEventData {
    pub fn new(avs_script_execution_event_data: AvsScriptExecutionEventData) -> Self {
        Self {
            avs_script_execution_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for Microsoft.AVS/scriptExecutions events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsScriptExecutionEventData {
    #[doc = "Id of the operation that caused this event."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[doc = "Cmdlet referenced in the execution that caused this event."]
    #[serde(rename = "cmdletId")]
    pub cmdlet_id: String,
    #[doc = "Stdout outputs from the execution, if any."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output: Vec<String>,
}
impl AvsScriptExecutionEventData {
    pub fn new(operation_id: String, cmdlet_id: String) -> Self {
        Self {
            operation_id,
            cmdlet_id,
            output: Vec::new(),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ScriptExecutionFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsScriptExecutionFailedEventData {
    #[serde(flatten)]
    pub avs_script_execution_event_data: AvsScriptExecutionEventData,
    #[doc = "Failure reason of an event."]
    #[serde(rename = "failureMessage", default, skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}
impl AvsScriptExecutionFailedEventData {
    pub fn new(avs_script_execution_event_data: AvsScriptExecutionEventData) -> Self {
        Self {
            avs_script_execution_event_data,
            failure_message: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ScriptExecutionFinished event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsScriptExecutionFinishedEventData {
    #[serde(flatten)]
    pub avs_script_execution_event_data: AvsScriptExecutionEventData,
    #[doc = "Named outputs of completed execution, if any."]
    #[serde(rename = "namedOutputs")]
    pub named_outputs: serde_json::Value,
}
impl AvsScriptExecutionFinishedEventData {
    pub fn new(avs_script_execution_event_data: AvsScriptExecutionEventData, named_outputs: serde_json::Value) -> Self {
        Self {
            avs_script_execution_event_data,
            named_outputs,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.AVS.ScriptExecutionStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvsScriptExecutionStartedEventData {
    #[serde(flatten)]
    pub avs_script_execution_event_data: AvsScriptExecutionEventData,
}
impl AvsScriptExecutionStartedEventData {
    pub fn new(avs_script_execution_event_data: AvsScriptExecutionEventData) -> Self {
        Self {
            avs_script_execution_event_data,
        }
    }
}
#[doc = "Properties of an event published to an Event Grid topic using the CloudEvent 1.0 Schema"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEventEvent {
    #[doc = "An identifier for the event. The combination of id and source must be unique for each distinct event."]
    pub id: String,
    #[doc = "Identifies the context in which an event happened. The combination of id and source must be unique for each distinct event."]
    pub source: String,
    #[doc = "Event data specific to the event type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[doc = "Event data specific to the event type, encoded as a base64 string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_base64: Option<String>,
    #[doc = "Type of event related to the originating occurrence."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The time (in UTC) the event was generated, in RFC3339 format."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<::time::OffsetDateTime>,
    #[doc = "The version of the CloudEvents specification which the event uses."]
    pub specversion: String,
    #[doc = "Identifies the schema that data adheres to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataschema: Option<String>,
    #[doc = "Content type of data value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datacontenttype: Option<String>,
    #[doc = "This describes the subject of the event in the context of the event producer (identified by source)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}
impl CloudEventEvent {
    pub fn new(id: String, source: String, type_: String, specversion: String) -> Self {
        Self {
            id,
            source,
            data: None,
            data_base64: None,
            type_,
            time: None,
            specversion,
            dataschema: None,
            datacontenttype: None,
            subject: None,
        }
    }
}
#[doc = "Communication cloud environment model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CommunicationCloudEnvironmentModel")]
pub enum CommunicationCloudEnvironmentModel {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "dod")]
    Dod,
    #[serde(rename = "gcch")]
    Gcch,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CommunicationCloudEnvironmentModel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CommunicationCloudEnvironmentModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CommunicationCloudEnvironmentModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Public => serializer.serialize_unit_variant("CommunicationCloudEnvironmentModel", 0u32, "public"),
            Self::Dod => serializer.serialize_unit_variant("CommunicationCloudEnvironmentModel", 1u32, "dod"),
            Self::Gcch => serializer.serialize_unit_variant("CommunicationCloudEnvironmentModel", 2u32, "gcch"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationIdentifierModel {
    #[doc = "Communication model identifier kind"]
    pub kind: CommunicationIdentifierModelKind,
    #[doc = "Raw Id of the identifier. Optional in requests, required in responses."]
    #[serde(rename = "rawId", default, skip_serializing_if = "Option::is_none")]
    pub raw_id: Option<String>,
    #[doc = "A user that got created with an Azure Communication Services resource."]
    #[serde(rename = "communicationUser")]
    pub communication_user: CommunicationUserIdentifierModel,
    #[doc = "A phone number."]
    #[serde(rename = "phoneNumber")]
    pub phone_number: PhoneNumberIdentifierModel,
    #[doc = "A Microsoft Teams user."]
    #[serde(rename = "microsoftTeamsUser")]
    pub microsoft_teams_user: MicrosoftTeamsUserIdentifierModel,
    #[doc = "A Microsoft Teams application."]
    #[serde(rename = "microsoftTeamsApp")]
    pub microsoft_teams_app: MicrosoftTeamsAppIdentifierModel,
}
impl CommunicationIdentifierModel {
    pub fn new(
        kind: CommunicationIdentifierModelKind,
        communication_user: CommunicationUserIdentifierModel,
        phone_number: PhoneNumberIdentifierModel,
        microsoft_teams_user: MicrosoftTeamsUserIdentifierModel,
        microsoft_teams_app: MicrosoftTeamsAppIdentifierModel,
    ) -> Self {
        Self {
            kind,
            raw_id: None,
            communication_user,
            phone_number,
            microsoft_teams_user,
            microsoft_teams_app,
        }
    }
}
#[doc = "Communication model identifier kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CommunicationIdentifierModelKind")]
pub enum CommunicationIdentifierModelKind {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "communicationUser")]
    CommunicationUser,
    #[serde(rename = "phoneNumber")]
    PhoneNumber,
    #[serde(rename = "microsoftTeamsUser")]
    MicrosoftTeamsUser,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CommunicationIdentifierModelKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CommunicationIdentifierModelKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CommunicationIdentifierModelKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("CommunicationIdentifierModelKind", 0u32, "unknown"),
            Self::CommunicationUser => serializer.serialize_unit_variant("CommunicationIdentifierModelKind", 1u32, "communicationUser"),
            Self::PhoneNumber => serializer.serialize_unit_variant("CommunicationIdentifierModelKind", 2u32, "phoneNumber"),
            Self::MicrosoftTeamsUser => serializer.serialize_unit_variant("CommunicationIdentifierModelKind", 3u32, "microsoftTeamsUser"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A user that got created with an Azure Communication Services resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationUserIdentifierModel {
    #[doc = "The Id of the communication user."]
    pub id: String,
}
impl CommunicationUserIdentifierModel {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The content of the event request message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistryArtifactEventData {
    #[doc = "The event ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The time at which the event occurred."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The action that encompasses the provided event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The location of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The target of the event."]
    pub target: ContainerRegistryArtifactEventTarget,
    #[doc = "The connected registry information if the event is generated by a connected registry."]
    #[serde(rename = "connectedRegistry")]
    pub connected_registry: ContainerRegistryEventConnectedRegistry,
}
impl ContainerRegistryArtifactEventData {
    pub fn new(
        timestamp: ::time::OffsetDateTime,
        target: ContainerRegistryArtifactEventTarget,
        connected_registry: ContainerRegistryEventConnectedRegistry,
    ) -> Self {
        Self {
            id: None,
            timestamp,
            action: None,
            location: None,
            target,
            connected_registry,
        }
    }
}
#[doc = "The target of the event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryArtifactEventTarget {
    #[doc = "The MIME type of the artifact."]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "The size in bytes of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The digest of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "The repository name of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The tag of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The name of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ContainerRegistryArtifactEventTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerRegistry.ChartDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistryChartDeletedEventData {
    #[serde(flatten)]
    pub container_registry_artifact_event_data: ContainerRegistryArtifactEventData,
}
impl ContainerRegistryChartDeletedEventData {
    pub fn new(container_registry_artifact_event_data: ContainerRegistryArtifactEventData) -> Self {
        Self {
            container_registry_artifact_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerRegistry.ChartPushed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistryChartPushedEventData {
    #[serde(flatten)]
    pub container_registry_artifact_event_data: ContainerRegistryArtifactEventData,
}
impl ContainerRegistryChartPushedEventData {
    pub fn new(container_registry_artifact_event_data: ContainerRegistryArtifactEventData) -> Self {
        Self {
            container_registry_artifact_event_data,
        }
    }
}
#[doc = "The agent that initiated the event. For most situations, this could be from the authorization context of the request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryEventActor {
    #[doc = "The subject or username associated with the request context that generated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ContainerRegistryEventActor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connected registry information if the event is generated by a connected registry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryEventConnectedRegistry {
    #[doc = "The name of the connected registry that generated this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ContainerRegistryEventConnectedRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The content of the event request message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistryEventData {
    #[doc = "The event ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The time at which the event occurred."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The action that encompasses the provided event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The location of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The target of the event."]
    pub target: ContainerRegistryEventTarget,
    #[doc = "The request that generated the event."]
    pub request: ContainerRegistryEventRequest,
    #[doc = "The agent that initiated the event. For most situations, this could be from the authorization context of the request."]
    pub actor: ContainerRegistryEventActor,
    #[doc = "The registry node that generated the event. Put differently, while the actor initiates the event, the source generates it."]
    pub source: ContainerRegistryEventSource,
    #[doc = "The connected registry information if the event is generated by a connected registry."]
    #[serde(rename = "connectedRegistry")]
    pub connected_registry: ContainerRegistryEventConnectedRegistry,
}
impl ContainerRegistryEventData {
    pub fn new(
        timestamp: ::time::OffsetDateTime,
        target: ContainerRegistryEventTarget,
        request: ContainerRegistryEventRequest,
        actor: ContainerRegistryEventActor,
        source: ContainerRegistryEventSource,
        connected_registry: ContainerRegistryEventConnectedRegistry,
    ) -> Self {
        Self {
            id: None,
            timestamp,
            action: None,
            location: None,
            target,
            request,
            actor,
            source,
            connected_registry,
        }
    }
}
#[doc = "The request that generated the event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryEventRequest {
    #[doc = "The ID of the request that initiated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The IP or hostname and possibly port of the client connection that initiated the event. This is the RemoteAddr from the standard http request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[doc = "The externally accessible hostname of the registry instance, as specified by the http host header on incoming requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The request method that generated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "The user agent header of the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useragent: Option<String>,
}
impl ContainerRegistryEventRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The registry node that generated the event. Put differently, while the actor initiates the event, the source generates it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryEventSource {
    #[doc = "The IP or hostname and the port of the registry node that generated the event. Generally, this will be resolved by os.Hostname() along with the running port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[doc = "The running instance of an application. Changes after each restart."]
    #[serde(rename = "instanceID", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}
impl ContainerRegistryEventSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The target of the event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryEventTarget {
    #[doc = "The MIME type of the referenced object."]
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "The number of bytes of the content. Same as Length field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The digest of the content, as defined by the Registry V2 HTTP API Specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[doc = "The number of bytes of the content. Same as Size field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[doc = "The repository name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The direct URL to the content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The tag name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl ContainerRegistryEventTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerRegistry.ImageDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistryImageDeletedEventData {
    #[serde(flatten)]
    pub container_registry_event_data: ContainerRegistryEventData,
}
impl ContainerRegistryImageDeletedEventData {
    pub fn new(container_registry_event_data: ContainerRegistryEventData) -> Self {
        Self {
            container_registry_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerRegistry.ImagePushed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerRegistryImagePushedEventData {
    #[serde(flatten)]
    pub container_registry_event_data: ContainerRegistryEventData,
}
impl ContainerRegistryImagePushedEventData {
    pub fn new(container_registry_event_data: ContainerRegistryEventData) -> Self {
        Self {
            container_registry_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerService.ClusterSupportEnded event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceClusterSupportEndedEventData {
    #[serde(flatten)]
    pub container_service_cluster_support_event_data: ContainerServiceClusterSupportEventData,
}
impl ContainerServiceClusterSupportEndedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerService.ClusterSupportEnding event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceClusterSupportEndingEventData {
    #[serde(flatten)]
    pub container_service_cluster_support_event_data: ContainerServiceClusterSupportEventData,
}
impl ContainerServiceClusterSupportEndingEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of cluster support events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceClusterSupportEventData {
    #[doc = "The Kubernetes version of the ManagedCluster resource"]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
}
impl ContainerServiceClusterSupportEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerService.NewKubernetesVersionAvailable event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceNewKubernetesVersionAvailableEventData {
    #[doc = "The highest PATCH Kubernetes version for the highest MINOR version supported by ManagedCluster resource"]
    #[serde(rename = "latestSupportedKubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_supported_kubernetes_version: Option<String>,
    #[doc = "The highest PATCH Kubernetes version for the MINOR version considered stable for the ManagedCluster resource"]
    #[serde(rename = "latestStableKubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_stable_kubernetes_version: Option<String>,
    #[doc = "The highest PATCH Kubernetes version for the lowest applicable MINOR version available for the ManagedCluster resource"]
    #[serde(rename = "lowestMinorKubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub lowest_minor_kubernetes_version: Option<String>,
    #[doc = "The highest PATCH Kubernetes version considered preview for the ManagedCluster resource. There might not be any version in preview at the time of publishing the event"]
    #[serde(rename = "latestPreviewKubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_preview_kubernetes_version: Option<String>,
}
impl ContainerServiceNewKubernetesVersionAvailableEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of node pool rolling events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceNodePoolRollingEventData {
    #[doc = "The name of the node pool in the ManagedCluster resource"]
    #[serde(rename = "nodePoolName", default, skip_serializing_if = "Option::is_none")]
    pub node_pool_name: Option<String>,
}
impl ContainerServiceNodePoolRollingEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerService.NodePoolRollingFailed event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceNodePoolRollingFailedEventData {
    #[serde(flatten)]
    pub container_service_node_pool_rolling_event_data: ContainerServiceNodePoolRollingEventData,
}
impl ContainerServiceNodePoolRollingFailedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerService.NodePoolRollingStarted event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceNodePoolRollingStartedEventData {
    #[serde(flatten)]
    pub container_service_node_pool_rolling_event_data: ContainerServiceNodePoolRollingEventData,
}
impl ContainerServiceNodePoolRollingStartedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerService.NodePoolRollingSucceeded event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceNodePoolRollingSucceededEventData {
    #[serde(flatten)]
    pub container_service_node_pool_rolling_event_data: ContainerServiceNodePoolRollingEventData,
}
impl ContainerServiceNodePoolRollingSucceededEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an event published to an Event Grid topic using a custom schema"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomEventEvent {}
impl CustomEventEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.DataBox.CopyCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxCopyCompletedEventData {
    #[doc = "Serial Number of the device associated with the event. The list is comma separated if more than one serial number is associated."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Schema of DataBox Stage Name enumeration."]
    #[serde(rename = "stageName")]
    pub stage_name: DataBoxStageName,
    #[doc = "The time at which the stage happened."]
    #[serde(rename = "stageTime", with = "azure_core::date::rfc3339")]
    pub stage_time: ::time::OffsetDateTime,
}
impl DataBoxCopyCompletedEventData {
    pub fn new(stage_name: DataBoxStageName, stage_time: ::time::OffsetDateTime) -> Self {
        Self {
            serial_number: None,
            stage_name,
            stage_time,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.DataBox.CopyStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxCopyStartedEventData {
    #[doc = "Serial Number of the device associated with the event. The list is comma separated if more than one serial number is associated."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Schema of DataBox Stage Name enumeration."]
    #[serde(rename = "stageName")]
    pub stage_name: DataBoxStageName,
    #[doc = "The time at which the stage happened."]
    #[serde(rename = "stageTime", with = "azure_core::date::rfc3339")]
    pub stage_time: ::time::OffsetDateTime,
}
impl DataBoxCopyStartedEventData {
    pub fn new(stage_name: DataBoxStageName, stage_time: ::time::OffsetDateTime) -> Self {
        Self {
            serial_number: None,
            stage_name,
            stage_time,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.DataBox.OrderCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxOrderCompletedEventData {
    #[doc = "Serial Number of the device associated with the event. The list is comma separated if more than one serial number is associated."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Schema of DataBox Stage Name enumeration."]
    #[serde(rename = "stageName")]
    pub stage_name: DataBoxStageName,
    #[doc = "The time at which the stage happened."]
    #[serde(rename = "stageTime", with = "azure_core::date::rfc3339")]
    pub stage_time: ::time::OffsetDateTime,
}
impl DataBoxOrderCompletedEventData {
    pub fn new(stage_name: DataBoxStageName, stage_time: ::time::OffsetDateTime) -> Self {
        Self {
            serial_number: None,
            stage_name,
            stage_time,
        }
    }
}
#[doc = "Schema of DataBox Stage Name enumeration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataBoxStageName")]
pub enum DataBoxStageName {
    CopyStarted,
    CopyCompleted,
    OrderCompleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataBoxStageName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataBoxStageName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataBoxStageName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CopyStarted => serializer.serialize_unit_variant("DataBoxStageName", 0u32, "CopyStarted"),
            Self::CopyCompleted => serializer.serialize_unit_variant("DataBoxStageName", 1u32, "CopyCompleted"),
            Self::OrderCompleted => serializer.serialize_unit_variant("DataBoxStageName", 2u32, "OrderCompleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information about the device connection state event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceConnectionStateEventInfo {
    #[doc = "Sequence number is string representation of a hexadecimal number. string compare can be used to identify the larger number because both in ASCII and HEX numbers come after alphabets. If you are converting the string to hex, then the number is a 256 bit number."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}
impl DeviceConnectionStateEventInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a device connection state event (DeviceConnected, DeviceDisconnected)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConnectionStateEventProperties {
    #[doc = "The unique identifier of the device. This case-sensitive string can be up to 128 characters long, and supports ASCII 7-bit alphanumeric characters plus the following special characters: - : . + % _ &#35; * ? ! ( ) , = `@` ; $ '."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "The unique identifier of the module. This case-sensitive string can be up to 128 characters long, and supports ASCII 7-bit alphanumeric characters plus the following special characters: - : . + % _ &#35; * ? ! ( ) , = `@` ; $ '."]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[doc = "Name of the IoT Hub where the device was created or deleted."]
    #[serde(rename = "hubName", default, skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[doc = "Information about the device connection state event."]
    #[serde(rename = "deviceConnectionStateEventInfo")]
    pub device_connection_state_event_info: DeviceConnectionStateEventInfo,
}
impl DeviceConnectionStateEventProperties {
    pub fn new(device_connection_state_event_info: DeviceConnectionStateEventInfo) -> Self {
        Self {
            device_id: None,
            module_id: None,
            hub_name: None,
            device_connection_state_event_info,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a device life cycle event (DeviceCreated, DeviceDeleted)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceLifeCycleEventProperties {
    #[doc = "The unique identifier of the device. This case-sensitive string can be up to 128 characters long, and supports ASCII 7-bit alphanumeric characters plus the following special characters: - : . + % _ &#35; * ? ! ( ) , = `@` ; $ '."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Name of the IoT Hub where the device was created or deleted."]
    #[serde(rename = "hubName", default, skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[doc = "Information about the device twin, which is the cloud representation of application device metadata."]
    pub twin: DeviceTwinInfo,
}
impl DeviceLifeCycleEventProperties {
    pub fn new(twin: DeviceTwinInfo) -> Self {
        Self {
            device_id: None,
            hub_name: None,
            twin,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a device telemetry event (DeviceTelemetry)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTelemetryEventProperties {
    #[doc = "The content of the message from the device."]
    pub body: serde_json::Value,
    #[doc = "Application properties are user-defined strings that can be added to the message. These fields are optional."]
    pub properties: serde_json::Value,
    #[doc = "System properties help identify contents and source of the messages."]
    #[serde(rename = "systemProperties")]
    pub system_properties: serde_json::Value,
}
impl DeviceTelemetryEventProperties {
    pub fn new(body: serde_json::Value, properties: serde_json::Value, system_properties: serde_json::Value) -> Self {
        Self {
            body,
            properties,
            system_properties,
        }
    }
}
#[doc = "Information about the device twin, which is the cloud representation of application device metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTwinInfo {
    #[doc = "Authentication type used for this device: either SAS, SelfSigned, or CertificateAuthority."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[doc = "Count of cloud to device messages sent to this device."]
    #[serde(rename = "cloudToDeviceMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub cloud_to_device_message_count: Option<f32>,
    #[doc = "Whether the device is connected or disconnected."]
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[doc = "The unique identifier of the device twin."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "A piece of information that describes the content of the device twin. Each etag is guaranteed to be unique per device twin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The ISO8601 timestamp of the last activity."]
    #[serde(rename = "lastActivityTime", default, skip_serializing_if = "Option::is_none")]
    pub last_activity_time: Option<String>,
    #[doc = "Properties JSON element."]
    pub properties: DeviceTwinInfoProperties,
    #[doc = "Whether the device twin is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The ISO8601 timestamp of the last device twin status update."]
    #[serde(rename = "statusUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub status_update_time: Option<String>,
    #[doc = "An integer that is incremented by one each time the device twin is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f32>,
    #[doc = "The thumbprint is a unique value for the x509 certificate, commonly used to find a particular certificate in a certificate store. The thumbprint is dynamically generated using the SHA1 algorithm, and does not physically exist in the certificate."]
    #[serde(rename = "x509Thumbprint")]
    pub x509_thumbprint: DeviceTwinInfoX509Thumbprint,
}
impl DeviceTwinInfo {
    pub fn new(properties: DeviceTwinInfoProperties, x509_thumbprint: DeviceTwinInfoX509Thumbprint) -> Self {
        Self {
            authentication_type: None,
            cloud_to_device_message_count: None,
            connection_state: None,
            device_id: None,
            etag: None,
            last_activity_time: None,
            properties,
            status: None,
            status_update_time: None,
            version: None,
            x509_thumbprint,
        }
    }
}
#[doc = "Properties JSON element."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTwinInfoProperties {
    #[doc = "A portion of the properties that can be written only by the application back-end, and read by the device."]
    pub desired: DeviceTwinProperties,
    #[doc = "A portion of the properties that can be written only by the application back-end, and read by the device."]
    pub reported: DeviceTwinProperties,
}
impl DeviceTwinInfoProperties {
    pub fn new(desired: DeviceTwinProperties, reported: DeviceTwinProperties) -> Self {
        Self { desired, reported }
    }
}
#[doc = "The thumbprint is a unique value for the x509 certificate, commonly used to find a particular certificate in a certificate store. The thumbprint is dynamically generated using the SHA1 algorithm, and does not physically exist in the certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceTwinInfoX509Thumbprint {
    #[doc = "Primary thumbprint for the x509 certificate."]
    #[serde(rename = "primaryThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub primary_thumbprint: Option<String>,
    #[doc = "Secondary thumbprint for the x509 certificate."]
    #[serde(rename = "secondaryThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub secondary_thumbprint: Option<String>,
}
impl DeviceTwinInfoX509Thumbprint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata information for the properties JSON document."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceTwinMetadata {
    #[doc = "The ISO8601 timestamp of the last time the properties were updated."]
    #[serde(rename = "lastUpdated", default, skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}
impl DeviceTwinMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A portion of the properties that can be written only by the application back-end, and read by the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTwinProperties {
    #[doc = "Metadata information for the properties JSON document."]
    pub metadata: DeviceTwinMetadata,
    #[doc = "Version of device twin properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f32>,
}
impl DeviceTwinProperties {
    pub fn new(metadata: DeviceTwinMetadata) -> Self {
        Self { metadata, version: None }
    }
}
#[doc = "Properties of an event published to an Event Grid topic using the EventGrid Schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridEvent {
    #[doc = "An unique identifier for the event."]
    pub id: String,
    #[doc = "The resource path of the event source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[doc = "A resource path relative to the topic path."]
    pub subject: String,
    #[doc = "Event data specific to the event type."]
    pub data: serde_json::Value,
    #[doc = "The type of the event that occurred."]
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[doc = "The time (in UTC) the event was generated."]
    #[serde(rename = "eventTime", with = "azure_core::date::rfc3339")]
    pub event_time: ::time::OffsetDateTime,
    #[doc = "The schema version of the event metadata."]
    #[serde(rename = "metadataVersion", default, skip_serializing_if = "Option::is_none")]
    pub metadata_version: Option<String>,
    #[doc = "The schema version of the data object."]
    #[serde(rename = "dataVersion")]
    pub data_version: String,
}
impl EventGridEvent {
    pub fn new(
        id: String,
        subject: String,
        data: serde_json::Value,
        event_type: String,
        event_time: ::time::OffsetDateTime,
        data_version: String,
    ) -> Self {
        Self {
            id,
            topic: None,
            subject,
            data,
            event_type,
            event_time,
            metadata_version: None,
            data_version,
        }
    }
}
#[doc = "Event data for Microsoft.EventGrid.MQTTClientCreatedOrUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridMqttClientCreatedOrUpdatedEventData {
    #[serde(flatten)]
    pub event_grid_mqtt_client_event_data: EventGridMqttClientEventData,
    #[doc = "EventGrid MQTT Client State"]
    pub state: EventGridMqttClientState,
    #[doc = "Time the client resource is created based on the provider's UTC time."]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339")]
    pub created_on: ::time::OffsetDateTime,
    #[doc = "Time the client resource is last updated based on the provider's UTC time. If\nthe client resource was never updated, this value is identical to the value of\nthe 'createdOn' property."]
    #[serde(rename = "updatedOn", with = "azure_core::date::rfc3339")]
    pub updated_on: ::time::OffsetDateTime,
    #[doc = "The key-value attributes that are assigned to the client resource."]
    pub attributes: serde_json::Value,
}
impl EventGridMqttClientCreatedOrUpdatedEventData {
    pub fn new(
        state: EventGridMqttClientState,
        created_on: ::time::OffsetDateTime,
        updated_on: ::time::OffsetDateTime,
        attributes: serde_json::Value,
    ) -> Self {
        Self {
            event_grid_mqtt_client_event_data: EventGridMqttClientEventData::default(),
            state,
            created_on,
            updated_on,
            attributes,
        }
    }
}
#[doc = "Event data for Microsoft.EventGrid.MQTTClientDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventGridMqttClientDeletedEventData {
    #[serde(flatten)]
    pub event_grid_mqtt_client_event_data: EventGridMqttClientEventData,
}
impl EventGridMqttClientDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "EventGrid MQTT Client Disconnection Reason"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EventGridMqttClientDisconnectionReason")]
pub enum EventGridMqttClientDisconnectionReason {
    ClientAuthenticationError,
    ClientAuthorizationError,
    ClientError,
    ClientInitiatedDisconnect,
    ConnectionLost,
    IpForbidden,
    QuotaExceeded,
    ServerError,
    ServerInitiatedDisconnect,
    SessionOverflow,
    SessionTakenOver,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EventGridMqttClientDisconnectionReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EventGridMqttClientDisconnectionReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EventGridMqttClientDisconnectionReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ClientAuthenticationError => {
                serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 0u32, "ClientAuthenticationError")
            }
            Self::ClientAuthorizationError => {
                serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 1u32, "ClientAuthorizationError")
            }
            Self::ClientError => serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 2u32, "ClientError"),
            Self::ClientInitiatedDisconnect => {
                serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 3u32, "ClientInitiatedDisconnect")
            }
            Self::ConnectionLost => serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 4u32, "ConnectionLost"),
            Self::IpForbidden => serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 5u32, "IpForbidden"),
            Self::QuotaExceeded => serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 6u32, "QuotaExceeded"),
            Self::ServerError => serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 7u32, "ServerError"),
            Self::ServerInitiatedDisconnect => {
                serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 8u32, "ServerInitiatedDisconnect")
            }
            Self::SessionOverflow => serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 9u32, "SessionOverflow"),
            Self::SessionTakenOver => {
                serializer.serialize_unit_variant("EventGridMqttClientDisconnectionReason", 10u32, "SessionTakenOver")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for MQTT Client state changes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventGridMqttClientEventData {
    #[doc = "Unique identifier for the MQTT client that the client presents to the service\nfor authentication. This case-sensitive string can be up to 128 characters\nlong, and supports UTF-8 characters."]
    #[serde(rename = "clientAuthenticationName", default, skip_serializing_if = "Option::is_none")]
    pub client_authentication_name: Option<String>,
    #[doc = "Name of the client resource in the Event Grid namespace."]
    #[serde(rename = "clientName", default, skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[doc = "Name of the Event Grid namespace where the MQTT client was created or updated."]
    #[serde(rename = "namespaceName", default, skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
}
impl EventGridMqttClientEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event data for Microsoft.EventGrid.MQTTClientSessionConnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventGridMqttClientSessionConnectedEventData {
    #[serde(flatten)]
    pub event_grid_mqtt_client_event_data: EventGridMqttClientEventData,
    #[doc = "Unique identifier for the MQTT client's session. This case-sensitive string can\nbe up to 128 characters long, and supports UTF-8 characters."]
    #[serde(rename = "clientSessionName", default, skip_serializing_if = "Option::is_none")]
    pub client_session_name: Option<String>,
    #[doc = "A number that helps indicate order of MQTT client session connected or\ndisconnected events. Latest event will have a sequence number that is higher\nthan the previous event."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
}
impl EventGridMqttClientSessionConnectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event data for Microsoft.EventGrid.MQTTClientSessionDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridMqttClientSessionDisconnectedEventData {
    #[serde(flatten)]
    pub event_grid_mqtt_client_event_data: EventGridMqttClientEventData,
    #[doc = "Unique identifier for the MQTT client's session. This case-sensitive string can\nbe up to 128 characters long, and supports UTF-8 characters."]
    #[serde(rename = "clientSessionName", default, skip_serializing_if = "Option::is_none")]
    pub client_session_name: Option<String>,
    #[doc = "A number that helps indicate order of MQTT client session connected or\ndisconnected events. Latest event will have a sequence number that is higher\nthan the previous event."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
    #[doc = "EventGrid MQTT Client Disconnection Reason"]
    #[serde(rename = "disconnectionReason")]
    pub disconnection_reason: EventGridMqttClientDisconnectionReason,
}
impl EventGridMqttClientSessionDisconnectedEventData {
    pub fn new(disconnection_reason: EventGridMqttClientDisconnectionReason) -> Self {
        Self {
            event_grid_mqtt_client_event_data: EventGridMqttClientEventData::default(),
            client_session_name: None,
            sequence_number: None,
            disconnection_reason,
        }
    }
}
#[doc = "EventGrid MQTT Client State"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EventGridMqttClientState")]
pub enum EventGridMqttClientState {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EventGridMqttClientState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EventGridMqttClientState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EventGridMqttClientState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EventGridMqttClientState", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EventGridMqttClientState", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.EventHub.CaptureFileCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubCaptureFileCreatedEventData {
    #[doc = "The path to the capture file."]
    #[serde(rename = "fileUrl", default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[doc = "The file type of the capture file."]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[doc = "The shard ID."]
    #[serde(rename = "partitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<String>,
    #[doc = "The file size."]
    #[serde(rename = "sizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i32>,
    #[doc = "The number of events in the file."]
    #[serde(rename = "eventCount", default, skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i32>,
    #[doc = "The smallest sequence number from the queue."]
    #[serde(rename = "firstSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub first_sequence_number: Option<i32>,
    #[doc = "The last sequence number from the queue."]
    #[serde(rename = "lastSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub last_sequence_number: Option<i32>,
    #[doc = "The first time from the queue."]
    #[serde(rename = "firstEnqueueTime", with = "azure_core::date::rfc3339")]
    pub first_enqueue_time: ::time::OffsetDateTime,
    #[doc = "The last time from the queue."]
    #[serde(rename = "lastEnqueueTime", with = "azure_core::date::rfc3339")]
    pub last_enqueue_time: ::time::OffsetDateTime,
}
impl EventHubCaptureFileCreatedEventData {
    pub fn new(first_enqueue_time: ::time::OffsetDateTime, last_enqueue_time: ::time::OffsetDateTime) -> Self {
        Self {
            file_url: None,
            file_type: None,
            partition_id: None,
            size_in_bytes: None,
            event_count: None,
            first_sequence_number: None,
            last_sequence_number: None,
            first_enqueue_time,
            last_enqueue_time,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.HealthcareApis.DicomImageCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthcareDicomImageCreatedEventData {
    #[doc = "Data partition name"]
    #[serde(rename = "partitionName", default, skip_serializing_if = "Option::is_none")]
    pub partition_name: Option<String>,
    #[doc = "Unique identifier for the Study"]
    #[serde(rename = "imageStudyInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_study_instance_uid: Option<String>,
    #[doc = "Unique identifier for the Series"]
    #[serde(rename = "imageSeriesInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_series_instance_uid: Option<String>,
    #[doc = "Unique identifier for the DICOM Image"]
    #[serde(rename = "imageSopInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_sop_instance_uid: Option<String>,
    #[doc = "Domain name of the DICOM account for this image."]
    #[serde(rename = "serviceHostName", default, skip_serializing_if = "Option::is_none")]
    pub service_host_name: Option<String>,
    #[doc = "Sequence number of the DICOM Service within Azure Health Data Services. It is unique for every image creation and deletion within the service."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
}
impl HealthcareDicomImageCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.HealthcareApis.DicomImageDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthcareDicomImageDeletedEventData {
    #[doc = "Data partition name"]
    #[serde(rename = "partitionName", default, skip_serializing_if = "Option::is_none")]
    pub partition_name: Option<String>,
    #[doc = "Unique identifier for the Study"]
    #[serde(rename = "imageStudyInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_study_instance_uid: Option<String>,
    #[doc = "Unique identifier for the Series"]
    #[serde(rename = "imageSeriesInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_series_instance_uid: Option<String>,
    #[doc = "Unique identifier for the DICOM Image"]
    #[serde(rename = "imageSopInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_sop_instance_uid: Option<String>,
    #[doc = "Host name of the DICOM account for this image."]
    #[serde(rename = "serviceHostName", default, skip_serializing_if = "Option::is_none")]
    pub service_host_name: Option<String>,
    #[doc = "Sequence number of the DICOM Service within Azure Health Data Services. It is unique for every image creation and deletion within the service."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
}
impl HealthcareDicomImageDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.HealthcareApis.DicomImageUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthcareDicomImageUpdatedEventData {
    #[doc = "Data partition name"]
    #[serde(rename = "partitionName", default, skip_serializing_if = "Option::is_none")]
    pub partition_name: Option<String>,
    #[doc = "Unique identifier for the Study"]
    #[serde(rename = "imageStudyInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_study_instance_uid: Option<String>,
    #[doc = "Unique identifier for the Series"]
    #[serde(rename = "imageSeriesInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_series_instance_uid: Option<String>,
    #[doc = "Unique identifier for the DICOM Image"]
    #[serde(rename = "imageSopInstanceUid", default, skip_serializing_if = "Option::is_none")]
    pub image_sop_instance_uid: Option<String>,
    #[doc = "Domain name of the DICOM account for this image."]
    #[serde(rename = "serviceHostName", default, skip_serializing_if = "Option::is_none")]
    pub service_host_name: Option<String>,
    #[doc = "Sequence number of the DICOM Service within Azure Health Data Services. It is unique for every image creation, updation and deletion within the service."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
}
impl HealthcareDicomImageUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.HealthcareApis.FhirResourceCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthcareFhirResourceCreatedEventData {
    #[doc = "Schema of FHIR resource type enumeration."]
    #[serde(rename = "resourceType")]
    pub resource_type: HealthcareFhirResourceType,
    #[doc = "Domain name of FHIR account for this resource."]
    #[serde(rename = "resourceFhirAccount", default, skip_serializing_if = "Option::is_none")]
    pub resource_fhir_account: Option<String>,
    #[doc = "Id of HL7 FHIR resource."]
    #[serde(rename = "resourceFhirId", default, skip_serializing_if = "Option::is_none")]
    pub resource_fhir_id: Option<String>,
    #[doc = "VersionId of HL7 FHIR resource. It changes when the resource is created, updated, or deleted(soft-deletion)."]
    #[serde(rename = "resourceVersionId", default, skip_serializing_if = "Option::is_none")]
    pub resource_version_id: Option<i64>,
}
impl HealthcareFhirResourceCreatedEventData {
    pub fn new(resource_type: HealthcareFhirResourceType) -> Self {
        Self {
            resource_type,
            resource_fhir_account: None,
            resource_fhir_id: None,
            resource_version_id: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.HealthcareApis.FhirResourceDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthcareFhirResourceDeletedEventData {
    #[doc = "Schema of FHIR resource type enumeration."]
    #[serde(rename = "resourceType")]
    pub resource_type: HealthcareFhirResourceType,
    #[doc = "Domain name of FHIR account for this resource."]
    #[serde(rename = "resourceFhirAccount", default, skip_serializing_if = "Option::is_none")]
    pub resource_fhir_account: Option<String>,
    #[doc = "Id of HL7 FHIR resource."]
    #[serde(rename = "resourceFhirId", default, skip_serializing_if = "Option::is_none")]
    pub resource_fhir_id: Option<String>,
    #[doc = "VersionId of HL7 FHIR resource. It changes when the resource is created, updated, or deleted(soft-deletion)."]
    #[serde(rename = "resourceVersionId", default, skip_serializing_if = "Option::is_none")]
    pub resource_version_id: Option<i64>,
}
impl HealthcareFhirResourceDeletedEventData {
    pub fn new(resource_type: HealthcareFhirResourceType) -> Self {
        Self {
            resource_type,
            resource_fhir_account: None,
            resource_fhir_id: None,
            resource_version_id: None,
        }
    }
}
#[doc = "Schema of FHIR resource type enumeration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthcareFhirResourceType")]
pub enum HealthcareFhirResourceType {
    Account,
    ActivityDefinition,
    AdverseEvent,
    AllergyIntolerance,
    Appointment,
    AppointmentResponse,
    AuditEvent,
    Basic,
    Binary,
    BiologicallyDerivedProduct,
    BodySite,
    BodyStructure,
    Bundle,
    CapabilityStatement,
    CarePlan,
    CareTeam,
    CatalogEntry,
    ChargeItem,
    ChargeItemDefinition,
    Claim,
    ClaimResponse,
    ClinicalImpression,
    CodeSystem,
    Communication,
    CommunicationRequest,
    CompartmentDefinition,
    Composition,
    ConceptMap,
    Condition,
    Consent,
    Contract,
    Coverage,
    CoverageEligibilityRequest,
    CoverageEligibilityResponse,
    DataElement,
    DetectedIssue,
    Device,
    DeviceComponent,
    DeviceDefinition,
    DeviceMetric,
    DeviceRequest,
    DeviceUseStatement,
    DiagnosticReport,
    DocumentManifest,
    DocumentReference,
    DomainResource,
    EffectEvidenceSynthesis,
    EligibilityRequest,
    EligibilityResponse,
    Encounter,
    Endpoint,
    EnrollmentRequest,
    EnrollmentResponse,
    EpisodeOfCare,
    EventDefinition,
    Evidence,
    EvidenceVariable,
    ExampleScenario,
    ExpansionProfile,
    ExplanationOfBenefit,
    FamilyMemberHistory,
    Flag,
    Goal,
    GraphDefinition,
    Group,
    GuidanceResponse,
    HealthcareService,
    ImagingManifest,
    ImagingStudy,
    Immunization,
    ImmunizationEvaluation,
    ImmunizationRecommendation,
    ImplementationGuide,
    InsurancePlan,
    Invoice,
    Library,
    Linkage,
    List,
    Location,
    Measure,
    MeasureReport,
    Media,
    Medication,
    MedicationAdministration,
    MedicationDispense,
    MedicationKnowledge,
    MedicationRequest,
    MedicationStatement,
    MedicinalProduct,
    MedicinalProductAuthorization,
    MedicinalProductContraindication,
    MedicinalProductIndication,
    MedicinalProductIngredient,
    MedicinalProductInteraction,
    MedicinalProductManufactured,
    MedicinalProductPackaged,
    MedicinalProductPharmaceutical,
    MedicinalProductUndesirableEffect,
    MessageDefinition,
    MessageHeader,
    MolecularSequence,
    NamingSystem,
    NutritionOrder,
    Observation,
    ObservationDefinition,
    OperationDefinition,
    OperationOutcome,
    Organization,
    OrganizationAffiliation,
    Parameters,
    Patient,
    PaymentNotice,
    PaymentReconciliation,
    Person,
    PlanDefinition,
    Practitioner,
    PractitionerRole,
    Procedure,
    ProcedureRequest,
    ProcessRequest,
    ProcessResponse,
    Provenance,
    Questionnaire,
    QuestionnaireResponse,
    ReferralRequest,
    RelatedPerson,
    RequestGroup,
    ResearchDefinition,
    ResearchElementDefinition,
    ResearchStudy,
    ResearchSubject,
    Resource,
    RiskAssessment,
    RiskEvidenceSynthesis,
    Schedule,
    SearchParameter,
    Sequence,
    ServiceDefinition,
    ServiceRequest,
    Slot,
    Specimen,
    SpecimenDefinition,
    StructureDefinition,
    StructureMap,
    Subscription,
    Substance,
    SubstanceNucleicAcid,
    SubstancePolymer,
    SubstanceProtein,
    SubstanceReferenceInformation,
    SubstanceSourceMaterial,
    SubstanceSpecification,
    SupplyDelivery,
    SupplyRequest,
    Task,
    TerminologyCapabilities,
    TestReport,
    TestScript,
    ValueSet,
    VerificationResult,
    VisionPrescription,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthcareFhirResourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthcareFhirResourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthcareFhirResourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Account => serializer.serialize_unit_variant("HealthcareFhirResourceType", 0u32, "Account"),
            Self::ActivityDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 1u32, "ActivityDefinition"),
            Self::AdverseEvent => serializer.serialize_unit_variant("HealthcareFhirResourceType", 2u32, "AdverseEvent"),
            Self::AllergyIntolerance => serializer.serialize_unit_variant("HealthcareFhirResourceType", 3u32, "AllergyIntolerance"),
            Self::Appointment => serializer.serialize_unit_variant("HealthcareFhirResourceType", 4u32, "Appointment"),
            Self::AppointmentResponse => serializer.serialize_unit_variant("HealthcareFhirResourceType", 5u32, "AppointmentResponse"),
            Self::AuditEvent => serializer.serialize_unit_variant("HealthcareFhirResourceType", 6u32, "AuditEvent"),
            Self::Basic => serializer.serialize_unit_variant("HealthcareFhirResourceType", 7u32, "Basic"),
            Self::Binary => serializer.serialize_unit_variant("HealthcareFhirResourceType", 8u32, "Binary"),
            Self::BiologicallyDerivedProduct => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 9u32, "BiologicallyDerivedProduct")
            }
            Self::BodySite => serializer.serialize_unit_variant("HealthcareFhirResourceType", 10u32, "BodySite"),
            Self::BodyStructure => serializer.serialize_unit_variant("HealthcareFhirResourceType", 11u32, "BodyStructure"),
            Self::Bundle => serializer.serialize_unit_variant("HealthcareFhirResourceType", 12u32, "Bundle"),
            Self::CapabilityStatement => serializer.serialize_unit_variant("HealthcareFhirResourceType", 13u32, "CapabilityStatement"),
            Self::CarePlan => serializer.serialize_unit_variant("HealthcareFhirResourceType", 14u32, "CarePlan"),
            Self::CareTeam => serializer.serialize_unit_variant("HealthcareFhirResourceType", 15u32, "CareTeam"),
            Self::CatalogEntry => serializer.serialize_unit_variant("HealthcareFhirResourceType", 16u32, "CatalogEntry"),
            Self::ChargeItem => serializer.serialize_unit_variant("HealthcareFhirResourceType", 17u32, "ChargeItem"),
            Self::ChargeItemDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 18u32, "ChargeItemDefinition"),
            Self::Claim => serializer.serialize_unit_variant("HealthcareFhirResourceType", 19u32, "Claim"),
            Self::ClaimResponse => serializer.serialize_unit_variant("HealthcareFhirResourceType", 20u32, "ClaimResponse"),
            Self::ClinicalImpression => serializer.serialize_unit_variant("HealthcareFhirResourceType", 21u32, "ClinicalImpression"),
            Self::CodeSystem => serializer.serialize_unit_variant("HealthcareFhirResourceType", 22u32, "CodeSystem"),
            Self::Communication => serializer.serialize_unit_variant("HealthcareFhirResourceType", 23u32, "Communication"),
            Self::CommunicationRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 24u32, "CommunicationRequest"),
            Self::CompartmentDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 25u32, "CompartmentDefinition"),
            Self::Composition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 26u32, "Composition"),
            Self::ConceptMap => serializer.serialize_unit_variant("HealthcareFhirResourceType", 27u32, "ConceptMap"),
            Self::Condition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 28u32, "Condition"),
            Self::Consent => serializer.serialize_unit_variant("HealthcareFhirResourceType", 29u32, "Consent"),
            Self::Contract => serializer.serialize_unit_variant("HealthcareFhirResourceType", 30u32, "Contract"),
            Self::Coverage => serializer.serialize_unit_variant("HealthcareFhirResourceType", 31u32, "Coverage"),
            Self::CoverageEligibilityRequest => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 32u32, "CoverageEligibilityRequest")
            }
            Self::CoverageEligibilityResponse => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 33u32, "CoverageEligibilityResponse")
            }
            Self::DataElement => serializer.serialize_unit_variant("HealthcareFhirResourceType", 34u32, "DataElement"),
            Self::DetectedIssue => serializer.serialize_unit_variant("HealthcareFhirResourceType", 35u32, "DetectedIssue"),
            Self::Device => serializer.serialize_unit_variant("HealthcareFhirResourceType", 36u32, "Device"),
            Self::DeviceComponent => serializer.serialize_unit_variant("HealthcareFhirResourceType", 37u32, "DeviceComponent"),
            Self::DeviceDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 38u32, "DeviceDefinition"),
            Self::DeviceMetric => serializer.serialize_unit_variant("HealthcareFhirResourceType", 39u32, "DeviceMetric"),
            Self::DeviceRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 40u32, "DeviceRequest"),
            Self::DeviceUseStatement => serializer.serialize_unit_variant("HealthcareFhirResourceType", 41u32, "DeviceUseStatement"),
            Self::DiagnosticReport => serializer.serialize_unit_variant("HealthcareFhirResourceType", 42u32, "DiagnosticReport"),
            Self::DocumentManifest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 43u32, "DocumentManifest"),
            Self::DocumentReference => serializer.serialize_unit_variant("HealthcareFhirResourceType", 44u32, "DocumentReference"),
            Self::DomainResource => serializer.serialize_unit_variant("HealthcareFhirResourceType", 45u32, "DomainResource"),
            Self::EffectEvidenceSynthesis => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 46u32, "EffectEvidenceSynthesis")
            }
            Self::EligibilityRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 47u32, "EligibilityRequest"),
            Self::EligibilityResponse => serializer.serialize_unit_variant("HealthcareFhirResourceType", 48u32, "EligibilityResponse"),
            Self::Encounter => serializer.serialize_unit_variant("HealthcareFhirResourceType", 49u32, "Encounter"),
            Self::Endpoint => serializer.serialize_unit_variant("HealthcareFhirResourceType", 50u32, "Endpoint"),
            Self::EnrollmentRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 51u32, "EnrollmentRequest"),
            Self::EnrollmentResponse => serializer.serialize_unit_variant("HealthcareFhirResourceType", 52u32, "EnrollmentResponse"),
            Self::EpisodeOfCare => serializer.serialize_unit_variant("HealthcareFhirResourceType", 53u32, "EpisodeOfCare"),
            Self::EventDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 54u32, "EventDefinition"),
            Self::Evidence => serializer.serialize_unit_variant("HealthcareFhirResourceType", 55u32, "Evidence"),
            Self::EvidenceVariable => serializer.serialize_unit_variant("HealthcareFhirResourceType", 56u32, "EvidenceVariable"),
            Self::ExampleScenario => serializer.serialize_unit_variant("HealthcareFhirResourceType", 57u32, "ExampleScenario"),
            Self::ExpansionProfile => serializer.serialize_unit_variant("HealthcareFhirResourceType", 58u32, "ExpansionProfile"),
            Self::ExplanationOfBenefit => serializer.serialize_unit_variant("HealthcareFhirResourceType", 59u32, "ExplanationOfBenefit"),
            Self::FamilyMemberHistory => serializer.serialize_unit_variant("HealthcareFhirResourceType", 60u32, "FamilyMemberHistory"),
            Self::Flag => serializer.serialize_unit_variant("HealthcareFhirResourceType", 61u32, "Flag"),
            Self::Goal => serializer.serialize_unit_variant("HealthcareFhirResourceType", 62u32, "Goal"),
            Self::GraphDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 63u32, "GraphDefinition"),
            Self::Group => serializer.serialize_unit_variant("HealthcareFhirResourceType", 64u32, "Group"),
            Self::GuidanceResponse => serializer.serialize_unit_variant("HealthcareFhirResourceType", 65u32, "GuidanceResponse"),
            Self::HealthcareService => serializer.serialize_unit_variant("HealthcareFhirResourceType", 66u32, "HealthcareService"),
            Self::ImagingManifest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 67u32, "ImagingManifest"),
            Self::ImagingStudy => serializer.serialize_unit_variant("HealthcareFhirResourceType", 68u32, "ImagingStudy"),
            Self::Immunization => serializer.serialize_unit_variant("HealthcareFhirResourceType", 69u32, "Immunization"),
            Self::ImmunizationEvaluation => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 70u32, "ImmunizationEvaluation")
            }
            Self::ImmunizationRecommendation => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 71u32, "ImmunizationRecommendation")
            }
            Self::ImplementationGuide => serializer.serialize_unit_variant("HealthcareFhirResourceType", 72u32, "ImplementationGuide"),
            Self::InsurancePlan => serializer.serialize_unit_variant("HealthcareFhirResourceType", 73u32, "InsurancePlan"),
            Self::Invoice => serializer.serialize_unit_variant("HealthcareFhirResourceType", 74u32, "Invoice"),
            Self::Library => serializer.serialize_unit_variant("HealthcareFhirResourceType", 75u32, "Library"),
            Self::Linkage => serializer.serialize_unit_variant("HealthcareFhirResourceType", 76u32, "Linkage"),
            Self::List => serializer.serialize_unit_variant("HealthcareFhirResourceType", 77u32, "List"),
            Self::Location => serializer.serialize_unit_variant("HealthcareFhirResourceType", 78u32, "Location"),
            Self::Measure => serializer.serialize_unit_variant("HealthcareFhirResourceType", 79u32, "Measure"),
            Self::MeasureReport => serializer.serialize_unit_variant("HealthcareFhirResourceType", 80u32, "MeasureReport"),
            Self::Media => serializer.serialize_unit_variant("HealthcareFhirResourceType", 81u32, "Media"),
            Self::Medication => serializer.serialize_unit_variant("HealthcareFhirResourceType", 82u32, "Medication"),
            Self::MedicationAdministration => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 83u32, "MedicationAdministration")
            }
            Self::MedicationDispense => serializer.serialize_unit_variant("HealthcareFhirResourceType", 84u32, "MedicationDispense"),
            Self::MedicationKnowledge => serializer.serialize_unit_variant("HealthcareFhirResourceType", 85u32, "MedicationKnowledge"),
            Self::MedicationRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 86u32, "MedicationRequest"),
            Self::MedicationStatement => serializer.serialize_unit_variant("HealthcareFhirResourceType", 87u32, "MedicationStatement"),
            Self::MedicinalProduct => serializer.serialize_unit_variant("HealthcareFhirResourceType", 88u32, "MedicinalProduct"),
            Self::MedicinalProductAuthorization => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 89u32, "MedicinalProductAuthorization")
            }
            Self::MedicinalProductContraindication => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 90u32, "MedicinalProductContraindication")
            }
            Self::MedicinalProductIndication => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 91u32, "MedicinalProductIndication")
            }
            Self::MedicinalProductIngredient => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 92u32, "MedicinalProductIngredient")
            }
            Self::MedicinalProductInteraction => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 93u32, "MedicinalProductInteraction")
            }
            Self::MedicinalProductManufactured => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 94u32, "MedicinalProductManufactured")
            }
            Self::MedicinalProductPackaged => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 95u32, "MedicinalProductPackaged")
            }
            Self::MedicinalProductPharmaceutical => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 96u32, "MedicinalProductPharmaceutical")
            }
            Self::MedicinalProductUndesirableEffect => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 97u32, "MedicinalProductUndesirableEffect")
            }
            Self::MessageDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 98u32, "MessageDefinition"),
            Self::MessageHeader => serializer.serialize_unit_variant("HealthcareFhirResourceType", 99u32, "MessageHeader"),
            Self::MolecularSequence => serializer.serialize_unit_variant("HealthcareFhirResourceType", 100u32, "MolecularSequence"),
            Self::NamingSystem => serializer.serialize_unit_variant("HealthcareFhirResourceType", 101u32, "NamingSystem"),
            Self::NutritionOrder => serializer.serialize_unit_variant("HealthcareFhirResourceType", 102u32, "NutritionOrder"),
            Self::Observation => serializer.serialize_unit_variant("HealthcareFhirResourceType", 103u32, "Observation"),
            Self::ObservationDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 104u32, "ObservationDefinition"),
            Self::OperationDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 105u32, "OperationDefinition"),
            Self::OperationOutcome => serializer.serialize_unit_variant("HealthcareFhirResourceType", 106u32, "OperationOutcome"),
            Self::Organization => serializer.serialize_unit_variant("HealthcareFhirResourceType", 107u32, "Organization"),
            Self::OrganizationAffiliation => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 108u32, "OrganizationAffiliation")
            }
            Self::Parameters => serializer.serialize_unit_variant("HealthcareFhirResourceType", 109u32, "Parameters"),
            Self::Patient => serializer.serialize_unit_variant("HealthcareFhirResourceType", 110u32, "Patient"),
            Self::PaymentNotice => serializer.serialize_unit_variant("HealthcareFhirResourceType", 111u32, "PaymentNotice"),
            Self::PaymentReconciliation => serializer.serialize_unit_variant("HealthcareFhirResourceType", 112u32, "PaymentReconciliation"),
            Self::Person => serializer.serialize_unit_variant("HealthcareFhirResourceType", 113u32, "Person"),
            Self::PlanDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 114u32, "PlanDefinition"),
            Self::Practitioner => serializer.serialize_unit_variant("HealthcareFhirResourceType", 115u32, "Practitioner"),
            Self::PractitionerRole => serializer.serialize_unit_variant("HealthcareFhirResourceType", 116u32, "PractitionerRole"),
            Self::Procedure => serializer.serialize_unit_variant("HealthcareFhirResourceType", 117u32, "Procedure"),
            Self::ProcedureRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 118u32, "ProcedureRequest"),
            Self::ProcessRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 119u32, "ProcessRequest"),
            Self::ProcessResponse => serializer.serialize_unit_variant("HealthcareFhirResourceType", 120u32, "ProcessResponse"),
            Self::Provenance => serializer.serialize_unit_variant("HealthcareFhirResourceType", 121u32, "Provenance"),
            Self::Questionnaire => serializer.serialize_unit_variant("HealthcareFhirResourceType", 122u32, "Questionnaire"),
            Self::QuestionnaireResponse => serializer.serialize_unit_variant("HealthcareFhirResourceType", 123u32, "QuestionnaireResponse"),
            Self::ReferralRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 124u32, "ReferralRequest"),
            Self::RelatedPerson => serializer.serialize_unit_variant("HealthcareFhirResourceType", 125u32, "RelatedPerson"),
            Self::RequestGroup => serializer.serialize_unit_variant("HealthcareFhirResourceType", 126u32, "RequestGroup"),
            Self::ResearchDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 127u32, "ResearchDefinition"),
            Self::ResearchElementDefinition => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 128u32, "ResearchElementDefinition")
            }
            Self::ResearchStudy => serializer.serialize_unit_variant("HealthcareFhirResourceType", 129u32, "ResearchStudy"),
            Self::ResearchSubject => serializer.serialize_unit_variant("HealthcareFhirResourceType", 130u32, "ResearchSubject"),
            Self::Resource => serializer.serialize_unit_variant("HealthcareFhirResourceType", 131u32, "Resource"),
            Self::RiskAssessment => serializer.serialize_unit_variant("HealthcareFhirResourceType", 132u32, "RiskAssessment"),
            Self::RiskEvidenceSynthesis => serializer.serialize_unit_variant("HealthcareFhirResourceType", 133u32, "RiskEvidenceSynthesis"),
            Self::Schedule => serializer.serialize_unit_variant("HealthcareFhirResourceType", 134u32, "Schedule"),
            Self::SearchParameter => serializer.serialize_unit_variant("HealthcareFhirResourceType", 135u32, "SearchParameter"),
            Self::Sequence => serializer.serialize_unit_variant("HealthcareFhirResourceType", 136u32, "Sequence"),
            Self::ServiceDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 137u32, "ServiceDefinition"),
            Self::ServiceRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 138u32, "ServiceRequest"),
            Self::Slot => serializer.serialize_unit_variant("HealthcareFhirResourceType", 139u32, "Slot"),
            Self::Specimen => serializer.serialize_unit_variant("HealthcareFhirResourceType", 140u32, "Specimen"),
            Self::SpecimenDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 141u32, "SpecimenDefinition"),
            Self::StructureDefinition => serializer.serialize_unit_variant("HealthcareFhirResourceType", 142u32, "StructureDefinition"),
            Self::StructureMap => serializer.serialize_unit_variant("HealthcareFhirResourceType", 143u32, "StructureMap"),
            Self::Subscription => serializer.serialize_unit_variant("HealthcareFhirResourceType", 144u32, "Subscription"),
            Self::Substance => serializer.serialize_unit_variant("HealthcareFhirResourceType", 145u32, "Substance"),
            Self::SubstanceNucleicAcid => serializer.serialize_unit_variant("HealthcareFhirResourceType", 146u32, "SubstanceNucleicAcid"),
            Self::SubstancePolymer => serializer.serialize_unit_variant("HealthcareFhirResourceType", 147u32, "SubstancePolymer"),
            Self::SubstanceProtein => serializer.serialize_unit_variant("HealthcareFhirResourceType", 148u32, "SubstanceProtein"),
            Self::SubstanceReferenceInformation => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 149u32, "SubstanceReferenceInformation")
            }
            Self::SubstanceSourceMaterial => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 150u32, "SubstanceSourceMaterial")
            }
            Self::SubstanceSpecification => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 151u32, "SubstanceSpecification")
            }
            Self::SupplyDelivery => serializer.serialize_unit_variant("HealthcareFhirResourceType", 152u32, "SupplyDelivery"),
            Self::SupplyRequest => serializer.serialize_unit_variant("HealthcareFhirResourceType", 153u32, "SupplyRequest"),
            Self::Task => serializer.serialize_unit_variant("HealthcareFhirResourceType", 154u32, "Task"),
            Self::TerminologyCapabilities => {
                serializer.serialize_unit_variant("HealthcareFhirResourceType", 155u32, "TerminologyCapabilities")
            }
            Self::TestReport => serializer.serialize_unit_variant("HealthcareFhirResourceType", 156u32, "TestReport"),
            Self::TestScript => serializer.serialize_unit_variant("HealthcareFhirResourceType", 157u32, "TestScript"),
            Self::ValueSet => serializer.serialize_unit_variant("HealthcareFhirResourceType", 158u32, "ValueSet"),
            Self::VerificationResult => serializer.serialize_unit_variant("HealthcareFhirResourceType", 159u32, "VerificationResult"),
            Self::VisionPrescription => serializer.serialize_unit_variant("HealthcareFhirResourceType", 160u32, "VisionPrescription"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.HealthcareApis.FhirResourceUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthcareFhirResourceUpdatedEventData {
    #[doc = "Schema of FHIR resource type enumeration."]
    #[serde(rename = "resourceType")]
    pub resource_type: HealthcareFhirResourceType,
    #[doc = "Domain name of FHIR account for this resource."]
    #[serde(rename = "resourceFhirAccount", default, skip_serializing_if = "Option::is_none")]
    pub resource_fhir_account: Option<String>,
    #[doc = "Id of HL7 FHIR resource."]
    #[serde(rename = "resourceFhirId", default, skip_serializing_if = "Option::is_none")]
    pub resource_fhir_id: Option<String>,
    #[doc = "VersionId of HL7 FHIR resource. It changes when the resource is created, updated, or deleted(soft-deletion)."]
    #[serde(rename = "resourceVersionId", default, skip_serializing_if = "Option::is_none")]
    pub resource_version_id: Option<i64>,
}
impl HealthcareFhirResourceUpdatedEventData {
    pub fn new(resource_type: HealthcareFhirResourceType) -> Self {
        Self {
            resource_type,
            resource_fhir_account: None,
            resource_fhir_id: None,
            resource_version_id: None,
        }
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceConnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDeviceConnectedEventData {
    #[serde(flatten)]
    pub device_connection_state_event_properties: DeviceConnectionStateEventProperties,
}
impl IotHubDeviceConnectedEventData {
    pub fn new(device_connection_state_event_properties: DeviceConnectionStateEventProperties) -> Self {
        Self {
            device_connection_state_event_properties,
        }
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDeviceCreatedEventData {
    #[serde(flatten)]
    pub device_life_cycle_event_properties: DeviceLifeCycleEventProperties,
}
impl IotHubDeviceCreatedEventData {
    pub fn new(device_life_cycle_event_properties: DeviceLifeCycleEventProperties) -> Self {
        Self {
            device_life_cycle_event_properties,
        }
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDeviceDeletedEventData {
    #[serde(flatten)]
    pub device_life_cycle_event_properties: DeviceLifeCycleEventProperties,
}
impl IotHubDeviceDeletedEventData {
    pub fn new(device_life_cycle_event_properties: DeviceLifeCycleEventProperties) -> Self {
        Self {
            device_life_cycle_event_properties,
        }
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDeviceDisconnectedEventData {
    #[serde(flatten)]
    pub device_connection_state_event_properties: DeviceConnectionStateEventProperties,
}
impl IotHubDeviceDisconnectedEventData {
    pub fn new(device_connection_state_event_properties: DeviceConnectionStateEventProperties) -> Self {
        Self {
            device_connection_state_event_properties,
        }
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceTelemetry event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDeviceTelemetryEventData {
    #[serde(flatten)]
    pub device_telemetry_event_properties: DeviceTelemetryEventProperties,
}
impl IotHubDeviceTelemetryEventData {
    pub fn new(device_telemetry_event_properties: DeviceTelemetryEventProperties) -> Self {
        Self {
            device_telemetry_event_properties,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.VaultAccessPolicyChanged event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultAccessPolicyChangedEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultAccessPolicyChangedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.CertificateExpired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCertificateExpiredEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultCertificateExpiredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.CertificateNearExpiry event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCertificateNearExpiryEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultCertificateNearExpiryEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.CertificateNewVersionCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCertificateNewVersionCreatedEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultCertificateNewVersionCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.KeyExpired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultKeyExpiredEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultKeyExpiredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.KeyNearExpiry event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultKeyNearExpiryEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultKeyNearExpiryEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.KeyNewVersionCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultKeyNewVersionCreatedEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultKeyNewVersionCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.SecretExpired event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretExpiredEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultSecretExpiredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.SecretNearExpiry event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretNearExpiryEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultSecretNearExpiryEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.SecretNewVersionCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultSecretNewVersionCreatedEventData {
    #[doc = "The id of the object that triggered this event."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Key vault name of the object that triggered this event."]
    #[serde(rename = "VaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "The type of the object that triggered this event"]
    #[serde(rename = "ObjectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The name of the object that triggered this event"]
    #[serde(rename = "ObjectName", default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[doc = "The version of the object that triggered this event"]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Not before date of the object that triggered this event"]
    #[serde(rename = "NBF", default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f32>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f32>,
}
impl KeyVaultSecretNewVersionCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.DatasetDriftDetected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineLearningServicesDatasetDriftDetectedEventData {
    #[doc = "The ID of the data drift monitor that triggered the event."]
    #[serde(rename = "dataDriftId", default, skip_serializing_if = "Option::is_none")]
    pub data_drift_id: Option<String>,
    #[doc = "The name of the data drift monitor that triggered the event."]
    #[serde(rename = "dataDriftName", default, skip_serializing_if = "Option::is_none")]
    pub data_drift_name: Option<String>,
    #[doc = "The ID of the Run that detected data drift."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The ID of the base Dataset used to detect drift."]
    #[serde(rename = "baseDatasetId", default, skip_serializing_if = "Option::is_none")]
    pub base_dataset_id: Option<String>,
    #[doc = "The ID of the target Dataset used to detect drift."]
    #[serde(rename = "targetDatasetId", default, skip_serializing_if = "Option::is_none")]
    pub target_dataset_id: Option<String>,
    #[doc = "The coefficient result that triggered the event."]
    #[serde(rename = "driftCoefficient", default, skip_serializing_if = "Option::is_none")]
    pub drift_coefficient: Option<f64>,
    #[doc = "The start time of the target dataset time series that resulted in drift detection."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: ::time::OffsetDateTime,
    #[doc = "The end time of the target dataset time series that resulted in drift detection."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: ::time::OffsetDateTime,
}
impl MachineLearningServicesDatasetDriftDetectedEventData {
    pub fn new(start_time: ::time::OffsetDateTime, end_time: ::time::OffsetDateTime) -> Self {
        Self {
            data_drift_id: None,
            data_drift_name: None,
            run_id: None,
            base_dataset_id: None,
            target_dataset_id: None,
            drift_coefficient: None,
            start_time,
            end_time,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.ModelDeployed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineLearningServicesModelDeployedEventData {
    #[doc = "The name of the deployed service."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "The compute type (e.g. ACI, AKS) of the deployed service."]
    #[serde(rename = "serviceComputeType", default, skip_serializing_if = "Option::is_none")]
    pub service_compute_type: Option<String>,
    #[doc = "A common separated list of model IDs. The IDs of the models deployed in the service."]
    #[serde(rename = "modelIds", default, skip_serializing_if = "Option::is_none")]
    pub model_ids: Option<String>,
    #[doc = "The tags of the deployed service."]
    #[serde(rename = "serviceTags")]
    pub service_tags: serde_json::Value,
    #[doc = "The properties of the deployed service."]
    #[serde(rename = "serviceProperties")]
    pub service_properties: serde_json::Value,
}
impl MachineLearningServicesModelDeployedEventData {
    pub fn new(service_tags: serde_json::Value, service_properties: serde_json::Value) -> Self {
        Self {
            service_name: None,
            service_compute_type: None,
            model_ids: None,
            service_tags,
            service_properties,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.ModelRegistered event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineLearningServicesModelRegisteredEventData {
    #[doc = "The name of the model that was registered."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "The version of the model that was registered."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    #[doc = "The tags of the model that was registered."]
    #[serde(rename = "modelTags")]
    pub model_tags: serde_json::Value,
    #[doc = "The properties of the model that was registered."]
    #[serde(rename = "modelProperties")]
    pub model_properties: serde_json::Value,
}
impl MachineLearningServicesModelRegisteredEventData {
    pub fn new(model_tags: serde_json::Value, model_properties: serde_json::Value) -> Self {
        Self {
            model_name: None,
            model_version: None,
            model_tags,
            model_properties,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.RunCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineLearningServicesRunCompletedEventData {
    #[doc = "The ID of the experiment that the run belongs to."]
    #[serde(rename = "experimentId", default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[doc = "The name of the experiment that the run belongs to."]
    #[serde(rename = "experimentName", default, skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[doc = "The ID of the Run that was completed."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The Run Type of the completed Run."]
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    #[doc = "The tags of the completed Run."]
    #[serde(rename = "runTags")]
    pub run_tags: serde_json::Value,
    #[doc = "The properties of the completed Run."]
    #[serde(rename = "runProperties")]
    pub run_properties: serde_json::Value,
}
impl MachineLearningServicesRunCompletedEventData {
    pub fn new(run_tags: serde_json::Value, run_properties: serde_json::Value) -> Self {
        Self {
            experiment_id: None,
            experiment_name: None,
            run_id: None,
            run_type: None,
            run_tags,
            run_properties,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.RunStatusChanged event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineLearningServicesRunStatusChangedEventData {
    #[doc = "The ID of the experiment that the Machine Learning Run belongs to."]
    #[serde(rename = "experimentId", default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[doc = "The name of the experiment that the Machine Learning Run belongs to."]
    #[serde(rename = "experimentName", default, skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[doc = "The ID of the Machine Learning Run."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The Run Type of the Machine Learning Run."]
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    #[doc = "The tags of the Machine Learning Run."]
    #[serde(rename = "runTags")]
    pub run_tags: serde_json::Value,
    #[doc = "The properties of the Machine Learning Run."]
    #[serde(rename = "runProperties")]
    pub run_properties: serde_json::Value,
    #[doc = "The status of the Machine Learning Run."]
    #[serde(rename = "runStatus", default, skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
}
impl MachineLearningServicesRunStatusChangedEventData {
    pub fn new(run_tags: serde_json::Value, run_properties: serde_json::Value) -> Self {
        Self {
            experiment_id: None,
            experiment_name: None,
            run_id: None,
            run_type: None,
            run_tags,
            run_properties,
            run_status: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Maps.GeofenceEntered event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsGeofenceEnteredEventData {
    #[serde(flatten)]
    pub maps_geofence_event_properties: MapsGeofenceEventProperties,
}
impl MapsGeofenceEnteredEventData {
    pub fn new(maps_geofence_event_properties: MapsGeofenceEventProperties) -> Self {
        Self {
            maps_geofence_event_properties,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Geofence event (GeofenceEntered, GeofenceExited, GeofenceResult)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsGeofenceEventProperties {
    #[doc = "Lists of the geometry ID of the geofence which is expired relative to the user time in the request."]
    #[serde(
        rename = "expiredGeofenceGeometryId",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expired_geofence_geometry_id: Vec<String>,
    #[doc = "Lists the fence geometries that either fully contain the coordinate position or have an overlap with the searchBuffer around the fence."]
    pub geometries: Vec<MapsGeofenceGeometry>,
    #[doc = "Lists of the geometry ID of the geofence which is in invalid period relative to the user time in the request."]
    #[serde(
        rename = "invalidPeriodGeofenceGeometryId",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub invalid_period_geofence_geometry_id: Vec<String>,
    #[doc = "True if at least one event is published to the Azure Maps event subscriber, false if no event is published to the Azure Maps event subscriber."]
    #[serde(rename = "isEventPublished", default, skip_serializing_if = "Option::is_none")]
    pub is_event_published: Option<bool>,
}
impl MapsGeofenceEventProperties {
    pub fn new(geometries: Vec<MapsGeofenceGeometry>) -> Self {
        Self {
            expired_geofence_geometry_id: Vec::new(),
            geometries,
            invalid_period_geofence_geometry_id: Vec::new(),
            is_event_published: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Maps.GeofenceExited event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsGeofenceExitedEventData {
    #[serde(flatten)]
    pub maps_geofence_event_properties: MapsGeofenceEventProperties,
}
impl MapsGeofenceExitedEventData {
    pub fn new(maps_geofence_event_properties: MapsGeofenceEventProperties) -> Self {
        Self {
            maps_geofence_event_properties,
        }
    }
}
#[doc = "The geofence geometry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsGeofenceGeometry {
    #[doc = "ID of the device."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Distance from the coordinate to the closest border of the geofence. Positive means the coordinate is outside of the geofence. If the coordinate is outside of the geofence, but more than the value of searchBuffer away from the closest geofence border, then the value is 999. Negative means the coordinate is inside of the geofence. If the coordinate is inside the polygon, but more than the value of searchBuffer away from the closest geofencing border,then the value is -999. A value of 999 means that there is great confidence the coordinate is well outside the geofence. A value of -999 means that there is great confidence the coordinate is well within the geofence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,
    #[doc = "The unique ID for the geofence geometry."]
    #[serde(rename = "geometryId", default, skip_serializing_if = "Option::is_none")]
    pub geometry_id: Option<String>,
    #[doc = "Latitude of the nearest point of the geometry."]
    #[serde(rename = "nearestLat", default, skip_serializing_if = "Option::is_none")]
    pub nearest_lat: Option<f32>,
    #[doc = "Longitude of the nearest point of the geometry."]
    #[serde(rename = "nearestLon", default, skip_serializing_if = "Option::is_none")]
    pub nearest_lon: Option<f32>,
    #[doc = "The unique id returned from user upload service when uploading a geofence. Will not be included in geofencing post API."]
    #[serde(rename = "udId", default, skip_serializing_if = "Option::is_none")]
    pub ud_id: Option<String>,
}
impl MapsGeofenceGeometry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Maps.GeofenceResult event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsGeofenceResultEventData {
    #[serde(flatten)]
    pub maps_geofence_event_properties: MapsGeofenceEventProperties,
}
impl MapsGeofenceResultEventData {
    pub fn new(maps_geofence_event_properties: MapsGeofenceEventProperties) -> Self {
        Self {
            maps_geofence_event_properties,
        }
    }
}
#[doc = "Job canceled event data. Schema of the data property of an EventGridEvent for a\nMicrosoft.Media.JobCanceled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobCanceledEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
    #[doc = "Gets the Job outputs."]
    pub outputs: Vec<MediaJobOutputUnion>,
}
impl MediaJobCanceledEventData {
    pub fn new(media_job_state_change_event_data: MediaJobStateChangeEventData, outputs: Vec<MediaJobOutputUnion>) -> Self {
        Self {
            media_job_state_change_event_data,
            outputs,
        }
    }
}
#[doc = "Job canceling event data. Schema of the data property of an EventGridEvent for\na Microsoft.Media.JobCanceling event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobCancelingEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
}
impl MediaJobCancelingEventData {
    pub fn new(media_job_state_change_event_data: MediaJobStateChangeEventData) -> Self {
        Self {
            media_job_state_change_event_data,
        }
    }
}
#[doc = "Details of JobOutput errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobError {
    #[doc = "Media Job Error Codes."]
    pub code: MediaJobErrorCode,
    #[doc = "A human-readable language-dependent representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error categories for Media Job Errors."]
    pub category: MediaJobErrorCategory,
    #[doc = "Media Job Retry Options."]
    pub retry: MediaJobRetry,
    #[doc = "An array of details about specific errors that led to this reported error."]
    pub details: Vec<MediaJobErrorDetail>,
}
impl MediaJobError {
    pub fn new(code: MediaJobErrorCode, category: MediaJobErrorCategory, retry: MediaJobRetry, details: Vec<MediaJobErrorDetail>) -> Self {
        Self {
            code,
            message: None,
            category,
            retry,
            details,
        }
    }
}
#[doc = "Error categories for Media Job Errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MediaJobErrorCategory")]
pub enum MediaJobErrorCategory {
    Service,
    Download,
    Upload,
    Configuration,
    Content,
    Account,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MediaJobErrorCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MediaJobErrorCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MediaJobErrorCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Service => serializer.serialize_unit_variant("MediaJobErrorCategory", 0u32, "Service"),
            Self::Download => serializer.serialize_unit_variant("MediaJobErrorCategory", 1u32, "Download"),
            Self::Upload => serializer.serialize_unit_variant("MediaJobErrorCategory", 2u32, "Upload"),
            Self::Configuration => serializer.serialize_unit_variant("MediaJobErrorCategory", 3u32, "Configuration"),
            Self::Content => serializer.serialize_unit_variant("MediaJobErrorCategory", 4u32, "Content"),
            Self::Account => serializer.serialize_unit_variant("MediaJobErrorCategory", 5u32, "Account"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Media Job Error Codes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MediaJobErrorCode")]
pub enum MediaJobErrorCode {
    ServiceError,
    ServiceTransientError,
    DownloadNotAccessible,
    DownloadTransientError,
    UploadNotAccessible,
    UploadTransientError,
    ConfigurationUnsupported,
    ContentMalformed,
    ContentUnsupported,
    IdentityUnsupported,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MediaJobErrorCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MediaJobErrorCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MediaJobErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ServiceError => serializer.serialize_unit_variant("MediaJobErrorCode", 0u32, "ServiceError"),
            Self::ServiceTransientError => serializer.serialize_unit_variant("MediaJobErrorCode", 1u32, "ServiceTransientError"),
            Self::DownloadNotAccessible => serializer.serialize_unit_variant("MediaJobErrorCode", 2u32, "DownloadNotAccessible"),
            Self::DownloadTransientError => serializer.serialize_unit_variant("MediaJobErrorCode", 3u32, "DownloadTransientError"),
            Self::UploadNotAccessible => serializer.serialize_unit_variant("MediaJobErrorCode", 4u32, "UploadNotAccessible"),
            Self::UploadTransientError => serializer.serialize_unit_variant("MediaJobErrorCode", 5u32, "UploadTransientError"),
            Self::ConfigurationUnsupported => serializer.serialize_unit_variant("MediaJobErrorCode", 6u32, "ConfigurationUnsupported"),
            Self::ContentMalformed => serializer.serialize_unit_variant("MediaJobErrorCode", 7u32, "ContentMalformed"),
            Self::ContentUnsupported => serializer.serialize_unit_variant("MediaJobErrorCode", 8u32, "ContentUnsupported"),
            Self::IdentityUnsupported => serializer.serialize_unit_variant("MediaJobErrorCode", 9u32, "IdentityUnsupported"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details of JobOutput errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobErrorDetail {
    #[doc = "Code describing the error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl MediaJobErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job error state event data. Schema of the data property of an EventGridEvent\nfor a Microsoft.Media.JobErrored event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobErroredEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
    #[doc = "Gets the Job outputs."]
    pub outputs: Vec<MediaJobOutputUnion>,
}
impl MediaJobErroredEventData {
    pub fn new(media_job_state_change_event_data: MediaJobStateChangeEventData, outputs: Vec<MediaJobOutputUnion>) -> Self {
        Self {
            media_job_state_change_event_data,
            outputs,
        }
    }
}
#[doc = "Job finished event data. Schema of the data property of an EventGridEvent for a\nMicrosoft.Media.JobFinished event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobFinishedEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
    #[doc = "Gets the Job outputs."]
    pub outputs: Vec<MediaJobOutputUnion>,
}
impl MediaJobFinishedEventData {
    pub fn new(media_job_state_change_event_data: MediaJobStateChangeEventData, outputs: Vec<MediaJobOutputUnion>) -> Self {
        Self {
            media_job_state_change_event_data,
            outputs,
        }
    }
}
#[doc = "The event data for a Job output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutput {
    #[doc = "Details of JobOutput errors."]
    pub error: MediaJobError,
    #[doc = "Gets the Job output label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets the Job output progress."]
    pub progress: i64,
    #[doc = "State of a Media Job."]
    pub state: MediaJobState,
}
impl MediaJobOutput {
    pub fn new(error: MediaJobError, progress: i64, state: MediaJobState) -> Self {
        Self {
            error,
            label: None,
            progress,
            state,
        }
    }
}
#[doc = "The discriminator for derived types."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@odata.type")]
pub enum MediaJobOutputUnion {
    #[serde(rename = "#Microsoft.Media.JobOutputAsset")]
    MicrosoftMediaJobOutputAsset(MediaJobOutputAsset),
}
#[doc = "The event data for a Job output asset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputAsset {
    #[serde(flatten)]
    pub media_job_output: MediaJobOutput,
    #[doc = "Gets the Job output asset name."]
    #[serde(rename = "assetName", default, skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
}
impl MediaJobOutputAsset {
    pub fn new(media_job_output: MediaJobOutput) -> Self {
        Self {
            media_job_output,
            asset_name: None,
        }
    }
}
#[doc = "Job output canceled event data. Schema of the data property of an\nEventGridEvent for a Microsoft.Media.JobOutputCanceled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputCanceledEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputCanceledEventData {
    pub fn new(media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData) -> Self {
        Self {
            media_job_output_state_change_event_data,
        }
    }
}
#[doc = "Job output canceling event data. Schema of the data property of an\nEventGridEvent for a Microsoft.Media.JobOutputCanceling event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputCancelingEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputCancelingEventData {
    pub fn new(media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData) -> Self {
        Self {
            media_job_output_state_change_event_data,
        }
    }
}
#[doc = "Job output error event data. Schema of the data property of an EventGridEvent\nfor a Microsoft.Media.JobOutputErrored event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputErroredEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputErroredEventData {
    pub fn new(media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData) -> Self {
        Self {
            media_job_output_state_change_event_data,
        }
    }
}
#[doc = "Job output finished event data. Schema of the data property of an\nEventGridEvent for a Microsoft.Media.JobOutputFinished event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputFinishedEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputFinishedEventData {
    pub fn new(media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData) -> Self {
        Self {
            media_job_output_state_change_event_data,
        }
    }
}
#[doc = "Job output processing event data. Schema of the data property of an\nEventGridEvent for a Microsoft.Media.JobOutputProcessing event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputProcessingEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputProcessingEventData {
    pub fn new(media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData) -> Self {
        Self {
            media_job_output_state_change_event_data,
        }
    }
}
#[doc = "Job Output Progress Event Data. Schema of the Data property of an\n  EventGridEvent for a Microsoft.Media.JobOutputProgress event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputProgressEventData {
    #[doc = "Gets the Job output label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets the Job output progress."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[doc = "Gets the Job correlation data."]
    #[serde(rename = "jobCorrelationData")]
    pub job_correlation_data: serde_json::Value,
}
impl MediaJobOutputProgressEventData {
    pub fn new(job_correlation_data: serde_json::Value) -> Self {
        Self {
            label: None,
            progress: None,
            job_correlation_data,
        }
    }
}
#[doc = "Job output scheduled event data. Schema of the data property of an\nEventGridEvent for a Microsoft.Media.JobOutputScheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputScheduledEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputScheduledEventData {
    pub fn new(media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData) -> Self {
        Self {
            media_job_output_state_change_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a\n  Microsoft.Media.JobOutputStateChange event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutputStateChangeEventData {
    #[doc = "State of a Media Job."]
    #[serde(rename = "previousState")]
    pub previous_state: MediaJobState,
    #[doc = "The event data for a Job output."]
    pub output: MediaJobOutputUnion,
    #[doc = "Gets the Job correlation data."]
    #[serde(rename = "jobCorrelationData")]
    pub job_correlation_data: serde_json::Value,
}
impl MediaJobOutputStateChangeEventData {
    pub fn new(previous_state: MediaJobState, output: MediaJobOutputUnion, job_correlation_data: serde_json::Value) -> Self {
        Self {
            previous_state,
            output,
            job_correlation_data,
        }
    }
}
#[doc = "Job processing event data. Schema of the data property of an EventGridEvent for\na Microsoft.Media.JobProcessing event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobProcessingEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
}
impl MediaJobProcessingEventData {
    pub fn new(media_job_state_change_event_data: MediaJobStateChangeEventData) -> Self {
        Self {
            media_job_state_change_event_data,
        }
    }
}
#[doc = "Media Job Retry Options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MediaJobRetry")]
pub enum MediaJobRetry {
    DoNotRetry,
    MayRetry,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MediaJobRetry {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MediaJobRetry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MediaJobRetry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DoNotRetry => serializer.serialize_unit_variant("MediaJobRetry", 0u32, "DoNotRetry"),
            Self::MayRetry => serializer.serialize_unit_variant("MediaJobRetry", 1u32, "MayRetry"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Job scheduled event data. Schema of the data property of an EventGridEvent for\na Microsoft.Media.JobScheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobScheduledEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
}
impl MediaJobScheduledEventData {
    pub fn new(media_job_state_change_event_data: MediaJobStateChangeEventData) -> Self {
        Self {
            media_job_state_change_event_data,
        }
    }
}
#[doc = "State of a Media Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MediaJobState")]
pub enum MediaJobState {
    Canceled,
    Canceling,
    Error,
    Finished,
    Processing,
    Queued,
    Scheduled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MediaJobState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MediaJobState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MediaJobState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Canceled => serializer.serialize_unit_variant("MediaJobState", 0u32, "Canceled"),
            Self::Canceling => serializer.serialize_unit_variant("MediaJobState", 1u32, "Canceling"),
            Self::Error => serializer.serialize_unit_variant("MediaJobState", 2u32, "Error"),
            Self::Finished => serializer.serialize_unit_variant("MediaJobState", 3u32, "Finished"),
            Self::Processing => serializer.serialize_unit_variant("MediaJobState", 4u32, "Processing"),
            Self::Queued => serializer.serialize_unit_variant("MediaJobState", 5u32, "Queued"),
            Self::Scheduled => serializer.serialize_unit_variant("MediaJobState", 6u32, "Scheduled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a\n  Microsoft.Media.JobStateChange event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobStateChangeEventData {
    #[doc = "State of a Media Job."]
    #[serde(rename = "previousState")]
    pub previous_state: MediaJobState,
    #[doc = "State of a Media Job."]
    pub state: MediaJobState,
    #[doc = "Gets the Job correlation data."]
    #[serde(rename = "correlationData")]
    pub correlation_data: serde_json::Value,
}
impl MediaJobStateChangeEventData {
    pub fn new(previous_state: MediaJobState, state: MediaJobState, correlation_data: serde_json::Value) -> Self {
        Self {
            previous_state,
            state,
            correlation_data,
        }
    }
}
#[doc = "Channel Archive heartbeat event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventChannelArchiveHeartbeat event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaLiveEventChannelArchiveHeartbeatEventData {
    #[doc = "Gets the channel latency in ms."]
    #[serde(rename = "channelLatencyMs")]
    pub channel_latency_ms: String,
    #[doc = "Gets the latency result code."]
    #[serde(rename = "latencyResultCode")]
    pub latency_result_code: String,
}
impl MediaLiveEventChannelArchiveHeartbeatEventData {
    pub fn new(channel_latency_ms: String, latency_result_code: String) -> Self {
        Self {
            channel_latency_ms,
            latency_result_code,
        }
    }
}
#[doc = "Encoder connection rejected event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventConnectionRejected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventConnectionRejectedEventData {
    #[doc = "Gets the ingest URL provided by the live event."]
    #[serde(rename = "ingestUrl", default, skip_serializing_if = "Option::is_none")]
    pub ingest_url: Option<String>,
    #[doc = "Gets the stream Id."]
    #[serde(rename = "streamId", default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[doc = "Gets the remote IP."]
    #[serde(rename = "encoderIp", default, skip_serializing_if = "Option::is_none")]
    pub encoder_ip: Option<String>,
    #[doc = "Gets the remote port."]
    #[serde(rename = "encoderPort", default, skip_serializing_if = "Option::is_none")]
    pub encoder_port: Option<String>,
    #[doc = "Gets the result code."]
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
}
impl MediaLiveEventConnectionRejectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encoder connect event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventEncoderConnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventEncoderConnectedEventData {
    #[doc = "Gets the ingest URL provided by the live event."]
    #[serde(rename = "ingestUrl", default, skip_serializing_if = "Option::is_none")]
    pub ingest_url: Option<String>,
    #[doc = "Gets the stream Id."]
    #[serde(rename = "streamId", default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[doc = "Gets the remote IP."]
    #[serde(rename = "encoderIp", default, skip_serializing_if = "Option::is_none")]
    pub encoder_ip: Option<String>,
    #[doc = "Gets the remote port."]
    #[serde(rename = "encoderPort", default, skip_serializing_if = "Option::is_none")]
    pub encoder_port: Option<String>,
}
impl MediaLiveEventEncoderConnectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encoder disconnected event data. Schema of the Data property of an EventGridEvent for a Microsoft.Media.LiveEventEncoderDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventEncoderDisconnectedEventData {
    #[doc = "Gets the ingest URL provided by the live event."]
    #[serde(rename = "ingestUrl", default, skip_serializing_if = "Option::is_none")]
    pub ingest_url: Option<String>,
    #[doc = "Gets the stream Id."]
    #[serde(rename = "streamId", default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[doc = "Gets the remote IP."]
    #[serde(rename = "encoderIp", default, skip_serializing_if = "Option::is_none")]
    pub encoder_ip: Option<String>,
    #[doc = "Gets the remote port."]
    #[serde(rename = "encoderPort", default, skip_serializing_if = "Option::is_none")]
    pub encoder_port: Option<String>,
    #[doc = "Gets the result code."]
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
}
impl MediaLiveEventEncoderDisconnectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ingest fragment dropped event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventIncomingDataChunkDropped event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventIncomingDataChunkDroppedEventData {
    #[doc = "Gets the timestamp of the data chunk dropped."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "Gets the type of the track (Audio / Video)."]
    #[serde(rename = "trackType", default, skip_serializing_if = "Option::is_none")]
    pub track_type: Option<String>,
    #[doc = "Gets the bitrate of the track."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[doc = "Gets the timescale of the Timestamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timescale: Option<String>,
    #[doc = "Gets the result code for fragment drop operation."]
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    #[doc = "Gets the name of the track for which fragment is dropped."]
    #[serde(rename = "trackName", default, skip_serializing_if = "Option::is_none")]
    pub track_name: Option<String>,
}
impl MediaLiveEventIncomingDataChunkDroppedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encoder connect event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventIncomingStreamReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventIncomingStreamReceivedEventData {
    #[doc = "Gets the ingest URL provided by the live event."]
    #[serde(rename = "ingestUrl", default, skip_serializing_if = "Option::is_none")]
    pub ingest_url: Option<String>,
    #[doc = "Gets the type of the track (Audio / Video)."]
    #[serde(rename = "trackType", default, skip_serializing_if = "Option::is_none")]
    pub track_type: Option<String>,
    #[doc = "Gets the track name."]
    #[serde(rename = "trackName", default, skip_serializing_if = "Option::is_none")]
    pub track_name: Option<String>,
    #[doc = "Gets the bitrate of the track."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[doc = "Gets the remote IP."]
    #[serde(rename = "encoderIp", default, skip_serializing_if = "Option::is_none")]
    pub encoder_ip: Option<String>,
    #[doc = "Gets the remote port."]
    #[serde(rename = "encoderPort", default, skip_serializing_if = "Option::is_none")]
    pub encoder_port: Option<String>,
    #[doc = "Gets the first timestamp of the data chunk received."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "Gets the duration of the first data chunk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Gets the timescale in which timestamp is represented."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timescale: Option<String>,
}
impl MediaLiveEventIncomingStreamReceivedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Incoming streams out of sync event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventIncomingStreamsOutOfSync event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventIncomingStreamsOutOfSyncEventData {
    #[doc = "Gets the minimum last timestamp received."]
    #[serde(rename = "minLastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub min_last_timestamp: Option<String>,
    #[doc = "Gets the type of stream with minimum last timestamp."]
    #[serde(rename = "typeOfStreamWithMinLastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub type_of_stream_with_min_last_timestamp: Option<String>,
    #[doc = "Gets the maximum timestamp among all the tracks (audio or video)."]
    #[serde(rename = "maxLastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub max_last_timestamp: Option<String>,
    #[doc = "Gets the type of stream with maximum last timestamp."]
    #[serde(rename = "typeOfStreamWithMaxLastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub type_of_stream_with_max_last_timestamp: Option<String>,
    #[doc = "Gets the timescale in which \\\"MinLastTimestamp\\\" is represented."]
    #[serde(rename = "timescaleOfMinLastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub timescale_of_min_last_timestamp: Option<String>,
    #[doc = "Gets the timescale in which \\\"MaxLastTimestamp\\\" is represented."]
    #[serde(rename = "timescaleOfMaxLastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub timescale_of_max_last_timestamp: Option<String>,
}
impl MediaLiveEventIncomingStreamsOutOfSyncEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Incoming video stream out of sync event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventIncomingVideoStreamsOutOfSync event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventIncomingVideoStreamsOutOfSyncEventData {
    #[doc = "Gets the first timestamp received for one of the quality levels."]
    #[serde(rename = "firstTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    #[doc = "Gets the duration of the data chunk with first timestamp."]
    #[serde(rename = "firstDuration", default, skip_serializing_if = "Option::is_none")]
    pub first_duration: Option<String>,
    #[doc = "Gets the timestamp received for some other quality levels."]
    #[serde(rename = "secondTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub second_timestamp: Option<String>,
    #[doc = "Gets the duration of the data chunk with second timestamp."]
    #[serde(rename = "secondDuration", default, skip_serializing_if = "Option::is_none")]
    pub second_duration: Option<String>,
    #[doc = "Gets the timescale in which both the timestamps and durations are represented."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timescale: Option<String>,
}
impl MediaLiveEventIncomingVideoStreamsOutOfSyncEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ingest heartbeat event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventIngestHeartbeat event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventIngestHeartbeatEventData {
    #[doc = "Gets the type of the track (Audio / Video)."]
    #[serde(rename = "trackType", default, skip_serializing_if = "Option::is_none")]
    pub track_type: Option<String>,
    #[doc = "Gets the track name."]
    #[serde(rename = "trackName", default, skip_serializing_if = "Option::is_none")]
    pub track_name: Option<String>,
    #[doc = "Gets the Live Transcription language."]
    #[serde(rename = "transcriptionLanguage", default, skip_serializing_if = "Option::is_none")]
    pub transcription_language: Option<String>,
    #[doc = "Gets the Live Transcription state."]
    #[serde(rename = "transcriptionState", default, skip_serializing_if = "Option::is_none")]
    pub transcription_state: Option<String>,
    #[doc = "Gets the bitrate of the track."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[doc = "Gets the incoming bitrate."]
    #[serde(rename = "incomingBitrate", default, skip_serializing_if = "Option::is_none")]
    pub incoming_bitrate: Option<i64>,
    #[doc = "Gets the track ingest drift value."]
    #[serde(rename = "ingestDriftValue", default, skip_serializing_if = "Option::is_none")]
    pub ingest_drift_value: Option<String>,
    #[doc = "Gets the arrival UTC time of the last fragment."]
    #[serde(rename = "lastFragmentArrivalTime", default, skip_serializing_if = "Option::is_none")]
    pub last_fragment_arrival_time: Option<String>,
    #[doc = "Gets the last timestamp."]
    #[serde(rename = "lastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    #[doc = "Gets the timescale of the last timestamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timescale: Option<String>,
    #[doc = "Gets the fragment Overlap count."]
    #[serde(rename = "overlapCount", default, skip_serializing_if = "Option::is_none")]
    pub overlap_count: Option<i64>,
    #[doc = "Gets the fragment Discontinuity count."]
    #[serde(rename = "discontinuityCount", default, skip_serializing_if = "Option::is_none")]
    pub discontinuity_count: Option<i64>,
    #[doc = "Gets Non increasing count."]
    #[serde(rename = "nonincreasingCount", default, skip_serializing_if = "Option::is_none")]
    pub nonincreasing_count: Option<i64>,
    #[doc = "Gets a value indicating whether unexpected bitrate is present or not."]
    #[serde(rename = "unexpectedBitrate", default, skip_serializing_if = "Option::is_none")]
    pub unexpected_bitrate: Option<bool>,
    #[doc = "Gets the state of the live event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Gets a value indicating whether preview is healthy or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
}
impl MediaLiveEventIngestHeartbeatEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Ingest track discontinuity detected event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.LiveEventTrackDiscontinuityDetected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaLiveEventTrackDiscontinuityDetectedEventData {
    #[doc = "Gets the type of the track (Audio / Video)."]
    #[serde(rename = "trackType", default, skip_serializing_if = "Option::is_none")]
    pub track_type: Option<String>,
    #[doc = "Gets the track name."]
    #[serde(rename = "trackName", default, skip_serializing_if = "Option::is_none")]
    pub track_name: Option<String>,
    #[doc = "Gets the bitrate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[doc = "Gets the timestamp of the previous fragment."]
    #[serde(rename = "previousTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub previous_timestamp: Option<String>,
    #[doc = "Gets the timestamp of the current fragment."]
    #[serde(rename = "newTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub new_timestamp: Option<String>,
    #[doc = "Gets the timescale in which both timestamps and discontinuity gap are represented."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timescale: Option<String>,
    #[doc = "Gets the discontinuity gap between PreviousTimestamp and NewTimestamp."]
    #[serde(rename = "discontinuityGap", default, skip_serializing_if = "Option::is_none")]
    pub discontinuity_gap: Option<String>,
}
impl MediaLiveEventTrackDiscontinuityDetectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Microsoft Teams application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftTeamsAppIdentifierModel {
    #[doc = "The Id of the Microsoft Teams application"]
    #[serde(rename = "appId")]
    pub app_id: String,
    #[doc = "Communication cloud environment model."]
    pub cloud: CommunicationCloudEnvironmentModel,
}
impl MicrosoftTeamsAppIdentifierModel {
    pub fn new(app_id: String, cloud: CommunicationCloudEnvironmentModel) -> Self {
        Self { app_id, cloud }
    }
}
#[doc = "A Microsoft Teams user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftTeamsUserIdentifierModel {
    #[doc = "The Id of the Microsoft Teams user. If not anonymous, this is the AAD object Id of the user."]
    #[serde(rename = "userId")]
    pub user_id: String,
    #[doc = "True if the Microsoft Teams user is anonymous. By default false if missing."]
    #[serde(rename = "isAnonymous", default, skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[doc = "Communication cloud environment model."]
    pub cloud: CommunicationCloudEnvironmentModel,
}
impl MicrosoftTeamsUserIdentifierModel {
    pub fn new(user_id: String, cloud: CommunicationCloudEnvironmentModel) -> Self {
        Self {
            user_id,
            is_anonymous: None,
            cloud,
        }
    }
}
#[doc = "A phone number."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhoneNumberIdentifierModel {
    #[doc = "The phone number in E.164 format."]
    pub value: String,
}
impl PhoneNumberIdentifierModel {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.PolicyInsights.PolicyStateChanged event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyInsightsPolicyStateChangedEventData {
    #[doc = "The time that the resource was scanned by Azure Policy in the Universal ISO 8601 DateTime format yyyy-MM-ddTHH:mm:ss.fffffffZ."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The resource ID of the policy assignment."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "The resource ID of the policy definition."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The reference ID for the policy definition inside the initiative definition, if the policy assignment is for an initiative. May be empty."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "The compliance state of the resource with respect to the policy assignment."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The compliance reason code. May be empty."]
    #[serde(rename = "complianceReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub compliance_reason_code: Option<String>,
}
impl PolicyInsightsPolicyStateChangedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            policy_assignment_id: None,
            policy_definition_id: None,
            policy_definition_reference_id: None,
            compliance_state: None,
            subscription_id: None,
            compliance_reason_code: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.PolicyInsights.PolicyStateCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyInsightsPolicyStateCreatedEventData {
    #[doc = "The time that the resource was scanned by Azure Policy in the Universal ISO 8601 DateTime format yyyy-MM-ddTHH:mm:ss.fffffffZ."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The resource ID of the policy assignment."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "The resource ID of the policy definition."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The reference ID for the policy definition inside the initiative definition, if the policy assignment is for an initiative. May be empty."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "The compliance state of the resource with respect to the policy assignment."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The compliance reason code. May be empty."]
    #[serde(rename = "complianceReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub compliance_reason_code: Option<String>,
}
impl PolicyInsightsPolicyStateCreatedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            policy_assignment_id: None,
            policy_definition_id: None,
            policy_definition_reference_id: None,
            compliance_state: None,
            subscription_id: None,
            compliance_reason_code: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.PolicyInsights.PolicyStateDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyInsightsPolicyStateDeletedEventData {
    #[doc = "The time that the resource was scanned by Azure Policy in the Universal ISO 8601 DateTime format yyyy-MM-ddTHH:mm:ss.fffffffZ."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The resource ID of the policy assignment."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "The resource ID of the policy definition."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The reference ID for the policy definition inside the initiative definition, if the policy assignment is for an initiative. May be empty."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "The compliance state of the resource with respect to the policy assignment."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The compliance reason code. May be empty."]
    #[serde(rename = "complianceReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub compliance_reason_code: Option<String>,
}
impl PolicyInsightsPolicyStateDeletedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            policy_assignment_id: None,
            policy_definition_id: None,
            policy_definition_reference_id: None,
            compliance_state: None,
            subscription_id: None,
            compliance_reason_code: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Cache.ExportRDBCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisExportRdbCompletedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The name of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of this event. Failed or  succeeded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RedisExportRdbCompletedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            name: None,
            status: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Cache.ImportRDBCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisImportRdbCompletedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The name of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of this event. Failed or  succeeded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RedisImportRdbCompletedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            name: None,
            status: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Cache.PatchingCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisPatchingCompletedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The name of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of this event. Failed or  succeeded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RedisPatchingCompletedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            name: None,
            status: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Cache.ScalingCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisScalingCompletedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The name of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of this event. Failed or  succeeded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RedisScalingCompletedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            name: None,
            status: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceActionCancel event. This is raised when a resource action operation is canceled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceActionCancelEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceActionCancelEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceActionFailure event. This is raised when a resource action operation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceActionFailureEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceActionFailureEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceActionSuccess event. This is raised when a resource action operation succeeds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceActionSuccessEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceActionSuccessEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "The details of the authorization for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceAuthorization {
    #[doc = "The scope of the authorization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The action being requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The evidence for the authorization."]
    pub evidence: serde_json::Value,
}
impl ResourceAuthorization {
    pub fn new(evidence: serde_json::Value) -> Self {
        Self {
            scope: None,
            action: None,
            evidence,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceDeleteCancel event. This is raised when a resource delete operation is canceled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceDeleteCancelEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceDeleteCancelEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceDeleteFailure event. This is raised when a resource delete operation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceDeleteFailureEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceDeleteFailureEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceDeleteSuccess event. This is raised when a resource delete operation succeeds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceDeleteSuccessEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceDeleteSuccessEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "The details of the HTTP request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceHttpRequest {
    #[doc = "The client request ID."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The client IP address."]
    #[serde(rename = "clientIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub client_ip_address: Option<String>,
    #[doc = "The request method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "The url used in the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ResourceHttpRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a\nMicrosoft.ResourceNotifications.HealthResources.ResourceAnnotated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotificationsHealthResourcesAnnotatedEventData {
    #[serde(flatten)]
    pub resource_notifications_resource_updated_event_data: ResourceNotificationsResourceUpdatedEventData,
}
impl ResourceNotificationsHealthResourcesAnnotatedEventData {
    pub fn new(resource_notifications_resource_updated_event_data: ResourceNotificationsResourceUpdatedEventData) -> Self {
        Self {
            resource_notifications_resource_updated_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a\nMicrosoft.ResourceNotifications.HealthResources.AvailabilityStatusChanged\nevent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotificationsHealthResourcesAvailabilityStatusChangedEventData {
    #[serde(flatten)]
    pub resource_notifications_resource_updated_event_data: ResourceNotificationsResourceUpdatedEventData,
}
impl ResourceNotificationsHealthResourcesAvailabilityStatusChangedEventData {
    pub fn new(resource_notifications_resource_updated_event_data: ResourceNotificationsResourceUpdatedEventData) -> Self {
        Self {
            resource_notifications_resource_updated_event_data,
        }
    }
}
#[doc = "details of operational info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotificationsOperationalDetails {
    #[doc = "Date and Time when resource was updated"]
    #[serde(rename = "resourceEventTime", with = "azure_core::date::rfc3339")]
    pub resource_event_time: ::time::OffsetDateTime,
}
impl ResourceNotificationsOperationalDetails {
    pub fn new(resource_event_time: ::time::OffsetDateTime) -> Self {
        Self { resource_event_time }
    }
}
#[doc = "Describes the schema of the properties under resource info which are common\nacross all ARN system topic delete events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNotificationsResourceDeletedDetails {
    #[doc = "id of the resource for which the event is being emitted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of the resource for which the event is being emitted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the type of the resource for which the event is being emitted"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ResourceNotificationsResourceDeletedDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the schema of the common properties across all ARN system topic\ndelete events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotificationsResourceDeletedEventData {
    #[doc = "Describes the schema of the properties under resource info which are common\nacross all ARN system topic delete events"]
    #[serde(rename = "resourceInfo")]
    pub resource_info: ResourceNotificationsResourceDeletedDetails,
    #[doc = "details of operational info"]
    #[serde(rename = "operationalInfo")]
    pub operational_info: ResourceNotificationsOperationalDetails,
}
impl ResourceNotificationsResourceDeletedEventData {
    pub fn new(
        resource_info: ResourceNotificationsResourceDeletedDetails,
        operational_info: ResourceNotificationsOperationalDetails,
    ) -> Self {
        Self {
            resource_info,
            operational_info,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a\nMicrosoft.ResourceNotifications.Resources.CreatedOrUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotificationsResourceManagementCreatedOrUpdatedEventData {
    #[serde(flatten)]
    pub resource_notifications_resource_updated_event_data: ResourceNotificationsResourceUpdatedEventData,
}
impl ResourceNotificationsResourceManagementCreatedOrUpdatedEventData {
    pub fn new(resource_notifications_resource_updated_event_data: ResourceNotificationsResourceUpdatedEventData) -> Self {
        Self {
            resource_notifications_resource_updated_event_data,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a\nMicrosoft.ResourceNotifications.Resources.Deleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotificationsResourceManagementDeletedEventData {
    #[serde(flatten)]
    pub resource_notifications_resource_deleted_event_data: ResourceNotificationsResourceDeletedEventData,
}
impl ResourceNotificationsResourceManagementDeletedEventData {
    pub fn new(resource_notifications_resource_deleted_event_data: ResourceNotificationsResourceDeletedEventData) -> Self {
        Self {
            resource_notifications_resource_deleted_event_data,
        }
    }
}
#[doc = "Describes the schema of the properties under resource info which are common\nacross all ARN system topic events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotificationsResourceUpdatedDetails {
    #[doc = "id of the resource for which the event is being emitted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of the resource for which the event is being emitted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the type of the resource for which the event is being emitted"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "the location of the resource for which the event is being emitted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "the tags on the resource for which the event is being emitted"]
    pub tags: serde_json::Value,
    #[doc = "properties in the payload of the resource for which the event is being emitted"]
    pub properties: serde_json::Value,
}
impl ResourceNotificationsResourceUpdatedDetails {
    pub fn new(tags: serde_json::Value, properties: serde_json::Value) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location: None,
            tags,
            properties,
        }
    }
}
#[doc = "Describes the schema of the common properties across all ARN system topic events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotificationsResourceUpdatedEventData {
    #[doc = "Describes the schema of the properties under resource info which are common\nacross all ARN system topic events"]
    #[serde(rename = "resourceInfo")]
    pub resource_info: ResourceNotificationsResourceUpdatedDetails,
    #[doc = "details of operational info"]
    #[serde(rename = "operationalInfo")]
    pub operational_info: ResourceNotificationsOperationalDetails,
    #[doc = "api version of the resource properties bag"]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}
impl ResourceNotificationsResourceUpdatedEventData {
    pub fn new(
        resource_info: ResourceNotificationsResourceUpdatedDetails,
        operational_info: ResourceNotificationsOperationalDetails,
    ) -> Self {
        Self {
            resource_info,
            operational_info,
            api_version: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceWriteCancel event. This is raised when a resource create or update operation is canceled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceWriteCancelEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceWriteCancelEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceWriteFailure event. This is raised when a resource create or update operation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceWriteFailureEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceWriteFailureEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceWriteSuccess event. This is raised when a resource create or update operation succeeds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceWriteSuccessEventData {
    #[doc = "The tenant ID of the resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The subscription ID of the resource."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group of the resource."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "The resource provider performing the operation."]
    #[serde(rename = "resourceProvider", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider: Option<String>,
    #[doc = "The URI of the resource in the operation."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The operation that was performed."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The details of the authorization for the resource."]
    pub authorization: ResourceAuthorization,
    #[doc = "The properties of the claims."]
    pub claims: serde_json::Value,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest")]
    pub http_request: ResourceHttpRequest,
}
impl ResourceWriteSuccessEventData {
    pub fn new(authorization: ResourceAuthorization, claims: serde_json::Value, http_request: ResourceHttpRequest) -> Self {
        Self {
            tenant_id: None,
            subscription_id: None,
            resource_group: None,
            resource_provider: None,
            resource_uri: None,
            operation_name: None,
            status: None,
            authorization,
            claims,
            correlation_id: None,
            http_request,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ServiceBus.ActiveMessagesAvailablePeriodicNotifications event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusActiveMessagesAvailablePeriodicNotificationsEventData {
    #[doc = "The namespace name of the Microsoft.ServiceBus resource."]
    #[serde(rename = "namespaceName", default, skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    #[doc = "The endpoint of the Microsoft.ServiceBus resource."]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[doc = "The entity type of the Microsoft.ServiceBus resource. Could be one of 'queue' or 'subscriber'."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus queue. If the entity type is of type 'subscriber', then this value will be null."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus topic. If the entity type is of type 'queue', then this value will be null."]
    #[serde(rename = "topicName", default, skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus topic's subscription. If the entity type is of type 'queue', then this value will be null."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}
impl ServiceBusActiveMessagesAvailablePeriodicNotificationsEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ServiceBus.ActiveMessagesAvailableWithNoListeners event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusActiveMessagesAvailableWithNoListenersEventData {
    #[doc = "The namespace name of the Microsoft.ServiceBus resource."]
    #[serde(rename = "namespaceName", default, skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    #[doc = "The endpoint of the Microsoft.ServiceBus resource."]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[doc = "The entity type of the Microsoft.ServiceBus resource. Could be one of 'queue' or 'subscriber'."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus queue. If the entity type is of type 'subscriber', then this value will be null."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus topic. If the entity type is of type 'queue', then this value will be null."]
    #[serde(rename = "topicName", default, skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus topic's subscription. If the entity type is of type 'queue', then this value will be null."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}
impl ServiceBusActiveMessagesAvailableWithNoListenersEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ServiceBus.DeadletterMessagesAvailablePeriodicNotifications event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusDeadletterMessagesAvailablePeriodicNotificationsEventData {
    #[doc = "The namespace name of the Microsoft.ServiceBus resource."]
    #[serde(rename = "namespaceName", default, skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    #[doc = "The endpoint of the Microsoft.ServiceBus resource."]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[doc = "The entity type of the Microsoft.ServiceBus resource. Could be one of 'queue' or 'subscriber'."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus queue. If the entity type is of type 'subscriber', then this value will be null."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus topic. If the entity type is of type 'queue', then this value will be null."]
    #[serde(rename = "topicName", default, skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus topic's subscription. If the entity type is of type 'queue', then this value will be null."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}
impl ServiceBusDeadletterMessagesAvailablePeriodicNotificationsEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ServiceBus.DeadletterMessagesAvailableWithNoListeners event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusDeadletterMessagesAvailableWithNoListenersEventData {
    #[doc = "The namespace name of the Microsoft.ServiceBus resource."]
    #[serde(rename = "namespaceName", default, skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    #[doc = "The endpoint of the Microsoft.ServiceBus resource."]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[doc = "The entity type of the Microsoft.ServiceBus resource. Could be one of 'queue' or 'subscriber'."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus queue. If the entity type is of type 'subscriber', then this value will be null."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus topic. If the entity type is of type 'queue', then this value will be null."]
    #[serde(rename = "topicName", default, skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[doc = "The name of the Microsoft.ServiceBus topic's subscription. If the entity type is of type 'queue', then this value will be null."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}
impl ServiceBusDeadletterMessagesAvailableWithNoListenersEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.SignalRService.ClientConnectionConnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalRServiceClientConnectionConnectedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The hub of connected client connection."]
    #[serde(rename = "hubName", default, skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[doc = "The connection Id of connected client connection."]
    #[serde(rename = "connectionId", default, skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[doc = "The user Id of connected client connection."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl SignalRServiceClientConnectionConnectedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            hub_name: None,
            connection_id: None,
            user_id: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.SignalRService.ClientConnectionDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalRServiceClientConnectionDisconnectedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: ::time::OffsetDateTime,
    #[doc = "The hub of connected client connection."]
    #[serde(rename = "hubName", default, skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[doc = "The connection Id of connected client connection."]
    #[serde(rename = "connectionId", default, skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[doc = "The user Id of connected client connection."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The message of error that cause the client connection disconnected."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl SignalRServiceClientConnectionDisconnectedEventData {
    pub fn new(timestamp: ::time::OffsetDateTime) -> Self {
        Self {
            timestamp,
            hub_name: None,
            connection_id: None,
            user_id: None,
            error_message: None,
        }
    }
}
#[doc = "Kind of environment where app service plan is."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StampKind")]
pub enum StampKind {
    Public,
    AseV1,
    AseV2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StampKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StampKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StampKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Public => serializer.serialize_unit_variant("StampKind", 0u32, "Public"),
            Self::AseV1 => serializer.serialize_unit_variant("StampKind", 1u32, "AseV1"),
            Self::AseV2 => serializer.serialize_unit_variant("StampKind", 2u32, "AseV2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.AsyncOperationInitiated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAsyncOperationInitiatedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the storage service for the storage API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The content type of the blob. This is the same as what would be returned in the Content-Type header from the blob."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The size of the blob in bytes. This is the same as what would be returned in the Content-Length header from the blob."]
    #[serde(rename = "contentLength", default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[doc = "The type of blob."]
    #[serde(rename = "blobType", default, skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<String>,
    #[doc = "The path to the blob."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "An opaque string value representing the logical sequence of events for any particular blob name. Users can use standard string comparison to understand the relative sequence of two events on the same blob name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequencer: Option<String>,
    #[doc = "The identity of the requester that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "For service use only. Diagnostic data occasionally included by the Azure Storage service. This property should be ignored by event consumers."]
    #[serde(rename = "storageDiagnostics")]
    pub storage_diagnostics: serde_json::Value,
}
impl StorageAsyncOperationInitiatedEventData {
    pub fn new(storage_diagnostics: serde_json::Value) -> Self {
        Self {
            api: None,
            client_request_id: None,
            request_id: None,
            content_type: None,
            content_length: None,
            blob_type: None,
            url: None,
            sequencer: None,
            identity: None,
            storage_diagnostics,
        }
    }
}
#[doc = "The access tier of the blob."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StorageBlobAccessTier")]
pub enum StorageBlobAccessTier {
    Hot,
    Cool,
    Cold,
    Archive,
    Default,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StorageBlobAccessTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StorageBlobAccessTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StorageBlobAccessTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Hot => serializer.serialize_unit_variant("StorageBlobAccessTier", 0u32, "Hot"),
            Self::Cool => serializer.serialize_unit_variant("StorageBlobAccessTier", 1u32, "Cool"),
            Self::Cold => serializer.serialize_unit_variant("StorageBlobAccessTier", 2u32, "Cold"),
            Self::Archive => serializer.serialize_unit_variant("StorageBlobAccessTier", 3u32, "Archive"),
            Self::Default => serializer.serialize_unit_variant("StorageBlobAccessTier", 4u32, "Default"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.BlobCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageBlobCreatedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the storage service for the storage API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The etag of the blob at the time this event was triggered."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "The content type of the blob. This is the same as what would be returned in the Content-Type header from the blob."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The size of the blob in bytes. This is the same as what would be returned in the Content-Length header from the blob."]
    #[serde(rename = "contentLength", default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[doc = "The offset of the blob in bytes."]
    #[serde(rename = "contentOffset", default, skip_serializing_if = "Option::is_none")]
    pub content_offset: Option<i64>,
    #[doc = "The type of blob."]
    #[serde(rename = "blobType", default, skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<String>,
    #[doc = "The access tier of the blob."]
    #[serde(rename = "accessTier")]
    pub access_tier: StorageBlobAccessTier,
    #[doc = "The path to the blob."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "An opaque string value representing the logical sequence of events for any particular blob name. Users can use standard string comparison to understand the relative sequence of two events on the same blob name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequencer: Option<String>,
    #[doc = "The identity of the requester that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "For service use only. Diagnostic data occasionally included by the Azure Storage service. This property should be ignored by event consumers."]
    #[serde(rename = "storageDiagnostics")]
    pub storage_diagnostics: serde_json::Value,
}
impl StorageBlobCreatedEventData {
    pub fn new(access_tier: StorageBlobAccessTier, storage_diagnostics: serde_json::Value) -> Self {
        Self {
            api: None,
            client_request_id: None,
            request_id: None,
            e_tag: None,
            content_type: None,
            content_length: None,
            content_offset: None,
            blob_type: None,
            access_tier,
            url: None,
            sequencer: None,
            identity: None,
            storage_diagnostics,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.BlobDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageBlobDeletedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the storage service for the storage API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The content type of the blob. This is the same as what would be returned in the Content-Type header from the blob."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The type of blob."]
    #[serde(rename = "blobType", default, skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<String>,
    #[doc = "The path to the blob."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "An opaque string value representing the logical sequence of events for any particular blob name. Users can use standard string comparison to understand the relative sequence of two events on the same blob name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequencer: Option<String>,
    #[doc = "The identity of the requester that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "For service use only. Diagnostic data occasionally included by the Azure Storage service. This property should be ignored by event consumers."]
    #[serde(rename = "storageDiagnostics")]
    pub storage_diagnostics: serde_json::Value,
}
impl StorageBlobDeletedEventData {
    pub fn new(storage_diagnostics: serde_json::Value) -> Self {
        Self {
            api: None,
            client_request_id: None,
            request_id: None,
            content_type: None,
            blob_type: None,
            url: None,
            sequencer: None,
            identity: None,
            storage_diagnostics,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Storage.BlobInventoryPolicyCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageBlobInventoryPolicyCompletedEventData {
    #[doc = "The time at which inventory policy was scheduled."]
    #[serde(rename = "scheduleDateTime", with = "azure_core::date::rfc3339")]
    pub schedule_date_time: ::time::OffsetDateTime,
    #[doc = "The account name for which inventory policy is registered."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The rule name for inventory policy."]
    #[serde(rename = "ruleName", default, skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[doc = "The status of inventory run, it can be Succeeded/PartiallySucceeded/Failed."]
    #[serde(rename = "policyRunStatus", default, skip_serializing_if = "Option::is_none")]
    pub policy_run_status: Option<String>,
    #[doc = "The status message for inventory run."]
    #[serde(rename = "policyRunStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub policy_run_status_message: Option<String>,
    #[doc = "The policy run id for inventory run."]
    #[serde(rename = "policyRunId", default, skip_serializing_if = "Option::is_none")]
    pub policy_run_id: Option<String>,
    #[doc = "The blob URL for manifest file for inventory run."]
    #[serde(rename = "manifestBlobUrl", default, skip_serializing_if = "Option::is_none")]
    pub manifest_blob_url: Option<String>,
}
impl StorageBlobInventoryPolicyCompletedEventData {
    pub fn new(schedule_date_time: ::time::OffsetDateTime) -> Self {
        Self {
            schedule_date_time,
            account_name: None,
            rule_name: None,
            policy_run_status: None,
            policy_run_status_message: None,
            policy_run_id: None,
            manifest_blob_url: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.BlobRenamed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageBlobRenamedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the storage service for the storage API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The path to the blob that was renamed."]
    #[serde(rename = "sourceUrl", default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[doc = "The new path to the blob after the rename operation."]
    #[serde(rename = "destinationUrl", default, skip_serializing_if = "Option::is_none")]
    pub destination_url: Option<String>,
    #[doc = "An opaque string value representing the logical sequence of events for any particular blob name. Users can use standard string comparison to understand the relative sequence of two events on the same blob name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequencer: Option<String>,
    #[doc = "The identity of the requester that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "For service use only. Diagnostic data occasionally included by the Azure Storage service. This property should be ignored by event consumers."]
    #[serde(rename = "storageDiagnostics")]
    pub storage_diagnostics: serde_json::Value,
}
impl StorageBlobRenamedEventData {
    pub fn new(storage_diagnostics: serde_json::Value) -> Self {
        Self {
            api: None,
            client_request_id: None,
            request_id: None,
            source_url: None,
            destination_url: None,
            sequencer: None,
            identity: None,
            storage_diagnostics,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.BlobTierChanged event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageBlobTierChangedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the storage service for the storage API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The content type of the blob. This is the same as what would be returned in the Content-Type header from the blob."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The size of the blob in bytes. This is the same as what would be returned in the Content-Length header from the blob."]
    #[serde(rename = "contentLength", default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[doc = "The type of blob."]
    #[serde(rename = "blobType", default, skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<String>,
    #[doc = "The access tier of the blob."]
    #[serde(rename = "accessTier")]
    pub access_tier: StorageBlobAccessTier,
    #[doc = "The access tier of the blob."]
    #[serde(rename = "previousTier")]
    pub previous_tier: StorageBlobAccessTier,
    #[doc = "The path to the blob."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "An opaque string value representing the logical sequence of events for any particular blob name. Users can use standard string comparison to understand the relative sequence of two events on the same blob name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequencer: Option<String>,
    #[doc = "The identity of the requester that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "For service use only. Diagnostic data occasionally included by the Azure Storage service. This property should be ignored by event consumers."]
    #[serde(rename = "storageDiagnostics")]
    pub storage_diagnostics: serde_json::Value,
}
impl StorageBlobTierChangedEventData {
    pub fn new(access_tier: StorageBlobAccessTier, previous_tier: StorageBlobAccessTier, storage_diagnostics: serde_json::Value) -> Self {
        Self {
            api: None,
            client_request_id: None,
            request_id: None,
            content_type: None,
            content_length: None,
            blob_type: None,
            access_tier,
            previous_tier,
            url: None,
            sequencer: None,
            identity: None,
            storage_diagnostics,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.DirectoryCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageDirectoryCreatedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the storage service for the storage API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The etag of the directory at the time this event was triggered."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "The path to the directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "An opaque string value representing the logical sequence of events for any particular directory name. Users can use standard string comparison to understand the relative sequence of two events on the same directory name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequencer: Option<String>,
    #[doc = "The identity of the requester that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "For service use only. Diagnostic data occasionally included by the Azure Storage service. This property should be ignored by event consumers."]
    #[serde(rename = "storageDiagnostics")]
    pub storage_diagnostics: serde_json::Value,
}
impl StorageDirectoryCreatedEventData {
    pub fn new(storage_diagnostics: serde_json::Value) -> Self {
        Self {
            api: None,
            client_request_id: None,
            request_id: None,
            e_tag: None,
            url: None,
            sequencer: None,
            identity: None,
            storage_diagnostics,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.DirectoryDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageDirectoryDeletedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the storage service for the storage API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The path to the deleted directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Is this event for a recursive delete operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive: Option<String>,
    #[doc = "An opaque string value representing the logical sequence of events for any particular directory name. Users can use standard string comparison to understand the relative sequence of two events on the same directory name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequencer: Option<String>,
    #[doc = "The identity of the requester that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "For service use only. Diagnostic data occasionally included by the Azure Storage service. This property should be ignored by event consumers."]
    #[serde(rename = "storageDiagnostics")]
    pub storage_diagnostics: serde_json::Value,
}
impl StorageDirectoryDeletedEventData {
    pub fn new(storage_diagnostics: serde_json::Value) -> Self {
        Self {
            api: None,
            client_request_id: None,
            request_id: None,
            url: None,
            recursive: None,
            sequencer: None,
            identity: None,
            storage_diagnostics,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.DirectoryRenamed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageDirectoryRenamedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the storage service for the storage API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The path to the directory that was renamed."]
    #[serde(rename = "sourceUrl", default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[doc = "The new path to the directory after the rename operation."]
    #[serde(rename = "destinationUrl", default, skip_serializing_if = "Option::is_none")]
    pub destination_url: Option<String>,
    #[doc = "An opaque string value representing the logical sequence of events for any particular directory name. Users can use standard string comparison to understand the relative sequence of two events on the same directory name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequencer: Option<String>,
    #[doc = "The identity of the requester that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "For service use only. Diagnostic data occasionally included by the Azure Storage service. This property should be ignored by event consumers."]
    #[serde(rename = "storageDiagnostics")]
    pub storage_diagnostics: serde_json::Value,
}
impl StorageDirectoryRenamedEventData {
    pub fn new(storage_diagnostics: serde_json::Value) -> Self {
        Self {
            api: None,
            client_request_id: None,
            request_id: None,
            source_url: None,
            destination_url: None,
            sequencer: None,
            identity: None,
            storage_diagnostics,
        }
    }
}
#[doc = "Execution statistics of a specific policy action in a Blob Management cycle."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageLifecyclePolicyActionSummaryDetail {
    #[doc = "Total number of objects to be acted on by this action."]
    #[serde(rename = "totalObjectsCount", default, skip_serializing_if = "Option::is_none")]
    pub total_objects_count: Option<i64>,
    #[doc = "Number of success operations of this action."]
    #[serde(rename = "successCount", default, skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i64>,
    #[doc = "Error messages of this action if any."]
    #[serde(rename = "errorList", default, skip_serializing_if = "Option::is_none")]
    pub error_list: Option<String>,
}
impl StorageLifecyclePolicyActionSummaryDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.LifecyclePolicyCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageLifecyclePolicyCompletedEventData {
    #[doc = "The time the policy task was scheduled."]
    #[serde(rename = "scheduleTime", default, skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    #[doc = "Execution statistics of a specific policy action in a Blob Management cycle."]
    #[serde(rename = "deleteSummary")]
    pub delete_summary: StorageLifecyclePolicyActionSummaryDetail,
    #[doc = "Execution statistics of a specific policy action in a Blob Management cycle."]
    #[serde(rename = "tierToCoolSummary")]
    pub tier_to_cool_summary: StorageLifecyclePolicyActionSummaryDetail,
    #[doc = "Execution statistics of a specific policy action in a Blob Management cycle."]
    #[serde(rename = "tierToArchiveSummary")]
    pub tier_to_archive_summary: StorageLifecyclePolicyActionSummaryDetail,
    #[doc = "Execution statistics of a specific policy action in a Blob Management cycle."]
    #[serde(rename = "tierToColdSummary")]
    pub tier_to_cold_summary: StorageLifecyclePolicyActionSummaryDetail,
}
impl StorageLifecyclePolicyCompletedEventData {
    pub fn new(
        delete_summary: StorageLifecyclePolicyActionSummaryDetail,
        tier_to_cool_summary: StorageLifecyclePolicyActionSummaryDetail,
        tier_to_archive_summary: StorageLifecyclePolicyActionSummaryDetail,
        tier_to_cold_summary: StorageLifecyclePolicyActionSummaryDetail,
    ) -> Self {
        Self {
            schedule_time: None,
            delete_summary,
            tier_to_cool_summary,
            tier_to_archive_summary,
            tier_to_cold_summary,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Storage.StorageTaskAssignmentCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskAssignmentCompletedEventData {
    #[doc = "The status for a storage task."]
    pub status: StorageTaskAssignmentCompletedStatus,
    #[doc = "The time at which a storage task was completed."]
    #[serde(rename = "completedDateTime", with = "azure_core::date::rfc3339")]
    pub completed_date_time: ::time::OffsetDateTime,
    #[doc = "The execution id for a storage task."]
    #[serde(rename = "taskExecutionId", default, skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    #[doc = "The task name for a storage task."]
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[doc = "The summary report blob url for a storage task"]
    #[serde(rename = "summaryReportBlobUrl")]
    pub summary_report_blob_url: String,
}
impl StorageTaskAssignmentCompletedEventData {
    pub fn new(
        status: StorageTaskAssignmentCompletedStatus,
        completed_date_time: ::time::OffsetDateTime,
        summary_report_blob_url: String,
    ) -> Self {
        Self {
            status,
            completed_date_time,
            task_execution_id: None,
            task_name: None,
            summary_report_blob_url,
        }
    }
}
#[doc = "The status for a storage task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StorageTaskAssignmentCompletedStatus")]
pub enum StorageTaskAssignmentCompletedStatus {
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StorageTaskAssignmentCompletedStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StorageTaskAssignmentCompletedStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StorageTaskAssignmentCompletedStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("StorageTaskAssignmentCompletedStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("StorageTaskAssignmentCompletedStatus", 1u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Storage.StorageTaskAssignmentQueued event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskAssignmentQueuedEventData {
    #[doc = "The time at which a storage task was queued."]
    #[serde(rename = "queuedDateTime", with = "azure_core::date::rfc3339")]
    pub queued_date_time: ::time::OffsetDateTime,
    #[doc = "The execution id for a storage task."]
    #[serde(rename = "taskExecutionId", default, skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
}
impl StorageTaskAssignmentQueuedEventData {
    pub fn new(queued_date_time: ::time::OffsetDateTime) -> Self {
        Self {
            queued_date_time,
            task_execution_id: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Storage.StorageTaskCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskCompletedEventData {
    #[doc = "The status for a storage task."]
    pub status: StorageTaskCompletedStatus,
    #[doc = "The time at which a storage task was completed."]
    #[serde(rename = "completedDateTime", with = "azure_core::date::rfc3339")]
    pub completed_date_time: ::time::OffsetDateTime,
    #[doc = "The execution id for a storage task."]
    #[serde(rename = "taskExecutionId", default, skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    #[doc = "The task name for a storage task."]
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[doc = "The summary report blob url for a storage task"]
    #[serde(rename = "summaryReportBlobUrl")]
    pub summary_report_blob_url: String,
}
impl StorageTaskCompletedEventData {
    pub fn new(status: StorageTaskCompletedStatus, completed_date_time: ::time::OffsetDateTime, summary_report_blob_url: String) -> Self {
        Self {
            status,
            completed_date_time,
            task_execution_id: None,
            task_name: None,
            summary_report_blob_url,
        }
    }
}
#[doc = "The status for a storage task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StorageTaskCompletedStatus")]
pub enum StorageTaskCompletedStatus {
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StorageTaskCompletedStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StorageTaskCompletedStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StorageTaskCompletedStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("StorageTaskCompletedStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("StorageTaskCompletedStatus", 1u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Storage.StorageTaskQueued event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskQueuedEventData {
    #[doc = "The time at which a storage task was queued."]
    #[serde(rename = "queuedDateTime", with = "azure_core::date::rfc3339")]
    pub queued_date_time: ::time::OffsetDateTime,
    #[doc = "The execution id for a storage task."]
    #[serde(rename = "taskExecutionId", default, skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
}
impl StorageTaskQueuedEventData {
    pub fn new(queued_date_time: ::time::OffsetDateTime) -> Self {
        Self {
            queued_date_time,
            task_execution_id: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a\nMicrosoft.EventGrid.SubscriptionDeletedEvent event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionDeletedEventData {
    #[doc = "The Azure resource ID of the deleted event subscription."]
    #[serde(rename = "eventSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub event_subscription_id: Option<String>,
}
impl SubscriptionDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.EventGrid.SubscriptionValidationEvent event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionValidationEventData {
    #[doc = "The validation code sent by Azure Event Grid to validate an event subscription.\nTo complete the validation handshake, the subscriber must either respond with this validation code as part of the validation response,\nor perform a GET request on the validationUrl (available starting version 2018-05-01-preview)."]
    #[serde(rename = "validationCode", default, skip_serializing_if = "Option::is_none")]
    pub validation_code: Option<String>,
    #[doc = "The validation URL sent by Azure Event Grid (available starting version 2018-05-01-preview).\nTo complete the validation handshake, the subscriber must either respond with the validationCode as part of the validation response,\nor perform a GET request on the validationUrl (available starting version 2018-05-01-preview)."]
    #[serde(rename = "validationUrl", default, skip_serializing_if = "Option::is_none")]
    pub validation_url: Option<String>,
}
impl SubscriptionValidationEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "To complete an event subscription validation handshake, a subscriber can use\neither the validationCode or the validationUrl received in a\nSubscriptionValidationEvent. When the validationCode is used, the\nSubscriptionValidationResponse can be used to build the response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionValidationResponse {
    #[doc = "The validation response sent by the subscriber to Azure Event Grid to complete the validation of an event subscription."]
    #[serde(rename = "validationResponse", default, skip_serializing_if = "Option::is_none")]
    pub validation_response: Option<String>,
}
impl SubscriptionValidationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.AppServicePlanUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppServicePlanUpdatedEventData {
    #[doc = "Detail of action on the app service plan."]
    #[serde(rename = "appServicePlanEventTypeDetail")]
    pub app_service_plan_event_type_detail: AppServicePlanEventTypeDetail,
    #[doc = "sku of app service plan."]
    pub sku: WebAppServicePlanUpdatedEventDataSku,
    #[doc = "name of the app service plan that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the app service plan API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the app service plan API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the app service plan API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebAppServicePlanUpdatedEventData {
    pub fn new(app_service_plan_event_type_detail: AppServicePlanEventTypeDetail, sku: WebAppServicePlanUpdatedEventDataSku) -> Self {
        Self {
            app_service_plan_event_type_detail,
            sku,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "sku of app service plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppServicePlanUpdatedEventDataSku {
    #[doc = "name of app service plan sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "tier of app service plan sku."]
    #[serde(rename = "Tier", default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "size of app service plan sku."]
    #[serde(rename = "Size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "family of app service plan sku."]
    #[serde(rename = "Family", default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "capacity of app service plan sku."]
    #[serde(rename = "Capacity", default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
}
impl WebAppServicePlanUpdatedEventDataSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.AppUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppUpdatedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebAppUpdatedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.BackupOperationCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebBackupOperationCompletedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebBackupOperationCompletedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.BackupOperationFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebBackupOperationFailedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebBackupOperationFailedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.BackupOperationStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebBackupOperationStartedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebBackupOperationStartedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.RestoreOperationCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebRestoreOperationCompletedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebRestoreOperationCompletedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.RestoreOperationFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebRestoreOperationFailedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebRestoreOperationFailedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.RestoreOperationStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebRestoreOperationStartedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebRestoreOperationStartedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSlotSwapCompletedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebSlotSwapCompletedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSlotSwapFailedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebSlotSwapFailedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSlotSwapStartedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebSlotSwapStartedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapWithPreviewCancelled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSlotSwapWithPreviewCancelledEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebSlotSwapWithPreviewCancelledEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapWithPreviewStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSlotSwapWithPreviewStartedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail")]
    pub app_event_type_detail: AppEventTypeDetail,
    #[doc = "name of the web site that had this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The client request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The correlation request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "correlationRequestId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_request_id: Option<String>,
    #[doc = "The request id generated by the app service for the site API operation that triggered this event."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "HTTP request URL of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "HTTP verb of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
impl WebSlotSwapWithPreviewStartedEventData {
    pub fn new(app_event_type_detail: AppEventTypeDetail) -> Self {
        Self {
            app_event_type_detail,
            name: None,
            client_request_id: None,
            correlation_request_id: None,
            request_id: None,
            address: None,
            verb: None,
        }
    }
}
#[doc = "Recording channel type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecordingChannelType")]
pub enum RecordingChannelType {
    Mixed,
    Unmixed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecordingChannelType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecordingChannelType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecordingChannelType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Mixed => serializer.serialize_unit_variant("RecordingChannelType", 0u32, "Mixed"),
            Self::Unmixed => serializer.serialize_unit_variant("RecordingChannelType", 1u32, "Unmixed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Recording content type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecordingContentType")]
pub enum RecordingContentType {
    AudioVideo,
    Audio,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecordingContentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecordingContentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecordingContentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AudioVideo => serializer.serialize_unit_variant("RecordingContentType", 0u32, "AudioVideo"),
            Self::Audio => serializer.serialize_unit_variant("RecordingContentType", 1u32, "Audio"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Recording format type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecordingFormatType")]
pub enum RecordingFormatType {
    Wav,
    Mp3,
    Mp4,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecordingFormatType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecordingFormatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecordingFormatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Wav => serializer.serialize_unit_variant("RecordingFormatType", 0u32, "Wav"),
            Self::Mp3 => serializer.serialize_unit_variant("RecordingFormatType", 1u32, "Mp3"),
            Self::Mp4 => serializer.serialize_unit_variant("RecordingFormatType", 2u32, "Mp4"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
