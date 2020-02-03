use crate::headers::{IF_SEQUENCE_NUMBER_EQ, IF_SEQUENCE_NUMBER_LE, IF_SEQUENCE_NUMBER_LT};
use chrono::{DateTime, Utc};
use http::request::Builder;
use hyper::header::{IF_MATCH, IF_MODIFIED_SINCE, IF_NONE_MATCH, IF_UNMODIFIED_SINCE};

#[derive(Debug, Clone, PartialEq)]
pub enum IfSinceCondition {
    Modified(DateTime<Utc>),
    Unmodified(DateTime<Utc>),
}

impl IfSinceCondition {
    pub(crate) fn add_header(&self, builder: Builder) -> Builder {
        match self {
            IfSinceCondition::Modified(date) => {
                builder.header(IF_MODIFIED_SINCE, &date.to_rfc2822() as &str)
            }
            IfSinceCondition::Unmodified(date) => {
                builder.header(IF_UNMODIFIED_SINCE, &date.to_rfc2822() as &str)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfMatchCondition<'a> {
    Match(&'a str),
    NotMatch(&'a str),
}

impl<'a> IfMatchCondition<'a> {
    pub(crate) fn add_header(&self, builder: Builder) -> Builder {
        match self {
            IfMatchCondition::Match(etag) => builder.header(IF_MATCH, *etag),
            IfMatchCondition::NotMatch(etag) => builder.header(IF_NONE_MATCH, *etag),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SequenceNumberCondition {
    Less(u64),
    LessOrEqual(u64),
    Equal(u64),
}

impl SequenceNumberCondition {
    pub(crate) fn add_header(&self, builder: Builder) -> Builder {
        match self {
            SequenceNumberCondition::Equal(val) => {
                builder.header(IF_SEQUENCE_NUMBER_EQ, &val.to_string() as &str)
            }
            SequenceNumberCondition::LessOrEqual(val) => {
                builder.header(IF_SEQUENCE_NUMBER_LE, &val.to_string() as &str)
            }
            SequenceNumberCondition::Less(val) => {
                builder.header(IF_SEQUENCE_NUMBER_LT, &val.to_string() as &str)
            }
        }
    }
}
