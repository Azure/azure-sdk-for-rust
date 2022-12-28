//! This module contains the types for working with Service Bus rules.

use std::marker::PhantomData;

use fe2o3_amqp_types::primitives::Timestamp;
use serde_amqp::{
    described::Described, descriptor::Descriptor, DeserializeComposite, SerializeComposite,
};

use filters::RuleFilter;
use time::OffsetDateTime;

mod filters;
pub use filters::*; // re-export to match the namespace

/// Properties of a rule.
#[derive(Clone)]
pub struct RuleProperties {
    /// The filter expression used to match messages.
    pub filters: RuleFilter,

    /// The action to perform if the message matches the filter expression.
    pub actions: RuleAction,

    /// The name of the rule.
    pub name: String,

    /// The time the rule was created.
    pub created_at: Option<OffsetDateTime>,

    // Prevents construction outside of this crate
    // TODO: is this necessary?
    _sealed: PhantomData<()>,
}

// Manual implementation of Debug to avoid printing the `_sealed` field
impl std::fmt::Debug for RuleProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuleProperties")
            .field("filters", &self.filters)
            .field("actions", &self.actions)
            .field("name", &self.name)
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl RuleProperties {
    /// The name of the default rule.
    pub const DEFAULT_RULE_NAME: &str = "$Default";
}

impl From<RuleDescription> for RuleProperties {
    fn from(desc: RuleDescription) -> Self {
        Self {
            filters: desc.filters,
            actions: desc.actions,
            name: desc.name,
            created_at: desc.created_at.map(|t| t.into()),

            _sealed: PhantomData,
        }
    }
}

impl From<RuleProperties> for RuleDescription {
    fn from(props: RuleProperties) -> Self {
        Self {
            filters: props.filters,
            actions: props.actions,
            name: props.name,
            created_at: props.created_at.map(|t| t.into()),
        }
    }
}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:enumerate-rules:list",
    code = "0x0000_0137_0000_0004",
    encoding = "list",
    rename_all = "kebab-case" // This should not matter because we're using the list encoding
)]
pub(crate) struct RuleDescription {
    pub filters: RuleFilter,
    pub actions: RuleAction,
    pub name: String,
    pub created_at: Option<Timestamp>,
}

/// No rule action present
#[derive(
    Debug, Clone, SerializeComposite, DeserializeComposite, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[amqp_contract(
    name = "com.microsoft:empty-rule-action:list",
    code = "0x0000_0137_0000_0005",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct EmptyRuleAction {}

/// SQL rule action
#[derive(Clone, SerializeComposite, DeserializeComposite, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[amqp_contract(
    name = "com.microsoft:sql-rule-action:list",
    code = "0x0000_0137_0000_0006",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct SqlRuleAction {
    /// SQL rule action's expression.
    pub expression: String,

    /// There is an undocumented field here that is present in the responses
    _undocumented_int: Option<i32>,
}

impl std::fmt::Debug for SqlRuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlRuleAction")
            .field("expression", &self.expression)
            .finish()
    }
}

impl SqlRuleAction {
    /// Creates a new SQL rule action.
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),

            // Set it to `None` so that it is not serialized
            _undocumented_int: None,
        }
    }
}

impl From<SqlRuleAction> for Described<String> {
    fn from(action: SqlRuleAction) -> Self {
        Described {
            descriptor: Descriptor::Code(0x0000_0137_0000_0006),
            value: action.expression,
        }
    }
}

/// Represents the filter actions which are allowed for the transformation
/// of a message that have been matched by a filter expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuleAction {
    /// No rule action present
    Empty(EmptyRuleAction),

    /// SQL rule action
    Sql(SqlRuleAction),
}

mod rule_action_impl {
    use serde::{
        de::{self, VariantAccess},
        ser,
    };

    use super::RuleAction;

    impl ser::Serialize for RuleAction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                RuleAction::Empty(rule) => rule.serialize(serializer),
                RuleAction::Sql(rule) => rule.serialize(serializer),
            }
        }
    }

    struct FieldVisitor {}

    enum Field {
        Empty,
        Sql,
    }

    impl<'de> de::Visitor<'de> for FieldVisitor {
        type Value = Field;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Descriptor for either an empty rule action or a SQL rule action")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                "com.microsoft:empty-rule-action:list" => Ok(Field::Empty),
                "com.microsoft:sql-rule-action:list" => Ok(Field::Sql),
                _ => Err(de::Error::unknown_variant(
                    v,
                    &[
                        "com.microsoft:empty-rule-action:list",
                        "com.microsoft:sql-rule-action:list",
                    ],
                )),
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                0x0000_0137_0000_0005 => Ok(Field::Empty),
                0x0000_0137_0000_0006 => Ok(Field::Sql),
                _ => Err(de::Error::unknown_variant(
                    &v.to_string(),
                    &["0x0000_0137_0000_0005", "0x0000_0137_0000_0006"],
                )),
            }
        }
    }

    impl<'de> de::Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_identifier(FieldVisitor {})
        }
    }

    struct Visitor {}

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = RuleAction;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Either an empty rule action or a SQL rule action")
        }

        fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
        where
            A: de::EnumAccess<'de>,
        {
            let (val, variant) = data.variant()?;
            match val {
                Field::Empty => {
                    let rule = variant.newtype_variant()?;
                    Ok(RuleAction::Empty(rule))
                }
                Field::Sql => {
                    let rule = variant.newtype_variant()?;
                    Ok(RuleAction::Sql(rule))
                }
            }
        }
    }

    impl<'de> de::Deserialize<'de> for RuleAction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_enum(
                serde_amqp::__constants::UNTAGGED_ENUM,
                &["Empty", "Sql"],
                Visitor {},
            )
        }
    }
}
