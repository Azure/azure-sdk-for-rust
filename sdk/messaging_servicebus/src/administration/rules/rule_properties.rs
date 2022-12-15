use super::{RuleAction, RuleFilter, filters::TrueFilter, EmptyRuleAction};

/// Represents the properties of a rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleProperties {
    pub name: String,
    pub filter: RuleFilter,
    pub action: RuleAction,
}

impl RuleProperties {
    pub const DEFAULT_RULE_NAME: &'static str = "$Default";
}
