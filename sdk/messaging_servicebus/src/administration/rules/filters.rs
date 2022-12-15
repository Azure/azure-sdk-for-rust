use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::{
    DeserializeComposite, SerializeComposite, Value,
};
use std::marker::PhantomData;

use crate::amqp::{
    error::CorrelationFilterError,
    management_constants::properties::{
        CONTENT_TYPE, CORRELATION_ID, CORRELATION_RULE_FILTER_PROPERTIES, LABEL, MESSAGE_ID,
        REPLY_TO, REPLY_TO_SESSION_ID, SESSION_ID, TO,
    },
};

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[amqp_contract(
    name = "com.microsoft:sql-filter:list",
    code = "0x0000_0013_7000_0006",
    encoding = "list",
    rename_all = "kebab-case" // This shouldn't matter because we're using the list encoding
)]
pub struct SqlRuleFilter {
    pub expression: String,
}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[amqp_contract(
    name = "com.microsoft:correlation-filter:list",
    code = "0x0000_0013_7000_0009",
    encoding = "list",
    rename_all = "kebab-case" // This shouldn't matter because we're using the list encoding
)]
pub struct CorrelationRuleFilter {
    pub correlation_id: Option<String>,
    pub message_id: Option<String>,
    pub to: Option<String>,
    pub reply_to: Option<String>,
    pub subject: Option<String>,
    pub session_id: Option<String>,
    pub reply_to_session_id: Option<String>,
    pub content_type: Option<String>,
    pub properties: Option<OrderedMap<String, Value>>,
}

impl CorrelationRuleFilter {
    pub fn builder() -> CorrelationRuleFilterBuilder<()> {
        CorrelationRuleFilterBuilder::default()
    }
}

/// Type state for the correlation filter builder
#[derive(Debug, Clone)]
pub struct Initialized {}

#[derive(Debug, Default, Clone)]
pub struct CorrelationRuleFilterBuilder<T> {
    pub correlation_id: Option<String>,
    pub message_id: Option<String>,
    pub to: Option<String>,
    pub reply_to: Option<String>,
    pub subject: Option<String>,
    pub session_id: Option<String>,
    pub reply_to_session_id: Option<String>,
    pub content_type: Option<String>,
    pub properties: Option<OrderedMap<String, Value>>,

    marker: PhantomData<T>
}

impl<T> CorrelationRuleFilterBuilder<T> {
    fn map_to_initialized(self) -> CorrelationRuleFilterBuilder<Initialized> {
        CorrelationRuleFilterBuilder {
            correlation_id: self.correlation_id,
            message_id: self.message_id,
            to: self.to,
            reply_to: self.reply_to,
            subject: self.subject,
            session_id: self.session_id,
            reply_to_session_id: self.reply_to_session_id,
            content_type: self.content_type,
            properties: self.properties,
            marker: PhantomData
        }
    }

    pub fn correlation_id(mut self, correlation_id: impl Into<String>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.correlation_id = Some(correlation_id.into());
        self.map_to_initialized()
    }

    pub fn message_id(mut self, message_id: impl Into<String>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.message_id = Some(message_id.into());
        self.map_to_initialized()
    }

    pub fn to(mut self, to: impl Into<String>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.to = Some(to.into());
        self.map_to_initialized()
    }

    pub fn reply_to(mut self, reply_to: impl Into<String>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.reply_to = Some(reply_to.into());
        self.map_to_initialized()
    }

    pub fn subject(mut self, subject: impl Into<String>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.subject = Some(subject.into());
        self.map_to_initialized()
    }

    pub fn session_id(mut self, session_id: impl Into<String>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.session_id = Some(session_id.into());
        self.map_to_initialized()
    }

    pub fn reply_to_session_id(mut self, reply_to_session_id: impl Into<String>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.reply_to_session_id = Some(reply_to_session_id.into());
        self.map_to_initialized()
    }

    pub fn content_type(mut self, content_type: impl Into<String>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.content_type = Some(content_type.into());
        self.map_to_initialized()
    }

    pub fn properties(mut self, properties: OrderedMap<String, Value>) -> CorrelationRuleFilterBuilder<Initialized> {
        self.properties = Some(properties);
        self.map_to_initialized()
    }
}

impl CorrelationRuleFilterBuilder<Initialized> {
    pub fn build(self) -> CorrelationRuleFilter {
        CorrelationRuleFilter {
            correlation_id: self.correlation_id,
            message_id: self.message_id,
            to: self.to,
            reply_to: self.reply_to,
            subject: self.subject,
            session_id: self.session_id,
            reply_to_session_id: self.reply_to_session_id,
            content_type: self.content_type,
            properties: self.properties,
        }
    }
}

impl TryFrom<CorrelationRuleFilter> for OrderedMap<Value, Value> {
    type Error = CorrelationFilterError;

    fn try_from(filter: CorrelationRuleFilter) -> Result<Self, Self::Error> {
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
        if let Some(label) = filter.subject {
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

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[amqp_contract(
    name = "com.microsoft:true-filter:list",
    code = "0x0000_0013_7000_0007",
    encoding = "list"
)]
pub struct TrueRuleFilter {}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[amqp_contract(
    name = "com.microsoft:false-filter:list",
    code = "0x0000_0013_7000_0008",
    encoding = "list"
)]
pub struct FalseRuleFilter {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuleFilter {
    Sql(SqlRuleFilter),
    Correlation(CorrelationRuleFilter),
    True(TrueRuleFilter),
    False(FalseRuleFilter),
}

mod filter_impl {
    use serde::{
        de::{self, VariantAccess},
        ser,
    };

    use super::RuleFilter;

    impl ser::Serialize for RuleFilter {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            match self {
                RuleFilter::Sql(filter) => filter.serialize(serializer),
                RuleFilter::Correlation(filter) => filter.serialize(serializer),
                RuleFilter::True(filter) => filter.serialize(serializer),
                RuleFilter::False(filter) => filter.serialize(serializer),
            }
        }
    }

    struct FieldVisitor {}

    enum Field {
        Sql,
        Correlation,
        True,
        False,
    }

    impl<'de> de::Visitor<'de> for FieldVisitor {
        type Value = Field;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(
                "Descriptor for one of the following filters: Sql, Correlation, True, False",
            )
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                "com.microsoft:sql-filter:list" => Ok(Field::Sql),
                "com.microsoft:correlation-filter:list" => Ok(Field::Correlation),
                "com.microsoft:true-filter:list" => Ok(Field::True),
                "com.microsoft:false-filter:list" => Ok(Field::False),
                _ => Err(de::Error::custom(format!(
                    "Unknown filter descriptor: {}",
                    v
                ))),
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                0x0000_0013_7000_0006 => Ok(Field::Sql),
                0x0000_0013_7000_0009 => Ok(Field::Correlation),
                0x0000_0013_7000_0007 => Ok(Field::True),
                0x0000_0013_7000_0008 => Ok(Field::False),
                _ => Err(de::Error::custom(format!(
                    "Unknown filter descriptor: {}",
                    v
                ))),
            }
        }
    }

    impl<'de> de::Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_identifier(FieldVisitor {})
        }
    }

    struct Visitor {}

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = RuleFilter;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("One of the following filters: Sql, Correlation, True, False")
        }

        fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
        where
            A: de::EnumAccess<'de>,
        {
            let (field, value) = data.variant::<Field>()?;
            match field {
                Field::Sql => Ok(RuleFilter::Sql(value.newtype_variant()?)),
                Field::Correlation => Ok(RuleFilter::Correlation(value.newtype_variant()?)),
                Field::True => Ok(RuleFilter::True(value.newtype_variant()?)),
                Field::False => Ok(RuleFilter::False(value.newtype_variant()?)),
            }
        }
    }

    impl<'de> de::Deserialize<'de> for RuleFilter {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_enum(
                serde_amqp::__constants::UNTAGGED_ENUM,
                &["Sql", "Correlation", "True", "False"],
                Visitor {},
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_true_filter() {
        let filter = super::TrueRuleFilter {};
        let value = serde_amqp::to_value(&filter).unwrap();
        println!("value: {:?}", value);
    }
}
