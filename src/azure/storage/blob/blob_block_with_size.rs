use azure::storage::blob::BlobBlockType;
use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq)]
pub struct BlobBlockWithSize<T>
where
    T: Borrow<str>,
{
    pub block_list_type: BlobBlockType<T>,
    pub size_in_bytes: u64,
}
