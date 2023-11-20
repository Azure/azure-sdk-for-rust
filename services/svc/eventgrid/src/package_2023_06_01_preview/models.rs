#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Array of lock tokens for the corresponding received Cloud Events to be acknowledged."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcknowledgeOptions {
    #[doc = "Array of lock tokens."]
    #[serde(rename = "lockTokens")]
    pub lock_tokens: Vec<String>,
}
impl AcknowledgeOptions {
    pub fn new(lock_tokens: Vec<String>) -> Self {
        Self { lock_tokens }
    }
}
#[doc = "The result of the Acknowledge operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcknowledgeResult {
    #[doc = "Array of FailedLockToken for failed cloud events. Each FailedLockToken includes the lock token along with the related error information (namely, the error code and description)."]
    #[serde(rename = "failedLockTokens")]
    pub failed_lock_tokens: Vec<FailedLockToken>,
    #[doc = "Array of lock tokens for the successfully acknowledged cloud events."]
    #[serde(rename = "succeededLockTokens")]
    pub succeeded_lock_tokens: Vec<String>,
}
impl AcknowledgeResult {
    pub fn new(failed_lock_tokens: Vec<FailedLockToken>, succeeded_lock_tokens: Vec<String>) -> Self {
        Self {
            failed_lock_tokens,
            succeeded_lock_tokens,
        }
    }
}
#[doc = "Schema of common properties of all chat events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatEventBaseProperties {
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "recipientCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub recipient_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The transaction id will be used as co-relation vector"]
    #[serde(rename = "transactionId", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[doc = "The chat thread id"]
    #[serde(rename = "threadId", default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}
impl AcsChatEventBaseProperties {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatMessageDeletedEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties,
    #[doc = "The time at which the message was deleted"]
    #[serde(rename = "deleteTime", default, with = "azure_core::date::rfc3339::option")]
    pub delete_time: Option<time::OffsetDateTime>,
}
impl AcsChatMessageDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageDeletedInThread event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatMessageDeletedInThreadEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
    #[doc = "The time at which the message was deleted"]
    #[serde(rename = "deleteTime", default, with = "azure_core::date::rfc3339::option")]
    pub delete_time: Option<time::OffsetDateTime>,
}
impl AcsChatMessageDeletedInThreadEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageEdited event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatMessageEditedEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties,
    #[doc = "The body of the chat message"]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[doc = "The chat message metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The time at which the message was edited"]
    #[serde(rename = "editTime", default, with = "azure_core::date::rfc3339::option")]
    pub edit_time: Option<time::OffsetDateTime>,
}
impl AcsChatMessageEditedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageEditedInThread event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatMessageEditedInThreadEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
    #[doc = "The body of the chat message"]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[doc = "The chat message metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The time at which the message was edited"]
    #[serde(rename = "editTime", default, with = "azure_core::date::rfc3339::option")]
    pub edit_time: Option<time::OffsetDateTime>,
}
impl AcsChatMessageEditedInThreadEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of all chat message events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatMessageEventBaseProperties {
    #[serde(flatten)]
    pub acs_chat_event_base_properties: AcsChatEventBaseProperties,
    #[doc = "The chat message id"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "senderCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub sender_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The display name of the sender"]
    #[serde(rename = "senderDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub sender_display_name: Option<String>,
    #[doc = "The original compose time of the message"]
    #[serde(rename = "composeTime", default, with = "azure_core::date::rfc3339::option")]
    pub compose_time: Option<time::OffsetDateTime>,
    #[doc = "The type of the message"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The version of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatMessageEventBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of all thread-level chat message events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatMessageEventInThreadBaseProperties {
    #[serde(flatten)]
    pub acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties,
    #[doc = "The chat message id"]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "senderCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub sender_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The display name of the sender"]
    #[serde(rename = "senderDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub sender_display_name: Option<String>,
    #[doc = "The original compose time of the message"]
    #[serde(rename = "composeTime", default, with = "azure_core::date::rfc3339::option")]
    pub compose_time: Option<time::OffsetDateTime>,
    #[doc = "The type of the message"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The version of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatMessageEventInThreadBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatMessageReceivedEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_base_properties: AcsChatMessageEventBaseProperties,
    #[doc = "The body of the chat message"]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[doc = "The chat message metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl AcsChatMessageReceivedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatMessageReceivedInThread event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatMessageReceivedInThreadEventData {
    #[serde(flatten)]
    pub acs_chat_message_event_in_thread_base_properties: AcsChatMessageEventInThreadBaseProperties,
    #[doc = "The body of the chat message"]
    #[serde(rename = "messageBody", default, skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[doc = "The chat message metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl AcsChatMessageReceivedInThreadEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadParticipantAdded event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatParticipantAddedToThreadEventData {
    #[serde(flatten)]
    pub acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties,
    #[doc = "The time at which the user was added to the thread"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "addedByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub added_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "Schema of the chat thread participant"]
    #[serde(rename = "participantAdded", default, skip_serializing_if = "Option::is_none")]
    pub participant_added: Option<AcsChatThreadParticipantProperties>,
    #[doc = "The version of the thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatParticipantAddedToThreadEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatParticipantAddedToThreadWithUser event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatParticipantAddedToThreadWithUserEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "The time at which the user was added to the thread"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "addedByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub added_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "Schema of the chat thread participant"]
    #[serde(rename = "participantAdded", default, skip_serializing_if = "Option::is_none")]
    pub participant_added: Option<AcsChatThreadParticipantProperties>,
}
impl AcsChatParticipantAddedToThreadWithUserEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadParticipantRemoved event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatParticipantRemovedFromThreadEventData {
    #[serde(flatten)]
    pub acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties,
    #[doc = "The time at which the user was removed to the thread"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "removedByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub removed_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "Schema of the chat thread participant"]
    #[serde(rename = "participantRemoved", default, skip_serializing_if = "Option::is_none")]
    pub participant_removed: Option<AcsChatThreadParticipantProperties>,
    #[doc = "The version of the thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatParticipantRemovedFromThreadEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatParticipantRemovedFromThreadWithUser event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatParticipantRemovedFromThreadWithUserEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "The time at which the user was removed to the thread"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "removedByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub removed_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "Schema of the chat thread participant"]
    #[serde(rename = "participantRemoved", default, skip_serializing_if = "Option::is_none")]
    pub participant_removed: Option<AcsChatThreadParticipantProperties>,
}
impl AcsChatParticipantRemovedFromThreadWithUserEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadCreatedEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "createdByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub created_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The thread properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The list of properties of participants who are part of the thread"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub participants: Vec<AcsChatThreadParticipantProperties>,
}
impl AcsChatThreadCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadCreatedWithUser event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadCreatedWithUserEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "createdByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub created_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The thread properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The list of properties of participants who are part of the thread"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub participants: Vec<AcsChatThreadParticipantProperties>,
}
impl AcsChatThreadCreatedWithUserEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadDeletedEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "deletedByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub deleted_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The deletion time of the thread"]
    #[serde(rename = "deleteTime", default, with = "azure_core::date::rfc3339::option")]
    pub delete_time: Option<time::OffsetDateTime>,
}
impl AcsChatThreadDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of all chat thread events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadEventBaseProperties {
    #[serde(flatten)]
    pub acs_chat_event_base_properties: AcsChatEventBaseProperties,
    #[doc = "The original creation time of the thread"]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The version of the thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatThreadEventBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of all chat thread events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadEventInThreadBaseProperties {
    #[serde(flatten)]
    pub acs_chat_event_in_thread_base_properties: AcsChatEventInThreadBaseProperties,
    #[doc = "The original creation time of the thread"]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "The version of the thread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AcsChatThreadEventInThreadBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the chat thread participant"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadParticipantProperties {
    #[doc = "The name of the user"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "participantCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub participant_communication_identifier: Option<CommunicationIdentifierModel>,
}
impl AcsChatThreadParticipantProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadPropertiesUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadPropertiesUpdatedEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_in_thread_base_properties: AcsChatThreadEventInThreadBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "editedByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub edited_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The time at which the properties of the thread were updated"]
    #[serde(rename = "editTime", default, with = "azure_core::date::rfc3339::option")]
    pub edit_time: Option<time::OffsetDateTime>,
    #[doc = "The updated thread properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AcsChatThreadPropertiesUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadPropertiesUpdatedPerUser event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadPropertiesUpdatedPerUserEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "editedByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub edited_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The time at which the properties of the thread were updated"]
    #[serde(rename = "editTime", default, with = "azure_core::date::rfc3339::option")]
    pub edit_time: Option<time::OffsetDateTime>,
    #[doc = "The updated thread properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AcsChatThreadPropertiesUpdatedPerUserEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.ChatThreadWithUserDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsChatThreadWithUserDeletedEventData {
    #[serde(flatten)]
    pub acs_chat_thread_event_base_properties: AcsChatThreadEventBaseProperties,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "deletedByCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub deleted_by_communication_identifier: Option<CommunicationIdentifierModel>,
    #[doc = "The deletion time of the thread"]
    #[serde(rename = "deleteTime", default, with = "azure_core::date::rfc3339::option")]
    pub delete_time: Option<time::OffsetDateTime>,
}
impl AcsChatThreadWithUserDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.EmailDeliveryReportReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<acs_email_delivery_report_received_event_data::Status>,
    #[doc = "Detailed information about the status if any"]
    #[serde(rename = "deliveryStatusDetails", default, skip_serializing_if = "Option::is_none")]
    pub delivery_status_details: Option<AcsEmailDeliveryReportStatusDetails>,
    #[doc = "The time at which the email delivery report received timestamp"]
    #[serde(rename = "deliveryAttemptTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub delivery_attempt_time_stamp: Option<time::OffsetDateTime>,
}
impl AcsEmailDeliveryReportReceivedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod acs_email_delivery_report_received_event_data {
    use super::*;
    #[doc = "The status of the email. Any value other than Delivered is considered failed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Bounced,
        Delivered,
        Failed,
        FilteredSpam,
        Quarantined,
        Suppressed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Bounced => serializer.serialize_unit_variant("Status", 0u32, "Bounced"),
                Self::Delivered => serializer.serialize_unit_variant("Status", 1u32, "Delivered"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::FilteredSpam => serializer.serialize_unit_variant("Status", 3u32, "FilteredSpam"),
                Self::Quarantined => serializer.serialize_unit_variant("Status", 4u32, "Quarantined"),
                Self::Suppressed => serializer.serialize_unit_variant("Status", 5u32, "Suppressed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "userActionTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub user_action_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "The context of the type of engagement user had with email"]
    #[serde(rename = "engagementContext", default, skip_serializing_if = "Option::is_none")]
    pub engagement_context: Option<String>,
    #[doc = "The user agent interacting with the email"]
    #[serde(rename = "userAgent", default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[doc = "The type of engagement user have with email"]
    #[serde(rename = "engagementType", default, skip_serializing_if = "Option::is_none")]
    pub engagement_type: Option<acs_email_engagement_tracking_report_received_event_data::EngagementType>,
}
impl AcsEmailEngagementTrackingReportReceivedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod acs_email_engagement_tracking_report_received_event_data {
    use super::*;
    #[doc = "The type of engagement user have with email"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EngagementType")]
    pub enum EngagementType {
        #[serde(rename = "view")]
        View,
        #[serde(rename = "click")]
        Click,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EngagementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EngagementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EngagementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::View => serializer.serialize_unit_variant("EngagementType", 0u32, "view"),
                Self::Click => serializer.serialize_unit_variant("EngagementType", 1u32, "click"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Custom Context of Incoming Call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsIncomingCallCustomContext {
    #[doc = "Sip Headers for incoming call"]
    #[serde(rename = "sipHeaders", default, skip_serializing_if = "Option::is_none")]
    pub sip_headers: Option<serde_json::Value>,
    #[doc = "Voip Headers for incoming call"]
    #[serde(rename = "voipHeaders", default, skip_serializing_if = "Option::is_none")]
    pub voip_headers: Option<serde_json::Value>,
}
impl AcsIncomingCallCustomContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Communication.IncomingCall event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsIncomingCallEventData {
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<CommunicationIdentifierModel>,
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<CommunicationIdentifierModel>,
    #[doc = "The Id of the server call"]
    #[serde(rename = "serverCallId", default, skip_serializing_if = "Option::is_none")]
    pub server_call_id: Option<String>,
    #[doc = "Display name of caller."]
    #[serde(rename = "callerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub caller_display_name: Option<String>,
    #[doc = "Custom Context of Incoming Call"]
    #[serde(rename = "customContext", default, skip_serializing_if = "Option::is_none")]
    pub custom_context: Option<AcsIncomingCallCustomContext>,
    #[doc = "Signed incoming call context."]
    #[serde(rename = "incomingCallContext", default, skip_serializing_if = "Option::is_none")]
    pub incoming_call_context: Option<String>,
    #[doc = "CorrelationId (CallId)."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
}
impl AcsIncomingCallEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRecordingFileStatusUpdatedEventData {
    #[doc = "Schema for all properties of Recording Storage Information."]
    #[serde(rename = "recordingStorageInfo", default, skip_serializing_if = "Option::is_none")]
    pub recording_storage_info: Option<AcsRecordingStorageInfoProperties>,
    #[doc = "The time at which the recording started"]
    #[serde(rename = "recordingStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub recording_start_time: Option<time::OffsetDateTime>,
    #[doc = "The recording duration in milliseconds"]
    #[serde(rename = "recordingDurationMs", default, skip_serializing_if = "Option::is_none")]
    pub recording_duration_ms: Option<i64>,
    #[doc = "The recording content type- AudioVideo, or Audio"]
    #[serde(rename = "recordingContentType", default, skip_serializing_if = "Option::is_none")]
    pub recording_content_type: Option<acs_recording_file_status_updated_event_data::RecordingContentType>,
    #[doc = "The recording  channel type - Mixed, Unmixed"]
    #[serde(rename = "recordingChannelType", default, skip_serializing_if = "Option::is_none")]
    pub recording_channel_type: Option<acs_recording_file_status_updated_event_data::RecordingChannelType>,
    #[doc = "The recording format type - Mp4, Mp3, Wav"]
    #[serde(rename = "recordingFormatType", default, skip_serializing_if = "Option::is_none")]
    pub recording_format_type: Option<acs_recording_file_status_updated_event_data::RecordingFormatType>,
    #[doc = "The reason for ending recording session"]
    #[serde(rename = "sessionEndReason", default, skip_serializing_if = "Option::is_none")]
    pub session_end_reason: Option<String>,
}
impl AcsRecordingFileStatusUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod acs_recording_file_status_updated_event_data {
    use super::*;
    #[doc = "The recording content type- AudioVideo, or Audio"]
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
    #[doc = "The recording  channel type - Mixed, Unmixed"]
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
    #[doc = "The recording format type - Mp4, Mp3, Wav"]
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
}
#[doc = "Schema for all properties of Recording Storage Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRecordingStorageInfoProperties {
    #[doc = "List of details of recording chunks information"]
    #[serde(
        rename = "recordingChunks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recording_chunks: Vec<AcsRecordingChunkInfoProperties>,
}
impl AcsRecordingStorageInfoProperties {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AcsRouterCommunicationError>>,
    #[doc = "List of Router Communication Errors"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AcsRouterCommunicationError>,
}
impl AcsRouterCommunicationError {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobClassificationFailed event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterJobClassificationFailedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Classification Policy Id"]
    #[serde(rename = "classificationPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub classification_policy_id: Option<String>,
    #[doc = "Router Job Classification Failed Errors"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<AcsRouterCommunicationError>,
}
impl AcsRouterJobClassificationFailedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobClassified event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterJobClassifiedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Queue Details"]
    #[serde(rename = "queueDetails", default, skip_serializing_if = "Option::is_none")]
    pub queue_details: Option<AcsRouterQueueDetails>,
    #[doc = "Router Job Classification Policy Id"]
    #[serde(rename = "classificationPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub classification_policy_id: Option<String>,
    #[doc = "Router Job Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Attached Worker Selector"]
    #[serde(
        rename = "attachedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
}
impl AcsRouterJobClassifiedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobClosed event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobCompleted event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobDeleted event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterJobDeletedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
}
impl AcsRouterJobDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of common properties of all Router Job events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterJobEventData {
    #[serde(flatten)]
    pub acs_router_event_data: AcsRouterEventData,
    #[doc = "Router Job events Queue Id"]
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[doc = "Router Job events Labels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
    #[doc = "Router Jobs events Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AcsRouterJobEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobExceptionTriggered event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobQueued event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterJobQueuedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Queued Attached Worker Selector"]
    #[serde(
        rename = "attachedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Queued Requested Worker Selector"]
    #[serde(
        rename = "requestedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
}
impl AcsRouterJobQueuedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobReceived event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsRouterJobReceivedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Received Job Status"]
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<acs_router_job_received_event_data::JobStatus>,
    #[doc = "Router Job Classification Policy Id"]
    #[serde(rename = "classificationPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub classification_policy_id: Option<String>,
    #[doc = "Router Job Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Received Requested Worker Selectors"]
    #[serde(
        rename = "requestedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Received Scheduled Time in UTC"]
    #[serde(rename = "scheduledOn", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_on: Option<time::OffsetDateTime>,
    #[doc = "Unavailable For Matching for Router Job Received"]
    #[serde(rename = "unavailableForMatching")]
    pub unavailable_for_matching: bool,
}
impl AcsRouterJobReceivedEventData {
    pub fn new(unavailable_for_matching: bool) -> Self {
        Self {
            acs_router_job_event_data: AcsRouterJobEventData::default(),
            job_status: None,
            classification_policy_id: None,
            priority: None,
            requested_worker_selectors: Vec::new(),
            scheduled_on: None,
            unavailable_for_matching,
        }
    }
}
pub mod acs_router_job_received_event_data {
    use super::*;
    #[doc = "Router Job Received Job Status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobStatus")]
    pub enum JobStatus {
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
    impl FromStr for JobStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JobStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JobStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PendingClassification => serializer.serialize_unit_variant("JobStatus", 0u32, "PendingClassification"),
                Self::Queued => serializer.serialize_unit_variant("JobStatus", 1u32, "Queued"),
                Self::Assigned => serializer.serialize_unit_variant("JobStatus", 2u32, "Assigned"),
                Self::Completed => serializer.serialize_unit_variant("JobStatus", 3u32, "Completed"),
                Self::Closed => serializer.serialize_unit_variant("JobStatus", 4u32, "Closed"),
                Self::Cancelled => serializer.serialize_unit_variant("JobStatus", 5u32, "Cancelled"),
                Self::ClassificationFailed => serializer.serialize_unit_variant("JobStatus", 6u32, "ClassificationFailed"),
                Self::Created => serializer.serialize_unit_variant("JobStatus", 7u32, "Created"),
                Self::PendingSchedule => serializer.serialize_unit_variant("JobStatus", 8u32, "PendingSchedule"),
                Self::Scheduled => serializer.serialize_unit_variant("JobStatus", 9u32, "Scheduled"),
                Self::ScheduleFailed => serializer.serialize_unit_variant("JobStatus", 10u32, "ScheduleFailed"),
                Self::WaitingForActivation => serializer.serialize_unit_variant("JobStatus", 11u32, "WaitingForActivation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobSchedulingFailed event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterJobSchedulingFailedEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Priority"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Router Job Scheduling Failed Attached Worker Selector Expired"]
    #[serde(
        rename = "expiredAttachedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Scheduling Failed Requested Worker Selector Expired"]
    #[serde(
        rename = "expiredRequestedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Scheduling Failed Scheduled Time in UTC"]
    #[serde(rename = "scheduledOn", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_on: Option<time::OffsetDateTime>,
    #[doc = "Router Job Scheduling Failed Reason"]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}
impl AcsRouterJobSchedulingFailedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobUnassigned event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    pub fn new() -> Self {
        Self::default()
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
    #[serde(
        rename = "expiredAttachedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Waiting For Activation Requested Worker Selector Expired"]
    #[serde(
        rename = "expiredRequestedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Waiting For Activation Scheduled Time in UTC"]
    #[serde(rename = "scheduledOn", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_on: Option<time::OffsetDateTime>,
    #[doc = "Router Job Waiting For Activation Unavailable For Matching"]
    #[serde(rename = "unavailableForMatching")]
    pub unavailable_for_matching: bool,
}
impl AcsRouterJobWaitingForActivationEventData {
    pub fn new(unavailable_for_matching: bool) -> Self {
        Self {
            acs_router_job_event_data: AcsRouterJobEventData::default(),
            priority: None,
            expired_attached_worker_selectors: Vec::new(),
            expired_requested_worker_selectors: Vec::new(),
            scheduled_on: None,
            unavailable_for_matching,
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.RouterJobWorkerSelectorsExpired event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterJobWorkerSelectorsExpiredEventData {
    #[serde(flatten)]
    pub acs_router_job_event_data: AcsRouterJobEventData,
    #[doc = "Router Job Worker Selectors Expired Requested Worker Selectors"]
    #[serde(
        rename = "expiredRequestedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expired_requested_worker_selectors: Vec<AcsRouterWorkerSelector>,
    #[doc = "Router Job Worker Selectors Expired Attached Worker Selectors"]
    #[serde(
        rename = "expiredAttachedWorkerSelectors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expired_attached_worker_selectors: Vec<AcsRouterWorkerSelector>,
}
impl AcsRouterJobWorkerSelectorsExpiredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Router Queue Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterQueueDetails {
    #[doc = "Router Queue Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Router Queue Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Router Queue Labels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
}
impl AcsRouterQueueDetails {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "workerLabels", default, skip_serializing_if = "Option::is_none")]
    pub worker_labels: Option<serde_json::Value>,
    #[doc = "Router Worker Offer Accepted Worker Tags"]
    #[serde(rename = "workerTags", default, skip_serializing_if = "Option::is_none")]
    pub worker_tags: Option<serde_json::Value>,
    #[doc = "Router Worker Offer Accepted Job Labels"]
    #[serde(rename = "jobLabels", default, skip_serializing_if = "Option::is_none")]
    pub job_labels: Option<serde_json::Value>,
    #[doc = "Router Worker Offer Accepted Job Tags"]
    #[serde(rename = "jobTags", default, skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<serde_json::Value>,
}
impl AcsRouterWorkerOfferAcceptedEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "workerLabels", default, skip_serializing_if = "Option::is_none")]
    pub worker_labels: Option<serde_json::Value>,
    #[doc = "Router Worker Offer Issued Time in UTC"]
    #[serde(rename = "offeredOn", default, with = "azure_core::date::rfc3339::option")]
    pub offered_on: Option<time::OffsetDateTime>,
    #[doc = "Router Worker Offer Issued Expiration Time in UTC"]
    #[serde(rename = "expiresOn", default, with = "azure_core::date::rfc3339::option")]
    pub expires_on: Option<time::OffsetDateTime>,
    #[doc = "Router Worker Offer Issued Worker Tags"]
    #[serde(rename = "workerTags", default, skip_serializing_if = "Option::is_none")]
    pub worker_tags: Option<serde_json::Value>,
    #[doc = "Router Worker Offer Issued Job Labels"]
    #[serde(rename = "jobLabels", default, skip_serializing_if = "Option::is_none")]
    pub job_labels: Option<serde_json::Value>,
    #[doc = "Router Worker Offer Issued Job Tags"]
    #[serde(rename = "jobTags", default, skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<serde_json::Value>,
}
impl AcsRouterWorkerOfferIssuedEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterWorkerRegisteredEventData {
    #[doc = "Router Worker Registered Worker Id"]
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[doc = "Router Worker Registered Queue Info"]
    #[serde(
        rename = "queueAssignments",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub queue_assignments: Vec<AcsRouterQueueDetails>,
    #[doc = "Router Worker Registered Channel Configuration"]
    #[serde(
        rename = "channelConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channel_configurations: Vec<AcsRouterChannelConfiguration>,
    #[doc = "Router Worker Register Total Capacity"]
    #[serde(rename = "totalCapacity", default, skip_serializing_if = "Option::is_none")]
    pub total_capacity: Option<i32>,
    #[doc = "Router Worker Registered Labels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
    #[doc = "Router Worker Registered Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AcsRouterWorkerRegisteredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Router Job Worker Selector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsRouterWorkerSelector {
    #[doc = "Router Job Worker Selector Key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Router Job Worker Selector Label Operator"]
    #[serde(rename = "labelOperator", default, skip_serializing_if = "Option::is_none")]
    pub label_operator: Option<acs_router_worker_selector::LabelOperator>,
    #[doc = "Router Job Worker Selector Value"]
    #[serde(rename = "labelValue", default, skip_serializing_if = "Option::is_none")]
    pub label_value: Option<serde_json::Value>,
    #[doc = "Router Job Worker Selector Time to Live in Seconds"]
    #[serde(rename = "ttlSeconds", default, skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<f64>,
    #[doc = "Router Job Worker Selector State"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<acs_router_worker_selector::State>,
    #[doc = "Router Job Worker Selector Expiration Time"]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<time::OffsetDateTime>,
}
impl AcsRouterWorkerSelector {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod acs_router_worker_selector {
    use super::*;
    #[doc = "Router Job Worker Selector Label Operator"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LabelOperator")]
    pub enum LabelOperator {
        Equal,
        NotEqual,
        Greater,
        Less,
        GreaterThanOrEqual,
        LessThanOrEqual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LabelOperator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LabelOperator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LabelOperator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equal => serializer.serialize_unit_variant("LabelOperator", 0u32, "Equal"),
                Self::NotEqual => serializer.serialize_unit_variant("LabelOperator", 1u32, "NotEqual"),
                Self::Greater => serializer.serialize_unit_variant("LabelOperator", 2u32, "Greater"),
                Self::Less => serializer.serialize_unit_variant("LabelOperator", 3u32, "Less"),
                Self::GreaterThanOrEqual => serializer.serialize_unit_variant("LabelOperator", 4u32, "GreaterThanOrEqual"),
                Self::LessThanOrEqual => serializer.serialize_unit_variant("LabelOperator", 5u32, "LessThanOrEqual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Router Job Worker Selector State"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "expired")]
        Expired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("State", 0u32, "active"),
                Self::Expired => serializer.serialize_unit_variant("State", 1u32, "expired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema for details of a delivery attempt"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsSmsDeliveryAttemptProperties {
    #[doc = "TimeStamp when delivery was attempted"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Number of segments that were successfully delivered"]
    #[serde(rename = "segmentsSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub segments_succeeded: Option<i64>,
    #[doc = "Number of segments whose delivery failed"]
    #[serde(rename = "segmentsFailed", default, skip_serializing_if = "Option::is_none")]
    pub segments_failed: Option<i64>,
}
impl AcsSmsDeliveryAttemptProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Communication.SMSDeliveryReportReceived event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(
        rename = "deliveryAttempts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub delivery_attempts: Vec<AcsSmsDeliveryAttemptProperties>,
    #[doc = "The time at which the SMS delivery report was received"]
    #[serde(rename = "receivedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub received_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Customer Content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl AcsSmsDeliveryReportReceivedEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsSmsReceivedEventData {
    #[serde(flatten)]
    pub acs_sms_event_base_properties: AcsSmsEventBaseProperties,
    #[doc = "The SMS content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The time at which the SMS was received"]
    #[serde(rename = "receivedTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub received_timestamp: Option<time::OffsetDateTime>,
}
impl AcsSmsReceivedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Communication.UserDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcsUserDisconnectedEventData {
    #[doc = "Identifies a participant in Azure Communication services. A participant is, for example, a phone number or an Azure communication user. This model must be interpreted as a union: Apart from rawId, at most one further property may be set."]
    #[serde(rename = "userCommunicationIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub user_communication_identifier: Option<CommunicationIdentifierModel>,
}
impl AcsUserDisconnectedEventData {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppEventTypeDetail {
    #[doc = "Type of action of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<app_event_type_detail::Action>,
}
impl AppEventTypeDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_event_type_detail {
    use super::*;
    #[doc = "Type of action of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Restarted,
        Stopped,
        ChangedAppSettings,
        Started,
        Completed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Restarted => serializer.serialize_unit_variant("Action", 0u32, "Restarted"),
                Self::Stopped => serializer.serialize_unit_variant("Action", 1u32, "Stopped"),
                Self::ChangedAppSettings => serializer.serialize_unit_variant("Action", 2u32, "ChangedAppSettings"),
                Self::Started => serializer.serialize_unit_variant("Action", 3u32, "Started"),
                Self::Completed => serializer.serialize_unit_variant("Action", 4u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Action", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Detail of action on the app service plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServicePlanEventTypeDetail {
    #[doc = "Kind of environment where app service plan is."]
    #[serde(rename = "stampKind", default, skip_serializing_if = "Option::is_none")]
    pub stamp_kind: Option<app_service_plan_event_type_detail::StampKind>,
    #[doc = "Type of action on the app service plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<app_service_plan_event_type_detail::Action>,
    #[doc = "Asynchronous operation status of the operation on the app service plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AsyncStatus>,
}
impl AppServicePlanEventTypeDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_service_plan_event_type_detail {
    use super::*;
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
    #[doc = "Type of action on the app service plan."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Updated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Updated => serializer.serialize_unit_variant("Action", 0u32, "Updated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsError {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AzureCoreFoundationsError>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<AzureCoreFoundationsInnerError>,
}
impl AzureCoreFoundationsError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "A response containing error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsErrorResponse {
    #[doc = "The error object."]
    pub error: AzureCoreFoundationsError,
}
impl AzureCoreFoundationsErrorResponse {
    pub fn new(error: AzureCoreFoundationsError) -> Self {
        Self { error }
    }
}
#[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreFoundationsInnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AzureCoreFoundationsInnerError>>,
}
impl AzureCoreFoundationsInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Event Broker operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrokerProperties {
    #[doc = "The token of the lock on the event."]
    #[serde(rename = "lockToken")]
    pub lock_token: String,
    #[doc = "The attempt count for delivering the event."]
    #[serde(rename = "deliveryCount")]
    pub delivery_count: i32,
}
impl BrokerProperties {
    pub fn new(lock_token: String, delivery_count: i32) -> Self {
        Self {
            lock_token,
            delivery_count,
        }
    }
}
#[doc = "Properties of an event published to an Azure Messaging EventGrid Namespace topic using the CloudEvent 1.0 Schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEvent {
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
    pub time: Option<time::OffsetDateTime>,
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
impl CloudEvent {
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
    pub time: Option<time::OffsetDateTime>,
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
#[doc = "The cloud that the identifier belongs to."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationIdentifierModel {
    #[doc = "Raw Id of the identifier. Optional in requests, required in responses."]
    #[serde(rename = "rawId", default, skip_serializing_if = "Option::is_none")]
    pub raw_id: Option<String>,
    #[doc = "A user that got created with an Azure Communication Services resource."]
    #[serde(rename = "communicationUser", default, skip_serializing_if = "Option::is_none")]
    pub communication_user: Option<CommunicationUserIdentifierModel>,
    #[doc = "A phone number."]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumberIdentifierModel>,
    #[doc = "A Microsoft Teams user."]
    #[serde(rename = "microsoftTeamsUser", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_teams_user: Option<MicrosoftTeamsUserIdentifierModel>,
}
impl CommunicationIdentifierModel {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryArtifactEventData {
    #[doc = "The event ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The action that encompasses the provided event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The location of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The target of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ContainerRegistryArtifactEventTarget>,
    #[doc = "The connected registry information if the event is generated by a connected registry."]
    #[serde(rename = "connectedRegistry", default, skip_serializing_if = "Option::is_none")]
    pub connected_registry: Option<ContainerRegistryEventConnectedRegistry>,
}
impl ContainerRegistryArtifactEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryChartDeletedEventData {
    #[serde(flatten)]
    pub container_registry_artifact_event_data: ContainerRegistryArtifactEventData,
}
impl ContainerRegistryChartDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerRegistry.ChartPushed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryChartPushedEventData {
    #[serde(flatten)]
    pub container_registry_artifact_event_data: ContainerRegistryArtifactEventData,
}
impl ContainerRegistryChartPushedEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryEventData {
    #[doc = "The event ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The action that encompasses the provided event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The location of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The target of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ContainerRegistryEventTarget>,
    #[doc = "The request that generated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<ContainerRegistryEventRequest>,
    #[doc = "The agent that initiated the event. For most situations, this could be from the authorization context of the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<ContainerRegistryEventActor>,
    #[doc = "The registry node that generated the event. Put differently, while the actor initiates the event, the source generates it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ContainerRegistryEventSource>,
    #[doc = "The connected registry information if the event is generated by a connected registry."]
    #[serde(rename = "connectedRegistry", default, skip_serializing_if = "Option::is_none")]
    pub connected_registry: Option<ContainerRegistryEventConnectedRegistry>,
}
impl ContainerRegistryEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryImageDeletedEventData {
    #[serde(flatten)]
    pub container_registry_event_data: ContainerRegistryEventData,
}
impl ContainerRegistryImageDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ContainerRegistry.ImagePushed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryImagePushedEventData {
    #[serde(flatten)]
    pub container_registry_event_data: ContainerRegistryEventData,
}
impl ContainerRegistryImagePushedEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxCopyCompletedEventData {
    #[doc = "Serial Number of the device associated with the event. The list is comma separated if more than one serial number is associated."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Schema of DataBox Stage Name enumeration."]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<DataBoxStageName>,
    #[doc = "The time at which the stage happened."]
    #[serde(rename = "stageTime", default, with = "azure_core::date::rfc3339::option")]
    pub stage_time: Option<time::OffsetDateTime>,
}
impl DataBoxCopyCompletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.DataBox.CopyStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxCopyStartedEventData {
    #[doc = "Serial Number of the device associated with the event. The list is comma separated if more than one serial number is associated."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Schema of DataBox Stage Name enumeration."]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<DataBoxStageName>,
    #[doc = "The time at which the stage happened."]
    #[serde(rename = "stageTime", default, with = "azure_core::date::rfc3339::option")]
    pub stage_time: Option<time::OffsetDateTime>,
}
impl DataBoxCopyStartedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.DataBox.OrderCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxOrderCompletedEventData {
    #[doc = "Serial Number of the device associated with the event. The list is comma separated if more than one serial number is associated."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Schema of DataBox Stage Name enumeration."]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<DataBoxStageName>,
    #[doc = "The time at which the stage happened."]
    #[serde(rename = "stageTime", default, with = "azure_core::date::rfc3339::option")]
    pub stage_time: Option<time::OffsetDateTime>,
}
impl DataBoxOrderCompletedEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceConnectionStateEventProperties {
    #[doc = "The unique identifier of the device. This case-sensitive string can be up to 128 characters long, and supports ASCII 7-bit alphanumeric characters plus the following special characters: - : . + % _ &#35; * ? ! ( ) , = @ ; $ '."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "The unique identifier of the module. This case-sensitive string can be up to 128 characters long, and supports ASCII 7-bit alphanumeric characters plus the following special characters: - : . + % _ &#35; * ? ! ( ) , = @ ; $ '."]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[doc = "Name of the IoT Hub where the device was created or deleted."]
    #[serde(rename = "hubName", default, skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[doc = "Information about the device connection state event."]
    #[serde(rename = "deviceConnectionStateEventInfo", default, skip_serializing_if = "Option::is_none")]
    pub device_connection_state_event_info: Option<DeviceConnectionStateEventInfo>,
}
impl DeviceConnectionStateEventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a device life cycle event (DeviceCreated, DeviceDeleted)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceLifeCycleEventProperties {
    #[doc = "The unique identifier of the device. This case-sensitive string can be up to 128 characters long, and supports ASCII 7-bit alphanumeric characters plus the following special characters: - : . + % _ &#35; * ? ! ( ) , = @ ; $ '."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Name of the IoT Hub where the device was created or deleted."]
    #[serde(rename = "hubName", default, skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[doc = "Information about the device twin, which is the cloud representation of application device metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twin: Option<DeviceTwinInfo>,
}
impl DeviceLifeCycleEventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a device telemetry event (DeviceTelemetry)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceTelemetryEventProperties {
    #[doc = "The content of the message from the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[doc = "Application properties are user-defined strings that can be added to the message. These fields are optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "System properties help identify contents and source of the messages."]
    #[serde(rename = "systemProperties", default, skip_serializing_if = "Option::is_none")]
    pub system_properties: Option<serde_json::Value>,
}
impl DeviceTelemetryEventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the device twin, which is the cloud representation of application device metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceTwinInfo {
    #[doc = "Authentication type used for this device: either SAS, SelfSigned, or CertificateAuthority."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[doc = "Count of cloud to device messages sent to this device."]
    #[serde(rename = "cloudToDeviceMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub cloud_to_device_message_count: Option<f64>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<device_twin_info::Properties>,
    #[doc = "Whether the device twin is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The ISO8601 timestamp of the last device twin status update."]
    #[serde(rename = "statusUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub status_update_time: Option<String>,
    #[doc = "An integer that is incremented by one each time the device twin is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    #[doc = "The thumbprint is a unique value for the x509 certificate, commonly used to find a particular certificate in a certificate store. The thumbprint is dynamically generated using the SHA1 algorithm, and does not physically exist in the certificate."]
    #[serde(rename = "x509Thumbprint", default, skip_serializing_if = "Option::is_none")]
    pub x509_thumbprint: Option<device_twin_info::X509Thumbprint>,
}
impl DeviceTwinInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod device_twin_info {
    use super::*;
    #[doc = "Properties JSON element."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "A portion of the properties that can be written only by the application back-end, and read by the device."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub desired: Option<DeviceTwinProperties>,
        #[doc = "A portion of the properties that can be written only by the application back-end, and read by the device."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reported: Option<DeviceTwinProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The thumbprint is a unique value for the x509 certificate, commonly used to find a particular certificate in a certificate store. The thumbprint is dynamically generated using the SHA1 algorithm, and does not physically exist in the certificate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct X509Thumbprint {
        #[doc = "Primary thumbprint for the x509 certificate."]
        #[serde(rename = "primaryThumbprint", default, skip_serializing_if = "Option::is_none")]
        pub primary_thumbprint: Option<String>,
        #[doc = "Secondary thumbprint for the x509 certificate."]
        #[serde(rename = "secondaryThumbprint", default, skip_serializing_if = "Option::is_none")]
        pub secondary_thumbprint: Option<String>,
    }
    impl X509Thumbprint {
        pub fn new() -> Self {
            Self::default()
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceTwinProperties {
    #[doc = "Metadata information for the properties JSON document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DeviceTwinMetadata>,
    #[doc = "Version of device twin properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
}
impl DeviceTwinProperties {
    pub fn new() -> Self {
        Self::default()
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
    pub event_time: time::OffsetDateTime,
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
        event_time: time::OffsetDateTime,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventGridMqttClientCreatedOrUpdatedEventData {
    #[serde(flatten)]
    pub event_grid_mqtt_client_event_data: EventGridMqttClientEventData,
    #[doc = "Configured state of the client. The value could be Enabled or Disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<event_grid_mqtt_client_created_or_updated_event_data::State>,
    #[doc = "Time the client resource is created based on the provider's UTC time."]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Time the client resource is last updated based on the provider's UTC time. If the client resource was never updated, this value is identical to the value of the 'createdOn' property."]
    #[serde(rename = "updatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[doc = "The key-value attributes that are assigned to the client resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}
impl EventGridMqttClientCreatedOrUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_grid_mqtt_client_created_or_updated_event_data {
    use super::*;
    #[doc = "Configured state of the client. The value could be Enabled or Disabled"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "Schema of the Data property of an EventGridEvent for MQTT Client state changes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventGridMqttClientEventData {
    #[doc = "Unique identifier for the MQTT client that the client presents to the service for authentication. This case-sensitive string can be up to 128 characters long, and supports UTF-8 characters."]
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
    #[doc = "Unique identifier for the MQTT client's session. This case-sensitive string can be up to 128 characters long, and supports UTF-8 characters."]
    #[serde(rename = "clientSessionName", default, skip_serializing_if = "Option::is_none")]
    pub client_session_name: Option<String>,
    #[doc = "A number that helps indicate order of MQTT client session connected or disconnected events. Latest event will have a sequence number that is higher than the previous event."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<f64>,
}
impl EventGridMqttClientSessionConnectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event data for Microsoft.EventGrid.MQTTClientSessionDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventGridMqttClientSessionDisconnectedEventData {
    #[serde(flatten)]
    pub event_grid_mqtt_client_event_data: EventGridMqttClientEventData,
    #[doc = "Unique identifier for the MQTT client's session. This case-sensitive string can be up to 128 characters long, and supports UTF-8 characters."]
    #[serde(rename = "clientSessionName", default, skip_serializing_if = "Option::is_none")]
    pub client_session_name: Option<String>,
    #[doc = "A number that helps indicate order of MQTT client session connected or disconnected events. Latest event will have a sequence number that is higher than the previous event."]
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
    #[doc = "Reason for the disconnection of the MQTT client's session. The value could be one of the values in the disconnection reasons table."]
    #[serde(rename = "disconnectionReason", default, skip_serializing_if = "Option::is_none")]
    pub disconnection_reason: Option<event_grid_mqtt_client_session_disconnected_event_data::DisconnectionReason>,
}
impl EventGridMqttClientSessionDisconnectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_grid_mqtt_client_session_disconnected_event_data {
    use super::*;
    #[doc = "Reason for the disconnection of the MQTT client's session. The value could be one of the values in the disconnection reasons table."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DisconnectionReason")]
    pub enum DisconnectionReason {
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
    impl FromStr for DisconnectionReason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DisconnectionReason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DisconnectionReason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ClientAuthenticationError => {
                    serializer.serialize_unit_variant("DisconnectionReason", 0u32, "ClientAuthenticationError")
                }
                Self::ClientAuthorizationError => {
                    serializer.serialize_unit_variant("DisconnectionReason", 1u32, "ClientAuthorizationError")
                }
                Self::ClientError => serializer.serialize_unit_variant("DisconnectionReason", 2u32, "ClientError"),
                Self::ClientInitiatedDisconnect => {
                    serializer.serialize_unit_variant("DisconnectionReason", 3u32, "ClientInitiatedDisconnect")
                }
                Self::ConnectionLost => serializer.serialize_unit_variant("DisconnectionReason", 4u32, "ConnectionLost"),
                Self::IpForbidden => serializer.serialize_unit_variant("DisconnectionReason", 5u32, "IpForbidden"),
                Self::QuotaExceeded => serializer.serialize_unit_variant("DisconnectionReason", 6u32, "QuotaExceeded"),
                Self::ServerError => serializer.serialize_unit_variant("DisconnectionReason", 7u32, "ServerError"),
                Self::ServerInitiatedDisconnect => {
                    serializer.serialize_unit_variant("DisconnectionReason", 8u32, "ServerInitiatedDisconnect")
                }
                Self::SessionOverflow => serializer.serialize_unit_variant("DisconnectionReason", 9u32, "SessionOverflow"),
                Self::SessionTakenOver => serializer.serialize_unit_variant("DisconnectionReason", 10u32, "SessionTakenOver"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.EventHub.CaptureFileCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    pub size_in_bytes: Option<i64>,
    #[doc = "The number of events in the file."]
    #[serde(rename = "eventCount", default, skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    #[doc = "The smallest sequence number from the queue."]
    #[serde(rename = "firstSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub first_sequence_number: Option<i64>,
    #[doc = "The last sequence number from the queue."]
    #[serde(rename = "lastSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub last_sequence_number: Option<i64>,
    #[doc = "The first time from the queue."]
    #[serde(rename = "firstEnqueueTime", default, with = "azure_core::date::rfc3339::option")]
    pub first_enqueue_time: Option<time::OffsetDateTime>,
    #[doc = "The last time from the queue."]
    #[serde(rename = "lastEnqueueTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_enqueue_time: Option<time::OffsetDateTime>,
}
impl EventHubCaptureFileCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Failed LockToken information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedLockToken {
    #[doc = "The lock token of an entry in the request."]
    #[serde(rename = "lockToken")]
    pub lock_token: String,
    #[doc = "The error object."]
    pub error: AzureCoreFoundationsError,
}
impl FailedLockToken {
    pub fn new(lock_token: String, error: AzureCoreFoundationsError) -> Self {
        Self { lock_token, error }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthcareFhirResourceCreatedEventData {
    #[doc = "Schema of FHIR resource type enumeration."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<HealthcareFhirResourceType>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.HealthcareApis.FhirResourceDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthcareFhirResourceDeletedEventData {
    #[doc = "Schema of FHIR resource type enumeration."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<HealthcareFhirResourceType>,
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
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthcareFhirResourceUpdatedEventData {
    #[doc = "Schema of FHIR resource type enumeration."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<HealthcareFhirResourceType>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceConnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubDeviceConnectedEventData {
    #[serde(flatten)]
    pub device_connection_state_event_properties: DeviceConnectionStateEventProperties,
}
impl IotHubDeviceConnectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubDeviceCreatedEventData {
    #[serde(flatten)]
    pub device_life_cycle_event_properties: DeviceLifeCycleEventProperties,
}
impl IotHubDeviceCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubDeviceDeletedEventData {
    #[serde(flatten)]
    pub device_life_cycle_event_properties: DeviceLifeCycleEventProperties,
}
impl IotHubDeviceDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubDeviceDisconnectedEventData {
    #[serde(flatten)]
    pub device_connection_state_event_properties: DeviceConnectionStateEventProperties,
}
impl IotHubDeviceDisconnectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event data for Microsoft.Devices.DeviceTelemetry event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IotHubDeviceTelemetryEventData {
    #[serde(flatten)]
    pub device_telemetry_event_properties: DeviceTelemetryEventProperties,
}
impl IotHubDeviceTelemetryEventData {
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
}
impl KeyVaultSecretNewVersionCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.KeyVault.VaultAccessPolicyChanged event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultVaultAccessPolicyChangedEventData {
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
    pub nbf: Option<f64>,
    #[doc = "The expiration date of the object that triggered this event"]
    #[serde(rename = "EXP", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
}
impl KeyVaultVaultAccessPolicyChangedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.DatasetDriftDetected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the target dataset time series that resulted in drift detection."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl MachineLearningServicesDatasetDriftDetectedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.ModelDeployed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "serviceTags", default, skip_serializing_if = "Option::is_none")]
    pub service_tags: Option<serde_json::Value>,
    #[doc = "The properties of the deployed service."]
    #[serde(rename = "serviceProperties", default, skip_serializing_if = "Option::is_none")]
    pub service_properties: Option<serde_json::Value>,
}
impl MachineLearningServicesModelDeployedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.ModelRegistered event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineLearningServicesModelRegisteredEventData {
    #[doc = "The name of the model that was registered."]
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[doc = "The version of the model that was registered."]
    #[serde(rename = "modelVersion", default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    #[doc = "The tags of the model that was registered."]
    #[serde(rename = "modelTags", default, skip_serializing_if = "Option::is_none")]
    pub model_tags: Option<serde_json::Value>,
    #[doc = "The properties of the model that was registered."]
    #[serde(rename = "modelProperties", default, skip_serializing_if = "Option::is_none")]
    pub model_properties: Option<serde_json::Value>,
}
impl MachineLearningServicesModelRegisteredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.RunCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "runTags", default, skip_serializing_if = "Option::is_none")]
    pub run_tags: Option<serde_json::Value>,
    #[doc = "The properties of the completed Run."]
    #[serde(rename = "runProperties", default, skip_serializing_if = "Option::is_none")]
    pub run_properties: Option<serde_json::Value>,
}
impl MachineLearningServicesRunCompletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.MachineLearningServices.RunStatusChanged event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "runTags", default, skip_serializing_if = "Option::is_none")]
    pub run_tags: Option<serde_json::Value>,
    #[doc = "The properties of the Machine Learning Run."]
    #[serde(rename = "runProperties", default, skip_serializing_if = "Option::is_none")]
    pub run_properties: Option<serde_json::Value>,
    #[doc = "The status of the Machine Learning Run."]
    #[serde(rename = "runStatus", default, skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
}
impl MachineLearningServicesRunStatusChangedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Maps.GeofenceEntered event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsGeofenceEnteredEventData {
    #[serde(flatten)]
    pub maps_geofence_event_properties: MapsGeofenceEventProperties,
}
impl MapsGeofenceEnteredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Geofence event (GeofenceEntered, GeofenceExited, GeofenceResult)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Maps.GeofenceExited event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsGeofenceExitedEventData {
    #[serde(flatten)]
    pub maps_geofence_event_properties: MapsGeofenceEventProperties,
}
impl MapsGeofenceExitedEventData {
    pub fn new() -> Self {
        Self::default()
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
    pub distance: Option<f64>,
    #[doc = "The unique ID for the geofence geometry."]
    #[serde(rename = "geometryId", default, skip_serializing_if = "Option::is_none")]
    pub geometry_id: Option<String>,
    #[doc = "Latitude of the nearest point of the geometry."]
    #[serde(rename = "nearestLat", default, skip_serializing_if = "Option::is_none")]
    pub nearest_lat: Option<f64>,
    #[doc = "Longitude of the nearest point of the geometry."]
    #[serde(rename = "nearestLon", default, skip_serializing_if = "Option::is_none")]
    pub nearest_lon: Option<f64>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsGeofenceResultEventData {
    #[serde(flatten)]
    pub maps_geofence_event_properties: MapsGeofenceEventProperties,
}
impl MapsGeofenceResultEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job canceled event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobCanceled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobCanceledEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
    #[doc = "Gets the Job outputs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outputs: Vec<MediaJobOutputUnion>,
}
impl MediaJobCanceledEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job canceling event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobCanceling event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobCancelingEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
}
impl MediaJobCancelingEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of JobOutput errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobError {
    #[doc = "Error code describing the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<media_job_error::Code>,
    #[doc = "A human-readable language-dependent representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Helps with categorization of errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<media_job_error::Category>,
    #[doc = "Indicates that it may be possible to retry the Job. If retry is unsuccessful, please contact Azure support via Azure Portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<media_job_error::Retry>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<MediaJobErrorDetail>,
}
impl MediaJobError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod media_job_error {
    use super::*;
    #[doc = "Error code describing the error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Code {
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
    }
    #[doc = "Helps with categorization of errors."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        Service,
        Download,
        Upload,
        Configuration,
        Content,
        Account,
    }
    #[doc = "Indicates that it may be possible to retry the Job. If retry is unsuccessful, please contact Azure support via Azure Portal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Retry {
        DoNotRetry,
        MayRetry,
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
#[doc = "Job error state event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobErrored event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobErroredEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
    #[doc = "Gets the Job outputs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outputs: Vec<MediaJobOutputUnion>,
}
impl MediaJobErroredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job finished event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobFinished event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobFinishedEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
    #[doc = "Gets the Job outputs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outputs: Vec<MediaJobOutputUnion>,
}
impl MediaJobFinishedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The event data for a Job output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaJobOutput {
    #[doc = "Details of JobOutput errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<MediaJobError>,
    #[doc = "Gets the Job output label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets the Job output progress."]
    pub progress: i64,
    #[doc = "Gets the Job output state."]
    pub state: media_job_output::State,
}
impl MediaJobOutput {
    pub fn new(progress: i64, state: media_job_output::State) -> Self {
        Self {
            error: None,
            label: None,
            progress,
            state,
        }
    }
}
pub mod media_job_output {
    use super::*;
    #[doc = "Gets the Job output state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Canceled,
        Canceling,
        Error,
        Finished,
        Processing,
        Queued,
        Scheduled,
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
#[doc = "Job output canceled event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobOutputCanceled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobOutputCanceledEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputCanceledEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job output canceling event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobOutputCanceling event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobOutputCancelingEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputCancelingEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job output error event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobOutputErrored event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobOutputErroredEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputErroredEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job output finished event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobOutputFinished event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobOutputFinishedEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputFinishedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job output processing event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobOutputProcessing event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobOutputProcessingEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputProcessingEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job Output Progress Event Data. Schema of the Data property of an EventGridEvent for a Microsoft.Media.JobOutputProgress event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobOutputProgressEventData {
    #[doc = "Gets the Job output label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets the Job output progress."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[doc = "Gets the Job correlation data."]
    #[serde(rename = "jobCorrelationData", default, skip_serializing_if = "Option::is_none")]
    pub job_correlation_data: Option<serde_json::Value>,
}
impl MediaJobOutputProgressEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job output scheduled event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobOutputScheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobOutputScheduledEventData {
    #[serde(flatten)]
    pub media_job_output_state_change_event_data: MediaJobOutputStateChangeEventData,
}
impl MediaJobOutputScheduledEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Media.JobOutputStateChange event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobOutputStateChangeEventData {
    #[doc = "The previous state of the Job."]
    #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<media_job_output_state_change_event_data::PreviousState>,
    #[doc = "The event data for a Job output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<MediaJobOutputUnion>,
    #[doc = "Gets the Job correlation data."]
    #[serde(rename = "jobCorrelationData", default, skip_serializing_if = "Option::is_none")]
    pub job_correlation_data: Option<serde_json::Value>,
}
impl MediaJobOutputStateChangeEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod media_job_output_state_change_event_data {
    use super::*;
    #[doc = "The previous state of the Job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreviousState {
        Canceled,
        Canceling,
        Error,
        Finished,
        Processing,
        Queued,
        Scheduled,
    }
}
#[doc = "Job processing event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobProcessing event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobProcessingEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
}
impl MediaJobProcessingEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job scheduled event data. Schema of the data property of an EventGridEvent for a Microsoft.Media.JobScheduled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobScheduledEventData {
    #[serde(flatten)]
    pub media_job_state_change_event_data: MediaJobStateChangeEventData,
}
impl MediaJobScheduledEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Media.JobStateChange event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaJobStateChangeEventData {
    #[doc = "The previous state of the Job."]
    #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<media_job_state_change_event_data::PreviousState>,
    #[doc = "The new state of the Job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<media_job_state_change_event_data::State>,
    #[doc = "Gets the Job correlation data."]
    #[serde(rename = "correlationData", default, skip_serializing_if = "Option::is_none")]
    pub correlation_data: Option<serde_json::Value>,
}
impl MediaJobStateChangeEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod media_job_state_change_event_data {
    use super::*;
    #[doc = "The previous state of the Job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreviousState {
        Canceled,
        Canceling,
        Error,
        Finished,
        Processing,
        Queued,
        Scheduled,
    }
    #[doc = "The new state of the Job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Canceled,
        Canceling,
        Error,
        Finished,
        Processing,
        Queued,
        Scheduled,
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
    #[doc = "Gets the timescale in which \"MinLastTimestamp\" is represented."]
    #[serde(rename = "timescaleOfMinLastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub timescale_of_min_last_timestamp: Option<String>,
    #[doc = "Gets the timescale in which \"MaxLastTimestamp\" is represented."]
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
#[doc = "A Microsoft Teams user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftTeamsUserIdentifierModel {
    #[doc = "The Id of the Microsoft Teams user. If not anonymous, this is the AAD object Id of the user."]
    #[serde(rename = "userId")]
    pub user_id: String,
    #[doc = "True if the Microsoft Teams user is anonymous. By default false if missing."]
    #[serde(rename = "isAnonymous", default, skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[doc = "The cloud that the identifier belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<CommunicationCloudEnvironmentModel>,
}
impl MicrosoftTeamsUserIdentifierModel {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            is_anonymous: None,
            cloud: None,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyInsightsPolicyStateChangedEventData {
    #[doc = "The time that the resource was scanned by Azure Policy in the Universal ISO 8601 DateTime format yyyy-MM-ddTHH:mm:ss.fffffffZ."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.PolicyInsights.PolicyStateCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyInsightsPolicyStateCreatedEventData {
    #[doc = "The time that the resource was scanned by Azure Policy in the Universal ISO 8601 DateTime format yyyy-MM-ddTHH:mm:ss.fffffffZ."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.PolicyInsights.PolicyStateDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyInsightsPolicyStateDeletedEventData {
    #[doc = "The time that the resource was scanned by Azure Policy in the Universal ISO 8601 DateTime format yyyy-MM-ddTHH:mm:ss.fffffffZ."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the Publish operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishResult {}
impl PublishResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Receive operation details per Cloud Event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiveDetails {
    #[doc = "Properties of the Event Broker operation."]
    #[serde(rename = "brokerProperties")]
    pub broker_properties: BrokerProperties,
    #[doc = "Properties of an event published to an Azure Messaging EventGrid Namespace topic using the CloudEvent 1.0 Schema."]
    pub event: CloudEvent,
}
impl ReceiveDetails {
    pub fn new(broker_properties: BrokerProperties, event: CloudEvent) -> Self {
        Self { broker_properties, event }
    }
}
#[doc = "Details of the Receive operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiveResult {
    #[doc = "Array of receive responses, one per cloud event."]
    pub value: Vec<ReceiveDetails>,
}
impl ReceiveResult {
    pub fn new(value: Vec<ReceiveDetails>) -> Self {
        Self { value }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Cache.ExportRDBCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisExportRdbCompletedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The name of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of this event. Failed or  succeeded "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RedisExportRdbCompletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Cache.ImportRDBCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisImportRdbCompletedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The name of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of this event. Failed or  succeeded "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RedisImportRdbCompletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Cache.PatchingCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisPatchingCompletedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The name of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of this event. Failed or  succeeded "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RedisPatchingCompletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Cache.ScalingCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisScalingCompletedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The name of this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of this event. Failed or  succeeded "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RedisScalingCompletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Array of lock tokens for the corresponding received Cloud Events to be rejected."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RejectOptions {
    #[doc = "Array of lock tokens."]
    #[serde(rename = "lockTokens")]
    pub lock_tokens: Vec<String>,
}
impl RejectOptions {
    pub fn new(lock_tokens: Vec<String>) -> Self {
        Self { lock_tokens }
    }
}
#[doc = "The result of the Reject operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RejectResult {
    #[doc = "Array of FailedLockToken for failed cloud events. Each FailedLockToken includes the lock token along with the related error information (namely, the error code and description)."]
    #[serde(rename = "failedLockTokens")]
    pub failed_lock_tokens: Vec<FailedLockToken>,
    #[doc = "Array of lock tokens for the successfully rejected cloud events."]
    #[serde(rename = "succeededLockTokens")]
    pub succeeded_lock_tokens: Vec<String>,
}
impl RejectResult {
    pub fn new(failed_lock_tokens: Vec<FailedLockToken>, succeeded_lock_tokens: Vec<String>) -> Self {
        Self {
            failed_lock_tokens,
            succeeded_lock_tokens,
        }
    }
}
#[doc = "Array of lock tokens for the corresponding received Cloud Events to be released."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReleaseOptions {
    #[doc = "Array of lock tokens."]
    #[serde(rename = "lockTokens")]
    pub lock_tokens: Vec<String>,
}
impl ReleaseOptions {
    pub fn new(lock_tokens: Vec<String>) -> Self {
        Self { lock_tokens }
    }
}
#[doc = "The result of the Release operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReleaseResult {
    #[doc = "Array of FailedLockToken for failed cloud events. Each FailedLockToken includes the lock token along with the related error information (namely, the error code and description)."]
    #[serde(rename = "failedLockTokens")]
    pub failed_lock_tokens: Vec<FailedLockToken>,
    #[doc = "Array of lock tokens for the successfully released cloud events."]
    #[serde(rename = "succeededLockTokens")]
    pub succeeded_lock_tokens: Vec<String>,
}
impl ReleaseResult {
    pub fn new(failed_lock_tokens: Vec<FailedLockToken>, succeeded_lock_tokens: Vec<String>) -> Self {
        Self {
            failed_lock_tokens,
            succeeded_lock_tokens,
        }
    }
}
#[doc = "The result of the RenewLock operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenewCloudEventLocksResult {
    #[doc = "Array of FailedLockToken for failed cloud events. Each FailedLockToken includes the lock token along with the related error information (namely, the error code and description)."]
    #[serde(rename = "failedLockTokens")]
    pub failed_lock_tokens: Vec<FailedLockToken>,
    #[doc = "Array of lock tokens for the successfully renewed locks."]
    #[serde(rename = "succeededLockTokens")]
    pub succeeded_lock_tokens: Vec<String>,
}
impl RenewCloudEventLocksResult {
    pub fn new(failed_lock_tokens: Vec<FailedLockToken>, succeeded_lock_tokens: Vec<String>) -> Self {
        Self {
            failed_lock_tokens,
            succeeded_lock_tokens,
        }
    }
}
#[doc = "Array of lock tokens for the corresponding received Cloud Events to be renewed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenewLockOptions {
    #[doc = "Array of lock tokens."]
    #[serde(rename = "lockTokens")]
    pub lock_tokens: Vec<String>,
}
impl RenewLockOptions {
    pub fn new(lock_tokens: Vec<String>) -> Self {
        Self { lock_tokens }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceActionCancel event. This is raised when a resource action operation is canceled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceActionCancelData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceActionCancelData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceActionFailure event. This is raised when a resource action operation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceActionFailureData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceActionFailureData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceActionSuccess event. This is raised when a resource action operation succeeds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceActionSuccessData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceActionSuccessData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the authorization for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceAuthorization {
    #[doc = "The scope of the authorization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The action being requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The evidence for the authorization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<serde_json::Value>,
}
impl ResourceAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceDeleteCancel event. This is raised when a resource delete operation is canceled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceDeleteCancelData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceDeleteCancelData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceDeleteFailure event. This is raised when a resource delete operation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceDeleteFailureData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceDeleteFailureData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceDeleteSuccess event. This is raised when a resource delete operation succeeds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceDeleteSuccessData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceDeleteSuccessData {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ResourceNotifications.HealthResources.ResourceAnnotated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNotificationsHealthResourcesAnnotatedEventData {
    #[serde(flatten)]
    pub resource_notifications_resource_updated_event_data: ResourceNotificationsResourceUpdatedEventData,
}
impl ResourceNotificationsHealthResourcesAnnotatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.ResourceNotifications.HealthResources.AvailabilityStatusChanged event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNotificationsHealthResourcesAvailabilityStatusChangedEventData {
    #[serde(flatten)]
    pub resource_notifications_resource_updated_event_data: ResourceNotificationsResourceUpdatedEventData,
}
impl ResourceNotificationsHealthResourcesAvailabilityStatusChangedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "details of operational info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNotificationsOperationalDetails {
    #[doc = "Date and Time when resource was updated"]
    #[serde(rename = "resourceEventTime", default, with = "azure_core::date::rfc3339::option")]
    pub resource_event_time: Option<time::OffsetDateTime>,
}
impl ResourceNotificationsOperationalDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the schema of the properties under resource info which are common across all ARN system topic events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[doc = "properties in the payload of the resource for which the event is being emitted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ResourceNotificationsResourceUpdatedDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the schema of the common properties across all ARN system topic events"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNotificationsResourceUpdatedEventData {
    #[doc = "Describes the schema of the properties under resource info which are common across all ARN system topic events"]
    #[serde(rename = "resourceInfo", default, skip_serializing_if = "Option::is_none")]
    pub resource_info: Option<ResourceNotificationsResourceUpdatedDetails>,
    #[doc = "details of operational info"]
    #[serde(rename = "operationalInfo", default, skip_serializing_if = "Option::is_none")]
    pub operational_info: Option<ResourceNotificationsOperationalDetails>,
    #[doc = "api version of the resource properties bag"]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}
impl ResourceNotificationsResourceUpdatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceWriteCancel event. This is raised when a resource create or update operation is canceled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceWriteCancelData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceWriteCancelData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceWriteFailure event. This is raised when a resource create or update operation fails."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceWriteFailureData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceWriteFailureData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Resources.ResourceWriteSuccess event. This is raised when a resource create or update operation succeeds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceWriteSuccessData {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ResourceAuthorization>,
    #[doc = "The properties of the claims."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "An operation ID used for troubleshooting."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The details of the HTTP request."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<ResourceHttpRequest>,
}
impl ResourceWriteSuccessData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRServiceClientConnectionConnectedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.SignalRService.ClientConnectionDisconnected event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRServiceClientConnectionDisconnectedEventData {
    #[doc = "The time at which the event occurred."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.AsyncOperationInitiated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAsyncOperationInitiatedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the Storage service for the storage API operation that triggered this event."]
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
    #[serde(rename = "storageDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub storage_diagnostics: Option<serde_json::Value>,
}
impl StorageAsyncOperationInitiatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.BlobCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageBlobCreatedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the Storage service for the storage API operation that triggered this event."]
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
    #[serde(rename = "storageDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub storage_diagnostics: Option<serde_json::Value>,
}
impl StorageBlobCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.BlobDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageBlobDeletedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the Storage service for the storage API operation that triggered this event."]
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
    #[serde(rename = "storageDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub storage_diagnostics: Option<serde_json::Value>,
}
impl StorageBlobDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Storage.BlobInventoryPolicyCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageBlobInventoryPolicyCompletedEventData {
    #[doc = "The time at which inventory policy was scheduled."]
    #[serde(rename = "scheduleDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub schedule_date_time: Option<time::OffsetDateTime>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.BlobRenamed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "storageDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub storage_diagnostics: Option<serde_json::Value>,
}
impl StorageBlobRenamedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.BlobTierChanged event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageBlobTierChangedEventData {
    #[doc = "The name of the API/operation that triggered this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[doc = "A request id provided by the client of the storage API operation that triggered this event."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The request id generated by the Storage service for the storage API operation that triggered this event."]
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
    #[serde(rename = "storageDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub storage_diagnostics: Option<serde_json::Value>,
}
impl StorageBlobTierChangedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.DirectoryCreated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "storageDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub storage_diagnostics: Option<serde_json::Value>,
}
impl StorageDirectoryCreatedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.DirectoryDeleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "storageDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub storage_diagnostics: Option<serde_json::Value>,
}
impl StorageDirectoryDeletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Storage.DirectoryRenamed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(rename = "storageDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub storage_diagnostics: Option<serde_json::Value>,
}
impl StorageDirectoryRenamedEventData {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageLifecyclePolicyCompletedEventData {
    #[doc = "The time the policy task was scheduled."]
    #[serde(rename = "scheduleTime", default, skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    #[doc = "Execution statistics of a specific policy action in a Blob Management cycle."]
    #[serde(rename = "deleteSummary", default, skip_serializing_if = "Option::is_none")]
    pub delete_summary: Option<StorageLifecyclePolicyActionSummaryDetail>,
    #[doc = "Execution statistics of a specific policy action in a Blob Management cycle."]
    #[serde(rename = "tierToCoolSummary", default, skip_serializing_if = "Option::is_none")]
    pub tier_to_cool_summary: Option<StorageLifecyclePolicyActionSummaryDetail>,
    #[doc = "Execution statistics of a specific policy action in a Blob Management cycle."]
    #[serde(rename = "tierToArchiveSummary", default, skip_serializing_if = "Option::is_none")]
    pub tier_to_archive_summary: Option<StorageLifecyclePolicyActionSummaryDetail>,
}
impl StorageLifecyclePolicyCompletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Storage.StorageTaskCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskCompletedEventData {
    #[doc = "The status for a storage task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<storage_task_completed_event_data::Status>,
    #[doc = "The time at which a storage task was completed."]
    #[serde(rename = "completedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub completed_date_time: Option<time::OffsetDateTime>,
    #[doc = "The execution id for a storage task."]
    #[serde(rename = "taskExecutionId", default, skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    #[doc = "The task name for a storage task."]
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[doc = "The summary report blob url for a storage task"]
    #[serde(rename = "summaryReportBlobUrl", default, skip_serializing_if = "Option::is_none")]
    pub summary_report_blob_url: Option<String>,
}
impl StorageTaskCompletedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_task_completed_event_data {
    use super::*;
    #[doc = "The status for a storage task."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for an Microsoft.Storage.StorageTaskQueued event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskQueuedEventData {
    #[doc = "The time at which a storage task was queued."]
    #[serde(rename = "queuedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub queued_date_time: Option<time::OffsetDateTime>,
    #[doc = "The execution id for a storage task."]
    #[serde(rename = "taskExecutionId", default, skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
}
impl StorageTaskQueuedEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.EventGrid.SubscriptionDeletedEvent event."]
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
    #[doc = "The validation code sent by Azure Event Grid to validate an event subscription. To complete the validation handshake, the subscriber must either respond with this validation code as part of the validation response, or perform a GET request on the validationUrl (available starting version 2018-05-01-preview)."]
    #[serde(rename = "validationCode", default, skip_serializing_if = "Option::is_none")]
    pub validation_code: Option<String>,
    #[doc = "The validation URL sent by Azure Event Grid (available starting version 2018-05-01-preview). To complete the validation handshake, the subscriber must either respond with the validationCode as part of the validation response, or perform a GET request on the validationUrl (available starting version 2018-05-01-preview)."]
    #[serde(rename = "validationUrl", default, skip_serializing_if = "Option::is_none")]
    pub validation_url: Option<String>,
}
impl SubscriptionValidationEventData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "To complete an event subscription validation handshake, a subscriber can use either the validationCode or the validationUrl received in a SubscriptionValidationEvent. When the validationCode is used, the SubscriptionValidationResponse can be used to build the response."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppServicePlanUpdatedEventData {
    #[doc = "Detail of action on the app service plan."]
    #[serde(rename = "appServicePlanEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_service_plan_event_type_detail: Option<AppServicePlanEventTypeDetail>,
    #[doc = "sku of app service plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<web_app_service_plan_updated_event_data::Sku>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod web_app_service_plan_updated_event_data {
    use super::*;
    #[doc = "sku of app service plan."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Sku {
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
    impl Sku {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.AppUpdated event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppUpdatedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.BackupOperationCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebBackupOperationCompletedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.BackupOperationFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebBackupOperationFailedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.BackupOperationStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebBackupOperationStartedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.RestoreOperationCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebRestoreOperationCompletedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.RestoreOperationFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebRestoreOperationFailedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.RestoreOperationStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebRestoreOperationStartedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapCompleted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebSlotSwapCompletedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapFailed event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebSlotSwapFailedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebSlotSwapStartedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapWithPreviewCancelled event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebSlotSwapWithPreviewCancelledEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema of the Data property of an EventGridEvent for a Microsoft.Web.SlotSwapWithPreviewStarted event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebSlotSwapWithPreviewStartedEventData {
    #[doc = "Detail of action on the app."]
    #[serde(rename = "appEventTypeDetail", default, skip_serializing_if = "Option::is_none")]
    pub app_event_type_detail: Option<AppEventTypeDetail>,
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
    pub fn new() -> Self {
        Self::default()
    }
}
