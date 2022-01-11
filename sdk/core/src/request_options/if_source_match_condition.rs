use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, PartialEq)]
pub enum IfSourceMatchCondition {
    Match(String),
    NotMatch(String),
}

impl AddAsHeader for IfSourceMatchCondition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            IfSourceMatchCondition::Match(etag) => builder.header(SOURCE_IF_MATCH, etag),
            IfSourceMatchCondition::NotMatch(etag) => builder.header(SOURCE_IF_NONE_MATCH, etag),
        }
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
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
