use bytes::Bytes;

// TODO: Change from Bytes to BlockId?
#[derive(Debug, Clone, PartialEq)]
pub enum BlobBlockType {
    Committed(Bytes),
    Uncommitted(Bytes),
    Latest(Bytes),
}

impl BlobBlockType {
    pub fn new_committed(b: impl Into<Bytes>) -> Self {
        BlobBlockType::Committed(b.into())
    }

    pub fn new_uncommitted(b: impl Into<Bytes>) -> Self {
        BlobBlockType::Uncommitted(b.into())
    }

    pub fn new_latest(b: impl Into<Bytes>) -> Self {
        BlobBlockType::Latest(b.into())
    }
}
