use crate::blob::blob::BlobBlockType;

#[derive(Debug, Clone, PartialEq)]
pub struct BlobBlockWithSize {
    pub block_list_type: BlobBlockType,
    pub size_in_bytes: u64,
}
