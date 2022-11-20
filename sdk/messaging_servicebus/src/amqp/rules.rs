use serde_amqp::{DeserializeComposite, SerializeComposite};

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:enumerate-rules:list",
    code = "0x0000_0137_0000_0004",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct RuleDescription {}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:empty-rule-action:list",
    code = "0x0000_0137_0000_0005",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct EmptyRuleAction {}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:sql-rule-action:list",
    code = "0x0000_0137_0000_0006",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct SqlRuleAction {
    pub expression: String,
}

#[derive(Debug, Clone)]
pub enum RuleAction {
    Empty(EmptyRuleAction),
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
