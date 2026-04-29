// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A generic reference to any Cosmos DB resource, used by [`CosmosOperation`](crate::models::CosmosOperation).
//!
//! `CosmosResourceReference` unifies account, database, container, item, stored
//! procedure, trigger, and UDF references into a single type that carries enough
//! information to compute **resource links** (for authorization signing) and
//! **request paths** (for URL construction).

use crate::models::{
    resource_id::{ResourceId, ResourceIdentifier, ResourceName},
    AccountReference, ContainerReference, DatabaseReference, ItemReference, ResourceType,
    StoredProcedureReference, TriggerReference, UdfReference,
};

use std::borrow::Cow;

/// Pre-computed request path and signing-link for a single Cosmos DB request.
///
/// Obtained via [`CosmosResourceReference::compute_paths`]. Both values are derived
/// from a single `String` allocation so that `request_path` and `signing_link`
/// are zero-copy sub-slices wherever possible.
///
/// # Layout of `buf`
///
/// | Case | `buf` | `signing_link` |
/// |------|-------|----------------|
/// | Non-feed op | `/dbs/x/colls/y/docs/z` | `buf[1..]` (`signing_end == buf.len()`) |
/// | Feed op | `/dbs/x/colls/y/docs` | `buf[1..signing_end]` (parent prefix) |
/// | Account | `""` | `""` (empty) |
/// | Offer | `/offers/{rid}` | lowercase RID (`signing_override`) |
pub(crate) struct ResourcePaths {
    /// The full request path (may have a leading `/`).
    buf: String,
    /// Byte index in `buf` where the signing link ends (exclusive).
    ///
    /// For non-feed:  `buf.len()` → signing link = `buf[1..]`
    /// For feed:      `parent.len()` → signing link = `buf[1..signing_end]`
    /// Always `>= 1` when `buf` is non-empty (skips the leading `/`).
    signing_end: usize,
    /// Signing link override for offer resources.
    ///
    /// Offers use a lowercased RID that is unrelated to the URL path, so it
    /// cannot be derived as a sub-slice of `buf`.
    signing_override: Option<String>,
}

impl ResourcePaths {
    fn empty() -> Self {
        Self {
            buf: String::new(),
            signing_end: 0,
            signing_override: None,
        }
    }

    /// Returns the request path (used to set the URL path).
    pub(crate) fn request_path(&self) -> &str {
        &self.buf
    }

    /// Returns the signing link (path without the leading `/`, used for auth).
    pub(crate) fn signing_link(&self) -> &str {
        if let Some(ref s) = self.signing_override {
            return s;
        }
        if self.buf.is_empty() {
            return "";
        }
        &self.buf[1..self.signing_end]
    }
}

/// A generic reference to any Cosmos DB resource.
///
/// Used internally by [`CosmosOperation`](crate::models::CosmosOperation) to capture the
/// resource-routing information needed before the operation is converted into
/// an HTTP request.
///
/// Instances are created via `From` conversions from concrete reference types
/// (e.g., `DatabaseReference`, `ContainerReference`, `ItemReference`) or via
/// builder-style methods (`with_resource_type`, `with_name`, `with_rid`,
/// `into_feed_reference`).
#[derive(Clone, Debug)]
pub struct CosmosResourceReference {
    /// The type of resource being referenced.
    resource_type: ResourceType,
    /// The parent account.
    account: AccountReference,
    /// Optional database reference (present for database-level and below).
    database: Option<DatabaseReference>,
    /// Optional container reference (present for container-level and below).
    container: Option<ContainerReference>,
    /// Optional resource identifier (name or RID) for the leaf resource.
    id: Option<ResourceIdentifier>,
    /// When true, this reference targets a feed (collection of resources)
    /// rather than a single resource.
    is_feed: bool,
}

impl std::fmt::Display for CosmosResourceReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}('{}')", self.resource_type, self.request_path())
    }
}

impl CosmosResourceReference {
    /// Returns the resource type.
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    /// Returns a reference to the parent account.
    pub fn account(&self) -> &AccountReference {
        &self.account
    }

    /// Returns the container reference, if this operation targets a container-level
    /// or child resource.
    pub fn container(&self) -> Option<&ContainerReference> {
        self.container.as_ref()
    }

    /// Sets a name-based identifier on this reference.
    pub fn with_name(mut self, name: Cow<'static, str>) -> Self {
        self.id = Some(ResourceIdentifier::by_name(ResourceName::new(name)));
        self.is_feed = false;
        self
    }

    /// Sets a RID-based identifier on this reference.
    pub fn with_rid(mut self, rid: Cow<'static, str>) -> Self {
        self.id = Some(ResourceIdentifier::by_rid(ResourceId::new(rid)));
        self.is_feed = false;
        self
    }

    /// Overrides the resource type.
    ///
    /// Used when building feed references that target a child resource type
    /// relative to the current reference. For example, starting from a
    /// `DatabaseReference` (which is `ResourceType::Database`) and overriding
    /// to `ResourceType::DocumentCollection` for a "list containers" operation.
    pub fn with_resource_type(mut self, resource_type: ResourceType) -> Self {
        self.resource_type = resource_type;
        self
    }

    /// Marks this reference as targeting a feed (collection of resources).
    ///
    /// Feed references drop the leaf identifier so that the resource link
    /// points to the parent, and the request path includes the child resource
    /// type's path segment.
    pub fn into_feed_reference(mut self) -> Self {
        self.id = None;
        self.is_feed = true;
        self
    }

    /// Computes paths treating this reference as a feed operation.
    ///
    /// Used by Create and Upsert which carry an [`ItemReference`] (with an
    /// item id) but still POST to the parent (collection) URL and sign
    /// against the parent resource.
    pub(crate) fn compute_feed_paths(&self) -> ResourcePaths {
        // Temporarily treat the reference as a feed for path computation.
        let parent = self.parent_link_cow();
        let segment = self.resource_type.path_segment();
        let buf = if parent.is_empty() {
            format!("/{}", segment)
        } else {
            format!("{}/{}", parent, segment)
        };
        let signing_end = if parent.is_empty() { 1 } else { parent.len() };
        ResourcePaths {
            buf,
            signing_end,
            signing_override: None,
        }
    }

    /// Returns the resource link used for authorization signing.
    ///
    /// For feed operations this is the **parent** resource's link (because
    /// Cosmos DB signs against the parent when listing children). For single-
    /// resource operations it is the full resource link.
    ///
    /// The returned string includes the leading `/` for all non-empty, non-offer
    /// paths. Use `compute_paths` on the hot path to avoid
    /// an extra allocation.
    pub fn link_for_signing(&self) -> String {
        let paths = self.compute_paths();
        if paths.signing_override.is_some() {
            // Offers: signing link is a lowercase RID, no leading '/'.
            return paths.signing_link().to_owned();
        }
        let link = paths.signing_link();
        if link.is_empty() {
            String::new()
        } else {
            format!("/{}", link)
        }
    }

    /// Returns the URL request path for this reference.
    ///
    /// For feed operations this appends the child resource type's path segment
    /// to the parent link. For single-resource operations it is the same as
    /// the resolved resource link.
    pub fn request_path(&self) -> String {
        self.compute_paths().request_path().to_owned()
    }

    /// Computes the request path and signing link in a single pass.
    ///
    /// Both values are derived from one `String` allocation: `signing_link` is
    /// a sub-slice of `request_path` in all common cases (non-offer, non-empty).
    /// Use this in performance-sensitive code instead of calling
    /// [`link_for_signing`](Self::link_for_signing) and
    /// [`request_path`](Self::request_path) separately.
    pub(crate) fn compute_paths(&self) -> ResourcePaths {
        if self.resource_type == ResourceType::DatabaseAccount {
            return ResourcePaths::empty();
        }

        if self.resource_type == ResourceType::Offer {
            // Offers use a lowercase RID as the signing link, unrelated to the URL path.
            let (buf, signing_override) = if let Some(ref id) = self.id {
                let id_str = Self::identifier_str(id);
                (format!("/offers/{}", id_str), Some(id_str.to_lowercase()))
            } else {
                ("/offers".to_owned(), None)
            };
            return ResourcePaths {
                buf,
                signing_end: 1,
                signing_override,
            };
        }

        if self.is_feed {
            // Feed: request_path = parent_link + "/" + segment
            //       signing_link  = parent_link (without leading '/')
            let parent = self.parent_link_cow();
            let segment = self.resource_type.path_segment();
            let buf = if parent.is_empty() {
                // Account-level feed (e.g., list databases): just "/dbs".
                format!("/{}", segment)
            } else {
                format!("{}/{}", parent, segment)
            };
            // signing_end marks the boundary between parent and "/{segment}" suffix.
            let signing_end = if parent.is_empty() { 1 } else { parent.len() };
            ResourcePaths {
                buf,
                signing_end,
                signing_override: None,
            }
        } else {
            // Non-feed: request_path == signing_link (modulo the leading '/').
            let buf = self.resolved_resource_link();
            let signing_end = buf.len();
            ResourcePaths {
                buf,
                signing_end,
                signing_override: None,
            }
        }
    }

    // ===== Private Helpers =====

    /// Computes the full resource link for the leaf resource.
    ///
    /// This combines the parent chain (account -> database -> container) with
    /// the leaf identifier and resource type path segment.
    fn resolved_resource_link(&self) -> String {
        match self.resource_type {
            ResourceType::DatabaseAccount => {
                // Account-level: empty link.
                String::new()
            }
            ResourceType::Database => {
                // /dbs/{id}
                self.db_link()
            }
            ResourceType::DocumentCollection => {
                // /dbs/{db}/colls/{id}
                self.container_link().into_owned()
            }
            ResourceType::Document
            | ResourceType::StoredProcedure
            | ResourceType::Trigger
            | ResourceType::UserDefinedFunction
            | ResourceType::PartitionKeyRange => {
                // /dbs/{db}/colls/{container}/{segment}/{id}
                let container_path = self.container_link();
                let segment = self.resource_type.path_segment();
                if let Some(ref id) = self.id {
                    let id_str = Self::identifier_str(id);
                    format!("{}/{}/{}", container_path, segment, id_str)
                } else {
                    // Feed path for child resources -- should not happen
                    // for resolved links, but provide a reasonable fallback.
                    format!("{}/{}", container_path, segment)
                }
            }
            ResourceType::Offer => {
                // Offers are top-level, addressed by RID.
                if let Some(ref id) = self.id {
                    let id_str = Self::identifier_str(id);
                    format!("/offers/{}", id_str)
                } else {
                    "/offers".to_string()
                }
            }
        }
    }

    /// Returns the parent resource link for feed operations (with leading `/`).
    ///
    /// Returns a borrowed `&str` when the container path is pre-computed (the
    /// common case), avoiding an allocation on the hot path.
    fn parent_link_cow(&self) -> Cow<'_, str> {
        match self.resource_type {
            ResourceType::DatabaseAccount | ResourceType::Database | ResourceType::Offer => {
                // Parent is the account — empty link.
                Cow::Borrowed("")
            }
            ResourceType::DocumentCollection => {
                // Parent is the database.
                Cow::Owned(self.db_link())
            }
            ResourceType::Document
            | ResourceType::StoredProcedure
            | ResourceType::Trigger
            | ResourceType::UserDefinedFunction
            | ResourceType::PartitionKeyRange => {
                // Parent is the container.
                self.container_link()
            }
        }
    }

    /// Builds the database portion of the link from the database reference.
    fn db_link(&self) -> String {
        if let Some(ref db) = self.database {
            if let Some(name) = db.name() {
                return format!("/dbs/{}", name);
            }
            if let Some(rid) = db.rid() {
                return format!("/dbs/{}", rid);
            }
        }
        // Fallback: use the leaf id if no database reference is set.
        if let Some(ref id) = self.id {
            let id_str = Self::identifier_str(id);
            return format!("/dbs/{}", id_str);
        }
        String::new()
    }

    /// Returns the container path.
    ///
    /// Returns `Cow::Borrowed` when a `ContainerReference` is present so that the
    /// pre-computed `Arc<str>` path is reused without any allocation. Falls back to
    /// `Cow::Owned` for the rare cases where no container reference is available.
    fn container_link(&self) -> Cow<'_, str> {
        if let Some(ref container) = self.container {
            // Hot path: borrow the pre-computed Arc<str> — no allocation.
            return Cow::Borrowed(container.name_based_path());
        }
        // If we have a database but no container, try using the leaf id.
        if let Some(ref id) = self.id {
            let db = self.db_link();
            let id_str = Self::identifier_str(id);
            return Cow::Owned(format!("{}/colls/{}", db, id_str));
        }
        Cow::Owned(self.db_link())
    }

    /// Extracts a string representation from a `ResourceIdentifier`.
    fn identifier_str(id: &ResourceIdentifier) -> &str {
        if let Some(name) = id.name() {
            name
        } else {
            id.rid().unwrap_or_default()
        }
    }
}

// =============================================================================
// From Implementations
// =============================================================================

impl From<AccountReference> for CosmosResourceReference {
    fn from(account: AccountReference) -> Self {
        Self {
            resource_type: ResourceType::DatabaseAccount,
            account,
            database: None,
            container: None,
            id: None,
            is_feed: false,
        }
    }
}

impl From<DatabaseReference> for CosmosResourceReference {
    fn from(database: DatabaseReference) -> Self {
        let account = database.account().clone();
        Self {
            resource_type: ResourceType::Database,
            account,
            database: Some(database),
            container: None,
            id: None,
            is_feed: false,
        }
    }
}

impl From<ContainerReference> for CosmosResourceReference {
    fn from(container: ContainerReference) -> Self {
        let account = container.account().clone();
        Self {
            resource_type: ResourceType::DocumentCollection,
            account,
            database: None,
            container: Some(container),
            id: None,
            is_feed: false,
        }
    }
}

impl From<ItemReference> for CosmosResourceReference {
    fn from(item: ItemReference) -> Self {
        let account = item.account().clone();
        let container = item.container().clone();
        let id = if let Some(name) = item.name() {
            Some(ResourceIdentifier::by_name(ResourceName::new(
                name.to_owned(),
            )))
        } else {
            item.rid()
                .map(|rid| ResourceIdentifier::by_rid(ResourceId::new(rid.to_owned())))
        };
        Self {
            resource_type: ResourceType::Document,
            account,
            database: None,
            container: Some(container),
            id,
            is_feed: false,
        }
    }
}

impl From<StoredProcedureReference> for CosmosResourceReference {
    fn from(sp: StoredProcedureReference) -> Self {
        let account = sp.account().clone();
        let container = sp.container().clone();
        let id = if let Some(name) = sp.name() {
            Some(ResourceIdentifier::by_name(ResourceName::new(
                name.to_owned(),
            )))
        } else {
            sp.rid()
                .map(|rid| ResourceIdentifier::by_rid(ResourceId::new(rid.to_owned())))
        };
        Self {
            resource_type: ResourceType::StoredProcedure,
            account,
            database: None,
            container: Some(container),
            id,
            is_feed: false,
        }
    }
}

impl From<TriggerReference> for CosmosResourceReference {
    fn from(trigger: TriggerReference) -> Self {
        let account = trigger.account().clone();
        let container = trigger.container().clone();
        let id = if let Some(name) = trigger.name() {
            Some(ResourceIdentifier::by_name(ResourceName::new(
                name.to_owned(),
            )))
        } else {
            trigger
                .rid()
                .map(|rid| ResourceIdentifier::by_rid(ResourceId::new(rid.to_owned())))
        };
        Self {
            resource_type: ResourceType::Trigger,
            account,
            database: None,
            container: Some(container),
            id,
            is_feed: false,
        }
    }
}

impl From<UdfReference> for CosmosResourceReference {
    fn from(udf: UdfReference) -> Self {
        let account = udf.account().clone();
        let container = udf.container().clone();
        let id = if let Some(name) = udf.name() {
            Some(ResourceIdentifier::by_name(ResourceName::new(
                name.to_owned(),
            )))
        } else {
            udf.rid()
                .map(|rid| ResourceIdentifier::by_rid(ResourceId::new(rid.to_owned())))
        };
        Self {
            resource_type: ResourceType::UserDefinedFunction,
            account,
            database: None,
            container: Some(container),
            id,
            is_feed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ContainerProperties, PartitionKey, PartitionKeyDefinition};
    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    fn test_partition_key_definition() -> PartitionKeyDefinition {
        serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap()
    }

    fn test_container_props() -> ContainerProperties {
        ContainerProperties {
            id: "testcontainer".into(),
            partition_key: test_partition_key_definition(),
            system_properties: Default::default(),
        }
    }

    fn test_container() -> ContainerReference {
        ContainerReference::new(
            test_account(),
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &test_container_props(),
        )
    }

    #[test]
    fn from_account_reference() {
        let account = test_account();
        let r: CosmosResourceReference = account.into();
        assert_eq!(r.resource_type(), ResourceType::DatabaseAccount);
        assert!(r.container().is_none());
        assert_eq!(r.link_for_signing(), "");
        assert_eq!(r.request_path(), "");
    }

    #[test]
    fn from_database_reference() {
        let db = DatabaseReference::from_name(test_account(), "mydb");
        let r: CosmosResourceReference = db.into();
        assert_eq!(r.resource_type(), ResourceType::Database);
        assert!(r.container().is_none());
        assert_eq!(r.link_for_signing(), "/dbs/mydb");
    }

    #[test]
    fn database_feed_reference() {
        let account = test_account();
        let r: CosmosResourceReference = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        assert_eq!(r.resource_type(), ResourceType::Database);
        // Signing link for database feed is the account (empty).
        assert_eq!(r.link_for_signing(), "");
        // Request path for database feed is /dbs.
        assert_eq!(r.request_path(), "/dbs");
    }

    #[test]
    fn container_feed_reference() {
        let db = DatabaseReference::from_name(test_account(), "mydb");
        let r: CosmosResourceReference = CosmosResourceReference::from(db)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        // Signing link for container feed is the parent database.
        assert_eq!(r.link_for_signing(), "/dbs/mydb");
        // Request path includes the colls segment.
        assert_eq!(r.request_path(), "/dbs/mydb/colls");
    }

    #[test]
    fn from_container_reference() {
        let r: CosmosResourceReference = test_container().into();
        assert_eq!(r.resource_type(), ResourceType::DocumentCollection);
        assert!(r.container().is_some());
        assert_eq!(r.link_for_signing(), "/dbs/testdb/colls/testcontainer");
    }

    #[test]
    fn item_feed_reference() {
        let container = test_container();
        let r: CosmosResourceReference = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        // Signing link for item feed is the container.
        assert_eq!(r.link_for_signing(), "/dbs/testdb/colls/testcontainer");
        // Request path includes docs segment.
        assert_eq!(r.request_path(), "/dbs/testdb/colls/testcontainer/docs");
    }

    #[test]
    fn from_item_reference() {
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let r: CosmosResourceReference = item.into();
        assert_eq!(r.resource_type(), ResourceType::Document);
        assert!(r.container().is_some());
        assert_eq!(
            r.link_for_signing(),
            "/dbs/testdb/colls/testcontainer/docs/doc1"
        );
    }

    #[test]
    fn read_container_by_name_reference() {
        let db = DatabaseReference::from_name(test_account(), "mydb");
        let r = CosmosResourceReference::from(db)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_name("mycontainer".into());
        assert_eq!(r.resource_type(), ResourceType::DocumentCollection);
        assert_eq!(r.link_for_signing(), "/dbs/mydb/colls/mycontainer");
    }

    #[test]
    fn from_stored_procedure_reference() {
        let sp = StoredProcedureReference::from_name(&test_container(), "mysproc");
        let r: CosmosResourceReference = sp.into();
        assert_eq!(r.resource_type(), ResourceType::StoredProcedure);
        assert_eq!(
            r.link_for_signing(),
            "/dbs/testdb/colls/testcontainer/sprocs/mysproc"
        );
    }

    #[test]
    fn from_trigger_reference() {
        let trigger = TriggerReference::from_name(&test_container(), "mytrigger");
        let r: CosmosResourceReference = trigger.into();
        assert_eq!(r.resource_type(), ResourceType::Trigger);
        assert_eq!(
            r.link_for_signing(),
            "/dbs/testdb/colls/testcontainer/triggers/mytrigger"
        );
    }

    #[test]
    fn from_udf_reference() {
        let udf = UdfReference::from_name(&test_container(), "myudf");
        let r: CosmosResourceReference = udf.into();
        assert_eq!(r.resource_type(), ResourceType::UserDefinedFunction);
        assert_eq!(
            r.link_for_signing(),
            "/dbs/testdb/colls/testcontainer/udfs/myudf"
        );
    }

    // ===== compute_paths tests =====

    /// Helper: assert that compute_paths() produces the same values as the
    /// separate link_for_signing() / request_path() APIs, and additionally
    /// verify the signing_link() is a sub-slice of the request_path buffer
    /// where applicable (not for offers / empty paths).
    fn assert_compute_paths_consistent(r: &CosmosResourceReference) {
        let paths = r.compute_paths();
        // The signing link produced by compute_paths (without leading '/').
        let computed_signing = paths.signing_link().to_owned();
        // The signing link from the legacy API (with leading '/'), trimmed.
        let legacy_signing = r.link_for_signing();
        let legacy_signing_trimmed = legacy_signing.trim_start_matches('/');

        assert_eq!(
            computed_signing, legacy_signing_trimmed,
            "compute_paths signing_link mismatch"
        );
        assert_eq!(
            paths.request_path(),
            r.request_path(),
            "compute_paths request_path mismatch"
        );

        // Verify signing_link is a zero-copy sub-slice of the request_path buffer
        // when it's derived from the same allocation (no signing_override, non-empty).
        if paths.signing_override.is_none() && !paths.request_path().is_empty() {
            let req_ptr = paths.request_path().as_ptr() as usize;
            let req_end = req_ptr + paths.request_path().len();
            let sign_ptr = paths.signing_link().as_ptr() as usize;
            let sign_end = sign_ptr + paths.signing_link().len();
            assert!(
                sign_ptr >= req_ptr && sign_end <= req_end,
                "signing_link is not a sub-slice of request_path buffer \
                (signing_link ptr={:#x} len={}, request_path ptr={:#x} len={})",
                sign_ptr,
                paths.signing_link().len(),
                req_ptr,
                paths.request_path().len()
            );
        }
    }

    #[test]
    fn compute_paths_account() {
        let r: CosmosResourceReference = test_account().into();
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "");
        assert_eq!(paths.signing_link(), "");
        assert_compute_paths_consistent(&r);
    }

    #[test]
    fn compute_paths_database_feed() {
        let account = test_account();
        let r = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Database)
            .into_feed_reference();
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/dbs");
        assert_eq!(paths.signing_link(), "");
        assert_compute_paths_consistent(&r);
    }

    #[test]
    fn compute_paths_container_feed() {
        let db = DatabaseReference::from_name(test_account(), "mydb");
        let r = CosmosResourceReference::from(db)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/dbs/mydb/colls");
        assert_eq!(paths.signing_link(), "dbs/mydb");
        // signing_link is a sub-slice of request_path (no separate allocation).
        assert!(paths.request_path().starts_with('/'));
        assert_compute_paths_consistent(&r);
    }

    #[test]
    fn compute_paths_item_point_op() {
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let r: CosmosResourceReference = item.into();
        let paths = r.compute_paths();
        let expected = "/dbs/testdb/colls/testcontainer/docs/doc1";
        assert_eq!(paths.request_path(), expected);
        // signing_link is the same path without the leading '/'.
        assert_eq!(paths.signing_link(), &expected[1..]);
        assert_compute_paths_consistent(&r);
    }

    #[test]
    fn compute_paths_item_feed() {
        let r = CosmosResourceReference::from(test_container())
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/dbs/testdb/colls/testcontainer/docs");
        assert_eq!(paths.signing_link(), "dbs/testdb/colls/testcontainer");
        assert_compute_paths_consistent(&r);
    }

    #[test]
    fn compute_feed_paths_item_reference() {
        // An ItemReference carries the document id, but compute_feed_paths
        // must produce the same feed-style paths as compute_paths on a
        // feed reference (without the item id in the URL).
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let r: CosmosResourceReference = item.into();

        let feed_paths = r.compute_feed_paths();
        assert_eq!(
            feed_paths.request_path(),
            "/dbs/testdb/colls/testcontainer/docs",
            "request path should target the collection feed, not the individual document"
        );
        assert_eq!(
            feed_paths.signing_link(),
            "dbs/testdb/colls/testcontainer",
            "signing link should be the parent container path"
        );

        // Verify consistency with compute_paths on an equivalent feed reference.
        let feed_ref = CosmosResourceReference::from(test_container())
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        let expected = feed_ref.compute_paths();
        assert_eq!(feed_paths.request_path(), expected.request_path());
        assert_eq!(feed_paths.signing_link(), expected.signing_link());
    }

    #[test]
    fn compute_paths_offer_uses_signing_override() {
        let account = test_account();
        let r = CosmosResourceReference::from(account)
            .with_resource_type(ResourceType::Offer)
            .with_rid("ABC123XYZ".into());
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/offers/ABC123XYZ");
        // Offers: signing link is the lowercase RID (not a sub-slice of buf).
        assert_eq!(paths.signing_link(), "abc123xyz");
        assert!(paths.signing_override.is_some());
    }
}
