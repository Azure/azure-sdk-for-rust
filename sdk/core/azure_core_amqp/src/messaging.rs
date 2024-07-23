// Copyright (c) Microsoft Corp. All Rights Reserved
//cspell: words amqp SMALLUINT SMALLULONG

use super::value::{AmqpList, AmqpOrderedMap, AmqpSymbol, AmqpTimestamp, AmqpValue};
use azure_core::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum TerminusDurability {
    None,
    Configuration,
    UnsettledState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TerminusExpiryPolicy {
    LinkDetach,
    SessionEnd,
    ConnectionClose,
    Never,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DistributionMode {
    Move,
    Copy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmqpOutcome {
    Accepted,
    Rejected,
    Released,
    Modified,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmqpMessageId {
    String(String),
    Uuid(uuid::Uuid),
    Binary(Vec<u8>),
    Ulong(u64),
}

impl Into<AmqpMessageId> for uuid::Uuid {
    fn into(self) -> AmqpMessageId {
        AmqpMessageId::Uuid(self)
    }
}

impl Into<AmqpMessageId> for String {
    fn into(self) -> AmqpMessageId {
        AmqpMessageId::String(self)
    }
}

impl Into<AmqpMessageId> for Vec<u8> {
    fn into(self) -> AmqpMessageId {
        AmqpMessageId::Binary(self)
    }
}

impl Into<AmqpMessageId> for u64 {
    fn into(self) -> AmqpMessageId {
        AmqpMessageId::Ulong(self)
    }
}

/// A target node in an AMQP message
#[derive(Debug, Clone, PartialEq)]
pub struct AmqpTarget {
    pub address: Option<String>,
    pub durable: Option<TerminusDurability>,
    pub expiry_policy: Option<TerminusExpiryPolicy>,
    pub timeout: Option<u32>,
    pub dynamic: Option<bool>,
    pub dynamic_node_properties: Option<AmqpOrderedMap<String, AmqpValue>>,
    pub capabilities: Option<Vec<AmqpValue>>,
}

impl AmqpTarget {
    pub fn builder() -> builders::AmqpTargetBuilder {
        builders::AmqpTargetBuilder::new()
    }
}

impl Into<String> for AmqpTarget {
    fn into(self) -> String {
        self.address.unwrap()
    }
}

impl Into<AmqpTarget> for String {
    fn into(self) -> AmqpTarget {
        AmqpTarget {
            address: Some(self),
            durable: None,
            expiry_policy: None,
            timeout: None,
            dynamic: None,
            dynamic_node_properties: None,
            capabilities: None,
        }
    }
}

/// A source node in an AMQP message
#[derive(Debug, Clone, PartialEq)]
pub struct AmqpSource {
    pub address: Option<String>,
    pub durable: Option<TerminusDurability>,
    pub expiry_policy: Option<TerminusExpiryPolicy>,
    pub timeout: Option<u32>,
    pub dynamic: Option<bool>,
    pub dynamic_node_properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    pub distribution_mode: Option<DistributionMode>,
    pub filter: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    pub default_outcome: Option<AmqpOutcome>,
    pub outcomes: Option<Vec<AmqpSymbol>>,
    pub capabilities: Option<Vec<AmqpSymbol>>,
}

impl AmqpSource {
    pub fn builder() -> builders::AmqpSourceBuilder {
        builders::AmqpSourceBuilder::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmqpMessageHeader {
    durable: Option<bool>,
    priority: Option<u8>,
    time_to_live: Option<std::time::Duration>,
    first_acquirer: Option<bool>,
    delivery_count: Option<u32>,
}

impl AmqpMessageHeader {
    pub fn builder() -> builders::AmqpMessageHeaderBuilder {
        builders::AmqpMessageHeaderBuilder::new()
    }

    pub fn durable(&self) -> Option<&bool> {
        self.durable.as_ref()
    }

    pub fn priority(&self) -> Option<&u8> {
        self.priority.as_ref()
    }

    pub fn time_to_live(&self) -> Option<&std::time::Duration> {
        self.time_to_live.as_ref()
    }

    pub fn first_acquirer(&self) -> Option<&bool> {
        self.first_acquirer.as_ref()
    }

    pub fn delivery_count(&self) -> Option<&u32> {
        self.delivery_count.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AmqpMessageProperties {
    message_id: Option<AmqpMessageId>,
    user_id: Option<Vec<u8>>,
    to: Option<String>,
    subject: Option<String>,
    reply_to: Option<String>,
    correlation_id: Option<AmqpMessageId>,
    content_type: Option<AmqpSymbol>,
    content_encoding: Option<AmqpSymbol>,
    absolute_expiry_time: Option<AmqpTimestamp>,
    creation_time: Option<AmqpTimestamp>,
    group_id: Option<String>,
    group_sequence: Option<u32>,
    reply_to_group_id: Option<String>,
}

impl AmqpMessageProperties {
    pub fn builder() -> builders::AmqpMessagePropertiesBuilder {
        builders::AmqpMessagePropertiesBuilder::new()
    }

    pub fn message_id(&self) -> Option<&AmqpMessageId> {
        self.message_id.as_ref()
    }

    pub fn user_id(&self) -> Option<&Vec<u8>> {
        self.user_id.as_ref()
    }

    pub fn to(&self) -> Option<&String> {
        self.to.as_ref()
    }

    pub fn subject(&self) -> Option<&String> {
        self.subject.as_ref()
    }

    pub fn reply_to(&self) -> Option<&String> {
        self.reply_to.as_ref()
    }

    pub fn correlation_id(&self) -> Option<&AmqpMessageId> {
        self.correlation_id.as_ref()
    }

    pub fn content_type(&self) -> Option<&AmqpSymbol> {
        self.content_type.as_ref()
    }

    pub fn content_encoding(&self) -> Option<&AmqpSymbol> {
        self.content_encoding.as_ref()
    }

    pub fn absolute_expiry_time(&self) -> Option<&AmqpTimestamp> {
        self.absolute_expiry_time.as_ref()
    }

    pub fn creation_time(&self) -> Option<&AmqpTimestamp> {
        self.creation_time.as_ref()
    }

    pub fn group_id(&self) -> Option<&String> {
        self.group_id.as_ref()
    }

    pub fn group_sequence(&self) -> Option<&u32> {
        self.group_sequence.as_ref()
    }

    pub fn reply_to_group_id(&self) -> Option<&String> {
        self.reply_to_group_id.as_ref()
    }

    pub fn set_message_id(&mut self, message_id: impl Into<AmqpMessageId>) {
        self.message_id = Some(message_id.into());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmqpMessageBody {
    Binary(Vec<Vec<u8>>),
    Sequence(Vec<AmqpList>),
    Value(AmqpValue),
    Empty,
}

impl From<Vec<u8>> for AmqpMessageBody {
    fn from(body: Vec<u8>) -> Self {
        AmqpMessageBody::Binary(vec![body])
    }
}

impl From<Vec<Vec<u8>>> for AmqpMessageBody {
    fn from(body: Vec<Vec<u8>>) -> Self {
        AmqpMessageBody::Binary(body)
    }
}

impl From<AmqpValue> for AmqpMessageBody {
    fn from(value: AmqpValue) -> Self {
        AmqpMessageBody::Value(value)
    }
}

impl From<AmqpList> for AmqpMessageBody {
    fn from(list: AmqpList) -> Self {
        AmqpMessageBody::Sequence(vec![list])
    }
}

impl From<Vec<AmqpList>> for AmqpMessageBody {
    fn from(lists: Vec<AmqpList>) -> Self {
        AmqpMessageBody::Sequence(lists)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmqpAnnotationKey {
    Symbol(AmqpSymbol),
    Ulong(u64),
}

impl Default for AmqpAnnotationKey {
    fn default() -> Self {
        Self::Ulong(0)
    }
}

impl Into<AmqpAnnotationKey> for AmqpSymbol {
    fn into(self) -> AmqpAnnotationKey {
        AmqpAnnotationKey::Symbol(self)
    }
}

impl Into<AmqpAnnotationKey> for u64 {
    fn into(self) -> AmqpAnnotationKey {
        AmqpAnnotationKey::Ulong(self)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AmqpAnnotations(pub AmqpOrderedMap<AmqpAnnotationKey, AmqpValue>);

impl AmqpAnnotations {
    pub fn new() -> Self {
        AmqpAnnotations(AmqpOrderedMap::new())
    }

    pub fn insert(&mut self, key: impl Into<AmqpAnnotationKey>, value: impl Into<AmqpValue>) {
        self.0.insert(key.into(), value.into());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmqpApplicationProperties(pub AmqpOrderedMap<String, AmqpValue>);

impl AmqpApplicationProperties {
    pub fn new() -> Self {
        AmqpApplicationProperties(AmqpOrderedMap::new())
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<AmqpValue>) {
        self.0.insert(key.into(), value.into());
    }
}

/// An AMQP message
/// This is a simplified version of the AMQP message
/// that is used in the Azure SDK for Event Hubs
/// and is not a complete implementation of the AMQP message
/// as defined in the AMQP specification
/// https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-messaging-v1.0-os.html
///
#[derive(Debug, Clone, PartialEq)]
pub struct AmqpMessage {
    body: AmqpMessageBody,
    header: Option<AmqpMessageHeader>,
    application_properties: Option<AmqpApplicationProperties>,
    message_annotations: Option<AmqpAnnotations>,
    delivery_annotations: Option<AmqpAnnotations>,
    properties: Option<AmqpMessageProperties>,
    footer: Option<AmqpAnnotations>,
}

impl AmqpMessage {
    pub fn builder() -> builders::AmqpMessageBuilder {
        builders::AmqpMessageBuilder::new()
    }

    pub fn body(&self) -> &AmqpMessageBody {
        &self.body
    }

    pub fn header(&self) -> Option<&AmqpMessageHeader> {
        self.header.as_ref()
    }

    pub fn application_properties(&self) -> Option<&AmqpApplicationProperties> {
        self.application_properties.as_ref()
    }

    pub fn message_annotations(&self) -> Option<&AmqpAnnotations> {
        self.message_annotations.as_ref()
    }

    pub fn delivery_annotations(&self) -> Option<&AmqpAnnotations> {
        self.delivery_annotations.as_ref()
    }

    pub fn properties(&self) -> Option<&AmqpMessageProperties> {
        self.properties.as_ref()
    }

    pub fn footer(&self) -> Option<&AmqpAnnotations> {
        self.footer.as_ref()
    }

    pub fn set_properties(&mut self, properties: AmqpMessageProperties) {
        self.properties = Some(properties);
    }

    pub fn set_message_annotations(&mut self, message_annotations: impl Into<AmqpAnnotations>) {
        self.message_annotations = Some(message_annotations.into());
    }

    pub fn set_message_body(&mut self, body: impl Into<AmqpMessageBody>) {
        self.body = body.into();
    }

    pub fn serialize(message: AmqpMessage) -> Result<Vec<u8>> {
        #[cfg(any(feature = "enable-fe2o3-amqp"))]
        {
            let amqp_message = Into::<
                fe2o3_amqp_types::messaging::Message<
                    fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
                >,
            >::into(message);
            let res = serde_amqp::ser::to_vec(
                &fe2o3_amqp_types::messaging::message::__private::Serializable(amqp_message),
            )
            .map_err(crate::fe2o3::error::AmqpSerializationError::from)?;
            Ok(res)
        }
        #[cfg(not(any(feature = "enable-fe2o3-amqp")))]
        unimplemented!()
    }
}

pub mod builders {
    use super::*;

    pub struct AmqpSourceBuilder {
        source: AmqpSource,
    }

    impl AmqpSourceBuilder {
        pub fn build(self) -> AmqpSource {
            self.source
        }
        pub(super) fn new() -> AmqpSourceBuilder {
            AmqpSourceBuilder {
                source: AmqpSource {
                    address: None,
                    durable: None,
                    expiry_policy: None,
                    timeout: None,
                    dynamic: None,
                    dynamic_node_properties: None,
                    distribution_mode: None,
                    filter: None,
                    default_outcome: None,
                    outcomes: None,
                    capabilities: None,
                },
            }
        }
        pub fn with_address(mut self, address: impl Into<String>) -> Self {
            self.source.address = Some(address.into());
            self
        }
        pub fn with_durable(mut self, durable: TerminusDurability) -> Self {
            self.source.durable = Some(durable);
            self
        }
        pub fn with_expiry_policy(mut self, expiry_policy: TerminusExpiryPolicy) -> Self {
            self.source.expiry_policy = Some(expiry_policy.into());
            self
        }
        pub fn with_timeout(mut self, timeout: u32) -> Self {
            self.source.timeout = Some(timeout);
            self
        }
        pub fn with_dynamic(mut self, dynamic: bool) -> Self {
            self.source.dynamic = Some(dynamic);
            self
        }
        pub fn with_dynamic_node_properties(
            mut self,
            dynamic_node_properties: impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
        ) -> Self {
            self.source.dynamic_node_properties = Some(dynamic_node_properties.into());
            self
        }
        pub fn with_distribution_mode(mut self, distribution_mode: DistributionMode) -> Self {
            self.source.distribution_mode = Some(distribution_mode);
            self
        }
        pub fn with_filter(
            mut self,
            filter: impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
        ) -> Self {
            self.source.filter = Some(filter.into());
            self
        }
        pub fn with_default_outcome(mut self, default_outcome: AmqpOutcome) -> Self {
            self.source.default_outcome = Some(default_outcome);
            self
        }
        pub fn with_outcomes(mut self, outcomes: Vec<AmqpSymbol>) -> Self {
            self.source.outcomes = Some(outcomes);
            self
        }
        pub fn with_capabilities(mut self, capabilities: Vec<AmqpSymbol>) -> Self {
            self.source.capabilities = Some(capabilities);
            self
        }
    }

    pub struct AmqpTargetBuilder {
        target: AmqpTarget,
    }

    impl AmqpTargetBuilder {
        pub fn build(self) -> AmqpTarget {
            self.target
        }
        pub(super) fn new() -> AmqpTargetBuilder {
            AmqpTargetBuilder {
                target: AmqpTarget {
                    address: None,
                    durable: None,
                    expiry_policy: None,
                    timeout: None,
                    dynamic: None,
                    dynamic_node_properties: None,
                    capabilities: None,
                },
            }
        }
        pub fn with_address(mut self, address: impl Into<String>) -> Self {
            self.target.address = Some(address.into());
            self
        }
        pub fn with_durable(mut self, durable: TerminusDurability) -> Self {
            self.target.durable = Some(durable);
            self
        }
        pub fn with_expiry_policy(mut self, expiry_policy: TerminusExpiryPolicy) -> Self {
            self.target.expiry_policy = Some(expiry_policy.into());
            self
        }
        pub fn with_timeout(mut self, timeout: u32) -> Self {
            self.target.timeout = Some(timeout);
            self
        }
        pub fn with_dynamic(mut self, dynamic: bool) -> Self {
            self.target.dynamic = Some(dynamic);
            self
        }
        pub fn with_dynamic_node_properties(
            mut self,
            dynamic_node_properties: impl Into<AmqpOrderedMap<String, AmqpValue>>,
        ) -> Self {
            self.target.dynamic_node_properties = Some(dynamic_node_properties.into());
            self
        }
        pub fn with_capabilities(mut self, capabilities: Vec<AmqpValue>) -> Self {
            self.target.capabilities = Some(capabilities);
            self
        }
    }

    pub struct AmqpMessageHeaderBuilder {
        header: AmqpMessageHeader,
    }

    impl AmqpMessageHeaderBuilder {
        pub fn build(self) -> AmqpMessageHeader {
            self.header
        }
        pub(super) fn new() -> AmqpMessageHeaderBuilder {
            AmqpMessageHeaderBuilder {
                header: AmqpMessageHeader {
                    durable: None,
                    priority: None,
                    time_to_live: None,
                    first_acquirer: None,
                    delivery_count: None,
                },
            }
        }
        pub fn with_durable(mut self, durable: bool) -> Self {
            self.header.durable = Some(durable);
            self
        }
        pub fn with_priority(mut self, priority: u8) -> Self {
            self.header.priority = Some(priority);
            self
        }
        pub fn with_time_to_live(mut self, time_to_live: impl Into<std::time::Duration>) -> Self {
            self.header.time_to_live = Some(time_to_live.into());
            self
        }
        pub fn with_first_acquirer(mut self, first_acquirer: bool) -> Self {
            self.header.first_acquirer = Some(first_acquirer);
            self
        }
        pub fn with_delivery_count(mut self, delivery_count: u32) -> Self {
            self.header.delivery_count = Some(delivery_count);
            self
        }
    }

    pub struct AmqpMessagePropertiesBuilder {
        properties: AmqpMessageProperties,
    }

    impl AmqpMessagePropertiesBuilder {
        pub fn build(self) -> AmqpMessageProperties {
            self.properties
        }
        pub(super) fn new() -> AmqpMessagePropertiesBuilder {
            AmqpMessagePropertiesBuilder {
                properties: AmqpMessageProperties {
                    message_id: None,
                    user_id: None,
                    to: None,
                    subject: None,
                    reply_to: None,
                    correlation_id: None,
                    content_type: None,
                    content_encoding: None,
                    absolute_expiry_time: None,
                    creation_time: None,
                    group_id: None,
                    group_sequence: None,
                    reply_to_group_id: None,
                },
            }
        }
        pub fn with_message_id(mut self, message_id: impl Into<AmqpMessageId>) -> Self {
            self.properties.message_id = Some(message_id.into());
            self
        }
        pub fn with_user_id(mut self, user_id: impl Into<Vec<u8>>) -> Self {
            self.properties.user_id = Some(user_id.into());
            self
        }
        pub fn with_to(mut self, to: impl Into<String>) -> Self {
            self.properties.to = Some(to.into());
            self
        }
        pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
            self.properties.subject = Some(subject.into());
            self
        }
        pub fn with_reply_to(mut self, reply_to: impl Into<String>) -> Self {
            self.properties.reply_to = Some(reply_to.into());
            self
        }
        pub fn with_correlation_id(mut self, correlation_id: impl Into<AmqpMessageId>) -> Self {
            self.properties.correlation_id = Some(correlation_id.into());
            self
        }
        pub fn with_content_type(mut self, content_type: impl Into<AmqpSymbol>) -> Self {
            self.properties.content_type = Some(content_type.into());
            self
        }
        pub fn with_content_encoding(mut self, content_encoding: AmqpSymbol) -> Self {
            self.properties.content_encoding = Some(content_encoding);
            self
        }
        pub fn with_absolute_expiry_time(
            mut self,
            absolute_expiry_time: impl Into<AmqpTimestamp>,
        ) -> Self {
            self.properties.absolute_expiry_time = Some(absolute_expiry_time.into());
            self
        }
        pub fn with_creation_time(mut self, creation_time: impl Into<AmqpTimestamp>) -> Self {
            self.properties.creation_time = Some(creation_time.into());
            self
        }
        pub fn with_group_id(mut self, group_id: impl Into<String>) -> Self {
            self.properties.group_id = Some(group_id.into());
            self
        }
        pub fn with_group_sequence(mut self, group_sequence: u32) -> Self {
            self.properties.group_sequence = Some(group_sequence);
            self
        }
        pub fn with_reply_to_group_id(mut self, reply_to_group_id: impl Into<String>) -> Self {
            self.properties.reply_to_group_id = Some(reply_to_group_id.into());
            self
        }
    }

    pub struct AmqpMessageBuilder {
        message: AmqpMessage,
    }

    impl AmqpMessageBuilder {
        pub fn build(self) -> AmqpMessage {
            self.message
        }
        pub(crate) fn new() -> AmqpMessageBuilder {
            AmqpMessageBuilder {
                message: AmqpMessage {
                    body: AmqpMessageBody::Empty,
                    header: None,
                    application_properties: None,
                    message_annotations: None,
                    delivery_annotations: None,
                    properties: None,
                    footer: None,
                },
            }
        }
        pub fn with_body(mut self, body: impl Into<AmqpMessageBody>) -> Self {
            self.message.body = body.into();
            self
        }
        pub fn with_header(mut self, header: AmqpMessageHeader) -> Self {
            self.message.header = Some(header);
            self
        }
        pub fn with_application_properties(
            mut self,
            application_properties: AmqpApplicationProperties,
        ) -> Self {
            self.message.application_properties = Some(application_properties);
            self
        }
        pub fn add_application_property(mut self, key: String, value: AmqpValue) -> Self {
            if let Some(application_properties) = &mut self.message.application_properties {
                application_properties.0.insert(key, value);
            } else {
                let mut application_properties = AmqpOrderedMap::new();
                application_properties.insert(key, value);
                self.message.application_properties =
                    Some(AmqpApplicationProperties(application_properties));
            }
            self
        }
        pub fn with_message_annotations(mut self, message_annotations: AmqpAnnotations) -> Self {
            self.message.message_annotations = Some(message_annotations);
            self
        }
        pub fn with_delivery_annotations(mut self, delivery_annotations: AmqpAnnotations) -> Self {
            self.message.delivery_annotations = Some(delivery_annotations);
            self
        }
        pub fn with_properties(mut self, properties: AmqpMessageProperties) -> Self {
            self.message.properties = Some(properties);
            self
        }
        pub fn with_footer(mut self, footer: AmqpAnnotations) -> Self {
            self.message.footer = Some(footer);
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_amqp_message_header_builder() {
        let header = AmqpMessageHeader::builder()
            .with_durable(true)
            .with_priority(5)
            .with_time_to_live(std::time::Duration::from_millis(1000))
            .with_first_acquirer(false)
            .with_delivery_count(3)
            .build();

        assert_eq!(header.durable, Some(true));
        assert_eq!(header.priority, Some(5));
        assert_eq!(
            header.time_to_live,
            Some(std::time::Duration::from_millis(1000))
        );
        assert_eq!(header.first_acquirer, Some(false));
        assert_eq!(header.delivery_count, Some(3));
    }

    #[test]
    fn test_amqp_message_properties_builder() {
        let time_now = SystemTime::now();
        let test_uuid1 = Uuid::new_v4();
        let test_uuid2 = Uuid::new_v4();
        let properties = AmqpMessageProperties::builder()
            .with_message_id(test_uuid1)
            .with_user_id(vec![1, 2, 3])
            .with_to("destination".to_string())
            .with_subject("subject")
            .with_reply_to("reply_to".to_string())
            .with_correlation_id(test_uuid2)
            .with_content_type("content_type")
            .with_content_encoding(AmqpSymbol::from("content_encoding"))
            .with_absolute_expiry_time(time_now)
            .with_creation_time(time_now)
            .with_group_id("group_id")
            .with_group_sequence(1)
            .with_reply_to_group_id("reply_to_group_id")
            .build();

        assert_eq!(properties.message_id, Some(test_uuid1.into()));
        assert_eq!(properties.user_id, Some(vec![1, 2, 3]));
        assert_eq!(properties.to, Some("destination".to_string()));
        assert_eq!(properties.subject, Some("subject".to_string()));
        assert_eq!(properties.reply_to, Some("reply_to".to_string()));
        assert_eq!(properties.correlation_id, Some(test_uuid2.into()));
        assert_eq!(
            properties.content_type,
            Some(AmqpSymbol::from("content_type"))
        );
        assert_eq!(
            properties.content_encoding,
            Some(AmqpSymbol::from("content_encoding"))
        );
        assert_eq!(properties.absolute_expiry_time, Some(time_now.into()));
        assert_eq!(properties.creation_time, Some(time_now.into()));
        assert_eq!(properties.group_id, Some("group_id".to_string()));
        assert_eq!(properties.group_sequence, Some(1));
        assert_eq!(
            properties.reply_to_group_id,
            Some("reply_to_group_id".to_string())
        );
    }

    #[test]
    fn test_amqp_message_builder() {
        let message = AmqpMessage::builder()
            .with_body(AmqpMessageBody::Binary(vec![vec![1, 2, 3]]))
            .with_header(AmqpMessageHeader::builder().build())
            .with_application_properties(AmqpApplicationProperties::new())
            .add_application_property("key".to_string(), AmqpValue::from(123))
            .with_message_annotations(AmqpAnnotations::new())
            .with_delivery_annotations(AmqpAnnotations::new())
            .with_properties(AmqpMessageProperties::builder().build())
            .with_footer(AmqpAnnotations::new())
            .build();

        assert_eq!(message.body, AmqpMessageBody::Binary(vec![vec![1, 2, 3]]));
        assert_eq!(message.header, Some(AmqpMessageHeader::builder().build()));
        let mut properties = AmqpApplicationProperties::new();
        properties.insert("key", AmqpValue::from(123));
        assert_eq!(message.application_properties, Some(properties));
        assert_eq!(message.message_annotations, Some(AmqpAnnotations::new()));
        assert_eq!(message.delivery_annotations, Some(AmqpAnnotations::new()));
        assert_eq!(
            message.properties,
            Some(AmqpMessageProperties::builder().build())
        );
        assert_eq!(message.footer, Some(AmqpAnnotations::new()));
    }

    #[test]
    fn test_amqp_source_builder() {
        let source = AmqpSource::builder()
            .with_address("address")
            .with_durable(TerminusDurability::Configuration)
            .with_expiry_policy(TerminusExpiryPolicy::ConnectionClose)
            .with_timeout(10)
            .with_dynamic(true)
            .with_dynamic_node_properties(AmqpOrderedMap::new())
            .with_distribution_mode(DistributionMode::Copy)
            .with_filter(AmqpOrderedMap::new())
            .with_default_outcome(AmqpOutcome::Accepted)
            .with_outcomes(vec![AmqpSymbol::from("outcome")])
            .build();

        assert_eq!(source.address, Some("address".to_string()));
        assert_eq!(source.durable, Some(TerminusDurability::Configuration));
        assert_eq!(
            source.expiry_policy,
            Some(TerminusExpiryPolicy::ConnectionClose)
        );
        assert_eq!(source.timeout, Some(10));
        assert_eq!(source.dynamic, Some(true));
        assert_eq!(source.dynamic_node_properties, Some(AmqpOrderedMap::new()));
        assert_eq!(source.distribution_mode, Some(DistributionMode::Copy));
        assert_eq!(source.filter, Some(AmqpOrderedMap::new()));
        assert_eq!(source.default_outcome, Some(AmqpOutcome::Accepted));
        assert_eq!(source.outcomes, Some(vec![AmqpSymbol::from("outcome")]));
    }

    #[test]
    fn test_amqp_target_builder() {
        let target = AmqpTarget::builder()
            .with_address("address")
            .with_durable(TerminusDurability::Configuration)
            .with_expiry_policy(TerminusExpiryPolicy::ConnectionClose)
            .with_timeout(10)
            .with_dynamic(true)
            .with_dynamic_node_properties(AmqpOrderedMap::new())
            .with_capabilities(vec![AmqpValue::from("capability")])
            .build();

        assert_eq!(target.address, Some("address".to_string()));
        assert_eq!(target.durable, Some(TerminusDurability::Configuration));
        assert_eq!(
            target.expiry_policy,
            Some(TerminusExpiryPolicy::ConnectionClose)
        );
        assert_eq!(target.timeout, Some(10));
        assert_eq!(target.dynamic, Some(true));
        assert_eq!(target.dynamic_node_properties, Some(AmqpOrderedMap::new()));
        assert_eq!(
            target.capabilities,
            Some(vec![AmqpValue::from("capability")])
        );
    }

    #[test]
    fn test_empty_message_serialization() {
        {
            let message = AmqpMessage::builder().build();
            let serialized = AmqpMessage::serialize(message).unwrap();
            assert_eq!(serialized, vec![0, 0x53, 0x77, 0x40]);
            #[cfg(any(feature = "enable-fe2o3-amqp"))]
            {
                // Verify that the serialization of an AmqpMessage with no body matches
                // the fe2o3-amqp serialization.
                let body_type: fe2o3_amqp_types::messaging::Body<()> =
                    fe2o3_amqp_types::messaging::Body::Empty;
                let amqp_message = fe2o3_amqp_types::messaging::message::Builder::new()
                    .body(body_type)
                    .build();
                let serialized_fe2o3 = serde_amqp::ser::to_vec(
                    &fe2o3_amqp_types::messaging::message::__private::Serializable(amqp_message),
                )
                .unwrap();
                assert_eq!(serialized, serialized_fe2o3);
            }
        }
        {
            let message = AmqpMessage::builder()
                .with_header(
                    AmqpMessageHeader::builder()
                        .with_time_to_live(std::time::Duration::from_millis(23))
                        .build(),
                )
                .build();
            let serialized = AmqpMessage::serialize(message).unwrap();

            // The serialized body should contain:
            // - 0x0 DESCRIPTOR
            // - 0x53 // smallulong
            // - 0x70: MESSAGE HEADER
            // - 0xc0: 0 (COMPOUND LIST)
            // - 0x5: 5 (WIDTH 5 bytes)
            // - 0x3: 3 (LENGTH 3 items)
            // - 0x40: (ITEM 0 - Durable - EMPTY)
            // - 0x40: (ITEM 1 - Priority - EMPTY)
            // - 0x52: (ITEM 2 - Time to Live - SMALLUINT)
            // - 23: 23 Time To Live == 23.
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x77: AMQP VALUE BODY
            // - 0x40: EMPTY

            assert_eq!(
                serialized,
                vec![0, 0x53, 0x70, 0xc0, 0x5, 0x3, 0x40, 0x40, 0x52, 23, 0x00, 0x53, 0x77, 0x40]
            );
        }
    }

    #[test]
    fn test_value_message_serialization() {
        {
            let message = AmqpMessage::builder()
                .with_body(AmqpValue::from(123))
                .build();
            let serialized = AmqpMessage::serialize(message).unwrap();
            // The serialized body should contain:
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x77: AMQP VALUE BODY
            // - 0x54: AMQP INT
            // - 0x7b: 0x7b: 123
            assert_eq!(serialized, vec![0, 0x53, 0x77, 0x54, 123]);
        }
        {
            let message = AmqpMessage::builder()
                .with_body(AmqpValue::from("hello"))
                .build();
            let serialized = AmqpMessage::serialize(message).unwrap();
            // The serialized body should contain:
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x77: AMQP VALUE BODY
            // - 0xa1: AMQP STRING
            // - 0x5:  LENGTH
            // - 0x68, 0x65, 0x6c, 0x6c, 0x6f: hello
            assert_eq!(
                serialized,
                vec![0, 0x53, 0x77, 0xa1, 0x5, 0x68, 0x65, 0x6c, 0x6c, 0x6f]
            );
        }
    }

    #[test]
    fn test_binary_message_serialization() {
        {
            let message = AmqpMessage::builder()
                .with_body(AmqpMessageBody::Binary(vec![vec![1, 2, 3]]))
                .build();
            let serialized = AmqpMessage::serialize(message).unwrap();
            // The serialized body should contain:
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x75: AMQP DATA
            // - 0x03: LENGTH OF DATA
            // - 0x01, 0x02, 0x03: DATA
            assert_eq!(serialized, vec![0x00, 0x53, 0x75, 160, 3, 1, 2, 3]);
        }
        {
            let message = AmqpMessage::builder()
                .with_body(AmqpMessageBody::Binary(vec![vec![1, 2, 3], vec![4, 5, 6]]))
                .build();
            let serialized = AmqpMessage::serialize(message).unwrap();
            // The serialized body should contain:
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x75: AMQP DATA
            // - 0x03: LENGTH OF DATA
            // - 0x01, 0x02, 0x03: DATA
            assert_eq!(
                serialized,
                vec![0x00, 0x53, 0x75, 0xA0, 3, 1, 2, 3, 0, 0x53, 0x75, 0xA0, 3, 4, 5, 6]
            );
        }

        {
            let message = AmqpMessage::builder()
                .with_body(vec![vec![1, 2, 3]])
                .build();
            let serialized = AmqpMessage::serialize(message).unwrap();
            // The serialized body should contain:
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x75: AMQP DATA
            // - 0x03: LENGTH OF DATA
            // - 0x01, 0x02, 0x03: DATA
            assert_eq!(serialized, vec![0x00, 0x53, 0x75, 160, 3, 1, 2, 3]);
        }

        {
            let message = AmqpMessage::builder().with_body(vec![1, 2, 3]).build();
            let serialized = AmqpMessage::serialize(message).unwrap();
            // The serialized body should contain:
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x75: AMQP DATA
            // - 0x03: LENGTH OF DATA
            // - 0x01, 0x02, 0x03: DATA
            assert_eq!(serialized, vec![0x00, 0x53, 0x75, 160, 3, 1, 2, 3]);
        }
    }

    #[test]
    fn test_sequence_message_serialization() {
        {
            let message = AmqpMessage::builder()
                .with_body(AmqpMessageBody::Sequence(vec![AmqpList::new()]))
                .build();

            let serialized = AmqpMessage::serialize(message).unwrap();

            // The serialized body should contain:
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x76: AMQP SEQUENCE
            // - 0x45: LIST 0
            assert_eq!(serialized, vec![0, 0x53, 0x76, 0x45]);
        }
        {
            let mut body = AmqpList::new();
            body.push(AmqpValue::from(123));
            body.push(AmqpValue::from("hello"));

            let message = AmqpMessage::builder()
                .with_body(AmqpMessageBody::Sequence(vec![body]))
                .build();

            let serialized = AmqpMessage::serialize(message).unwrap();

            // The serialized body should contain:
            // - 0x00: 0 DESCRIPTOR
            // - 0x53: SMALLULONG
            // - 0x76: AMQP SEQUENCE
            // - 0xC0 : Compound LIST8
            // - 0x0a - width of list
            // - 0x02 - length of list
            // - 0x54: AMQP INT
            // - 0x7b: 123
            // - 0xa1: AMQP STRING
            // - 0x5:  LENGTH
            // - 0x68, 0x65, 0x6c, 0x6c, 0x6f: hello
            assert_eq!(
                serialized,
                vec![
                    0, 0x53, 0x76, 0xC0, 10, 0x02, 0x54, 0x7B, 0xA1, 0x5, 0x68, 0x65, 0x6C, 0x6C,
                    0x6F
                ]
            );
        }
    }
}
