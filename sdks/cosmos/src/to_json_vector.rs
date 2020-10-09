use azure_sdk_core::errors::AzureError;
use serde::ser::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ToJsonVector {
    last_serialized_string: Option<String>,
    serialized_string: String,
}

impl ToJsonVector {
    pub fn new() -> Self {
        Self {
            last_serialized_string: None,
            serialized_string: String::from("["),
        }
    }

    // this hack builds the json by hands, concatenating the single json
    // serialization of the items. Ideally we should collect the &dyn Serialize
    // trait objects and serialize in one shot at the end but unfortunately
    // Serialize cannot be made a trait object so no dynamic dispatch :(
    pub fn push<T>(&mut self, t: T) -> Result<&mut Self, AzureError>
    where
        T: Serialize,
    {
        if self.serialized_string.len() > 1 {
            self.serialized_string.push_str(", ");
        }

        let serialized_string = serde_json::to_string(&t)?;
        self.serialized_string.push_str(&serialized_string);
        self.last_serialized_string = Some(serialized_string);

        Ok(self)
    }

    pub(crate) fn to_json(&self) -> String {
        format!("{}]", self.serialized_string)
    }

    #[allow(dead_code)]
    pub(crate) fn last_serialized_string(&self) -> Option<&str> {
        match self.last_serialized_string.as_ref() {
            Some(last_serialized_str) => Some(last_serialized_str),
            None => None,
        }
    }
}

impl std::default::Default for ToJsonVector {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::convert::TryFrom<&[T]> for ToJsonVector
where
    T: Serialize,
{
    type Error = AzureError;

    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        let mut to_json_vector = ToJsonVector::new();
        for item in slice {
            to_json_vector.push(item)?;
        }
        Ok(to_json_vector)
    }
}

impl<T> std::convert::TryFrom<&Vec<T>> for ToJsonVector
where
    T: Serialize,
{
    type Error = AzureError;

    fn try_from(v: &Vec<T>) -> Result<Self, Self::Error> {
        let mut to_json_vector = ToJsonVector::new();
        for item in v {
            to_json_vector.push(item)?;
        }
        Ok(to_json_vector)
    }
}

impl std::convert::From<&str> for ToJsonVector {
    fn from(t: &str) -> Self {
        let mut pk = Self::new();
        let _ = pk.push(t).unwrap();
        pk
    }
}

impl std::convert::From<Cow<'_, str>> for ToJsonVector {
    fn from(t: Cow<'_, str>) -> Self {
        let mut pk = Self::new();
        let _ = pk.push(t).unwrap();
        pk
    }
}

impl std::convert::From<&Cow<'_, str>> for ToJsonVector {
    fn from(t: &Cow<'_, str>) -> Self {
        let mut pk = Self::new();
        let _ = pk.push(t).unwrap();
        pk
    }
}

impl std::convert::From<&String> for ToJsonVector {
    fn from(t: &String) -> Self {
        let mut pk = Self::new();
        let _ = pk.push(t).unwrap();
        pk
    }
}

impl std::convert::From<u64> for ToJsonVector {
    fn from(t: u64) -> Self {
        let mut pk = Self::new();
        let _ = pk.push(t).unwrap();
        pk
    }
}

impl std::convert::From<i64> for ToJsonVector {
    fn from(t: i64) -> Self {
        let mut pk = Self::new();
        let _ = pk.push(t).unwrap();
        pk
    }
}

impl<'a> std::convert::From<ToJsonVector> for Cow<'a, ToJsonVector> {
    fn from(t: ToJsonVector) -> Self {
        Cow::Owned(t)
    }
}

impl<'a> std::convert::From<&'a ToJsonVector> for Cow<'a, ToJsonVector> {
    fn from(t: &'a ToJsonVector) -> Self {
        Cow::Borrowed(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn serialize() {
        let owned = "owned".to_owned();

        let serialized = ToJsonVector::new()
            .push("aaa")
            .unwrap()
            .push(&owned)
            .unwrap()
            .push(100u64)
            .unwrap()
            .to_json();
        assert_eq!(serialized, "[\"aaa\", \"owned\", 100]");

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
