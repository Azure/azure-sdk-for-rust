use std::ops::Deref;

#[derive(Debug)]
pub struct IncompleteVector<T> {
    next_marker: Option<String>,
    vector: Vec<T>,
}

#[allow(dead_code)]
impl<'a, T> IncompleteVector<T> {
    pub fn new(next_marker: Option<String>, vector: Vec<T>) -> IncompleteVector<T> {
        IncompleteVector {
            next_marker: next_marker,
            vector: vector,
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
}

impl<T> Deref for IncompleteVector<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.vector
    }
}

mod test {
    use super::*;

    #[test]
    fn test_incomplete_vector_complete() {
        let v = vec![0, 1, 2, 3, 4, 5];
        let ic = IncompleteVector::new(None, v);

        assert_eq!(ic.is_complete(), true);
    }

    #[test]
    fn test_incomplete_vector_incomplete() {
        let v = vec![0, 1, 2, 3, 4, 5];
        let ic = IncompleteVector::new(Some("aaa".to_owned()), v);

        assert_eq!(ic.is_complete(), false);
    }

    #[test]
    fn test_incomplete_vector_deref() {
        let v = vec![0, 1, 2, 3, 4, 5];
        let ic = IncompleteVector::new(None, v);
        assert_eq!(ic[0], 0);
    }
}
