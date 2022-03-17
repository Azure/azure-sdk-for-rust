use crate::headers;
use crate::Header;

#[derive(Debug, Clone, PartialEq)]
pub enum IfSourceMatchCondition {
    Match(String),
    NotMatch(String),
}

impl Header for IfSourceMatchCondition {
    fn name(&self) -> &'static str {
        match self {
            IfSourceMatchCondition::Match(_) => headers::SOURCE_IF_MATCH,
            IfSourceMatchCondition::NotMatch(_) => headers::SOURCE_IF_NONE_MATCH,
        }
    }

    fn value(&self) -> String {
        match self.clone() {
            IfSourceMatchCondition::Match(etag) => etag,
            IfSourceMatchCondition::NotMatch(etag) => etag,
        }
    }
}
