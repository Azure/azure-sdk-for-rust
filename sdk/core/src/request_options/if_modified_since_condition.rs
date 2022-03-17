use crate::Header;
use chrono::{DateTime, Utc};
use http::header::{IF_MODIFIED_SINCE, IF_UNMODIFIED_SINCE};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfModifiedSinceCondition {
    Modified(DateTime<Utc>),
    Unmodified(DateTime<Utc>),
}

impl Header for IfModifiedSinceCondition {
    fn name(&self) -> &'static str {
        match self {
            IfModifiedSinceCondition::Modified(_) => IF_MODIFIED_SINCE.as_str(),
            IfModifiedSinceCondition::Unmodified(_) => IF_UNMODIFIED_SINCE.as_str(),
        }
    }

    fn value(&self) -> String {
        match self {
            IfModifiedSinceCondition::Modified(date) => date.to_rfc2822(),
            IfModifiedSinceCondition::Unmodified(date) => date.to_rfc2822(),
        }
    }
}
