use crate::headers;
use azure_core::AddAsHeader;
use http::request::Builder;

/// A collection of keys to partition on
///
/// You can learn more about partitioning [here](https://docs.microsoft.com/en-us/azure/cosmos-db/partitioning-overview)
pub type PartitionKeys = crate::to_json_vector::ToJsonVector;

impl AddAsHeader for &'_ PartitionKeys {
    fn add_as_header(&self, builder: Builder) -> Builder {
        headers::add_partition_keys_header(self, builder)
    }
}
