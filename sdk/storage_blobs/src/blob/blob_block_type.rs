use crate::options::BlockId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobBlockType {
    Committed(BlockId),
    Uncommitted(BlockId),
    Latest(BlockId),
}

impl BlobBlockType {
    pub fn new_committed(b: impl Into<BlockId>) -> Self {
        BlobBlockType::Committed(b.into())
    }

    pub fn new_uncommitted(b: impl Into<BlockId>) -> Self {
        BlobBlockType::Uncommitted(b.into())
    }

    pub fn new_latest(b: impl Into<BlockId>) -> Self {
        BlobBlockType::Latest(b.into())
    }
}
