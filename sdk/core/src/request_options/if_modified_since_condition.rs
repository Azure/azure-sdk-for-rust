use crate::headers::{self, Header};
use chrono::{DateTime, Utc};
use headers::{IF_MODIFIED_SINCE, IF_UNMODIFIED_SINCE};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfModifiedSinceCondition {
    Modified(DateTime<Utc>),
    Unmodified(DateTime<Utc>),
}

impl Header for IfModifiedSinceCondition {
    fn name(&self) -> headers::HeaderName {
        match self {
            IfModifiedSinceCondition::Modified(_) => IF_MODIFIED_SINCE,
            IfModifiedSinceCondition::Unmodified(_) => IF_UNMODIFIED_SINCE,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfModifiedSinceCondition::Modified(date) => date.to_rfc2822(),
            IfModifiedSinceCondition::Unmodified(date) => date.to_rfc2822(),
        }
        .into()
    }
}
