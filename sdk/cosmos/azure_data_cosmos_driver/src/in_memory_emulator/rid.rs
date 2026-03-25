// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Hierarchical resource ID (RID) generation and encoding.
//!
//! Cosmos DB resource IDs follow a hierarchical binary encoding where each child
//! resource encodes its parent's RID. This module implements RID generation matching
//! the [Java SDK `ResourceId`](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/implementation/ResourceId.java).

use base64::Engine;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Generates hierarchical resource IDs with monotonically increasing counters.
pub(crate) struct RidGenerator {
    db_counter: AtomicU32,
    coll_counter: AtomicU32,
    doc_counter: AtomicU64,
}

impl RidGenerator {
    pub fn new() -> Self {
        Self {
            db_counter: AtomicU32::new(1),
            coll_counter: AtomicU32::new(1),
            doc_counter: AtomicU64::new(1),
        }
    }

    /// Generates a new database RID (4 bytes, big-endian).
    pub fn next_database_rid(&self) -> (u32, String) {
        let id = self.db_counter.fetch_add(1, Ordering::SeqCst);
        let bytes = id.to_be_bytes();
        (id, encode_rid(&bytes))
    }

    /// Generates a new collection RID (8 bytes: db_id BE + coll_id BE with high bit set).
    pub fn next_collection_rid(&self, db_id: u32) -> (u32, String) {
        let coll_id = self.coll_counter.fetch_add(1, Ordering::SeqCst);
        let coll_with_high_bit = coll_id | 0x80000000;
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&db_id.to_be_bytes());
        bytes[4..8].copy_from_slice(&coll_with_high_bit.to_be_bytes());
        (coll_id, encode_rid(&bytes))
    }

    /// Generates a new document RID (16 bytes: db_id + coll_id + doc_id with type nibble 0x0).
    pub fn next_document_rid(&self, db_id: u32, coll_id: u32) -> (u64, String) {
        let doc_id = self.doc_counter.fetch_add(1, Ordering::SeqCst);
        let doc_with_type = doc_id << 4; // type nibble 0x0 in the low nibble
        let coll_with_high_bit = coll_id | 0x80000000;
        let mut bytes = [0u8; 16];
        bytes[..4].copy_from_slice(&db_id.to_be_bytes());
        bytes[4..8].copy_from_slice(&coll_with_high_bit.to_be_bytes());
        bytes[8..16].copy_from_slice(&doc_with_type.to_be_bytes());
        (doc_id, encode_rid(&bytes))
    }

    /// Generates a new partition key range RID (16 bytes: db_id + coll_id + pkr_id with type nibble 0x5).
    pub fn next_pkrange_rid(&self, db_id: u32, coll_id: u32, pkrange_id: u32) -> String {
        let pkr_id = (pkrange_id as u64) << 4 | 0x05; // type nibble 0x5
        let coll_with_high_bit = coll_id | 0x80000000;
        let mut bytes = [0u8; 16];
        bytes[..4].copy_from_slice(&db_id.to_be_bytes());
        bytes[4..8].copy_from_slice(&coll_with_high_bit.to_be_bytes());
        bytes[8..16].copy_from_slice(&pkr_id.to_be_bytes());
        encode_rid(&bytes)
    }
}

/// Encodes a binary RID as base64 with `/` replaced by `-`.
fn encode_rid(bytes: &[u8]) -> String {
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    encoded.replace('/', "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_rid_4_bytes() {
        let gen = RidGenerator::new();
        let (id, rid) = gen.next_database_rid();
        assert_eq!(id, 1);
        // Decode and verify 4 bytes
        let decoded_str = rid.replace('-', "/");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&decoded_str)
            .unwrap();
        assert_eq!(bytes.len(), 4);
        let db_id = u32::from_be_bytes(bytes[..4].try_into().unwrap());
        assert_eq!(db_id, 1);
    }

    #[test]
    fn collection_rid_encodes_parent() {
        let gen = RidGenerator::new();
        let (db_id, _db_rid) = gen.next_database_rid();
        let (coll_id, coll_rid) = gen.next_collection_rid(db_id);
        assert_eq!(coll_id, 1);

        let decoded_str = coll_rid.replace('-', "/");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&decoded_str)
            .unwrap();
        assert_eq!(bytes.len(), 8);

        // Verify parent db_id
        let parent = u32::from_be_bytes(bytes[..4].try_into().unwrap());
        assert_eq!(parent, db_id);

        // Verify high bit set on coll_id
        let coll = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        assert!(coll & 0x80000000 != 0);
        assert_eq!(coll & 0x7FFFFFFF, coll_id);
    }

    #[test]
    fn document_rid_encodes_hierarchy() {
        let gen = RidGenerator::new();
        let (db_id, _) = gen.next_database_rid();
        let (coll_id, _) = gen.next_collection_rid(db_id);
        let (doc_id, doc_rid) = gen.next_document_rid(db_id, coll_id);
        assert_eq!(doc_id, 1);

        let decoded_str = doc_rid.replace('-', "/");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&decoded_str)
            .unwrap();
        assert_eq!(bytes.len(), 16);

        let parent_db = u32::from_be_bytes(bytes[..4].try_into().unwrap());
        assert_eq!(parent_db, db_id);

        // Type nibble should be 0x0 for documents
        let doc_raw = u64::from_be_bytes(bytes[8..16].try_into().unwrap());
        assert_eq!(doc_raw & 0x0F, 0x00);
    }

    #[test]
    fn pkrange_rid_type_nibble() {
        let gen = RidGenerator::new();
        let (db_id, _) = gen.next_database_rid();
        let (coll_id, _) = gen.next_collection_rid(db_id);
        let pkr_rid = gen.next_pkrange_rid(db_id, coll_id, 0);

        let decoded_str = pkr_rid.replace('-', "/");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&decoded_str)
            .unwrap();
        assert_eq!(bytes.len(), 16);

        let pkr_raw = u64::from_be_bytes(bytes[8..16].try_into().unwrap());
        assert_eq!(pkr_raw & 0x0F, 0x05);
    }

    #[test]
    fn monotonic_ids() {
        let gen = RidGenerator::new();
        let (id1, _) = gen.next_database_rid();
        let (id2, _) = gen.next_database_rid();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
}
