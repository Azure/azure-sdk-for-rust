// use std::ops::Index;

#[derive(Debug)]
pub struct IncompleteVector<T> {
    next_marker: Option<String>,
    vector: Vec<T>,
}

impl<T> IncompleteVector<T> {
    pub fn new(next_marker: Option<String>, vector: Vec<T>) -> IncompleteVector<T> {
        IncompleteVector{
            next_marker : next_marker,
            vector : vector
        }
    }

    pub fn next_marker(&self) -> Option<&str> {
        match self.next_marker {
            Some(ref nm) => Some(nm),
            None => None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.next_marker().is_none()
    }

    fn into(&self) -> &[T] {
        &self.vector
    }
}
