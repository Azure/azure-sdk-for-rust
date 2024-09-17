// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
//cspell: words amqp SMALLUINT SMALLULONG

use super::value::{AmqpList, AmqpOrderedMap, AmqpSymbol, AmqpTimestamp, AmqpValue};
#[cfg(feature = "cplusplus")]
use crate::Deserializable;
#[cfg(feature = "cplusplus")]
use azure_core::error::ErrorKind;
use azure_core::Result;

use crate::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum TerminusDurability {
    None,
    Configuration,
    UnsettledState,
}

impl From<TerminusDurability> for AmqpSymbol {
    fn from(durability: TerminusDurability) -> Self {
        match durability {
            TerminusDurability::None => AmqpSymbol::from("none"),
            TerminusDurability::Configuration => AmqpSymbol::from("configuration"),
            TerminusDurability::UnsettledState => AmqpSymbol::from("unsettled-state"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TerminusExpiryPolicy {
    LinkDetach,
    SessionEnd,
    ConnectionClose,
    Never,
}

impl From<TerminusExpiryPolicy> for AmqpSymbol {
    fn from(policy: TerminusExpiryPolicy) -> Self {
        match policy {
            TerminusExpiryPolicy::LinkDetach => AmqpSymbol::from("link-detach"),
            TerminusExpiryPolicy::SessionEnd => AmqpSymbol::from("session-end"),
            TerminusExpiryPolicy::ConnectionClose => AmqpSymbol::from("connection-close"),
            TerminusExpiryPolicy::Never => AmqpSymbol::from("never"),
        }
    }
}

impl From<AmqpSymbol> for TerminusExpiryPolicy {
    fn from(symbol: AmqpSymbol) -> Self {
        match symbol.0.as_str() {
            "link-detach" => TerminusExpiryPolicy::LinkDetach,
            "session-end" => TerminusExpiryPolicy::SessionEnd,
            "connection-close" => TerminusExpiryPolicy::ConnectionClose,
            "never" => TerminusExpiryPolicy::Never,
            _ => panic!("Invalid symbol for TerminusExpiryPolicy"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DistributionMode {
    Move,
    Copy,
}

impl From<DistributionMode> for AmqpSymbol {
    fn from(mode: DistributionMode) -> Self {
        match mode {
            DistributionMode::Move => AmqpSymbol::from("move"),
            DistributionMode::Copy => AmqpSymbol::from("copy"),
        }
    }
}

impl From<AmqpSymbol> for DistributionMode {
    fn from(symbol: AmqpSymbol) -> Self {
        match symbol.0.as_str() {
            "move" => DistributionMode::Move,
            "copy" => DistributionMode::Copy,
            _ => panic!("Invalid symbol for DistributionMode"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmqpOutcome {
    Accepted,
    Rejected,
    Released,
    Modified,
}

impl From<AmqpOutcome> for AmqpSymbol {
    fn from(outcome: AmqpOutcome) -> Self {
        match outcome {
            AmqpOutcome::Accepted => AmqpSymbol::from("amqp:accepted:list"),
            AmqpOutcome::Rejected => AmqpSymbol::from("amqp:rejected:list"),
            AmqpOutcome::Released => AmqpSymbol::from("amqp:released:list"),
            AmqpOutcome::Modified => AmqpSymbol::from("amqp:modified:list"),
        }
    }
}

impl From<AmqpSymbol> for AmqpOutcome {
    fn from(symbol: AmqpSymbol) -> Self {
        match symbol.0.as_str() {
            "amqp:accepted:list" => AmqpOutcome::Accepted,
            "amqp:rejected:list" => AmqpOutcome::Rejected,
            "amqp:released:list" => AmqpOutcome::Released,
            "amqp:modified:list" => AmqpOutcome::Modified,
            _ => panic!("Invalid symbol for AmqpOutcome"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmqpMessageId {
    String(String),
    Uuid(Uuid),
    Binary(Vec<u8>),
    Ulong(u64),
}

impl From<crate::Uuid> for AmqpMessageId {
    fn from(uuid: Uuid) -> Self {
        AmqpMessageId::Uuid(uuid)
    }
}

impl From<String> for AmqpMessageId {
    fn from(string: String) -> Self {
        AmqpMessageId::String(string)
    }
}

impl From<&str> for AmqpMessageId {
    fn from(string: &str) -> Self {
        AmqpMessageId::String(string.to_string())
    }
}

impl From<Vec<u8>> for AmqpMessageId {
    fn from(binary: Vec<u8>) -> Self {
        AmqpMessageId::Binary(binary)
    }
}

impl From<u64> for AmqpMessageId {
    fn from(ulong: u64) -> Self {
        AmqpMessageId::Ulong(ulong)
    }
}

impl From<AmqpMessageId> for AmqpValue {
    fn from(message_id: AmqpMessageId) -> Self {
        match message_id {
            AmqpMessageId::String(string) => AmqpValue::String(string),
            AmqpMessageId::Uuid(uuid) => AmqpValue::Uuid(uuid),
            AmqpMessageId::Binary(binary) => AmqpValue::Binary(binary),
            AmqpMessageId::Ulong(ulong) => AmqpValue::ULong(ulong),
        }
    }
}

/// A target node in an AMQP message
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AmqpTarget {
    address: Option<String>,
    durable: Option<TerminusDurability>,
    expiry_policy: Option<TerminusExpiryPolicy>,
    timeout: Option<u32>,
    dynamic: Option<bool>,
    dynamic_node_properties: Option<AmqpOrderedMap<String, AmqpValue>>,
    capabilities: Option<Vec<AmqpValue>>,
}

impl AmqpTarget {
    pub fn builder() -> builders::AmqpTargetBuilder {
        builders::AmqpTargetBuilder::new()
    }

    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    pub fn durable(&self) -> Option<&TerminusDurability> {
        self.durable.as_ref()
    }

    pub fn expiry_policy(&self) -> Option<&TerminusExpiryPolicy> {
        self.expiry_policy.as_ref()
    }

    pub fn timeout(&self) -> Option<&u32> {
        self.timeout.as_ref()
    }

    pub fn dynamic(&self) -> Option<&bool> {
        self.dynamic.as_ref()
    }

    pub fn dynamic_node_properties(&self) -> Option<&AmqpOrderedMap<String, AmqpValue>> {
        self.dynamic_node_properties.as_ref()
    }

    pub fn capabilities(&self) -> Option<&Vec<AmqpValue>> {
        self.capabilities.as_ref()
    }
}

impl From<AmqpTarget> for String {
    fn from(target: AmqpTarget) -> String {
        target.address.unwrap()
    }
}

impl From<String> for AmqpTarget {
    fn from(address: String) -> AmqpTarget {
        AmqpTarget {
            address: Some(address),
            durable: None,
            expiry_policy: None,
            timeout: None,
            dynamic: None,
            dynamic_node_properties: None,
            capabilities: None,
        }
    }
}

#[cfg(feature = "cplusplus")]
impl From<AmqpList> for AmqpTarget {
    fn from(list: AmqpList) -> Self {
        let mut builder = AmqpTarget::builder();
        let field_count = list.len();
        if field_count >= 1 {
            if let Some(AmqpValue::String(address)) = list.0.first() {
                builder.with_address(address.clone());
            }
        }
        if field_count >= 2 {
            if let Some(AmqpValue::UByte(durable)) = list.0.get(1) {
                match *durable {
                    0 => {
                        builder.with_durable(TerminusDurability::None);
                    }
                    1 => {
                        builder.with_durable(TerminusDurability::Configuration);
                    }
                    2 => {
                        builder.with_durable(TerminusDurability::UnsettledState);
                    }
                    _ => {
                        panic!("Invalid durable value");
                    }
                }
            }
        }
        if field_count >= 3 {
            if let Some(AmqpValue::Symbol(expiry_policy)) = list.0.get(2) {
                builder.with_expiry_policy(expiry_policy.clone().into());
            }
        }
        if field_count >= 4 {
            if let Some(AmqpValue::UInt(timeout)) = list.0.get(3) {
                builder.with_timeout(*timeout);
            }
        }
        if field_count >= 5 {
            if let Some(AmqpValue::Boolean(dynamic)) = list.0.get(4) {
                builder.with_dynamic(*dynamic);
            }
        }
        if field_count >= 6 {
            if let Some(AmqpValue::Map(dynamic_node_properties)) = list.0.get(5) {
                let dynamic_node_properties: AmqpOrderedMap<String, AmqpValue> =
                    dynamic_node_properties
                        .iter()
                        .map(|(k, v)| (k.clone().into(), v.clone()))
                        .collect();
                builder.with_dynamic_node_properties(dynamic_node_properties);
            }
        }
        if field_count >= 7 {
            if let Some(AmqpValue::Array(capabilities)) = list.0.get(6) {
                builder.with_capabilities(capabilities.to_vec());
            }
        }
        builder.build()
    }
}

#[cfg(feature = "cplusplus")]
impl From<AmqpTarget> for AmqpList {
    fn from(target: AmqpTarget) -> Self {
        let mut list = vec![AmqpValue::Null; 7];

        // Serialize the current value, if it exists. Otherwise serialize a null
        list[0] = target.address.map_or(AmqpValue::Null, AmqpValue::String);
        list[1] = target
            .durable
            .map_or(AmqpValue::Null, |v| AmqpValue::UByte(v as u8));
        list[2] = target
            .expiry_policy
            .map_or(AmqpValue::Null, |v| AmqpValue::Symbol(v.into()));
        list[3] = target.timeout.map_or(AmqpValue::Null, AmqpValue::UInt);
        list[4] = target.dynamic.map_or(AmqpValue::Null, AmqpValue::Boolean);
        list[5] = target.dynamic_node_properties.map_or(AmqpValue::Null, |v| {
            AmqpValue::Map(
                v.into_iter()
                    .map(|(k, v)| (AmqpValue::String(k), v))
                    .collect(),
            )
        });
        list[6] = target.capabilities.map_or(AmqpValue::Null, |v| {
            AmqpValue::Array(v.into_iter().collect())
        });

        let mut trailing_nulls = 0;
        for val in list.iter().rev() {
            if *val != AmqpValue::Null {
                break;
            }
            trailing_nulls += 1;
        }
        list.truncate(list.len() - trailing_nulls);

        AmqpList::from(list)
    }
}

#[derive(Debug, Default)]
pub struct AmqpSourceFilter {
    descriptor: &'static str,
    code: u64,
}

impl AmqpSourceFilter {
    fn new(descriptor: &'static str, code: u64) -> Self {
        Self { descriptor, code }
    }

    pub fn description(&self) -> &'static str {
        self.descriptor
    }
    pub fn code(&self) -> u64 {
        self.code
    }

    pub fn selector_filter() -> AmqpSourceFilter {
        AmqpSourceFilter::new("apache.org:selector-filter:string", 0x0000468_c00000004)
    }
}

/// A source node in an AMQP message
#[derive(Debug, Clone, Default, PartialEq)]
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

#[cfg(feature = "cplusplus")]
impl From<AmqpList> for AmqpSource {
    fn from(list: AmqpList) -> Self {
        let mut builder = AmqpSource::builder();
        let field_count = list.len();
        if field_count >= 1 {
            if let Some(AmqpValue::String(address)) = list.0.first() {
                builder.with_address(address.clone());
            }
        }
        if field_count >= 2 {
            if let Some(AmqpValue::UByte(durable)) = list.0.get(1) {
                match *durable {
                    0 => {
                        builder.with_durable(TerminusDurability::None);
                    }
                    1 => {
                        builder.with_durable(TerminusDurability::Configuration);
                    }
                    2 => {
                        builder.with_durable(TerminusDurability::UnsettledState);
                    }
                    _ => {
                        panic!("Invalid durable value");
                    }
                }
            }
        }
        if field_count >= 3 {
            if let Some(AmqpValue::Symbol(expiry_policy)) = list.0.get(2) {
                builder.with_expiry_policy(expiry_policy.clone().into());
            }
        }
        if field_count >= 4 {
            if let Some(AmqpValue::UInt(timeout)) = list.0.get(3) {
                builder.with_timeout(*timeout);
            }
        }
        if field_count >= 5 {
            if let Some(AmqpValue::Boolean(dynamic)) = list.0.get(4) {
                builder.with_dynamic(*dynamic);
            }
        }
        if field_count >= 6 {
            if let Some(AmqpValue::Map(dynamic_node_properties)) = list.0.get(5) {
                let dynamic_node_properties: AmqpOrderedMap<AmqpSymbol, AmqpValue> =
                    dynamic_node_properties
                        .iter()
                        .map(|(k, v)| (k.clone().into(), v.clone()))
                        .collect();
                builder.with_dynamic_node_properties(dynamic_node_properties);
            }
        }
        if field_count >= 7 {
            if let Some(AmqpValue::Symbol(distribution_mode)) = list.0.get(6) {
                builder.with_distribution_mode(distribution_mode.clone().into());
            }
        }
        if field_count >= 8 {
            if let Some(AmqpValue::Map(filter)) = list.0.get(7) {
                let filter: AmqpOrderedMap<AmqpSymbol, AmqpValue> = filter
                    .iter()
                    .map(|(k, v)| (k.clone().into(), v.clone()))
                    .collect();
                builder.with_filter(filter);
            }
        }
        if field_count >= 9 {
            if let Some(AmqpValue::Symbol(default_outcome)) = list.0.get(8) {
                builder.with_default_outcome(default_outcome.clone().into());
            }
        }
        if field_count >= 10 {
            if let Some(AmqpValue::Array(outcomes)) = list.0.get(9) {
                builder.with_outcomes(outcomes.iter().map(|v| v.clone().into()).collect());
            }
        }
        if field_count >= 11 {
            if let Some(AmqpValue::Array(capabilities)) = list.0.get(10) {
                builder.with_capabilities(capabilities.iter().map(|v| v.clone().into()).collect());
            }
        }
        builder.build()
    }
}

#[cfg(feature = "cplusplus")]
impl From<AmqpSource> for AmqpList {
    fn from(source: AmqpSource) -> Self {
        let mut list = vec![AmqpValue::Null; 11];

        // Serialize the current value, if it exists. Otherwise serialize a null
        list[0] = source.address.map_or(AmqpValue::Null, AmqpValue::String);
        list[1] = source
            .durable
            .map_or(AmqpValue::Null, |v| AmqpValue::UByte(v as u8));
        list[2] = source
            .expiry_policy
            .map_or(AmqpValue::Null, |v| AmqpValue::Symbol(v.into()));
        list[3] = source.timeout.map_or(AmqpValue::Null, AmqpValue::UInt);
        list[4] = source.dynamic.map_or(AmqpValue::Null, AmqpValue::Boolean);
        list[5] = source.dynamic_node_properties.map_or(AmqpValue::Null, |v| {
            AmqpValue::Map(
                v.into_iter()
                    .map(|(k, v)| (AmqpValue::Symbol(k), v))
                    .collect(),
            )
        });
        list[6] = source
            .distribution_mode
            .map_or(AmqpValue::Null, |v| AmqpValue::Symbol(v.into()));
        list[7] = source.filter.map_or(AmqpValue::Null, |v| {
            AmqpValue::Map(
                v.into_iter()
                    .map(|(k, v)| (AmqpValue::Symbol(k), v))
                    .collect(),
            )
        });
        list[8] = source
            .default_outcome
            .map_or(AmqpValue::Null, |v| AmqpValue::Symbol(v.into()));
        list[9] = source.outcomes.map_or(AmqpValue::Null, |v| {
            AmqpValue::Array(v.into_iter().map(AmqpValue::Symbol).collect())
        });
        list[10] = source.capabilities.map_or(AmqpValue::Null, |v| {
            AmqpValue::Array(v.into_iter().map(AmqpValue::Symbol).collect())
        });

        let mut trailing_nulls = 0;
        for val in list.iter().rev() {
            if *val != AmqpValue::Null {
                break;
            }
            trailing_nulls += 1;
        }
        list.truncate(list.len() - trailing_nulls);

        AmqpList::from(list)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmqpMessageHeader {
    durable: bool,
    priority: u8,
    time_to_live: Option<std::time::Duration>,
    first_acquirer: bool,
    delivery_count: u32,
}

impl Default for AmqpMessageHeader {
    fn default() -> Self {
        Self {
            durable: false,
            priority: 4,
            time_to_live: None,
            first_acquirer: false,
            delivery_count: 0,
        }
    }
}

impl AmqpMessageHeader {
    pub fn builder() -> builders::AmqpMessageHeaderBuilder {
        builders::AmqpMessageHeaderBuilder::new()
    }

    pub fn durable(&self) -> bool {
        self.durable
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn time_to_live(&self) -> Option<&std::time::Duration> {
        self.time_to_live.as_ref()
    }

    pub fn first_acquirer(&self) -> bool {
        self.first_acquirer
    }

    pub fn delivery_count(&self) -> u32 {
        self.delivery_count
    }
}

/// Extract an AmqpMessageHeader from an AmqpList.
///
/// This function will attempt to extract an AmqpMessageHeader from an AmqpList.
///
/// It is intended to be used when deserializing an AmqpMessageHeader from an AMQP composite type.
/// See also [Amqp Header](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-messaging-v1.0-os.html#type-header) for more information
///
///
#[cfg(feature = "cplusplus")]
impl From<AmqpList> for AmqpMessageHeader {
    fn from(list: AmqpList) -> Self {
        let mut builder = AmqpMessageHeader::builder();
        let field_count = list.len();
        if field_count >= 1 {
            if let Some(AmqpValue::Boolean(durable)) = list.0.first() {
                builder.with_durable(*durable);
            }
        }
        if field_count >= 2 {
            if let Some(AmqpValue::UByte(priority)) = list.0.get(1) {
                builder.with_priority(*priority);
            }
        }
        if field_count >= 3 {
            if let Some(AmqpValue::UInt(time_to_live)) = list.0.get(2) {
                builder.with_time_to_live(Some(std::time::Duration::from_millis(
                    *time_to_live as u64,
                )));
            }
        }
        if field_count >= 4 {
            if let Some(AmqpValue::Boolean(first_acquirer)) = list.0.get(3) {
                builder.with_first_acquirer(*first_acquirer);
            }
        }
        if field_count >= 5 {
            if let Some(AmqpValue::UInt(delivery_count)) = list.0.get(4) {
                builder.with_delivery_count(*delivery_count);
            }
        }
        builder.build()
    }
}

#[cfg(feature = "cplusplus")]
impl From<AmqpMessageHeader> for AmqpList {
    fn from(header: AmqpMessageHeader) -> AmqpList {
        let mut list = vec![AmqpValue::Null; 5];

        // Serialize the current value, if it exists. Otherwise serialize a null
        // value if there are other values to serialize.

        if header.durable {
            list[0] = AmqpValue::Boolean(header.durable())
        };
        if header.priority != 4 {
            list[1] = AmqpValue::UByte(header.priority)
        };
        list[2] = header.time_to_live.map_or(AmqpValue::Null, |ttl| {
            AmqpValue::UInt(ttl.as_millis() as u32)
        });
        if header.first_acquirer {
            list[3] = AmqpValue::Boolean(header.first_acquirer)
        };
        if header.delivery_count != 0 {
            list[4] = AmqpValue::UInt(header.delivery_count)
        };

        let mut trailing_nulls = 0;
        for val in list.iter().rev() {
            if *val != AmqpValue::Null {
                break;
            }
            trailing_nulls += 1;
        }
        list.truncate(list.len() - trailing_nulls);
        AmqpList::from(list)
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

/// Extract an AmqpMessageProperties from an AmqpList.
///
/// This function will attempt to extract an AmqpMessageProperties from an AmqpList.
///
/// It is intended to be used when deserializing an AmqpMessageProperties from an AMQP composite type.
/// See also [Amqp Header](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-messaging-v1.0-os.html#type-properties) for more information
///
///
#[cfg(feature = "cplusplus")]
impl From<AmqpList> for AmqpMessageProperties {
    fn from(list: AmqpList) -> Self {
        let mut builder = AmqpMessageProperties::builder();
        let field_count = list.len();
        if field_count >= 1 {
            match &list.0[0] {
                AmqpValue::ULong(message_id) => {
                    builder.with_message_id(AmqpMessageId::Ulong(*message_id));
                }
                AmqpValue::Uuid(message_id) => {
                    builder.with_message_id(AmqpMessageId::Uuid(*message_id));
                }
                AmqpValue::Binary(message_id) => {
                    builder.with_message_id(AmqpMessageId::Binary(message_id.clone()));
                }
                AmqpValue::String(message_id) => {
                    builder.with_message_id(AmqpMessageId::String(message_id.clone()));
                }
                _ => {}
            }
        }
        if field_count >= 2 {
            if let AmqpValue::Binary(user_id) = &list.0[1] {
                builder.with_user_id(user_id.clone());
            }
        }
        if field_count >= 3 {
            if let AmqpValue::String(to) = &list.0[2] {
                builder.with_to(to.clone());
            }
        }
        if field_count >= 4 {
            if let AmqpValue::String(subject) = &list.0[3] {
                builder.with_subject(subject.clone());
            }
        }
        if field_count >= 5 {
            if let AmqpValue::String(reply_to) = &list.0[4] {
                builder.with_reply_to(reply_to.clone());
            }
        }
        if field_count >= 6 {
            match &list.0[5] {
                AmqpValue::ULong(correlation_id) => {
                    builder.with_correlation_id(AmqpMessageId::Ulong(*correlation_id));
                }
                AmqpValue::Uuid(correlation_id) => {
                    builder.with_correlation_id(AmqpMessageId::Uuid(*correlation_id));
                }
                AmqpValue::Binary(correlation_id) => {
                    builder.with_correlation_id(AmqpMessageId::Binary(correlation_id.clone()));
                }
                AmqpValue::String(correlation_id) => {
                    builder.with_correlation_id(AmqpMessageId::String(correlation_id.clone()));
                }
                _ => {}
            }
        }
        if field_count >= 7 {
            if let AmqpValue::Symbol(content_type) = &list.0[6] {
                builder.with_content_type(content_type.clone());
            }
        }
        if field_count >= 8 {
            if let AmqpValue::Symbol(content_encoding) = &list.0[7] {
                builder.with_content_encoding(content_encoding.clone());
            }
        }
        if field_count >= 9 {
            if let AmqpValue::TimeStamp(absolute_expiry_time) = &list.0[8] {
                builder.with_absolute_expiry_time(absolute_expiry_time.clone());
            }
        }
        if field_count >= 10 {
            if let AmqpValue::TimeStamp(creation_time) = &list.0[9] {
                builder.with_creation_time(creation_time.clone());
            }
        }
        if field_count >= 11 {
            if let AmqpValue::String(group_id) = &list.0[10] {
                builder.with_group_id(group_id.clone());
            }
        }
        if field_count >= 12 {
            if let AmqpValue::UInt(group_sequence) = &list.0[11] {
                builder.with_group_sequence(*group_sequence);
            }
        }
        if field_count >= 13 {
            if let AmqpValue::String(reply_to_group_id) = &list.0[12] {
                builder.with_reply_to_group_id(reply_to_group_id.clone());
            }
        }

        builder.build()
    }
}

#[cfg(feature = "cplusplus")]
impl From<AmqpMessageProperties> for AmqpList {
    fn from(properties: AmqpMessageProperties) -> AmqpList {
        let mut list = vec![AmqpValue::Null; 13];

        // Serialize the current value, if it exists. Otherwise serialize a null
        list[0] = properties
            .message_id
            .map_or(AmqpValue::Null, AmqpValue::from);
        list[1] = properties
            .user_id
            .map_or(AmqpValue::Null, AmqpValue::Binary);
        list[2] = properties.to.map_or(AmqpValue::Null, AmqpValue::String);
        list[3] = properties
            .subject
            .map_or(AmqpValue::Null, AmqpValue::String);
        list[4] = properties
            .reply_to
            .map_or(AmqpValue::Null, AmqpValue::String);
        list[5] = properties
            .correlation_id
            .map_or(AmqpValue::Null, AmqpValue::from);
        list[6] = properties
            .content_type
            .map_or(AmqpValue::Null, AmqpValue::Symbol);
        list[7] = properties
            .content_encoding
            .map_or(AmqpValue::Null, AmqpValue::Symbol);
        list[8] = properties
            .absolute_expiry_time
            .map_or(AmqpValue::Null, AmqpValue::TimeStamp);
        list[9] = properties
            .creation_time
            .map_or(AmqpValue::Null, AmqpValue::TimeStamp);
        list[10] = properties
            .group_id
            .map_or(AmqpValue::Null, AmqpValue::String);
        list[11] = properties
            .group_sequence
            .map_or(AmqpValue::Null, AmqpValue::UInt);
        list[12] = properties
            .reply_to_group_id
            .map_or(AmqpValue::Null, AmqpValue::String);

        // We will potentially have a set of trailing Null values in the list at this point,
        // we don't ever want the trailing null values to appear in the list so we remove them.

        // This behavior is described by the [AMQP spec, section 1.4](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-types-v1.0-os.html#section-composite-type-representation)
        // "When the trailing elements of the list representation are null, they MAY be omitted".
        let mut trailing_nulls = 0;
        for val in list.iter().rev() {
            if *val != AmqpValue::Null {
                break;
            }
            trailing_nulls += 1;
        }
        list.truncate(list.len() - trailing_nulls);

        AmqpList::from(list)
    }
}

#[test]
fn test_size_of_serialized_timestamp() {
    let timestamp = fe2o3_amqp_types::primitives::Timestamp::from_milliseconds(12345);
    let mut list = fe2o3_amqp_types::primitives::List::new();
    list.push(fe2o3_amqp_types::primitives::Value::Timestamp(timestamp));

    let described = serde_amqp::described::Described {
        descriptor: serde_amqp::descriptor::Descriptor::Code(0x73),
        value: fe2o3_amqp_types::primitives::Value::List(list),
    };

    let value = fe2o3_amqp_types::primitives::Value::Described(Box::new(described));

    let vec_result = serde_amqp::ser::to_vec(&value);
    assert!(vec_result.is_ok());

    let size_result = serde_amqp::size_ser::serialized_size(&value);
    assert!(size_result.is_ok());
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AmqpMessageBody {
    Binary(Vec<Vec<u8>>),
    Sequence(Vec<AmqpList>),
    Value(AmqpValue),
    #[default]
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

impl From<Vec<AmqpValue>> for AmqpMessageBody {
    fn from(values: Vec<AmqpValue>) -> Self {
        AmqpMessageBody::Sequence(vec![AmqpList::from(values)])
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

impl From<AmqpSymbol> for AmqpAnnotationKey {
    fn from(symbol: AmqpSymbol) -> Self {
        AmqpAnnotationKey::Symbol(symbol)
    }
}

impl From<u64> for AmqpAnnotationKey {
    fn from(ulong: u64) -> Self {
        AmqpAnnotationKey::Ulong(ulong)
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

impl<K, V> From<Vec<(K, V)>> for AmqpAnnotations
where
    K: Into<AmqpAnnotationKey>,
    V: Into<AmqpValue>,
{
    fn from(vec: Vec<(K, V)>) -> Self {
        let mut map = AmqpOrderedMap::new();
        for (k, v) in vec {
            map.insert(k, v);
        }
        AmqpAnnotations(map)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
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
/// <https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-messaging-v1.0-os.html>
///
#[derive(Debug, Clone, PartialEq, Default)]
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

    pub fn set_header(&mut self, header: AmqpMessageHeader) {
        self.header = Some(header);
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

    pub fn set_application_properties(
        &mut self,
        application_properties: impl Into<AmqpApplicationProperties>,
    ) {
        self.application_properties = Some(application_properties.into());
    }

    pub fn set_delivery_annotations(&mut self, delivery_annotations: impl Into<AmqpAnnotations>) {
        self.delivery_annotations = Some(delivery_annotations.into());
    }

    pub fn set_footer(&mut self, footer: impl Into<AmqpAnnotations>) {
        self.footer = Some(footer.into());
    }

    pub fn add_message_body_binary(&mut self, body: &[u8]) {
        match &mut self.body {
            AmqpMessageBody::Binary(bodies) => {
                bodies.push(body.to_owned());
            }
            AmqpMessageBody::Empty => {
                self.body = AmqpMessageBody::Binary(vec![body.to_owned()]);
            }
            _ => {
                panic!("Cannot add binary body to non-binary body");
            }
        }
    }

    pub fn add_message_body_sequence(&mut self, body: AmqpList) {
        match &mut self.body {
            AmqpMessageBody::Sequence(bodies) => {
                bodies.push(body);
            }
            AmqpMessageBody::Empty => {
                self.body = AmqpMessageBody::Sequence(vec![body]);
            }
            _ => {
                panic!("Cannot add sequence body to non-sequence body");
            }
        }
    }

    #[allow(unused_variables)]
    pub fn serialize(message: AmqpMessage) -> Result<Vec<u8>> {
        #[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
        {
            let amqp_message = Into::<
                fe2o3_amqp_types::messaging::Message<
                    fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
                >,
            >::into(message);
            let res = serde_amqp::ser::to_vec(
                &fe2o3_amqp_types::messaging::message::__private::Serializable(amqp_message),
            )
            .map_err(crate::fe2o3::error::AmqpSerialization::from)?;
            Ok(res)
        }
        #[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
        {
            unimplemented!();
        }
    }
}

impl From<Vec<u8>> for AmqpMessage {
    fn from(body: Vec<u8>) -> Self {
        AmqpMessage {
            body: AmqpMessageBody::Binary(vec![body]),
            header: None,
            application_properties: None,
            message_annotations: None,
            delivery_annotations: None,
            properties: None,
            footer: None,
        }
    }
}

impl From<AmqpValue> for AmqpMessage {
    fn from(value: AmqpValue) -> Self {
        AmqpMessage {
            body: AmqpMessageBody::Value(value),
            header: None,
            application_properties: None,
            message_annotations: None,
            delivery_annotations: None,
            properties: None,
            footer: None,
        }
    }
}

impl From<AmqpList> for AmqpMessage {
    fn from(list: AmqpList) -> Self {
        AmqpMessage {
            body: AmqpMessageBody::Sequence(vec![list]),
            header: None,
            application_properties: None,
            message_annotations: None,
            delivery_annotations: None,
            properties: None,
            footer: None,
        }
    }
}
#[cfg(feature = "cplusplus")]
impl Deserializable<AmqpMessage> for AmqpMessage {
    fn decode(data: &[u8]) -> azure_core::Result<AmqpMessage> {
        #[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
        {
            let value = serde_amqp::de::from_slice::<
                fe2o3_amqp_types::messaging::message::__private::Deserializable<
                    fe2o3_amqp_types::messaging::Message<
                        fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
                    >,
                >,
            >(data)
            .map_err(|e| azure_core::error::Error::new(ErrorKind::Other, e))?;
            Ok(value.0.into())
        }
    }
}

pub mod builders {
    use super::*;

    pub struct AmqpSourceBuilder {
        source: AmqpSource,
    }

    impl AmqpSourceBuilder {
        pub(super) fn new() -> AmqpSourceBuilder {
            AmqpSourceBuilder {
                source: Default::default(),
            }
        }
        pub fn with_address(&mut self, address: impl Into<String>) -> &mut Self {
            self.source.address = Some(address.into());
            self
        }
        pub fn with_durable(&mut self, durable: TerminusDurability) -> &mut Self {
            self.source.durable = Some(durable);
            self
        }
        pub fn with_expiry_policy(&mut self, expiry_policy: TerminusExpiryPolicy) -> &mut Self {
            self.source.expiry_policy = Some(expiry_policy);
            self
        }
        pub fn with_timeout(&mut self, timeout: u32) -> &mut Self {
            self.source.timeout = Some(timeout);
            self
        }
        pub fn with_dynamic(&mut self, dynamic: bool) -> &mut Self {
            self.source.dynamic = Some(dynamic);
            self
        }
        pub fn with_dynamic_node_properties(
            &mut self,
            dynamic_node_properties: impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
        ) -> &mut Self {
            self.source.dynamic_node_properties = Some(dynamic_node_properties.into());
            self
        }
        pub fn with_distribution_mode(&mut self, distribution_mode: DistributionMode) -> &mut Self {
            self.source.distribution_mode = Some(distribution_mode);
            self
        }
        pub fn with_filter(
            &mut self,
            filter: impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
        ) -> &mut Self {
            self.source.filter = Some(filter.into());
            self
        }
        pub fn add_to_filter(
            &mut self,
            key: impl Into<AmqpSymbol>,
            value: impl Into<AmqpValue>,
        ) -> &mut Self {
            if let Some(filter) = &mut self.source.filter {
                filter.insert(key.into(), value.into());
            } else {
                let mut filter = AmqpOrderedMap::new();
                filter.insert(key.into(), value.into());
                self.source.filter = Some(filter);
            }
            self
        }
        pub fn with_default_outcome(&mut self, default_outcome: AmqpOutcome) -> &mut Self {
            self.source.default_outcome = Some(default_outcome);
            self
        }
        pub fn with_outcomes(&mut self, outcomes: Vec<AmqpSymbol>) -> &mut Self {
            self.source.outcomes = Some(outcomes);
            self
        }
        pub fn with_capabilities(&mut self, capabilities: Vec<AmqpSymbol>) -> &mut Self {
            self.source.capabilities = Some(capabilities);
            self
        }
        pub fn build(&mut self) -> AmqpSource {
            self.source.clone()
        }
    }

    pub struct AmqpTargetBuilder {
        target: AmqpTarget,
    }

    impl AmqpTargetBuilder {
        pub fn build(&mut self) -> AmqpTarget {
            self.target.clone()
        }
        pub(super) fn new() -> AmqpTargetBuilder {
            AmqpTargetBuilder {
                target: Default::default(),
            }
        }
        pub fn with_address(&mut self, address: impl Into<String>) -> &mut Self {
            self.target.address = Some(address.into());
            self
        }
        pub fn with_durable(&mut self, durable: TerminusDurability) -> &mut Self {
            self.target.durable = Some(durable);
            self
        }
        pub fn with_expiry_policy(&mut self, expiry_policy: TerminusExpiryPolicy) -> &mut Self {
            self.target.expiry_policy = Some(expiry_policy);
            self
        }
        pub fn with_timeout(&mut self, timeout: u32) -> &mut Self {
            self.target.timeout = Some(timeout);
            self
        }
        pub fn with_dynamic(&mut self, dynamic: bool) -> &mut Self {
            self.target.dynamic = Some(dynamic);
            self
        }
        pub fn with_dynamic_node_properties(
            &mut self,
            dynamic_node_properties: impl Into<AmqpOrderedMap<String, AmqpValue>>,
        ) -> &mut Self {
            self.target.dynamic_node_properties = Some(dynamic_node_properties.into());
            self
        }
        pub fn with_capabilities(&mut self, capabilities: Vec<AmqpValue>) -> &mut Self {
            self.target.capabilities = Some(capabilities);
            self
        }
    }

    pub struct AmqpMessageHeaderBuilder {
        header: AmqpMessageHeader,
    }

    impl AmqpMessageHeaderBuilder {
        pub fn build(&self) -> AmqpMessageHeader {
            self.header.clone()
        }
        pub(super) fn new() -> AmqpMessageHeaderBuilder {
            AmqpMessageHeaderBuilder {
                header: Default::default(),
            }
        }
        pub fn with_durable(&mut self, durable: bool) -> &mut Self {
            self.header.durable = durable;
            self
        }
        pub fn with_priority(&mut self, priority: u8) -> &mut Self {
            self.header.priority = priority;
            self
        }
        pub fn with_time_to_live(
            &mut self,
            time_to_live: Option<std::time::Duration>,
        ) -> &mut Self {
            self.header.time_to_live = time_to_live;
            self
        }
        pub fn with_first_acquirer(&mut self, first_acquirer: bool) -> &mut Self {
            self.header.first_acquirer = first_acquirer;
            self
        }
        pub fn with_delivery_count(&mut self, delivery_count: u32) -> &mut Self {
            self.header.delivery_count = delivery_count;
            self
        }
    }

    pub struct AmqpMessagePropertiesBuilder {
        properties: AmqpMessageProperties,
    }

    impl AmqpMessagePropertiesBuilder {
        pub fn build(&mut self) -> AmqpMessageProperties {
            self.properties.clone()
        }
        pub(super) fn new() -> AmqpMessagePropertiesBuilder {
            AmqpMessagePropertiesBuilder {
                properties: Default::default(),
            }
        }
        pub fn with_message_id(&mut self, message_id: impl Into<AmqpMessageId>) -> &mut Self {
            self.properties.message_id = Some(message_id.into());
            self
        }
        pub fn with_user_id(&mut self, user_id: impl Into<Vec<u8>>) -> &mut Self {
            self.properties.user_id = Some(user_id.into());
            self
        }
        pub fn with_to(&mut self, to: impl Into<String>) -> &mut Self {
            self.properties.to = Some(to.into());
            self
        }
        pub fn with_subject(&mut self, subject: impl Into<String>) -> &mut Self {
            self.properties.subject = Some(subject.into());
            self
        }
        pub fn with_reply_to(&mut self, reply_to: impl Into<String>) -> &mut Self {
            self.properties.reply_to = Some(reply_to.into());
            self
        }
        pub fn with_correlation_id(
            &mut self,
            correlation_id: impl Into<AmqpMessageId>,
        ) -> &mut Self {
            self.properties.correlation_id = Some(correlation_id.into());
            self
        }
        pub fn with_content_type(&mut self, content_type: impl Into<AmqpSymbol>) -> &mut Self {
            self.properties.content_type = Some(content_type.into());
            self
        }
        pub fn with_content_encoding(
            &mut self,
            content_encoding: impl Into<AmqpSymbol>,
        ) -> &mut Self {
            self.properties.content_encoding = Some(content_encoding.into());
            self
        }
        pub fn with_absolute_expiry_time(
            &mut self,
            absolute_expiry_time: impl Into<AmqpTimestamp>,
        ) -> &mut Self {
            self.properties.absolute_expiry_time = Some(absolute_expiry_time.into());
            self
        }
        pub fn with_creation_time(&mut self, creation_time: impl Into<AmqpTimestamp>) -> &mut Self {
            self.properties.creation_time = Some(creation_time.into());
            self
        }
        pub fn with_group_id(&mut self, group_id: impl Into<String>) -> &mut Self {
            self.properties.group_id = Some(group_id.into());
            self
        }
        pub fn with_group_sequence(&mut self, group_sequence: u32) -> &mut Self {
            self.properties.group_sequence = Some(group_sequence);
            self
        }
        pub fn with_reply_to_group_id(
            &mut self,
            reply_to_group_id: impl Into<String>,
        ) -> &mut Self {
            self.properties.reply_to_group_id = Some(reply_to_group_id.into());
            self
        }
    }

    pub struct AmqpMessageBuilder {
        message: AmqpMessage,
    }

    impl AmqpMessageBuilder {
        pub fn build(&mut self) -> AmqpMessage {
            self.message.clone()
        }
        pub(super) fn new() -> AmqpMessageBuilder {
            AmqpMessageBuilder {
                message: Default::default(),
            }
        }
        pub fn with_body(&mut self, body: impl Into<AmqpMessageBody>) -> &mut Self {
            self.message.body = body.into();
            self
        }
        pub fn with_header(&mut self, header: AmqpMessageHeader) -> &mut Self {
            self.message.header = Some(header);
            self
        }
        pub fn with_application_properties(
            &mut self,
            application_properties: AmqpApplicationProperties,
        ) -> &mut Self {
            self.message.application_properties = Some(application_properties);
            self
        }
        pub fn add_application_property(
            &mut self,
            key: impl Into<String>,
            value: impl Into<AmqpValue>,
        ) -> &mut Self {
            if let Some(application_properties) = &mut self.message.application_properties {
                application_properties.0.insert(key.into(), value.into());
            } else {
                let mut application_properties = AmqpOrderedMap::new();
                application_properties.insert(key.into(), value.into());
                self.message.application_properties =
                    Some(AmqpApplicationProperties(application_properties));
            }
            self
        }
        pub fn with_message_annotations(
            &mut self,
            message_annotations: AmqpAnnotations,
        ) -> &mut Self {
            self.message.message_annotations = Some(message_annotations);
            self
        }
        pub fn with_delivery_annotations(
            &mut self,
            delivery_annotations: AmqpAnnotations,
        ) -> &mut Self {
            self.message.delivery_annotations = Some(delivery_annotations);
            self
        }
        pub fn with_properties<T>(&mut self, properties: T) -> &mut Self
        where
            T: Into<AmqpMessageProperties>,
        {
            self.message.properties = Some(properties.into());
            self
        }
        pub fn with_footer(&mut self, footer: AmqpAnnotations) -> &mut Self {
            self.message.footer = Some(footer);
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fe2o3_amqp_types::messaging::Priority;
    use std::time::SystemTime;

    #[test]
    fn test_amqp_message_header_builder() {
        let header = AmqpMessageHeader::builder()
            .with_durable(true)
            .with_priority(5)
            .with_time_to_live(Some(std::time::Duration::from_millis(1000)))
            .with_first_acquirer(false)
            .with_delivery_count(3)
            .build();

        assert_eq!(header.durable, true);
        assert_eq!(header.priority, 5);
        assert_eq!(
            header.time_to_live,
            Some(std::time::Duration::from_millis(1000))
        );
        assert_eq!(header.first_acquirer, false);
        assert_eq!(header.delivery_count, 3);
    }

    #[test]
    fn test_header_serialization() {
        {
            let c_serialized = vec![0x00, 0x53, 0x70, 0xc0, 0x04, 0x02, 0x40, 0x50, 0x05];
            let deserialized_from_c: fe2o3_amqp_types::messaging::Header =
                serde_amqp::de::from_slice(&c_serialized.as_slice()).unwrap();

            let header = fe2o3_amqp_types::messaging::Header::builder()
                .priority(Priority::from(5))
                .build();
            let serialized = serde_amqp::ser::to_vec(&header).unwrap();

            assert_eq!(c_serialized, serialized);
            let deserialized: fe2o3_amqp_types::messaging::Header =
                serde_amqp::de::from_slice(&serialized.as_slice()).unwrap();

            assert_eq!(c_serialized, serialized);
            assert_eq!(header, deserialized);
            assert_eq!(header, deserialized_from_c);
        }

        //        assert_eq!(header, deserialized);
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
    fn amqp_message_body_inferences() {
        {
            // Message body from Value.
            let message_body = AmqpMessageBody::from(AmqpValue::from(123));
            assert_eq!(message_body, AmqpMessageBody::Value(AmqpValue::from(123)));
        }

        {
            // Message body from binary array.
            let message_body = AmqpMessageBody::from(vec![1, 2, 3]);
            assert_eq!(message_body, AmqpMessageBody::Binary(vec![vec![1, 2, 3]]));

            let message_body = AmqpMessageBody::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
            assert_eq!(
                message_body,
                AmqpMessageBody::Binary(vec![vec![1, 2, 3], vec![4, 5, 6]])
            );
        }

        {
            // Message body from array of values.
            let message_body =
                AmqpMessageBody::from(vec![AmqpValue::from(123), AmqpValue::from("ABC")]);
            assert_eq!(
                message_body,
                AmqpMessageBody::Sequence(vec![AmqpList::from(vec![
                    AmqpValue::from(123),
                    AmqpValue::from("ABC")
                ])])
            );
        }
    }

    #[test]
    fn amqp_message_inferences() {
        let message = AmqpMessage::from(vec![1, 2, 3]);
        assert_eq!(
            message.body(),
            &AmqpMessageBody::Binary(vec![vec![1, 2, 3]])
        );

        let message = AmqpMessage::from(AmqpValue::from(123));
        assert_eq!(
            message.body(),
            &AmqpMessageBody::Value(AmqpValue::from(123))
        );

        let message = AmqpMessage::from(AmqpList::from(vec![
            AmqpValue::from(123),
            AmqpValue::from("ABC"),
        ]));
        assert_eq!(
            message.body(),
            &AmqpMessageBody::Sequence(vec![AmqpList::from(vec![
                AmqpValue::from(123),
                AmqpValue::from("ABC")
            ])])
        );
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
            #[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
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
                        .with_time_to_live(Some(std::time::Duration::from_millis(23)))
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

    #[test]
    fn test_message_with_header_serialization() {
        let message = AmqpMessage::builder()
            .with_header(AmqpMessageHeader::builder().with_priority(5).build())
            .with_body(AmqpValue::from("String Value Body."))
            .with_properties(
                AmqpMessageProperties::builder()
                    .with_message_id("12345")
                    .build(),
            )
            .build();

        let serialized = AmqpMessage::serialize(message.clone()).unwrap();
        assert_eq!(
            serialized,
            vec![
                0x00, 0x53, 0x70, 0xc0, 0x04, 0x02, 0x40, 0x50, 0x05, 0x00, 0x53, 0x73, 0xc0, 0x08,
                0x01, 0xa1, 0x05, 0x31, 0x32, 0x33, 0x34, 0x35, 0x00, 0x53, 0x77, 0xa1, 0x12, 0x53,
                0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x42, 0x6f,
                0x64, 0x79, 0x2e
            ]
        );

        #[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
        {
            let amqp_message = fe2o3_amqp_types::messaging::message::Builder::new()
                .header(
                    fe2o3_amqp_types::messaging::Header::builder()
                        .priority(5)
                        .build(),
                )
                .properties(
                    fe2o3_amqp_types::messaging::Properties::builder()
                        .message_id("12345".to_string())
                        .build(),
                )
                .body(fe2o3_amqp_types::messaging::Body::Value::<
                    fe2o3_amqp_types::primitives::Value,
                >(fe2o3_amqp_types::messaging::AmqpValue(
                    fe2o3_amqp_types::primitives::Value::String("String Value Body.".to_string()),
                )))
                .build();

            let serialized_fe2o3 = serde_amqp::ser::to_vec(
                &fe2o3_amqp_types::messaging::message::__private::Serializable(
                    amqp_message.clone(),
                ),
            )
            .unwrap();
            assert_eq!(serialized, serialized_fe2o3);

            // Now deserialize the message and verify that it matches the original.

            let value = serde_amqp::de::from_slice::<
                fe2o3_amqp_types::messaging::message::__private::Deserializable<
                    fe2o3_amqp_types::messaging::Message<
                        fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
                    >,
                >,
            >(serialized_fe2o3.as_slice())
            .unwrap();
            assert_eq!(amqp_message, value.0);
        }

        #[cfg(feature = "cplusplus")]
        {
            let deserialized = AmqpMessage::decode(&serialized).unwrap();
            assert_eq!(deserialized, message);
        }
    }
}
