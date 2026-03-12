// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory session token cache keyed by collection resource ID.

use crate::models::{resource_id::ResourceId, vector_session_token::SessionTokenValue};
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

impl SessionContainer {
    /// Creates a new, empty session container.
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(SessionContainerInner::default()),
        }
    }

    /// Returns the composite session token string for the collection identified
    /// by `collection_rid`. Each partition-key-range contributes a
    /// `<pkRangeId>:<vector>` segment, separated by commas.
    ///
    /// Returns `None` if no tokens are cached for the given RID.
    // TODO(perf): This allocates on every resolve call. Consider caching the
    // composite string or using a `Cow` to avoid allocations on the hot path.
    #[allow(dead_code)] // Used by tests; primary callers use get_or_resolve_session_token
    pub(crate) fn get_session_token(&self, collection_rid: &str) -> Option<String> {
        let guard = self.inner.read().unwrap_or_else(|e| e.into_inner());
        Self::build_composite_token(&guard, collection_rid)
    }

    /// Builds a composite session token string from the cached inner state.
    ///
    /// Segments are sorted by partition-key-range ID for deterministic output.
    fn build_composite_token(
        inner: &SessionContainerInner,
        collection_rid: &str,
    ) -> Option<String> {
        let pk_map = inner.tokens.get(collection_rid)?;
        if pk_map.is_empty() {
            return None;
        }

        let mut entries: Vec<_> = pk_map.iter().collect();
        entries.sort_by(|(a, _), (b, _)| a.cmp(b));
        let composite: Vec<String> = entries
            .iter()
            .map(|(pk_range_id, vector)| format!("{pk_range_id}:{vector}"))
            .collect();
        Some(composite.join(","))
    }

    /// Resolves a collection name path to its cached RID.
    #[allow(dead_code)] // Used by tests; primary callers use get_or_resolve_session_token
    pub(crate) fn resolve_rid(&self, collection_name_path: &str) -> Option<String> {
        let guard = self.inner.read().unwrap_or_else(|e| e.into_inner());
        guard
            .name_to_rid
            .get(collection_name_path)
            .map(|rid| rid.as_str().to_owned())
    }

    /// Attempts to resolve a session token using first the collection RID, then
    /// falling back to name-based resolution. This avoids acquiring the read
    /// lock multiple times for the common resolve path.
    pub(crate) fn get_or_resolve_session_token(
        &self,
        collection_rid: &str,
        collection_name_path: &str,
    ) -> Option<String> {
        let guard = self.inner.read().unwrap_or_else(|e| e.into_inner());

        // Try direct RID lookup
        if let Some(token) = Self::build_composite_token(&guard, collection_rid) {
            return Some(token);
        }

        // Fall back to name → RID → token
        if let Some(resolved_rid) = guard.name_to_rid.get(collection_name_path) {
            return Self::build_composite_token(&guard, resolved_rid.as_str());
        }

        None
    }

    /// Stores (or merges) a session token for a given collection.
    ///
    /// The `session_token_value` is the raw `x-ms-session-token` header value
    /// which may contain multiple comma-separated `<pkRangeId>:<vector>` segments.
    ///
    /// If `collection_name_path` is provided, the name→RID index is also updated.
    /// **RID mismatch detection**: If the name was previously mapped to a different
    /// RID, the old RID's tokens are cleared (the container was likely recreated).
    pub(crate) fn set_session_token(
        &self,
        collection_rid: &str,
        collection_name_path: Option<&str>,
        session_token_value: &str,
    ) {
        let mut guard = self.inner.write().unwrap_or_else(|e| e.into_inner());

        // RID mismatch detection: if the name pointed at a different RID, clear old.
        if let Some(name_path) = collection_name_path {
            if let Some(old_rid) = guard.name_to_rid.get(name_path) {
                if old_rid.as_str() != collection_rid {
                    let old_rid = old_rid.clone();
                    guard.tokens.remove(&old_rid);
                }
            }
            guard.name_to_rid.insert(
                name_path.to_owned(),
                ResourceId::new(collection_rid.to_owned()),
            );
        }

        let pk_map = guard
            .tokens
            .entry(ResourceId::new(collection_rid.to_owned()))
            .or_default();

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

    /// Clears all tokens for a given collection name path (name-based clearing).
    ///
    /// Used on 404/1002 (ReadSessionNotAvailable) when the request used a
    /// name-based resource link — the container may have been recreated.
    pub(crate) fn clear_by_collection_name(&self, collection_name_path: &str) {
        let mut guard = self.inner.write().unwrap_or_else(|e| e.into_inner());
        if let Some(rid) = guard.name_to_rid.remove(collection_name_path) {
            guard.tokens.remove(&rid);
        }
    }

    /// Clears all tokens for a given collection RID.
    #[allow(dead_code)] // Kept for future use (RID-based clearing).
    pub(crate) fn clear_by_collection_rid(&self, collection_rid: &str) {
        let mut guard = self.inner.write().unwrap_or_else(|e| e.into_inner());
        guard.tokens.remove(collection_rid);
        guard
            .name_to_rid
            .retain(|_, rid| rid.as_str() != collection_rid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_container_returns_none() {
        let sc = SessionContainer::new();
        assert!(sc.get_session_token("rid1").is_none());
    }

    #[test]
    fn set_and_get_single_token() {
        let sc = SessionContainer::new();
        sc.set_session_token("rid1", Some("dbs/db1/colls/c1"), "0:1#100#1=10");
        let token = sc.get_session_token("rid1").unwrap();
        assert_eq!(token, "0:1#100#1=10");
    }

    #[test]
    fn merge_updates_existing() {
        let sc = SessionContainer::new();
        sc.set_session_token("rid1", None, "0:1#100#1=10");
        sc.set_session_token("rid1", None, "0:1#200#1=20");
        let token = sc.get_session_token("rid1").unwrap();
        assert!(token.contains("200"));
    }

    #[test]
    fn compound_token_multiple_pk_ranges() {
        let sc = SessionContainer::new();
        sc.set_session_token("rid1", None, "0:1#100#1=10,1:1#200#1=20");
        let token = sc.get_session_token("rid1").unwrap();
        assert!(token.contains("0:") && token.contains("1:"));
    }

    #[test]
    fn name_to_rid_resolution() {
        let sc = SessionContainer::new();
        sc.set_session_token("rid1", Some("dbs/db1/colls/c1"), "0:1#100");
        assert_eq!(sc.resolve_rid("dbs/db1/colls/c1"), Some("rid1".to_owned()));
    }

    #[test]
    fn rid_mismatch_clears_old_tokens() {
        let sc = SessionContainer::new();
        sc.set_session_token("rid_old", Some("dbs/db1/colls/c1"), "0:1#100#1=10");
        // Same name, different RID → container recreated
        sc.set_session_token("rid_new", Some("dbs/db1/colls/c1"), "0:1#50#1=5");
        assert!(sc.get_session_token("rid_old").is_none());
        assert!(sc.get_session_token("rid_new").is_some());
    }

    #[test]
    fn clear_by_name() {
        let sc = SessionContainer::new();
        sc.set_session_token("rid1", Some("dbs/db1/colls/c1"), "0:1#100");
        sc.clear_by_collection_name("dbs/db1/colls/c1");
        assert!(sc.get_session_token("rid1").is_none());
        assert!(sc.resolve_rid("dbs/db1/colls/c1").is_none());
    }

    #[test]
    fn clear_by_rid() {
        let sc = SessionContainer::new();
        sc.set_session_token("rid1", Some("dbs/db1/colls/c1"), "0:1#100");
        sc.clear_by_collection_rid("rid1");
        assert!(sc.get_session_token("rid1").is_none());
        assert!(sc.resolve_rid("dbs/db1/colls/c1").is_none());
    }
}
