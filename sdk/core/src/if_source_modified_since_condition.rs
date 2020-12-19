use crate::headers::*;
use crate::AddAsHeader;
use chrono::{DateTime, Utc};
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfSourceModifiedSinceCondition {
    Modified(DateTime<Utc>),
    Unmodified(DateTime<Utc>),
}

impl AddAsHeader for IfSourceModifiedSinceCondition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            IfSourceModifiedSinceCondition::Modified(date) => {
                builder.header(SOURCE_IF_MODIFIED_SINCE, &date.to_rfc2822() as &str)
            }
            IfSourceModifiedSinceCondition::Unmodified(date) => {
                builder.header(SOURCE_IF_UNMODIFIED_SINCE, &date.to_rfc2822() as &str)
            }
        }
    }
}
