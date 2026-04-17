// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory session token cache keyed by collection resource ID.

use crate::models::{
    resource_id::ResourceId, vector_session_token::SessionTokenValue, ContainerReference,
    SessionToken,
};
use std::collections::HashMap;
use std::sync::RwLock;

use azure_core::fmt::SafeDebug;

/// An in-memory cache of session tokens for Cosmos DB containers.
///
/// Tokens are stored **per partition-key-range within each collection**, keyed
/// by the collection's resource ID (RID). A secondary name→RID index allows
/// name-based lookups and container-recreation detection.
///
/// Thread-safety is provided via [`std::sync::RwLock`] because the lock is
/// never held across `.await` points.
#[derive(SafeDebug)]
pub(crate) struct SessionContainer {
    inner: RwLock<SessionContainerInner>,
}

#[derive(Debug, Default)]
struct SessionContainerInner {
    /// `collection_rid → (pk_range_id → SessionTokenValue)`
    tokens: HashMap<ResourceId, HashMap<String, SessionTokenValue>>,
    /// `collection_name_path → collection_rid` (name path = `dbs/{db}/colls/{coll}`)
    name_to_rid: HashMap<String, ResourceId>,
}

/// Builds a `dbs/{db}/colls/{coll}` name path from a [`ContainerReference`].
fn name_path(container: &ContainerReference) -> String {
    format!(
        "dbs/{}/colls/{}",
        container.database_name(),
        container.name()
    )
}

impl SessionContainer {
    /// Creates a new, empty session container.
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(SessionContainerInner::default()),
        }
    }

    /// Builds a composite session token string from the cached inner state.
    ///
    /// Segments are sorted by partition-key-range ID for deterministic output.
    fn build_composite_token(
        inner: &SessionContainerInner,
        collection_rid: &str,
    ) -> Option<SessionToken> {
        let pk_map = inner.tokens.get(collection_rid)?;
        if pk_map.is_empty() {
            return None;
        }

        let mut entries: Vec<_> = pk_map.iter().collect();
        entries.sort_by_key(|(a, _)| *a);
        let composite: Vec<String> = entries
            .iter()
            .map(|(pk_range_id, vector)| format!("{pk_range_id}:{vector}"))
            .collect();
        Some(SessionToken::new(composite.join(",")))
    }

    /// Resolves a session token for the given container, trying first by RID
    /// then falling back to name-based resolution.
    pub(crate) fn resolve_session_token(
        &self,
        container: &ContainerReference,
    ) -> Option<SessionToken> {
        let guard = self.inner.read().unwrap_or_else(|e| e.into_inner());
        let rid = container.rid();

        // Try direct RID lookup
        if let Some(token) = Self::build_composite_token(&guard, rid) {
            return Some(token);
        }

        // Fall back to name → RID → token
        let np = name_path(container);
        if let Some(resolved_rid) = guard.name_to_rid.get(&np) {
            return Self::build_composite_token(&guard, resolved_rid.as_str());
        }

        None
    }

    /// Stores (or merges) a session token for a given container.
    ///
    /// The `session_token_value` is the raw `x-ms-session-token` header value
    /// which may contain multiple comma-separated `<pkRangeId>:<vector>` segments.
    ///
    /// The name→RID index is always updated from the container reference.
    /// **RID mismatch detection**: If the name was previously mapped to a different
    /// RID, the old RID's tokens are cleared (the container was likely recreated).
    pub(crate) fn set_session_token(
        &self,
        container: &ContainerReference,
        session_token_value: &str,
    ) {
        let collection_rid = container.rid();
        let np = name_path(container);
        let mut guard = self.inner.write().unwrap_or_else(|e| e.into_inner());

        let rid = ResourceId::new(collection_rid.to_owned());

        // RID mismatch detection: if the name pointed at a different RID, clear old.
        if let Some(old_rid) = guard.name_to_rid.get(&np) {
            if old_rid.as_str() != collection_rid {
                let old_rid = old_rid.clone();
                guard.tokens.remove(&old_rid);
            }
        }
        guard.name_to_rid.insert(np, rid.clone());

        let pk_map = guard.tokens.entry(rid).or_default();

        for segment in session_token_value.split(',') {
            let segment = segment.trim();
            if segment.is_empty() {
                continue;
            }
            if let Some((pk_range_id, token_str)) = segment.split_once(':') {
                if let Ok(new_token) = SessionTokenValue::parse(token_str) {
                    pk_map
                        .entry(pk_range_id.to_owned())
                        .and_modify(|existing| {
                            existing.merge(&new_token);
                        })
                        .or_insert(new_token);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AccountReference, ContainerProperties, PartitionKeyDefinition, SystemProperties,
    };
    use std::borrow::Cow;
    use url::Url;

    fn test_container(db_name: &str, coll_name: &str, coll_rid: &str) -> ContainerReference {
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: Cow::Owned(coll_name.to_owned()),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        ContainerReference::new(
            account,
            db_name.to_owned(),
            "db_rid1".to_owned(),
            coll_name.to_owned(),
            coll_rid.to_owned(),
            &props,
        )
    }

    #[test]
    fn empty_container_returns_none() {
        let sc = SessionContainer::new();
        let c = test_container("db1", "c1", "rid1");
        assert!(sc.resolve_session_token(&c).is_none());
    }

    #[test]
    fn set_and_get_single_token() {
        let sc = SessionContainer::new();
        let c = test_container("db1", "c1", "rid1");
        sc.set_session_token(&c, "0:1#100#1=10");
        let token = sc.resolve_session_token(&c).unwrap();
        assert_eq!(token.as_str(), "0:1#100#1=10");
    }

    #[test]
    fn merge_updates_existing() {
        let sc = SessionContainer::new();
        let c = test_container("db1", "c1", "rid1");
        sc.set_session_token(&c, "0:1#100#1=10");
        sc.set_session_token(&c, "0:1#200#1=20");
        let token = sc.resolve_session_token(&c).unwrap();
        assert!(token.as_str().contains("200"));
    }

    #[test]
    fn compound_token_multiple_pk_ranges() {
        let sc = SessionContainer::new();
        let c = test_container("db1", "c1", "rid1");
        sc.set_session_token(&c, "0:1#100#1=10,1:1#200#1=20");
        let token = sc.resolve_session_token(&c).unwrap();
        let s = token.as_str();
        assert!(s.contains("0:") && s.contains("1:"));
    }

    #[test]
    fn name_based_resolution_fallback() {
        let sc = SessionContainer::new();
        // Set token with container having rid "rid_actual"
        let c_actual = test_container("db1", "c1", "rid_actual");
        sc.set_session_token(&c_actual, "0:1#100");

        // Resolve with a container having a different rid but same name
        // should fall back through name→RID index
        let c_lookup = test_container("db1", "c1", "rid_different");
        let token = sc.resolve_session_token(&c_lookup).unwrap();
        assert_eq!(token.as_str(), "0:1#100");
    }

    #[test]
    fn rid_mismatch_clears_old_tokens() {
        let sc = SessionContainer::new();
        let c_old = test_container("db1", "c1", "rid_old");
        sc.set_session_token(&c_old, "0:1#100#1=10");

        // Same name, different RID → container recreated
        let c_new = test_container("db1", "c1", "rid_new");
        sc.set_session_token(&c_new, "0:1#50#1=5");

        // The old RID's tokens were removed, but resolving via c_old still
        // works through the name→RID fallback (name now points to rid_new).
        let old_token = sc.resolve_session_token(&c_old).unwrap();
        assert!(
            old_token.as_str().contains("50"),
            "should resolve to new container's token via name fallback"
        );
        assert!(sc.resolve_session_token(&c_new).is_some());

        // Confirm the old RID's direct tokens are truly gone by checking
        // that a container with a completely different name but rid_old has nothing.
        let c_different_name = test_container("db1", "other", "rid_old");
        assert!(sc.resolve_session_token(&c_different_name).is_none());
    }
}
