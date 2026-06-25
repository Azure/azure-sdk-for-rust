// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Public types for addressing Cosmos DB databases and containers by name or RID.

use std::borrow::Cow;

/// A Cosmos DB resource identifier (RID).
///
/// RIDs are stable, Base64-encoded identifiers assigned by Cosmos DB. Unlike a
/// user-provided name, a RID does not change when a resource is renamed, so it
/// can be used to address a database or container regardless of its current name.
///
/// Use [`ResourceId`] together with [`ResourceIdentity`] to obtain RID-addressed
/// clients via [`CosmosClient::database_client`](crate::CosmosClient::database_client)
/// and [`DatabaseClient::container_client`](crate::clients::DatabaseClient::container_client).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceId(Cow<'static, str>);

impl ResourceId {
    /// Creates a resource identifier from a static string without allocating.
    pub const fn from_static(rid: &'static str) -> Self {
        Self(Cow::Borrowed(rid))
    }

    /// Returns the RID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ResourceId {
    fn from(rid: &str) -> Self {
        Self(Cow::Owned(rid.to_owned()))
    }
}

impl From<String> for ResourceId {
    fn from(rid: String) -> Self {
        Self(Cow::Owned(rid))
    }
}

impl From<&String> for ResourceId {
    fn from(rid: &String) -> Self {
        Self(Cow::Owned(rid.clone()))
    }
}

impl AsRef<str> for ResourceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identifies a Cosmos DB database or container, either by user-provided name or
/// by [`ResourceId`] (RID).
///
/// This is the parameter type accepted by
/// [`CosmosClient::database_client`](crate::CosmosClient::database_client) and
/// [`DatabaseClient::container_client`](crate::clients::DatabaseClient::container_client).
/// Both accept `impl Into<ResourceIdentity>`, so a plain `&str`/`String` selects
/// name addressing and a [`ResourceId`] selects RID addressing:
///
/// ```rust
/// use azure_data_cosmos::{ResourceId, ResourceIdentity};
///
/// let by_name: ResourceIdentity = "my-database".into();
/// let by_rid: ResourceIdentity = ResourceId::from("abc123==").into();
/// ```
///
/// Name and RID addressing cannot be mixed across the database/container
/// hierarchy: a RID-addressed database yields only RID-addressed containers, and
/// a name-addressed database yields only name-addressed containers.
///
/// This type does not carry a lifetime parameter (per the repository's guidance
/// against lifetimes in public types); converting from a borrowed `&str`
/// allocates an owned copy.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceIdentity {
    /// Address the resource by its user-provided name.
    Name(Cow<'static, str>),
    /// Address the resource by its [`ResourceId`] (RID).
    Rid(ResourceId),
}

impl ResourceIdentity {
    /// Returns the name when this identity addresses a resource by name.
    pub(crate) fn as_name(&self) -> Option<&str> {
        match self {
            Self::Name(name) => Some(name),
            Self::Rid(_) => None,
        }
    }

    /// Returns the RID when this identity addresses a resource by RID.
    pub(crate) fn as_rid(&self) -> Option<&ResourceId> {
        match self {
            Self::Rid(rid) => Some(rid),
            Self::Name(_) => None,
        }
    }

    /// Returns `true` if this identity addresses a resource by RID.
    pub fn is_rid(&self) -> bool {
        matches!(self, Self::Rid(_))
    }
}

impl From<&str> for ResourceIdentity {
    fn from(name: &str) -> Self {
        Self::Name(Cow::Owned(name.to_owned()))
    }
}

impl From<String> for ResourceIdentity {
    fn from(name: String) -> Self {
        Self::Name(Cow::Owned(name))
    }
}

impl From<&String> for ResourceIdentity {
    fn from(name: &String) -> Self {
        Self::Name(Cow::Owned(name.clone()))
    }
}

impl From<ResourceId> for ResourceIdentity {
    fn from(rid: ResourceId) -> Self {
        Self::Rid(rid)
    }
}

impl From<&ResourceId> for ResourceIdentity {
    fn from(rid: &ResourceId) -> Self {
        Self::Rid(rid.clone())
    }
}

impl From<&ResourceIdentity> for ResourceIdentity {
    fn from(identity: &ResourceIdentity) -> Self {
        identity.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_converts_to_name() {
        let id: ResourceIdentity = "mydb".into();
        assert_eq!(id.as_name(), Some("mydb"));
        assert!(id.as_rid().is_none());
        assert!(!id.is_rid());
    }

    #[test]
    fn string_converts_to_name() {
        let id: ResourceIdentity = String::from("mydb").into();
        assert_eq!(id.as_name(), Some("mydb"));
    }

    #[test]
    fn resource_id_converts_to_rid() {
        let id: ResourceIdentity = ResourceId::from("abc123").into();
        assert!(id.is_rid());
        assert_eq!(id.as_rid().map(|r| r.as_str()), Some("abc123"));
        assert!(id.as_name().is_none());
    }

    #[test]
    fn resource_id_from_static_does_not_allocate() {
        const RID: ResourceId = ResourceId::from_static("static-rid");
        assert_eq!(RID.as_str(), "static-rid");
    }

    #[test]
    fn resource_id_display_matches_str() {
        let rid = ResourceId::from("xyz==");
        assert_eq!(rid.to_string(), "xyz==");
    }

    #[test]
    fn identity_ref_round_trips_preserving_addressing() {
        let by_name: ResourceIdentity = "mydb".into();
        let name_again: ResourceIdentity = (&by_name).into();
        assert_eq!(name_again, by_name);
        assert_eq!(name_again.as_name(), Some("mydb"));

        let by_rid: ResourceIdentity = ResourceId::from("abc123==").into();
        let rid_again: ResourceIdentity = (&by_rid).into();
        assert_eq!(rid_again, by_rid);
        assert!(rid_again.is_rid());
        assert_eq!(rid_again.as_rid().map(|r| r.as_str()), Some("abc123=="));
    }
}
