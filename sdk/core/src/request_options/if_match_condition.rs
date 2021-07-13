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

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        let (header_name, header_value) = match self {
            IfMatchCondition::Match(etag) => (IF_MATCH, etag),
            IfMatchCondition::NotMatch(etag) => (IF_NONE_MATCH, etag),
        };

        request
            .headers_mut()
            .append(header_name, http::HeaderValue::from_str(header_value)?);

        Ok(())
    }
}
