use crate::headers::{self, Header};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfSourceModifiedSinceCondition {
    Modified(DateTime<Utc>),
    Unmodified(DateTime<Utc>),
}

impl Header for IfSourceModifiedSinceCondition {
    fn name(&self) -> headers::HeaderName {
        match self {
            IfSourceModifiedSinceCondition::Modified(_) => headers::SOURCE_IF_MODIFIED_SINCE,
            IfSourceModifiedSinceCondition::Unmodified(_) => headers::SOURCE_IF_UNMODIFIED_SINCE,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfSourceModifiedSinceCondition::Modified(date) => date.to_rfc2822(),
            IfSourceModifiedSinceCondition::Unmodified(date) => date.to_rfc2822(),
        }
        .into()
    }
}
