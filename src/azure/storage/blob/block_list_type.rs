#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockListType {
    Committed,
    Uncommitted,
    All,
}

impl BlockListType {
    pub fn to_str(&self) -> &str {
        match self {
            BlockListType::All => "all",
            BlockListType::Committed => "committed",
            BlockListType::Uncommitted => "uncommitted",
        }
    }
}
