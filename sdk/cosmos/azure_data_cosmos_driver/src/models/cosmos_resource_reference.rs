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
    /// Signing link override.
    ///
    /// Used for resources whose signing link is a lowercased RID that is
    /// unrelated to the URL path and so cannot be derived as a sub-slice of
    /// `buf`. This applies to offers and to any RID-addressed resource (where
    /// the master-key signature is computed over the lowercased leaf/parent RID
    /// only, matching `is_name_based = false` semantics in the service).
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
    /// The narrowest ancestor scope this reference is anchored to.
    scope: ResourceScope,
    /// Optional resource identifier (name or RID) for the leaf resource.
    id: Option<ResourceIdentifier>,
    /// When true, this reference targets a feed (collection of resources)
    /// rather than a single resource.
    is_feed: bool,
}

/// The narrowest ancestor a [`CosmosResourceReference`] is anchored to.
///
/// A reference is anchored to exactly one ancestor: a container-or-below
/// reference carries only the container (which already knows its database),
/// and a database-level reference carries only the database.
#[derive(Clone, Debug)]
enum ResourceScope {
    /// Account-level reference; no database or container ancestor.
    Account,
    /// Database-level reference.
    Database(DatabaseReference),
    /// Container-level or below (item, stored procedure, trigger, UDF, pkrange).
    Container(ContainerReference),
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
        match &self.scope {
            ResourceScope::Container(container) => Some(container),
            ResourceScope::Account | ResourceScope::Database(_) => None,
        }
    }

    /// Replaces the resolved metadata for the same name-addressed container.
    pub(crate) fn retarget_container(
        &mut self,
        replacement: ContainerReference,
    ) -> crate::error::Result<()> {
        let current = self.container().ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_BAD_REQUEST)
                .with_message("operation does not target a container")
                .build()
        })?;

        let same_logical_container = !current.is_by_rid()
            && !replacement.is_by_rid()
            && current.account().endpoint() == replacement.account().endpoint()
            && current.database_name() == replacement.database_name()
            && current.name() == replacement.name();
        if !same_logical_container {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_BAD_REQUEST)
                .with_message("replacement metadata does not identify the same named container")
                .build());
        }

        self.account = replacement.account().clone();
        self.scope = ResourceScope::Container(replacement);
        Ok(())
    }

    /// Returns the database reference, if this operation is anchored directly to a
    /// database (not a container-or-below resource).
    fn database(&self) -> Option<&DatabaseReference> {
        match &self.scope {
            ResourceScope::Database(database) => Some(database),
            ResourceScope::Account | ResourceScope::Container(_) => None,
        }
    }

    /// Reconstructs an [`ItemReference`] from this resource reference and the
    /// provided partition key.
    ///
    /// Returns `None` unless the reference targets a [`ResourceType::Document`]
    /// with both a container and a resource identifier — i.e. the kind of
    /// reference the patch handler can faithfully translate into an internal
    /// Read/Replace pair.
    pub(crate) fn try_into_item_reference(
        &self,
        partition_key: crate::models::PartitionKey,
    ) -> Option<ItemReference> {
        if self.resource_type != ResourceType::Document || self.is_feed {
            return None;
        }
        let container = self.container()?;
        let id = self.id.as_ref()?;
        if let Some(name) = id.name() {
            Some(ItemReference::from_name(
                container,
                partition_key,
                name.to_owned(),
            ))
        } else {
            id.rid()
                .map(|rid| ItemReference::from_rid(container, partition_key, rid.to_owned()))
        }
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
        #[cfg(debug_assertions)]
        self.debug_assert_addressing_consistent(false);

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
            signing_override: self.rid_signing_override(true),
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
        #[cfg(debug_assertions)]
        self.debug_assert_addressing_consistent(!self.is_feed);

        if self.resource_type == ResourceType::DatabaseAccount {
            return ResourcePaths::empty();
        }

        #[cfg(feature = "preview_dtx")]
        if self.resource_type == ResourceType::DistributedTransactionBatch {
            return ResourcePaths {
                buf: "/operations/dtc".to_owned(),
                signing_end: 1,
                signing_override: Some(String::new()),
            };
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
                signing_override: self.rid_signing_override(true),
            }
        } else {
            // Non-feed: request_path == signing_link (modulo the leading '/').
            let buf = self.resolved_resource_link();
            let signing_end = buf.len();
            ResourcePaths {
                buf,
                signing_end,
                signing_override: self.rid_signing_override(false),
            }
        }
    }

    // ===== Private Helpers =====

    /// Returns whether this reference's database/container parent chain is
    /// RID-addressed (`Some(true)`), name-addressed (`Some(false)`), or has no
    /// parent chain to classify (`None`, e.g. account-level references).
    fn parent_chain_is_rid(&self) -> Option<bool> {
        if let Some(container) = self.container() {
            return Some(container.is_by_rid());
        }
        if let Some(db) = self.database() {
            return Some(db.is_by_rid());
        }
        None
    }

    /// Detects mixed name/RID addressing anywhere in the request path this
    /// reference will produce. Returns a human-readable description of the
    /// conflict, or `None` when the addressing is consistent.
    ///
    /// The service classifies a request as name-based or RID-based from the
    /// `dbs` segment alone (mirroring `PathsHelper.IsNameBased` in the service
    /// SDKs): if that segment decodes as a valid RID, the *entire* URI is
    /// treated as RID-based and every remaining segment must also be a RID.
    /// There is no "RID parent, name leaf" mode. Mixing therefore fails — a
    /// mixed parent chain (e.g. `/dbs/{name}/colls/{rid}`) signs and routes
    /// inconsistently and the gateway rejects it with an opaque `401`, a name
    /// leaf under a RID-addressed parent (e.g.
    /// `/dbs/{rid}/colls/{rid}/docs/{name}`) is rejected with
    /// `400 Failed to parse the value '{name}' as ResourceId`, and a RID leaf
    /// under a name-addressed parent (e.g. `/dbs/{name}/colls/{name}/docs/{rid}`)
    /// signs the bare RID against a name-routed URL and is rejected with an
    /// opaque `401`.
    ///
    /// `leaf_in_path` selects whether the leaf `id` actually appears in the
    /// request path. Feed-style operations (including Create/Upsert, which
    /// carry an item id but POST to the parent collection URL) drop the leaf, so
    /// a name id is legal there even under a RID-addressed parent — only the
    /// parent chain is checked. Point operations put the leaf in the path, so it
    /// must match the parent chain's addressing mode in both directions.
    fn addressing_conflict(&self, leaf_in_path: bool) -> Option<String> {
        if let (Some(db), Some(container)) = (self.database(), self.container()) {
            if db.is_by_rid() != container.is_by_rid() {
                return Some(format!(
                    "database is {} but container is {}",
                    if db.is_by_rid() { "RID" } else { "name" },
                    if container.is_by_rid() { "RID" } else { "name" },
                ));
            }
        }

        // When the resource is itself a database or container addressed directly
        // by the leaf `id`, that id must match the parent chain's addressing.
        if matches!(
            self.resource_type,
            ResourceType::Database | ResourceType::DocumentCollection
        ) {
            if let (Some(id), Some(parent_is_rid)) = (self.id.as_ref(), self.parent_chain_is_rid())
            {
                if id.rid().is_some() != parent_is_rid {
                    return Some(format!(
                        "leaf id is {} but parent chain is {}",
                        if id.rid().is_some() { "RID" } else { "name" },
                        if parent_is_rid { "RID" } else { "name" },
                    ));
                }
            }
        }

        // A leaf that appears in the request path must match its parent chain's
        // addressing mode in *both* directions. The service classifies the whole
        // URI from the `dbs` segment, so:
        //   * a name leaf under a RID-addressed parent (e.g.
        //     `/dbs/{rid}/colls/{rid}/docs/{name}`) is parsed as a ResourceId and
        //     rejected with `400 Failed to parse the value '{name}'`, and
        //   * a RID leaf under a name-addressed parent (e.g.
        //     `/dbs/{name}/colls/{name}/docs/{rid}`) is routed name-based while
        //     `rid_signing_override` signs the bare RID, so the signature never
        //     matches and the gateway returns an opaque `401`.
        // Addressing the leaf the same way as its parent (`ItemReference::from_rid`
        // under a RID parent, `from_name` under a name parent) is the supported
        // way to reach it. The dedicated database/container check above already
        // covers those leaf kinds; this catches document and other sub-resource
        // leaves, whose type is not `Database`/`DocumentCollection`.
        if leaf_in_path {
            if let (Some(id), Some(parent_is_rid)) = (self.id.as_ref(), self.parent_chain_is_rid())
            {
                let leaf_is_rid = id.rid().is_some();
                if leaf_is_rid != parent_is_rid {
                    return Some(format!(
                        "{} leaf is addressed by {} but its parent chain is {}-addressed",
                        self.resource_type.as_str(),
                        if leaf_is_rid { "RID" } else { "name" },
                        if parent_is_rid { "RID" } else { "name" },
                    ));
                }
            }
        }

        None
    }

    /// Validates that this reference does not mix name and RID addressing.
    ///
    /// This is the release-mode counterpart to
    /// [`debug_assert_addressing_consistent`](Self::debug_assert_addressing_consistent):
    /// the debug assert turns a mixed reference into a loud panic during tests,
    /// while this returns a deterministic [`CLIENT_MIXED_NAME_RID_ADDRESSING`]
    /// error in every build before the request is signed and sent — converting an
    /// opaque gateway `401` (mixed parent chain) or `400 Failed to parse the
    /// value '{name}' as ResourceId` (name leaf under a RID parent) into a clear
    /// client-side failure. The driver calls this once per operation so the
    /// guarantee holds for references built through any construction path, not
    /// just the SDK's `ContainerClient`.
    ///
    /// See [`addressing_conflict`](Self::addressing_conflict) for `leaf_in_path`.
    ///
    /// [`CLIENT_MIXED_NAME_RID_ADDRESSING`]: crate::error::CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING
    pub(crate) fn validate_addressing(&self, leaf_in_path: bool) -> crate::error::Result<()> {
        if let Some(conflict) = self.addressing_conflict(leaf_in_path) {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING)
                .with_message(format!(
                    "mixed name/RID addressing is not allowed ({conflict}); the service \
                     classifies a request from its `dbs` segment, so a RID-addressed path must \
                     be RID-addressed end to end"
                ))
                .with_source(std::io::Error::other("mixed name/RID addressing"))
                .build());
        }
        Ok(())
    }

    /// Debug-only check that this reference does not mix name and RID addressing.
    ///
    /// See [`addressing_conflict`](Self::addressing_conflict) for the exact rule
    /// and the meaning of `leaf_in_path`. This guard turns such a programming
    /// error into a deterministic panic in debug/test builds;
    /// [`validate_addressing`](Self::validate_addressing) is the release-mode
    /// counterpart that returns a typed error.
    #[cfg(debug_assertions)]
    fn debug_assert_addressing_consistent(&self, leaf_in_path: bool) {
        if let Some(conflict) = self.addressing_conflict(leaf_in_path) {
            debug_assert!(false, "mixed name/RID addressing: {conflict}");
        }
    }

    /// Returns the lowercased RID to sign against when this reference is
    /// RID-addressed, or `None` when it is name-addressed (in which case the
    /// full, case-preserved resource link is used for signing).
    ///
    /// For RID-based requests the Cosmos master-key signature is computed over
    /// the lowercased RID of the **signing resource** only — the leaf for point
    /// operations and the parent for feed operations — not the full resource
    /// link. This mirrors the `is_name_based = false` path in the service SDKs.
    ///
    /// `is_feed` selects the parent (feed) vs. leaf (point op) signing resource.
    fn rid_signing_override(&self, is_feed: bool) -> Option<String> {
        if is_feed {
            // Feed/parent-signed: parent is the account (no RID), the database,
            // or the container, depending on the child resource type.
            match self.resource_type {
                ResourceType::DocumentCollection => self
                    .database()
                    .and_then(|db| db.rid())
                    .map(str::to_lowercase),
                ResourceType::Document
                | ResourceType::StoredProcedure
                | ResourceType::Trigger
                | ResourceType::UserDefinedFunction
                | ResourceType::PartitionKeyRange => self
                    .container()
                    .filter(|c| c.is_by_rid())
                    .map(|c| c.rid().to_lowercase()),
                // Database feed (list databases) signs the account: no RID.
                _ => None,
            }
        } else {
            // Point op: the leaf resource is the signing resource. The leaf RID
            // is carried either by an explicit `id` (e.g. a container or item
            // read addressed directly by RID) or, when no `id` is present, by the
            // addressed container/database itself.
            if let Some(rid) = self.id.as_ref().and_then(|id| id.rid()) {
                return Some(rid.to_lowercase());
            }
            match self.resource_type {
                ResourceType::Database => self
                    .database()
                    .and_then(|db| db.rid())
                    .map(str::to_lowercase),
                ResourceType::DocumentCollection => self
                    .container()
                    .filter(|c| c.is_by_rid())
                    .map(|c| c.rid().to_lowercase()),
                _ => None,
            }
        }
    }

    /// Returns `true` if any addressing segment of this reference is a RID, so
    /// the request path must be sent raw (without percent-encoding).
    ///
    /// Under the no-mix addressing rule a RID database implies a RID container,
    /// so checking the container and database covers database, container, and
    /// item operations. The leaf `id` is also checked to uphold the
    /// routing/signing invariant: whenever
    /// [`rid_signing_override`](Self::rid_signing_override) signs over a leaf RID,
    /// the path is also routed RID-based (raw), since a request signed as
    /// RID-based must be sent raw or the gateway rejects it with an opaque `401`.
    /// Offers are handled separately and are not considered RID-addressed for
    /// path-encoding purposes.
    ///
    /// A name leaf under a RID-addressed parent is *not* a valid combination —
    /// the service classifies the URI as RID-based from its `dbs` segment and
    /// then fails to parse the name as a ResourceId. Such references are
    /// rejected up front by
    /// [`validate_addressing`](Self::validate_addressing), so they never reach
    /// path computation.
    ///
    /// Also consumed by
    /// [`is_operation_supported_by_gateway_v2`](crate::driver::transport::is_operation_supported_by_gateway_v2)
    /// to route RID-addressed operations to standard Gateway, since Gateway 2.0
    /// derives its routing tokens by parsing the name-based signing link.
    pub(crate) fn is_rid_addressed(&self) -> bool {
        if let Some(container) = self.container() {
            if container.is_by_rid() {
                return true;
            }
        }
        if let Some(db) = self.database() {
            if db.is_by_rid() {
                return true;
            }
        }
        if self.id.as_ref().and_then(|id| id.rid()).is_some() {
            return true;
        }
        false
    }

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
            #[cfg(feature = "preview_dtx")]
            ResourceType::DistributedTransactionBatch => {
                if let Some(ref id) = self.id {
                    let id_str = Self::identifier_str(id);
                    format!("/operations/{}", id_str)
                } else {
                    "/operations".to_string()
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
            #[cfg(feature = "preview_dtx")]
            ResourceType::DistributedTransactionBatch => Cow::Borrowed(""),
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
        if let ResourceScope::Database(ref db) = self.scope {
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
    /// pre-computed path is reused without any allocation. Falls back to
    /// `Cow::Owned` for the rare cases where no container reference is available.
    fn container_link(&self) -> Cow<'_, str> {
        if let ResourceScope::Container(ref container) = self.scope {
            // Hot path: borrow the pre-computed path for the container's addressing
            // mode (name-based when named, RID-based otherwise) — no allocation.
            return Cow::Borrowed(container.base_path());
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
            scope: ResourceScope::Account,
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
            scope: ResourceScope::Database(database),
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
            scope: ResourceScope::Container(container),
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
            scope: ResourceScope::Container(container),
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
            scope: ResourceScope::Container(container),
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
            scope: ResourceScope::Container(container),
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
            scope: ResourceScope::Container(container),
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

    /// A container addressed purely by RID (no name-based path available).
    fn test_container_by_rid() -> ContainerReference {
        ContainerReference::new_by_rid(
            test_account(),
            "Lx1BAA==",
            "testcontainer",
            "Lx1BALxJyZ8=",
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

    // ===== RID-addressed signing + raw-path tests =====

    #[test]
    fn database_by_rid_signs_lowercased_rid() {
        // A RID-addressed database read signs over the lowercased database RID
        // only (not the full `/dbs/{rid}` link) and sends the path raw.
        let db = DatabaseReference::from_rid(test_account(), "Lx1BAA==");
        let r: CosmosResourceReference = db.into();
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/dbs/Lx1BAA==");
        assert_eq!(paths.signing_link(), "lx1baa==");
        // link_for_signing returns the bare lowercased RID (no leading '/').
        assert_eq!(r.link_for_signing(), "lx1baa==");
    }

    #[test]
    fn database_by_name_signs_full_link() {
        // Name-addressed databases are unchanged: full, case-preserved link and
        // a percent-encodable (non-raw) path.
        let db = DatabaseReference::from_name(test_account(), "MyDb");
        let r: CosmosResourceReference = db.into();
        let paths = r.compute_paths();
        assert_eq!(paths.signing_link(), "dbs/MyDb");
    }

    #[test]
    fn container_by_rid_signs_lowercased_rid() {
        let r: CosmosResourceReference = test_container_by_rid().into();
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/dbs/Lx1BAA==/colls/Lx1BALxJyZ8=");
        assert_eq!(paths.signing_link(), "lx1balxjyz8=");
        assert_eq!(r.link_for_signing(), "lx1balxjyz8=");
    }

    #[test]
    fn container_read_by_rid_via_id_signs_leaf_rid() {
        // A cold-cache container read is built as: database by RID + leaf `id`
        // by RID, with `container == None`. The signing resource is the leaf
        // collection RID carried by `id`.
        let db = DatabaseReference::from_rid(test_account(), "Lx1BAA==");
        let r = CosmosResourceReference::from(db)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_rid("Lx1BALxJyZ8=".into());
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/dbs/Lx1BAA==/colls/Lx1BALxJyZ8=");
        assert_eq!(paths.signing_link(), "lx1balxjyz8=");
    }

    #[test]
    fn leaf_rid_forces_raw_path_even_under_name_parent() {
        // Release-mode safety net for the raw-path/signing invariant: whenever
        // the signing override is a leaf RID (carried by `id`), the path must
        // also be sent raw. If `is_rid_addressed()` only inspected the parent,
        // this shape would be signed RID-based but routed name-encoded -> opaque
        // 401. The helpers are exercised directly (rather than via compute_paths)
        // because the debug-only consistency assert rejects this mixed shape; see
        // `mixed_name_parent_rid_leaf_panics_in_debug`.
        let db = DatabaseReference::from_name(test_account(), "testdb");
        let r = CosmosResourceReference::from(db)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_rid("Lx1BALxJyZ8=".into());
        // Signed over the lowercased leaf RID...
        assert_eq!(
            r.rid_signing_override(false),
            Some("lx1balxjyz8=".to_owned())
        );
        // ...so the path must be reported raw to match.
        assert!(r.is_rid_addressed());
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "mixed name/RID addressing")]
    fn mixed_name_parent_rid_leaf_panics_in_debug() {
        // A name-addressed database parent with a RID-addressed container leaf is
        // an invalid, mixable shape; compute_paths must fail fast in debug builds.
        let db = DatabaseReference::from_name(test_account(), "testdb");
        let r = CosmosResourceReference::from(db)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_rid("Lx1BALxJyZ8=".into());
        let _ = r.compute_paths();
    }

    #[test]
    fn validate_addressing_rejects_mixed_name_parent_rid_leaf() {
        // Release-mode guard: the same mixed shape that panics in debug must
        // return a typed CLIENT_MIXED_NAME_RID_ADDRESSING error in every build,
        // so the driver fails deterministically before signing instead of
        // emitting an opaque 401.
        let db = DatabaseReference::from_name(test_account(), "testdb");
        let r = CosmosResourceReference::from(db)
            .with_resource_type(ResourceType::DocumentCollection)
            .with_rid("Lx1BALxJyZ8=".into());
        let err = r
            .validate_addressing(true)
            .expect_err("mixed addressing must be rejected");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING
        );
    }

    #[test]
    fn validate_addressing_accepts_consistent_addressing() {
        // A fully name-addressed and a fully RID-addressed reference are both
        // consistent, so validation is a no-op (returns Ok) and the flow is
        // unchanged.
        let name_ref: CosmosResourceReference = test_container().into();
        name_ref
            .validate_addressing(true)
            .expect("name addressing is consistent");

        let rid_ref: CosmosResourceReference = test_container_by_rid().into();
        rid_ref
            .validate_addressing(true)
            .expect("RID addressing is consistent");
    }

    #[test]
    fn fully_rid_item_point_op_signs_lowercased_item_rid() {
        // The service accepts a point operation on a RID-addressed container
        // only when the leaf is also a RID. Verified against a live account:
        // GET /dbs/{dbRid}/colls/{collRid}/docs/{itemRid} signed with the
        // lowercased item RID succeeds, so lock in that exact shape.
        let item = ItemReference::from_rid(
            &test_container_by_rid(),
            PartitionKey::from("pk1"),
            "Lx1BALxJyZ8BAAAAAAAAAA==",
        );
        let r = CosmosResourceReference::from(item).with_resource_type(ResourceType::Document);
        r.validate_addressing(true)
            .expect("a RID leaf under a RID parent is consistent");

        let paths = r.compute_paths();
        assert_eq!(
            paths.request_path(),
            "/dbs/Lx1BAA==/colls/Lx1BALxJyZ8=/docs/Lx1BALxJyZ8BAAAAAAAAAA=="
        );
        assert_eq!(paths.signing_link(), "lx1balxjyz8baaaaaaaaaa==");
    }

    #[test]
    fn validate_addressing_rejects_name_leaf_under_rid_parent() {
        // The service classifies the URI as RID-based from its `dbs` segment and
        // then fails to parse the name leaf, returning
        // `400 Failed to parse the value '{name}' as ResourceId`. Reject it
        // client-side instead (confirmed against a live account).
        let item =
            ItemReference::from_name(&test_container_by_rid(), PartitionKey::from("pk1"), "doc1");
        let r = CosmosResourceReference::from(item).with_resource_type(ResourceType::Document);
        let err = r
            .validate_addressing(true)
            .expect_err("a name leaf under a RID parent must be rejected");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING
        );
    }

    #[test]
    fn validate_addressing_rejects_rid_leaf_under_name_parent() {
        // Mirror of the name-leaf-under-RID-parent case: a RID item leaf under a
        // name-addressed container routes name-based from the `dbs` segment while
        // `rid_signing_override` signs the bare item RID, so the signature never
        // matches and the gateway returns an opaque `401`. Reject it client-side
        // with the same deterministic error instead. The leaf type is `Document`,
        // so this is caught by the leaf-in-path rule rather than the
        // database/container check.
        let item = ItemReference::from_rid(
            &test_container(),
            PartitionKey::from("pk1"),
            "Lx1BALxJyZ8BAAAAAAAAAA==",
        );
        let r = CosmosResourceReference::from(item).with_resource_type(ResourceType::Document);
        let err = r
            .validate_addressing(true)
            .expect_err("a RID leaf under a name parent must be rejected");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING
        );
    }

    #[test]
    fn validate_addressing_allows_name_leaf_when_leaf_not_in_path() {
        // Create/Upsert carry an item id but POST to the parent collection URL,
        // so the name never reaches the wire and the service never tries to
        // parse it as a ResourceId. Confirmed live: create_item on a
        // RID-addressed container succeeds.
        let item =
            ItemReference::from_name(&test_container_by_rid(), PartitionKey::from("pk1"), "doc1");
        let r = CosmosResourceReference::from(item).with_resource_type(ResourceType::Document);
        r.validate_addressing(false)
            .expect("a name leaf that never reaches the path is allowed");
    }

    #[test]
    fn container_by_name_signs_full_link() {
        let r: CosmosResourceReference = test_container().into();
        let paths = r.compute_paths();
        assert_eq!(paths.signing_link(), "dbs/testdb/colls/testcontainer");
    }

    #[test]
    fn container_feed_under_rid_db_signs_parent_db_rid() {
        // Listing containers under a RID-addressed database signs the parent
        // database RID (lowercased) and sends the path raw.
        let db = DatabaseReference::from_rid(test_account(), "Lx1BAA==");
        let r = CosmosResourceReference::from(db)
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/dbs/Lx1BAA==/colls");
        assert_eq!(paths.signing_link(), "lx1baa==");
    }

    #[test]
    fn item_feed_on_rid_container_signs_parent_container_rid() {
        // Query/create items on a RID-addressed container signs the parent
        // container RID (lowercased). Covers both compute_paths (feed) and
        // compute_feed_paths (Create/Upsert).
        let feed = CosmosResourceReference::from(test_container_by_rid())
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        let paths = feed.compute_paths();
        assert_eq!(
            paths.request_path(),
            "/dbs/Lx1BAA==/colls/Lx1BALxJyZ8=/docs"
        );
        assert_eq!(paths.signing_link(), "lx1balxjyz8=");

        // Create/Upsert path: an ItemReference on a RID container.
        let item =
            ItemReference::from_name(&test_container_by_rid(), PartitionKey::from("pk1"), "doc1");
        let item_ref: CosmosResourceReference = item.into();
        let feed_paths = item_ref.compute_feed_paths();
        assert_eq!(
            feed_paths.request_path(),
            "/dbs/Lx1BAA==/colls/Lx1BALxJyZ8=/docs"
        );
        assert_eq!(feed_paths.signing_link(), "lx1balxjyz8=");
    }

    #[test]
    fn offer_signs_lowercased_rid_and_sent_raw() {
        // Offers sign the lowercased RID (`is_name_based = false`), so the
        // `/offers/{rid}` path must be routed raw like any other RID-addressed
        // resource. An offer RID can carry reserved base64 characters (`+`, `=`
        // padding); `Url::set_path` leaves those raw so the gateway accepts the
        // RID-based signature.
        let r = CosmosResourceReference::from(test_account())
            .with_resource_type(ResourceType::Offer)
            .with_rid("ABC123XYZ".into());
        let paths = r.compute_paths();
        assert_eq!(paths.request_path(), "/offers/ABC123XYZ");
        assert_eq!(paths.signing_link(), "abc123xyz");
    }
}
