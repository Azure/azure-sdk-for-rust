//cspell: words amqp

use super::value::{AmqpSymbol, AmqpTimestamp};
use crate::amqp_client::{
    fe2o3::error::AmqpSerializationError,
    value::{AmqpList, AmqpOrderedMap, AmqpValue},
};
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
    time_to_live: Option<u32>,
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

    pub fn time_to_live(&self) -> Option<&u32> {
        self.time_to_live.as_ref()
    }

    pub fn first_acquirer(&self) -> Option<&bool> {
        self.first_acquirer.as_ref()
    }

    pub fn delivery_count(&self) -> Option<&u32> {
        self.delivery_count.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct AmqpAnnotations(pub AmqpOrderedMap<AmqpAnnotationKey, AmqpValue>);

#[derive(Debug, Clone, PartialEq)]
pub struct AmqpApplicationProperties(pub AmqpOrderedMap<String, AmqpValue>);

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
            .map_err(AmqpSerializationError::from)?;
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
        pub fn with_time_to_live(mut self, time_to_live: u32) -> Self {
            self.header.time_to_live = Some(time_to_live);
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
        pub fn with_message_id(mut self, message_id: AmqpMessageId) -> Self {
            self.properties.message_id = Some(message_id);
            self
        }
        pub fn with_user_id(mut self, user_id: Vec<u8>) -> Self {
            self.properties.user_id = Some(user_id);
            self
        }
        pub fn with_to(mut self, to: String) -> Self {
            self.properties.to = Some(to);
            self
        }
        pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
            self.properties.subject = Some(subject.into());
            self
        }
        pub fn with_reply_to(mut self, reply_to: String) -> Self {
            self.properties.reply_to = Some(reply_to);
            self
        }
        pub fn with_correlation_id(mut self, correlation_id: AmqpMessageId) -> Self {
            self.properties.correlation_id = Some(correlation_id);
            self
        }
        pub fn with_content_type(mut self, content_type: AmqpSymbol) -> Self {
            self.properties.content_type = Some(content_type);
            self
        }
        pub fn with_content_encoding(mut self, content_encoding: AmqpSymbol) -> Self {
            self.properties.content_encoding = Some(content_encoding);
            self
        }
        pub fn with_absolute_expiry_time(mut self, absolute_expiry_time: AmqpTimestamp) -> Self {
            self.properties.absolute_expiry_time = Some(absolute_expiry_time);
            self
        }
        pub fn with_creation_time(mut self, creation_time: AmqpTimestamp) -> Self {
            self.properties.creation_time = Some(creation_time);
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
    use super::*;

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
}
