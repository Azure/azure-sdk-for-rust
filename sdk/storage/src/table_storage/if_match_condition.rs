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
}
