use std::ops::{Deref, DerefMut};

pub type ContinuationToken = String;

#[derive(Debug, Clone)]
pub struct IncompleteVector<T> {
    token: Option<String>,
    vector: Vec<T>,
}

impl<T> IncompleteVector<T> {
    pub fn new(token: Option<String>, vector: Vec<T>) -> IncompleteVector<T> {
        IncompleteVector {
            token: token,
            vector: vector,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.token().is_none()
    }

    fn token(&self) -> Option<&str> {
        if let Some(ref t) = self.token {
            Some(t)
        } else {
            None
        }
    }
}

impl<T> DerefMut for IncompleteVector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.vector
    }
}

impl<T> Deref for IncompleteVector<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.vector
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
