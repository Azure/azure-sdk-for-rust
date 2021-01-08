use crate::headers;
use crate::to_json_vector::ToJsonVector;
use azure_core::AddAsHeader;
use http::request::Builder;
use serde::Serialize;

/// A collection of keys to partition on
///
/// You can learn more about partitioning [here](https://docs.microsoft.com/en-us/azure/cosmos-db/partitioning-overview)
#[derive(Debug, Clone)]
pub struct PartitionKeys(ToJsonVector);

impl PartitionKeys {
    /// New partition keys
    pub fn new() -> Self {
        Self(ToJsonVector::new())
    }

    /// Push a serialized object into the collection
    pub fn push<T: Serialize>(&mut self, item: T) -> serde_json::Result<()> {
        self.0.push(item)
    }

    /// Convert into a JSON formatted string
    pub fn to_json(&self) -> String {
        self.0.to_json()
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
        impl<T: Serialize> From<$t> for PartitionKeys {
            fn from(s: $t) -> Self {
                Self(ToJsonVector::from(s))
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

impl AddAsHeader for &'_ PartitionKeys {
    fn add_as_header(&self, builder: Builder) -> Builder {
        headers::add_partition_keys_header(self, builder)
    }
}
