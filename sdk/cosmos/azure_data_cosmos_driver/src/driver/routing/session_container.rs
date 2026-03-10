// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Session token cache for Cosmos DB session consistency.
//!
//! The [`SessionContainer`] stores session tokens per collection, keyed by
//! collection RID (resource ID). It supports:
//! - Token resolution for read operations (merge of all partition range tokens)
//! - Token capture from response headers (merge-on-write semantics)
//! - RID mismatch detection (container recreation invalidation)
//! - Name-based and RID-based clearing

use std::collections::HashMap;
use std::sync::RwLock;

use crate::models::vector_session_token::VectorSessionToken;

/// Thread-safe session token cache for Cosmos DB session consistency.
///
/// Stores per-collection, per-partition-key-range session tokens. The cache
/// uses collection RID as the primary key and maintains a name→RID index
/// for name-based operations (e.g., clearing when a container is recreated).
///
/// # Thread Safety
///
/// Uses `std::sync::RwLock` for synchronization. The lock is never held across
/// async `.await` points, so a standard (non-async) lock is appropriate and
/// avoids the overhead of an async-aware mutex.
#[derive(Debug)]
pub(crate) struct SessionContainer {
    state: RwLock<SessionContainerState>,
}

#[derive(Debug, Default)]
struct SessionContainerState {
    /// Primary store: collection_rid → (pk_range_id → VectorSessionToken)
    rid_to_tokens: HashMap<String, HashMap<String, VectorSessionToken>>,
    /// Secondary index: collection_name_path → collection_rid
    name_to_rid: HashMap<String, String>,
}

impl SessionContainer {
    /// Creates an empty session container.
    pub(crate) fn new() -> Self {
        Self {
            state: RwLock::new(SessionContainerState::default()),
        }
    }

    /// Resolves a compound session token for all partition key ranges of a collection.
    ///
    /// Returns a comma-separated string of `pk_range_id:token` pairs, sorted by
    /// partition key range ID for deterministic output. Returns `None` if no
    /// tokens are stored for the given collection RID.
    pub(crate) fn resolve_global_session_token(&self, collection_rid: &str) -> Option<String> {
        let state = self.state.read().unwrap();
        let tokens = state.rid_to_tokens.get(collection_rid)?;
        if tokens.is_empty() {
            return None;
        }

        let mut parts: Vec<(&String, &VectorSessionToken)> = tokens.iter().collect();
        parts.sort_by_key(|(pk_range_id, _)| *pk_range_id);

        let compound = parts
            .into_iter()
            .map(|(pk_range_id, token)| format!("{pk_range_id}:{token}"))
            .collect::<Vec<_>>()
            .join(",");

        Some(compound)
    }

    /// Stores a session token received from a Cosmos DB response.
    ///
    /// The `token_str` is a compound token string (comma-separated
    /// `pk_range_id:version_vector` pairs). Each partition's token is merged
    /// with any existing token via point-wise max.
    ///
    /// If the collection name was previously mapped to a different RID (indicating
    /// the container was recreated), the old RID's tokens are cleared before
    /// storing the new token.
    pub(crate) fn set_session_token(
        &self,
        collection_name: &str,
        collection_rid: &str,
        token_str: &str,
    ) {
        let mut state = self.state.write().unwrap();

        // RID mismatch detection: if the name was previously mapped to a
        // different RID, the container was recreated — clear stale tokens.
        if let Some(old_rid) = state.name_to_rid.get(collection_name) {
            if old_rid != collection_rid {
                let old_rid = old_rid.clone();
                state.rid_to_tokens.remove(&old_rid);
            }
        }
        state
            .name_to_rid
            .insert(collection_name.to_string(), collection_rid.to_string());

        let collection_tokens = state
            .rid_to_tokens
            .entry(collection_rid.to_string())
            .or_default();

        for segment in token_str.split(',') {
            let segment = segment.trim();
            if segment.is_empty() {
                continue;
            }

            // Split on the first ':' to separate pk_range_id from the version vector.
            let Some((pk_range_id, vector_part)) = segment.split_once(':') else {
                continue;
            };

            let Ok(incoming) = vector_part.parse::<VectorSessionToken>() else {
                continue;
            };

            collection_tokens
                .entry(pk_range_id.to_string())
                .and_modify(|existing| {
                    *existing = existing.merge(&incoming);
                })
                .or_insert(incoming);
        }
    }

    /// Clears all session tokens for a collection identified by its RID.
    ///
    /// Also removes any name→RID entries that point to the cleared RID.
    pub(crate) fn clear_by_collection_rid(&self, collection_rid: &str) {
        let mut state = self.state.write().unwrap();
        state.rid_to_tokens.remove(collection_rid);
        state.name_to_rid.retain(|_, rid| rid != collection_rid);
    }

    /// Clears all session tokens for a collection identified by its name path.
    ///
    /// Looks up the corresponding RID from the name→RID index and removes
    /// entries from both maps.
    pub(crate) fn clear_by_collection_name(&self, collection_name: &str) {
        let mut state = self.state.write().unwrap();
        if let Some(rid) = state.name_to_rid.remove(collection_name) {
            state.rid_to_tokens.remove(&rid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_resolve_simple() {
        let container = SessionContainer::new();
        container.set_session_token("dbs/db1/colls/c1", "rid1", "0:1#100#1=10");

        let resolved = container
            .resolve_global_session_token("rid1")
            .expect("should resolve");
        assert_eq!(resolved, "0:1#100#1=10");
    }

    #[test]
    fn set_merges_existing() {
        let container = SessionContainer::new();
        container.set_session_token("dbs/db1/colls/c1", "rid1", "0:1#100#1=10");
        container.set_session_token("dbs/db1/colls/c1", "rid1", "0:1#50#1=20");

        let resolved = container
            .resolve_global_session_token("rid1")
            .expect("should resolve");

        // Merge takes point-wise max: version=1, global_lsn=100, region1=20
        assert_eq!(resolved, "0:1#100#1=20");
    }

    #[test]
    fn set_compound_token() {
        let container = SessionContainer::new();
        container.set_session_token("dbs/db1/colls/c1", "rid1", "0:1#100#1=20,1:1#200#2=30");

        let resolved = container
            .resolve_global_session_token("rid1")
            .expect("should resolve");
        assert_eq!(resolved, "0:1#100#1=20,1:1#200#2=30");
    }

    #[test]
    fn resolve_empty_returns_none() {
        let container = SessionContainer::new();
        assert!(container.resolve_global_session_token("rid1").is_none());
    }

    #[test]
    fn resolve_unknown_rid_returns_none() {
        let container = SessionContainer::new();
        container.set_session_token("dbs/db1/colls/c1", "rid1", "0:1#100#1=10");
        assert!(container.resolve_global_session_token("unknown").is_none());
    }

    #[test]
    fn rid_mismatch_clears_old() {
        let container = SessionContainer::new();
        container.set_session_token("dbs/db1/colls/c1", "r1", "0:1#100#1=10");
        container.set_session_token("dbs/db1/colls/c1", "r2", "0:1#200#1=20");

        // Old RID's tokens should be gone.
        assert!(container.resolve_global_session_token("r1").is_none());

        // New RID's tokens should be present.
        let resolved = container
            .resolve_global_session_token("r2")
            .expect("should resolve");
        assert_eq!(resolved, "0:1#200#1=20");
    }

    #[test]
    fn clear_by_rid() {
        let container = SessionContainer::new();
        container.set_session_token("dbs/db1/colls/c1", "rid1", "0:1#100#1=10");
        container.clear_by_collection_rid("rid1");
        assert!(container.resolve_global_session_token("rid1").is_none());
    }

    #[test]
    fn clear_by_name() {
        let container = SessionContainer::new();
        container.set_session_token("dbs/db1/colls/c1", "rid1", "0:1#100#1=10");
        container.clear_by_collection_name("dbs/db1/colls/c1");
        assert!(container.resolve_global_session_token("rid1").is_none());
    }
}
