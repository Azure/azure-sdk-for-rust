use crate::headers::{self, Header};
use headers::{IF_MATCH, IF_NONE_MATCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfMatchCondition {
    Match(String),
    NotMatch(String),
}

impl Header for IfMatchCondition {
    fn name(&self) -> headers::HeaderName {
        match self {
            IfMatchCondition::Match(_) => IF_MATCH,
            IfMatchCondition::NotMatch(_) => IF_NONE_MATCH,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self.clone() {
            IfMatchCondition::Match(etag) | IfMatchCondition::NotMatch(etag) => etag.into(),
        }
    }
}
