// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use url::Url;

use crate::utils::url_encode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // For the variants. Can be removed when we have them all implemented.
pub enum ResourceType {
    Databases,
    DatabaseAccount,
    Containers,
    Documents,
    StoredProcedures,
    Users,
    Permissions,
    PartitionKeyRanges,
    UserDefinedFunctions,
    Triggers,
    Offers,
}

impl ResourceType {
    pub fn path_segment(self) -> &'static str {
        match self {
            ResourceType::Databases => "dbs",
            ResourceType::DatabaseAccount => "",
            ResourceType::Containers => "colls",
            ResourceType::Documents => "docs",
            ResourceType::StoredProcedures => "sprocs",
            ResourceType::Users => "users",
            ResourceType::Permissions => "permissions",
            ResourceType::PartitionKeyRanges => "pkranges",
            ResourceType::UserDefinedFunctions => "udfs",
            ResourceType::Triggers => "triggers",
            ResourceType::Offers => "offers",
        }
    }

    pub fn is_meta_data(self) -> bool {
        matches!(
            self,
            ResourceType::Databases
                | ResourceType::DatabaseAccount
                | ResourceType::Containers
                | ResourceType::PartitionKeyRanges
        )
    }
}

/// Represents a "resource link" defining a sub-resource in Azure Cosmos DB
///
/// This value is URL encoded, and can be [`Url::join`]ed to the endpoint root to produce the full absolute URL for a Cosmos DB resource.
/// It's also intended for use by the signature algorithm used when authenticating with a primary key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLink {
    parent: Option<String>,
    item_id: Option<String>,
    resource_type: ResourceType,
}

impl ResourceLink {
    pub fn root(resource_type: ResourceType) -> Self {
        Self {
            parent: None,
            resource_type,
            item_id: None,
        }
    }

    pub fn feed(&self, resource_type: ResourceType) -> Self {
        Self {
            parent: Some(self.path()),
            resource_type,
            item_id: None,
        }
    }

    pub fn item(&self, item_id: &str) -> Self {
        let item_id = url_encode(item_id.as_bytes());
        Self {
            parent: self.parent.clone(),
            resource_type: self.resource_type,
            item_id: Some(item_id),
        }
    }

    /// Gets the resource "link" identified by this link, for use when generating the authentication signature.
    ///
    /// For links referring to items, this is the full path of the item.
    /// For links referring to feeds (for query, create, etc. requests where there is no item ID to reference), this is the path to the PARENT resource.
    ///
    /// See https://learn.microsoft.com/en-us/rest/api/cosmos-db/access-control-on-cosmosdb-resources#constructkeytoken for more details.
    #[cfg_attr(not(feature = "key_auth"), allow(dead_code))] // REASON: Currently only used in key_auth feature but we don't want to conditional-compile it.
    pub fn resource_link(&self) -> String {
        match (self.resource_type, self.item_id.as_ref()) {
            // Offers have a particular resource link format expected when requesting the offer itself.
            (ResourceType::Offers, Some(i)) => i.to_lowercase(),
            (_, Some(_)) => self.path(),
            (_, None) => self.parent.clone().unwrap_or_default(),
        }
    }

    /// Gets the [`ResourceType`] identified by this link, for use when generating the authentication signature.
    #[cfg_attr(not(feature = "key_auth"), allow(dead_code))] // REASON: Currently only used in key_auth feature but we don't want to conditional-compile it.
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    /// Gets the path that must be appended to the root account endpoint to access this resource.
    pub fn path(&self) -> String {
        match (self.parent.as_ref(), self.item_id.as_ref()) {
            (None, Some(item_id)) => {
                format!("{}/{}", self.resource_type.path_segment(), item_id)
            }
            (Some(parent), Some(item_id)) => format!(
                "{}/{}/{}",
                parent,
                self.resource_type.path_segment(),
                item_id
            ),
            (None, None) => self.resource_type.path_segment().to_string(),
            (Some(ref parent), None) => format!("{}/{}", parent, self.resource_type.path_segment()),
        }
    }

    /// Creates a new [`Url`] by joining the provided `endpoint` with the path from [`ResourceLink::path`].
    pub fn url(&self, endpoint: &Url) -> Url {
        endpoint
            .join(&self.path())
            .expect("ResourceLink should always be url-safe")
    }
}

#[cfg(test)]
mod tests {
    use crate::resource_context::{ResourceLink, ResourceType};

    #[test]
    pub fn root_link() {
        let link = ResourceLink::root(ResourceType::Databases);
        assert_eq!(
            ResourceLink {
                parent: None,
                resource_type: ResourceType::Databases,
                item_id: None,
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("", link.resource_link());
        assert_eq!(ResourceType::Databases, link.resource_type());
    }

    #[test]
    pub fn root_item_link() {
        let link = ResourceLink::root(ResourceType::Databases).item("TestDB");
        assert_eq!(
            ResourceLink {
                parent: None,
                resource_type: ResourceType::Databases,
                item_id: Some("TestDB".to_string()),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/TestDB",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/TestDB", link.resource_link());
        assert_eq!(ResourceType::Databases, link.resource_type());
    }

    #[test]
    pub fn child_feed_link() {
        let link = ResourceLink::root(ResourceType::Databases)
            .item("TestDB")
            .feed(ResourceType::Containers);
        assert_eq!(
            ResourceLink {
                parent: Some("dbs/TestDB".to_string()),
                resource_type: ResourceType::Containers,
                item_id: None,
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/TestDB/colls",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/TestDB", link.resource_link());
        assert_eq!(ResourceType::Containers, link.resource_type());
    }

    #[test]
    pub fn child_item_link() {
        let link = ResourceLink::root(ResourceType::Databases)
            .item("TestDB")
            .feed(ResourceType::Containers)
            .item("TestContainer");
        assert_eq!(
            ResourceLink {
                parent: Some("dbs/TestDB".to_string()),
                resource_type: ResourceType::Containers,
                item_id: Some("TestContainer".to_string()),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/TestDB/colls/TestContainer",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/TestDB/colls/TestContainer", link.resource_link());
        assert_eq!(ResourceType::Containers, link.resource_type());
    }

    #[test]
    pub fn resource_links_are_url_encoded() {
        let link = ResourceLink::root(ResourceType::Databases)
            .item("Test DB")
            .feed(ResourceType::Containers)
            .item("Test/Container");
        assert_eq!(
            ResourceLink {
                parent: Some("dbs/Test+DB".to_string()),
                resource_type: ResourceType::Containers,
                item_id: Some("Test%2FContainer".to_string())
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/Test+DB/colls/Test%2FContainer",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/Test+DB/colls/Test%2FContainer", link.resource_link());
        assert_eq!(ResourceType::Containers, link.resource_type());
    }
}
