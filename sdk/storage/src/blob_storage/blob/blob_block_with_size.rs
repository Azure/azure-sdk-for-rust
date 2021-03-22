use crate::blob_storage::blob::BlobBlockType;

#[derive(Debug, Clone, PartialEq)]
pub struct BlobBlockWithSize {
    pub block_list_type: BlobBlockType,
    pub size_in_bytes: u64,
}
