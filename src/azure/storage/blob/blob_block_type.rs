use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq)]
pub enum BlobBlockType<T>
where
    T: Borrow<str>,
{
    Committed(T),
    Uncommitted(T),
    Latest(T),
}
