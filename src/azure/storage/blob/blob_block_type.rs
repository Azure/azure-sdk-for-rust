#[derive(Debug, Clone, PartialEq)]
pub enum BlobBlockType<T> {
    Committed(T),
    Uncommitted(T),
    Latest(T),
}
