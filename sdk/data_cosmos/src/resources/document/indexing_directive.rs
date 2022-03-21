use crate::headers;
use azure_core::ParseError;
use http::request::Builder;
use std::fmt;

/// Whether the resource should be included in the index.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexingDirective {
    /// Follow the default indexing policy for the collection.
    Default,
    /// Add the resource to the index.
    Include,
    /// Omit the resource to the index.
    Exclude,
}

impl<'a> From<&'a IndexingDirective> for &'a str {
    fn from(s: &'a IndexingDirective) -> &'a str {
        match s {
            IndexingDirective::Default => "Default",
            IndexingDirective::Exclude => "Exclude",
            IndexingDirective::Include => "Include",
        }
    }
}

impl std::str::FromStr for IndexingDirective {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Default" => Ok(IndexingDirective::Default),
            "Exclude" => Ok(IndexingDirective::Exclude),
            "Include" => Ok(IndexingDirective::Include),
            _ => Err(ParseError::UnknownVariant {
                item: "IndexingDirective",
                variant: s.to_owned(),
            }),
        }
    }
}

impl fmt::Display for IndexingDirective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl azure_core::Header for IndexingDirective {
    fn name(&self) -> azure_core::headers::HeaderName {
        todo!()
    }

    fn value(&self) -> azure_core::headers::HeaderValue {
        todo!()
    }

    fn add_to_builder(&self, builder: Builder) -> Builder {
        match self {
            IndexingDirective::Default => builder,
            IndexingDirective::Exclude => {
                builder.header(headers::HEADER_INDEXING_DIRECTIVE, "Exclude")
            }
            IndexingDirective::Include => {
                builder.header(headers::HEADER_INDEXING_DIRECTIVE, "Include")
            }
        }
    }

    fn add_to_request(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        let (header_name, header_value) = match self {
            IndexingDirective::Default => return Ok(()),
            IndexingDirective::Exclude => (headers::HEADER_INDEXING_DIRECTIVE, "Exclude"),
            IndexingDirective::Include => (headers::HEADER_INDEXING_DIRECTIVE, "Include"),
        };

        request.headers_mut().append(
            header_name,
            http::header::HeaderValue::from_str(header_value)?,
        );
        Ok(())
    }
}
