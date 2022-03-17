use azure_core::{Etag, Header};
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

impl Header for IfMatchCondition {
    fn add_to_builder(&self, builder: Builder) -> Builder {
        match self {
            IfMatchCondition::Etag(etag) => builder.header(IF_MATCH, etag.as_ref()),
            IfMatchCondition::Any => builder.header(IF_MATCH, "*"),
        }
    }

    fn add_to_request(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
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

    fn name(&self) -> &'static str {
        IF_MATCH.as_str()
    }

    fn value(&self) -> String {
        match self {
            IfMatchCondition::Etag(etag) => etag.to_string(),
            IfMatchCondition::Any => "*".to_owned(),
        }
    }
}
