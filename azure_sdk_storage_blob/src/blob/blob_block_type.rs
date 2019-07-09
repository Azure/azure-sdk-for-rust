use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq)]
pub enum BlobBlockType<T>
where
    T: Borrow<[u8]>,
{
    Committed(T),
    Uncommitted(T),
    Latest(T),
}
