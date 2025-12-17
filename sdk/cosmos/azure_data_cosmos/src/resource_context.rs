// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use url::Url;

use crate::utils::url_encode;

/// Represents a segment of a resource link path, storing both encoded and unencoded forms.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LinkSegment {
    /// Unencoded form of the segment.
    unencoded: String,

    /// URL-encoded form of the segment. If this is `None`, the segment does not require encoding.
    encoded: Option<String>,
}

impl LinkSegment {
    /// Creates a new `LinkSegment` by encoding the provided value.
    fn new(value: impl Into<String>) -> Self {
        let unencoded = value.into();

        // TODO: There are probably ways we can optimize this to avoid storing both forms all the time, especially when encoding is unnecessary in most cases.
        let encoded = url_encode(unencoded.as_bytes());

        Self {
            unencoded,
            encoded: Some(encoded),
        }
    }

    /// Creates a new `LinkSegment` without encoding (e.g., for RIDs).
    fn identity(value: impl Into<String>) -> Self {
        Self {
            unencoded: value.into(),
            encoded: None,
        }
    }

    /// Gets the URL-encoded form of this segment.
    fn encoded(&self) -> &str {
        self.encoded.as_deref().unwrap_or(&self.unencoded)
    }

    /// Gets the unencoded (original) form of this segment.
    fn unencoded(&self) -> &str {
        &self.unencoded
    }
}

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
    parent: Option<LinkSegment>,
    item_id: Option<LinkSegment>,
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
            parent: Some(self.path_segment()),
            resource_type,
            item_id: None,
        }
    }

    pub fn item(&self, item_id: &str) -> Self {
        Self {
            parent: self.parent.clone(),
            resource_type: self.resource_type,
            item_id: Some(LinkSegment::new(item_id)),
        }
    }

    pub fn item_by_rid(&self, rid: &str) -> Self {
        // RIDs are not URL encoded
        Self {
            parent: self.parent.clone(),
            resource_type: self.resource_type,
            item_id: Some(LinkSegment::identity(rid)),
        }
    }

    /// Helper method to create a LinkSegment representing the current path (for use as parent).
    fn path_segment(&self) -> LinkSegment {
        LinkSegment {
            unencoded: self.unencoded_path(),
            encoded: Some(self.path()),
        }
    }

    /// Gets the resource "link" identified by this link, for use when generating the authentication signature.
    ///
    /// For links referring to items, this is the full path of the item.
    /// For links referring to feeds (for query, create, etc. requests where there is no item ID to reference), this is the path to the PARENT resource.
    ///
    /// This path is NOT URL-encoded, as required by the signature algorithm.
    ///
    /// See https://learn.microsoft.com/en-us/rest/api/cosmos-db/access-control-on-cosmosdb-resources#constructkeytoken for more details.
    #[cfg_attr(not(feature = "key_auth"), allow(dead_code))] // REASON: Currently only used in key_auth feature but we don't want to conditional-compile it.
    pub fn link_for_signing(&self) -> String {
        match (self.resource_type, self.item_id.as_ref()) {
            // Offers have a particular resource link format expected when requesting the offer itself.
            (ResourceType::Offers, Some(i)) => i.unencoded().to_lowercase(),
            (_, Some(_)) => self.unencoded_path(),
            (_, None) => self
                .parent
                .as_ref()
                .map(|p| p.unencoded().to_string())
                .unwrap_or_default(),
        }
    }

    /// Gets the [`ResourceType`] identified by this link, for use when generating the authentication signature.
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    /// Gets the URL-encoded path that must be appended to the root account endpoint to access this resource.
    pub fn path(&self) -> String {
        match (self.parent.as_ref(), self.item_id.as_ref()) {
            (None, Some(item_id)) => {
                format!(
                    "{}/{}",
                    self.resource_type.path_segment(),
                    item_id.encoded()
                )
            }
            (Some(parent), Some(item_id)) => format!(
                "{}/{}/{}",
                parent.encoded(),
                self.resource_type.path_segment(),
                item_id.encoded()
            ),
            (None, None) => self.resource_type.path_segment().to_string(),
            (Some(parent), None) => {
                format!("{}/{}", parent.encoded(), self.resource_type.path_segment())
            }
        }
    }

    /// Gets the unencoded path for use in authentication signatures.
    fn unencoded_path(&self) -> String {
        match (self.parent.as_ref(), self.item_id.as_ref()) {
            (None, Some(item_id)) => {
                format!(
                    "{}/{}",
                    self.resource_type.path_segment(),
                    item_id.unencoded()
                )
            }
            (Some(parent), Some(item_id)) => format!(
                "{}/{}/{}",
                parent.unencoded(),
                self.resource_type.path_segment(),
                item_id.unencoded()
            ),
            (None, None) => self.resource_type.path_segment().to_string(),
            (Some(parent), None) => {
                format!(
                    "{}/{}",
                    parent.unencoded(),
                    self.resource_type.path_segment()
                )
            }
        }
    }

    /// Creates a new [`Url`] by joining the provided `endpoint` with the path from [`ResourceLink::path`].
    pub fn url(&self, endpoint: &Url) -> Url {
        endpoint
            .join(&self.path())
            .expect("ResourceLink should always be url-safe")
    }
}

impl std::fmt::Display for ResourceLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{:?}, parent: {:?}, item: {:?}]",
            self.path(),
            self.resource_type,
            self.parent.as_ref().map(|s| s.encoded()),
            self.item_id.as_ref().map(|s| s.encoded()),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::resource_context::{LinkSegment, ResourceLink, ResourceType};

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
        assert_eq!("", link.link_for_signing());
        assert_eq!(ResourceType::Databases, link.resource_type());
    }

    #[test]
    pub fn root_item_link() {
        let link = ResourceLink::root(ResourceType::Databases).item("TestDB");
        assert_eq!(
            ResourceLink {
                parent: None,
                resource_type: ResourceType::Databases,
                item_id: Some(LinkSegment {
                    unencoded: "TestDB".to_string(),
                    encoded: "TestDB".to_string(),
                }),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/TestDB",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/TestDB", link.link_for_signing());
        assert_eq!(ResourceType::Databases, link.resource_type());
    }

    #[test]
    pub fn child_feed_link() {
        let link = ResourceLink::root(ResourceType::Databases)
            .item("TestDB")
            .feed(ResourceType::Containers);
        assert_eq!(
            ResourceLink {
                parent: Some(LinkSegment {
                    unencoded: "dbs/TestDB".to_string(),
                    encoded: "dbs/TestDB".to_string(),
                }),
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
        assert_eq!("dbs/TestDB", link.link_for_signing());
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
                parent: Some(LinkSegment {
                    unencoded: "dbs/TestDB".to_string(),
                    encoded: "dbs/TestDB".to_string(),
                }),
                resource_type: ResourceType::Containers,
                item_id: Some(LinkSegment {
                    unencoded: "TestContainer".to_string(),
                    encoded: "TestContainer".to_string(),
                }),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/TestDB/colls/TestContainer",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/TestDB/colls/TestContainer", link.link_for_signing());
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
                parent: Some(LinkSegment {
                    unencoded: "dbs/Test DB".to_string(),
                    encoded: "dbs/Test+DB".to_string(),
                }),
                resource_type: ResourceType::Containers,
                item_id: Some(LinkSegment {
                    unencoded: "Test/Container".to_string(),
                    encoded: "Test%2FContainer".to_string(),
                }),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/Test+DB/colls/Test%2FContainer",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        // link_for_signing should return UNENCODED path
        assert_eq!("dbs/Test DB/colls/Test/Container", link.link_for_signing());
        assert_eq!(ResourceType::Containers, link.resource_type());
    }

    #[test]
    pub fn rid_based_item_link() {
        let link = ResourceLink::root(ResourceType::Databases).item_by_rid("ABCDEF==");
        assert_eq!(
            ResourceLink {
                parent: None,
                resource_type: ResourceType::Databases,
                item_id: Some(LinkSegment {
                    unencoded: "ABCDEF==".to_string(),
                    encoded: "ABCDEF==".to_string(),
                }),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/ABCDEF==",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/ABCDEF==", link.link_for_signing());
        assert_eq!(ResourceType::Databases, link.resource_type());
    }

    #[test]
    pub fn rid_based_child_item_link() {
        let link = ResourceLink::root(ResourceType::Databases)
            .item_by_rid("DatabaseRID==")
            .feed(ResourceType::Containers)
            .item_by_rid("ContainerRID+=");
        assert_eq!(
            ResourceLink {
                parent: Some(LinkSegment {
                    unencoded: "dbs/DatabaseRID==".to_string(),
                    encoded: "dbs/DatabaseRID==".to_string(),
                }),
                resource_type: ResourceType::Containers,
                item_id: Some(LinkSegment {
                    unencoded: "ContainerRID+=".to_string(),
                    encoded: "ContainerRID+=".to_string(),
                }),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/DatabaseRID==/colls/ContainerRID+=",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!(
            "dbs/DatabaseRID==/colls/ContainerRID+=",
            link.link_for_signing()
        );
        assert_eq!(ResourceType::Containers, link.resource_type());
    }

    #[test]
    pub fn rid_based_links_are_not_url_encoded() {
        let link = ResourceLink::root(ResourceType::Databases)
            .item_by_rid("ABC+DEF=")
            .feed(ResourceType::Containers)
            .item_by_rid("XYZ/123==");
        // Verify that special characters like +, /, and = are NOT encoded in RID-based links
        assert_eq!(
            ResourceLink {
                parent: Some(LinkSegment {
                    unencoded: "dbs/ABC+DEF=".to_string(),
                    encoded: "dbs/ABC+DEF=".to_string(),
                }),
                resource_type: ResourceType::Containers,
                item_id: Some(LinkSegment {
                    unencoded: "XYZ/123==".to_string(),
                    encoded: "XYZ/123==".to_string(),
                }),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/ABC+DEF=/colls/XYZ/123==",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/ABC+DEF=/colls/XYZ/123==", link.link_for_signing());
    }

    #[test]
    pub fn mixed_rid_and_regular_links() {
        // Test that mixing RID-based and regular items works correctly
        let link = ResourceLink::root(ResourceType::Databases)
            .item("TestDB")
            .feed(ResourceType::Containers)
            .item_by_rid("ContainerRID==");
        assert_eq!(
            ResourceLink {
                parent: Some(LinkSegment {
                    unencoded: "dbs/TestDB".to_string(),
                    encoded: "dbs/TestDB".to_string(),
                }),
                resource_type: ResourceType::Containers,
                item_id: Some(LinkSegment {
                    unencoded: "ContainerRID==".to_string(),
                    encoded: "ContainerRID==".to_string(),
                }),
            },
            link
        );
        assert_eq!(
            "https://example.com/dbs/TestDB/colls/ContainerRID==",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
        assert_eq!("dbs/TestDB/colls/ContainerRID==", link.link_for_signing());
    }

    #[test]
    pub fn link_for_signing_is_unencoded() {
        // Verify that link_for_signing returns unencoded paths for proper signature generation
        let link = ResourceLink::root(ResourceType::Databases)
            .item("Test@DB")
            .feed(ResourceType::Documents)
            .item("Item@123");

        // Path should be URL-encoded
        assert_eq!("dbs/Test%40DB/docs/Item%40123", link.path());

        // link_for_signing should be unencoded
        assert_eq!("dbs/Test@DB/docs/Item@123", link.link_for_signing());

        // URL should use the encoded path
        assert_eq!(
            "https://example.com/dbs/Test%40DB/docs/Item%40123",
            link.url(&"https://example.com/".parse().unwrap())
                .to_string()
        );
    }

    #[test]
    pub fn link_segment_stores_both_forms() {
        // Verify that LinkSegment stores both encoded and unencoded forms
        let link = ResourceLink::root(ResourceType::Databases).item("SimpleDB");

        if let Some(segment) = &link.item_id {
            assert_eq!("SimpleDB", segment.unencoded());
            assert_eq!("SimpleDB", segment.encoded());
        } else {
            panic!("Expected item_id to be present");
        }

        // Verify that encoded and unencoded differ when encoding is needed
        let link_encoded = ResourceLink::root(ResourceType::Databases).item("DB With Spaces");

        if let Some(segment) = &link_encoded.item_id {
            assert_eq!("DB With Spaces", segment.unencoded());
            assert_eq!("DB+With+Spaces", segment.encoded());
        } else {
            panic!("Expected item_id to be present");
        }
    }
}
