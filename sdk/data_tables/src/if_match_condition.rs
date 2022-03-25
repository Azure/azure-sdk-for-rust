use azure_core::headers::{self, Header};
use azure_core::Etag;
use http::header::IF_MATCH;

#[derive(Debug, Clone, PartialEq)]
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
        IF_MATCH.into()
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfMatchCondition::Etag(etag) => etag.to_string(),
            IfMatchCondition::Any => "*".to_owned(),
        }
        .into()
    }
}
