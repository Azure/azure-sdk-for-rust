use azure_core::SasError;

use super::constants;
use std::str::FromStr;

/// Specifies the resource types accessible from an account level shared access signature.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TableAccountSasResourceType {
    /// Indicates whether service-level APIs are accessible
    /// from this shared access signature (e.g., Get/Set Service Properties, Get Service Stats, List Tables).
    Service,

    /// Indicates whether table account-level APIs are accessible
    /// from this shared access signature (e.g. Create/Delete/Query Table).
    Container,

    /// Indicates whether entity-level APIs are accessible from this shared access
    /// signature (e.g. Query/Insert/Update/Delete entity).
    Object,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct TableAccountSasResourceTypes([char; 3]);

impl TableAccountSasResourceTypes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn all() -> Self {
        Self(['s', 'c', 'o'])
    }

    pub fn add_resource(mut self, resource_type: TableAccountSasResourceType) -> Self {
        match resource_type {
            TableAccountSasResourceType::Service => self.0[0] = constants::SERVICE,
            TableAccountSasResourceType::Container => self.0[1] = constants::CONTAINER,
            TableAccountSasResourceType::Object => self.0[2] = constants::OBJECT,
        };
        self
    }
}

impl From<TableAccountSasResourceTypes> for String {
    fn from(builder: TableAccountSasResourceTypes) -> Self {
        let mut resource_types = String::with_capacity(builder.0.len());
        for resource_type in builder.0 {
            if resource_type != char::default() {
                resource_types.push(resource_type);
            }
        }
        resource_types
    }
}

impl FromStr for TableAccountSasResourceTypes {
    type Err = SasError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
