use crate::Header;
use http::header::{IF_MATCH, IF_NONE_MATCH};

#[derive(Debug, Clone, PartialEq)]
pub enum IfMatchCondition {
    Match(String),
    NotMatch(String),
}

impl Header for IfMatchCondition {
    fn name(&self) -> &'static str {
        match self {
            IfMatchCondition::Match(_) => IF_MATCH.as_str(),
            IfMatchCondition::NotMatch(_) => IF_NONE_MATCH.as_str(),
        }
    }

    fn value(&self) -> String {
        match self.clone() {
            IfMatchCondition::Match(etag) => etag,
            IfMatchCondition::NotMatch(etag) => etag,
        }
    }
}
