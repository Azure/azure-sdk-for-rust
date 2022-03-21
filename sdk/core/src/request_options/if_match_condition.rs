use crate::headers::{self, Header};
use http::header::{IF_MATCH, IF_NONE_MATCH};

#[derive(Debug, Clone, PartialEq)]
pub enum IfMatchCondition {
    Match(String),
    NotMatch(String),
}

impl Header for IfMatchCondition {
    fn name(&self) -> headers::HeaderName {
        match self {
            IfMatchCondition::Match(_) => IF_MATCH.into(),
            IfMatchCondition::NotMatch(_) => IF_NONE_MATCH.into(),
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self.clone() {
            IfMatchCondition::Match(etag) => etag,
            IfMatchCondition::NotMatch(etag) => etag,
        }
        .into()
    }
}
