use crate::{
    date,
    headers::{self, Header, HeaderName},
};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfSourceModifiedSinceCondition {
    Modified(OffsetDateTime),
    Unmodified(OffsetDateTime),
}

impl Header for IfSourceModifiedSinceCondition {
    fn name(&self) -> HeaderName {
        match self {
            IfSourceModifiedSinceCondition::Modified(_) => headers::SOURCE_IF_MODIFIED_SINCE,
            IfSourceModifiedSinceCondition::Unmodified(_) => headers::SOURCE_IF_UNMODIFIED_SINCE,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfSourceModifiedSinceCondition::Modified(date)
            | IfSourceModifiedSinceCondition::Unmodified(date) => date::to_rfc1123(date).into(),
        }
    }
}
