use std::ops::Index;

#[derive(Debug)]
pub struct IncompleteVector<T> {
    next_marker : String,
    vector : Vec<T>,
}

impl<T> IncompleteVector<T> {
    pub fn next_marker(&self) -> &str {
        &self.next_marker
    }

    pub fn is_complete(&self) -> bool {
        self.next_marker() == ""
    }

    pub fn vector(&self) -> &[T] {
        &self.vector
    }
}
