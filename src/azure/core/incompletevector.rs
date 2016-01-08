use std::convert::Into;
use std::ops::Deref;

#[derive(Debug)]
pub struct IncompleteVector<T> {
    next_marker: Option<String>,
    vector: Vec<T>,
}

#[allow(dead_code)]
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
}

// impl<'a, T> Into<&'a [T]> for IncompleteVector<T> {
//     fn into(self) -> &'a [T] {
//         &self.vector
//     }
// }

// impl<'a, T> Deref for IncompleteVector<T> {
//     type Target = &'a [T];
//
//     fn deref(&self) -> &'a [T] {
//         &self.vector
//     }
// }

mod test {
    use super::*;

    #[test]
    fn test_incomplete_vector_complete() {
        let v = vec![0,1,2,3,4,5];
        let ic = IncompleteVector::new(None, v);

        assert_eq!(ic.is_complete(), true);
    }

    #[test]
    fn test_incomplete_vector_incomplete() {
        let v = vec![0,1,2,3,4,5];
        let ic = IncompleteVector::new(Some("aaa".to_owned()), v);

        assert_eq!(ic.is_complete(), false);
    }

    // #[test]
    // fn test_incomplete_vector_deref() {
    //     let v = vec![0,1,2,3,4,5];
    //     let ic = IncompleteVector::new(None, v);
    //
    //     let slice : &[u32] = &ic;
    //     assert_eq!(slice[0], 0);
    // }
}
