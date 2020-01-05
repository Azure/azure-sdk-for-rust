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

// TODO: Remove this code smell
impl fmt::Display for IndexingDirective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{}", s)
    }
}
