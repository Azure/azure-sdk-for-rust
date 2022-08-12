use crate::blob::BlobBlockType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobBlockWithSize {
    pub block_list_type: BlobBlockType,
    pub size_in_bytes: u64,
}
