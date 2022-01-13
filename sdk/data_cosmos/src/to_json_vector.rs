use serde::Serialize;
use std::fmt;

#[derive(Clone)]
pub struct ToJsonVector {
    inner: String,
}

impl ToJsonVector {
    pub fn new() -> Self {
        Self {
            inner: String::from("["),
        }
    }

    pub fn push<T>(&mut self, t: T) -> serde_json::Result<()>
    where
        T: Serialize,
    {
        let is_first = self.inner == "[";
        if !is_first {
            self.inner.push(',');
            self.inner.push(' ');
        }
        self.inner.push_str(&serde_json::to_string(&t)?);
        Ok(())
    }

    pub(crate) fn to_json(&self) -> String {
        let mut result = self.inner.clone();
        result.push(']');
        result
    }
}

impl std::default::Default for ToJsonVector {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ToJsonVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

impl<T> From<&[T]> for ToJsonVector
where
    T: Serialize,
{
    fn from(slice: &[T]) -> Self {
        let mut to_json_vector = Self::new();
        for item in slice {
            to_json_vector.push(item).unwrap();
        }
        to_json_vector
    }
}

macro_rules! delegate_from_impl {
    ($($t:ty),*) => {
        $(
            delegate_from_impl!(@imp &$t);
            delegate_from_impl!(@imp $t);
        )*
    };
    (@imp $t:ty) => {
        impl<T: Serialize> From<$t> for ToJsonVector {
            fn from(s: $t) -> Self {
                let slice: &[T] = &s[..];
                slice.into()
            }
        }
    }
}

delegate_from_impl! {
    Vec<T>,
    [T; 0],
    [T; 1],
    [T; 2],
    [T; 3],
    [T; 4],
    [T; 5],
    [T; 6],
    [T; 7],
    [T; 8]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn serialize() {
        let owned = "owned".to_owned();

        let mut serialized = ToJsonVector::new();
        serialized.push("aaa").unwrap();
        serialized.push(&owned).unwrap();
        serialized.push(&100u64).unwrap();
        assert_eq!(serialized.to_json(), "[\"aaa\", \"owned\", 100]");

        let mut vector = vec!["pollo", "arrosto"];
        let to_json_vector: ToJsonVector = (&vector).try_into().unwrap();
        assert_eq!(to_json_vector.to_json(), "[\"pollo\", \"arrosto\"]");

        vector.push("limone");
        let slice: &[&str] = &vector;
        let to_json_vector: ToJsonVector = slice.try_into().unwrap();
        assert_eq!(
            to_json_vector.to_json(),
            "[\"pollo\", \"arrosto\", \"limone\"]"
        );
    }
}
