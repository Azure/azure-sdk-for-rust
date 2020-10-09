use crate::blob::blob::BlobBlockType;
use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq)]
pub struct BlobBlockWithSize<T>
where
    T: Borrow<[u8]>,
{
    pub block_list_type: BlobBlockType<T>,
    pub size_in_bytes: u64,
}
