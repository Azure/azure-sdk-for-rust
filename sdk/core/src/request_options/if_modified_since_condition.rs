use crate::{
    date,
    headers::{self, Header, HeaderName},
};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfModifiedSinceCondition {
    Modified(OffsetDateTime),
    Unmodified(OffsetDateTime),
}

impl Header for IfModifiedSinceCondition {
    fn name(&self) -> HeaderName {
        match self {
            IfModifiedSinceCondition::Modified(_) => headers::IF_MODIFIED_SINCE,
            IfModifiedSinceCondition::Unmodified(_) => headers::IF_UNMODIFIED_SINCE,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfModifiedSinceCondition::Modified(date)
            | IfModifiedSinceCondition::Unmodified(date) => date::to_rfc1123(date),
        }
        .into()
    }
}
