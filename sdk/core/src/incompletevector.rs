use crate::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct IncompleteVector<T> {
    next_marker: Option<NextMarker>,
    vector: Vec<T>,
}

impl<T> IncompleteVector<T> {
    pub fn new(next_marker: Option<NextMarker>, vector: Vec<T>) -> IncompleteVector<T> {
        IncompleteVector {
            next_marker,
            vector,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.next_marker().is_none()
    }

    pub fn next_marker(&self) -> Option<&NextMarker> {
        self.next_marker.as_ref()
    }
}

impl<T> Deref for IncompleteVector<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.vector
    }
}

impl<T> DerefMut for IncompleteVector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.vector
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::IncompleteVector;

    #[test]
    fn test_incomplete_vector_complete() {
        let v = vec![0, 1, 2, 3, 4, 5];
        let ic = IncompleteVector::new(None, v);

        assert!(ic.is_complete());
    }

    #[test]
    fn test_incomplete_vector_incomplete() {
        let v = vec![0, 1, 2, 3, 4, 5];
        let ic = IncompleteVector::new(Some("aaa".into()), v);

        assert!(!ic.is_complete());
    }

    #[test]
    fn test_incomplete_vector_deref() {
        let v = vec![0, 1, 2, 3, 4, 5];
        let ic = IncompleteVector::new(None, v);
        assert_eq!(ic[0], 0);
    }
}
