#[derive(Debug, Clone)]
pub enum BlobBlockType<T> {
    Committed(T),
    Uncommitted(T),
    Latest(T),
}
