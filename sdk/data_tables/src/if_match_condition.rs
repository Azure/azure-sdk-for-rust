use azure_core::{
    headers::{self, Header},
    Etag,
};
use headers::IF_MATCH;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfMatchCondition {
    Etag(Etag),
    Any,
}

impl From<Etag> for IfMatchCondition {
    fn from(etag: Etag) -> Self {
        Self::Etag(etag)
    }
}

impl Header for IfMatchCondition {
    fn name(&self) -> headers::HeaderName {
        IF_MATCH
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfMatchCondition::Etag(etag) => etag.to_string(),
            IfMatchCondition::Any => "*".to_owned(),
        }
        .into()
    }
}
