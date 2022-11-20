use fe2o3_amqp_types::primitives::{OrderedMap, Symbol};
use serde_amqp::{
    described::Described, descriptor::Descriptor, DeserializeComposite, SerializeComposite, Value,
};

use super::{
    error::CorrelationFilterError,
    management_constants::properties::{
        CONTENT_TYPE, CORRELATION_ID, CORRELATION_RULE_FILTER_PROPERTIES, LABEL, MESSAGE_ID,
        REPLY_TO, REPLY_TO_SESSION_ID, SESSION_ID, TO,
    },
};

// <type name="com.microsoft:session-filter" class="restricted" source="string" provides="filter">
//     <descriptor name="com.microsoft:session-filter" code="0x00000137:000000C"/>
// </type>
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeComposite, DeserializeComposite,
)]
#[amqp_contract(
    name = "com.microsoft:session-filter",
    code = "0x0000_0137:0x0000_000c",
    encoding = "basic"
)]
pub struct SessionFilter(pub String);

impl From<SessionFilter> for Described<String> {
    fn from(filter: SessionFilter) -> Self {
        Self {
            // descriptor: Descriptor::Code((0x0000_0137 << 32) | 0x0000_000c), // FIXME: descriptor code doesn't work yet
            descriptor: Descriptor::Name(Symbol::from("com.microsoft:session-filter")),
            value: filter.0,
        }
    }
}

impl From<SessionFilter> for Described<Value> {
    fn from(filter: SessionFilter) -> Self {
        let described: Described<String> = filter.into();
        Self {
            descriptor: described.descriptor,
            value: described.value.into(),
        }
    }
}

impl From<SessionFilter> for Option<Described<Value>> {
    fn from(filter: SessionFilter) -> Self {
        Some(filter.into())
    }
}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:sql-filter:list",
    code = "0x0000_0137:0x0000_0006",
    encoding = "list",
    rename_all = "kebab-case" // This shouldn't matter because we're using the list encoding
)]
pub struct SqlFilter {
    pub expression: String,
}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:correlation-filter:list",
    code = "0x0000_0137:0x0000_0009",
    encoding = "list",
    rename_all = "kebab-case" // This shouldn't matter because we're using the list encoding
)]
pub struct CorrelationFilter {
    pub correlation_id: Option<String>,
    pub message_id: Option<String>,
    pub to: Option<String>,
    pub reply_to: Option<String>,
    pub label: Option<String>,
    pub session_id: Option<String>,
    pub reply_to_session_id: Option<String>,
    pub content_type: Option<String>,
    pub properties: Option<OrderedMap<String, Value>>,
}

impl TryFrom<CorrelationFilter> for OrderedMap<Value, Value> {
    type Error = CorrelationFilterError;

    fn try_from(filter: CorrelationFilter) -> Result<Self, Self::Error> {
        let mut map = OrderedMap::new();
        if let Some(correlation_id) = filter.correlation_id {
            map.insert(
                Value::String(CORRELATION_ID.to_string()),
                Value::String(correlation_id),
            );
        }
        if let Some(message_id) = filter.message_id {
            map.insert(
                Value::String(MESSAGE_ID.to_string()),
                Value::String(message_id),
            );
        }
        if let Some(to) = filter.to {
            map.insert(Value::String(TO.to_string()), Value::String(to));
        }
        if let Some(reply_to) = filter.reply_to {
            map.insert(Value::String(REPLY_TO.to_string()), Value::String(reply_to));
        }
        if let Some(label) = filter.label {
            map.insert(Value::String(LABEL.to_string()), Value::String(label));
        }
        if let Some(session_id) = filter.session_id {
            map.insert(
                Value::String(SESSION_ID.to_string()),
                Value::String(session_id),
            );
        }
        if let Some(reply_to_session_id) = filter.reply_to_session_id {
            map.insert(
                Value::String(REPLY_TO_SESSION_ID.to_string()),
                Value::String(reply_to_session_id),
            );
        }
        if let Some(content_type) = filter.content_type {
            map.insert(
                Value::String(CONTENT_TYPE.to_string()),
                Value::String(content_type),
            );
        }
        if let Some(properties) = filter.properties {
            map.insert(
                Value::String(CORRELATION_RULE_FILTER_PROPERTIES.to_string()),
                properties.into(),
            );
        }
        if map.is_empty() {
            Err(CorrelationFilterError::EmptyFilter)
        } else {
            Ok(map)
        }
    }
}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:true-filter:list",
    code = "0x0000_0137:0x0000_0007",
    encoding = "list"
)]
pub struct TrueFilter {}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:false-filter:list",
    code = "0x0000_0137:0x0000_0008",
    encoding = "list"
)]
pub struct FalseFilter {}
