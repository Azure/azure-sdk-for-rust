use azure_core::prelude::Etag;
use azure_core::AddAsHeader;
use http::header::IF_MATCH;
use http::request::Builder;

#[derive(Debug, Clone, PartialEq)]
pub enum IfMatchCondition {
    Etag(Etag),
    Any,
}

impl From<Etag> for IfMatchCondition {
    fn from(etag: Etag) -> Self {
        Self::Etag(etag)
    }
}

impl AddAsHeader for IfMatchCondition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            IfMatchCondition::Etag(etag) => builder.header(IF_MATCH, etag.as_ref()),
            IfMatchCondition::Any => builder.header(IF_MATCH, "*"),
        }
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HTTPHeaderError> {
        let (header_name, header_value) = match self {
            IfMatchCondition::Etag(etag) => (IF_MATCH, etag.as_ref()),
            IfMatchCondition::Any => (IF_MATCH, "*"),
        };

        request.headers_mut().append(
            header_name,
            http::header::HeaderValue::from_str(header_value)?,
        );

        Ok(())
    }
}
