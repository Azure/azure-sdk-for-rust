use crate::headers::{self, Header};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfSourceMatchCondition {
    Match(String),
    NotMatch(String),
}

impl Header for IfSourceMatchCondition {
    fn name(&self) -> headers::HeaderName {
        match self {
            IfSourceMatchCondition::Match(_) => headers::SOURCE_IF_MATCH,
            IfSourceMatchCondition::NotMatch(_) => headers::SOURCE_IF_NONE_MATCH,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self.clone() {
            IfSourceMatchCondition::Match(etag) | IfSourceMatchCondition::NotMatch(etag) => {
                etag.into()
            }
        }
    }
}
