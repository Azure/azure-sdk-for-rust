use std::borrow::Borrow;

// TODO: Change from Borrow<[u8]> to BlockId
// Also, why Borrow? :| :|
#[derive(Debug, Clone, PartialEq)]
pub enum BlobBlockType<T>
where
    T: Borrow<[u8]>,
{
    Committed(T),
    Uncommitted(T),
    Latest(T),
}
