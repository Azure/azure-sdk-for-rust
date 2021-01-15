use crate::AddAsHeader;
use http::header::{IF_MATCH, IF_NONE_MATCH};
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfMatchCondition<'a> {
    Match(&'a str),
    NotMatch(&'a str),
}

impl<'a> AddAsHeader for IfMatchCondition<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            IfMatchCondition::Match(etag) => builder.header(IF_MATCH, *etag),
            IfMatchCondition::NotMatch(etag) => builder.header(IF_NONE_MATCH, *etag),
        }
    }
}
