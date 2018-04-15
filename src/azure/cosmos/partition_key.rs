use azure::core::errors::AzureError;
use serde_json;
use std::iter::IntoIterator;

#[derive(Debug, Clone)]
pub struct PartitionKey<'a> {
    pk: Option<Vec<&'a str>>,
}

impl<'a> PartitionKey<'a> {
    pub fn new() -> PartitionKey<'a> {
        PartitionKey { pk: None }
    }

    pub fn push(&mut self, key: &'a str) {
        match self.pk {
            Some(ref mut p) => p.push(key),
            None => self.pk = Some(vec![key]),
        }
    }

    pub fn to_json(&self) -> Result<Option<String>, AzureError> {
        match self.pk {
            // the partition key should be a json formatted string list
            Some(ref val) => Ok(Some(serde_json::to_string(val)?)),
            None => Ok(None),
        }
    }
}

impl<'a> ::std::default::Default for PartitionKey<'a> {
    fn default() -> Self {
        PartitionKey { pk: None }
    }
}

//impl<'a> Serialize for PartitionKey<'a> {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer,
//    {
//        match self.pk {
//            // the partition key should be a json formatted string list
//            Some(val) => serializer.serialize_seq(val)?,
//            None => S::Ok,
//        }
//    }
//}

impl<'a> IntoIterator for PartitionKey<'a> {
    type Item = &'a str;
    type IntoIter = ::std::vec::IntoIter<&'a str>;

    fn into_iter(self) -> Self::IntoIter {
        match self.pk {
            Some(p) => p.into_iter(),
            None => ::std::vec::Vec::<&str>::new().into_iter(),
        }
    }
}
