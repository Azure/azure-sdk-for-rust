use azure_core::headers::{self, AsHeaders};
use azure_core::ParseError;
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

impl AsHeaders for IndexingDirective {
    type Iter = std::vec::IntoIter<(headers::HeaderName, headers::HeaderValue)>;

    fn as_headers(&self) -> Self::Iter {
        match self {
            IndexingDirective::Default => vec![].into_iter(),
            IndexingDirective::Exclude => vec![(
                crate::headers::HEADER_INDEXING_DIRECTIVE.into(),
                "Exclude".into(),
            )]
            .into_iter(),
            IndexingDirective::Include => vec![(
                crate::headers::HEADER_INDEXING_DIRECTIVE.into(),
                "Include".into(),
            )]
            .into_iter(),
        }
    }
}
