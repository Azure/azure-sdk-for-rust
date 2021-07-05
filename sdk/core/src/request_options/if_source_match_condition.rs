use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfSourceMatchCondition<'a> {
    Match(&'a str),
    NotMatch(&'a str),
}

impl<'a> AddAsHeader for IfSourceMatchCondition<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            IfSourceMatchCondition::Match(etag) => builder.header(SOURCE_IF_MATCH, *etag),
            IfSourceMatchCondition::NotMatch(etag) => builder.header(SOURCE_IF_NONE_MATCH, *etag),
        }
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        let (header_name, header_value) = match self {
            IfSourceMatchCondition::Match(etag) => (SOURCE_IF_MATCH, etag),
            IfSourceMatchCondition::NotMatch(etag) => (SOURCE_IF_NONE_MATCH, etag),
        };

        request
            .headers_mut()
            .append(header_name, http::HeaderValue::from_str(header_value)?);

        Ok(())
    }
}
