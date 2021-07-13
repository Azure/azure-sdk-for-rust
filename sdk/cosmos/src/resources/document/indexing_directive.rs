use crate::headers;
use azure_core::ParsingError;
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

impl std::convert::Into<&str> for &IndexingDirective {
    fn into(self) -> &'static str {
        match self {
            IndexingDirective::Default => "Default",
            IndexingDirective::Exclude => "Exclude",
            IndexingDirective::Include => "Include",
        }
    }
}

impl std::str::FromStr for IndexingDirective {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Default" => Ok(IndexingDirective::Default),
            "Exclude" => Ok(IndexingDirective::Exclude),
            "Include" => Ok(IndexingDirective::Include),
            _ => Err(ParsingError::UnknownVariant {
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

impl azure_core::AddAsHeader for IndexingDirective {
    fn add_as_header(&self, builder: Builder) -> Builder {
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

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HTTPHeaderError> {
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
