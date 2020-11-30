use azure_core::enumerations::ParsingError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexingDirective {
    Default,
    Include,
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
            _ => Err(ParsingError::ElementNotFound(s.to_owned())),
        }
    }
}

impl fmt::Display for IndexingDirective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
