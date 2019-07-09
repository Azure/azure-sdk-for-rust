use azure_sdk_core::errors::AzureError;
use serde_json;
use smallvec::{IntoIter, SmallVec};
use std::borrow::Cow;
use std::iter::IntoIterator;

#[derive(Debug, Clone)]
pub struct PartitionKey<'a> {
    pk: Option<SmallVec<[Cow<'a, str>; 2]>>,
}

impl<'a> PartitionKey<'a> {
    pub fn chain<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        match self.pk {
            Some(ref mut p) => p.push(key.into()),
            None => {
                self.pk = {
                    let mut vec = SmallVec::new();
                    vec.push(key.into());
                    Some(vec)
                }
            }
        }
        self
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

impl<'a> IntoIterator for PartitionKey<'a> {
    type Item = Cow<'a, str>;
    type IntoIter = IntoIter<[Cow<'a, str>; 2]>;

    fn into_iter(self) -> Self::IntoIter {
        match self.pk {
            Some(p) => p.into_iter(),
            None => SmallVec::new().into_iter(),
        }
    }
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for PartitionKey<'a> {
    fn from(v: S) -> PartitionKey<'a> {
        PartitionKey::default().chain(v)
    }
}
