// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{ChangeFeedItem, ChangeFeedOperationType, LogicalSequenceNumber};
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

#[derive(Debug, PartialEq, Deserialize)]
struct Document {
    id: String,
    value: u64,
}

#[test]
fn latest_version_partial_metadata_and_flat_documents() {
    let event: ChangeFeedItem<Document> = serde_json::from_value(json!({
        "current": {"id": "a", "value": 1},
        "metadata": {"lsn": 7, "crts": 12}
    }))
    .unwrap();
    assert_eq!(
        event.current(),
        Some(&Document {
            id: "a".into(),
            value: 1
        })
    );
    assert!(event.previous().is_none());
    let metadata = event.metadata().unwrap();
    assert_eq!(metadata.lsn(), Some(LogicalSequenceNumber::from(7)));
    assert_eq!(
        metadata.conflict_resolution_timestamp(),
        Some(Duration::from_secs(12))
    );
    assert!(metadata.operation_type().is_none());
    for reserved in ["current", "previous", "metadata"] {
        let flat: ChangeFeedItem<Document> = serde_json::from_value(json!({
            "id": "flat", "value": 2, reserved: {"extra": "application field"}
        }))
        .unwrap();
        assert_eq!(
            flat.current(),
            Some(&Document {
                id: "flat".into(),
                value: 2
            })
        );
        assert!(flat.metadata().is_none());
        assert!(flat.previous().is_none());
    }
}

#[test]
fn avad_preserves_preimage_and_metadata() {
    let event: ChangeFeedItem<Document> = serde_json::from_value(json!({
        "current": {"id": "a", "value": 2},
        "previous": {"id": "a", "value": 1},
        "metadata": {"operationType": "replace", "lsn": 11, "previousImageLSN": 10}
    }))
    .unwrap();
    assert_eq!(
        event.current(),
        Some(&Document {
            id: "a".into(),
            value: 2
        })
    );
    assert_eq!(
        event.previous(),
        Some(&Document {
            id: "a".into(),
            value: 1
        })
    );
    assert_eq!(
        event.operation_type(),
        Some(ChangeFeedOperationType::Replace)
    );
    assert_eq!(
        event.metadata().unwrap().previous_image_lsn(),
        Some(LogicalSequenceNumber::from(10))
    );
}

#[test]
fn delete_missing_null_empty_images_and_identity() {
    for current in [None, Some(json!(null)), Some(json!({}))] {
        let mut input = json!({"metadata": {
            "operationType": "delete", "id": "deleted", "partitionKey": ["tenant", 4],
            "timeToLiveExpired": true, "crts": -1
        }});
        if let Some(current) = current {
            input["current"] = current;
        }
        let event: ChangeFeedItem<Document> = serde_json::from_value(input).unwrap();
        assert!(event.current().is_none());
        assert!(event.previous().is_none());
        let metadata = event.metadata().unwrap();
        assert_eq!(
            event.operation_type(),
            Some(ChangeFeedOperationType::Delete)
        );
        assert_eq!(metadata.id(), Some("deleted"));
        assert_eq!(metadata.partition_key(), Some(&json!(["tenant", 4])));
        assert_eq!(metadata.time_to_live_expired(), Some(true));
        assert_eq!(
            metadata.conflict_resolution_timestamp(),
            Some(Duration::ZERO)
        );
    }
    let event: ChangeFeedItem<Document> = serde_json::from_value(json!({
        "current": {}, "previous": {"id": "deleted", "value": 3},
        "metadata": {"operationType": "delete"}
    }))
    .unwrap();
    assert_eq!(
        event.previous(),
        Some(&Document {
            id: "deleted".into(),
            value: 3
        })
    );
}

#[test]
fn unknown_operations_and_missing_metadata_are_supported() {
    let event: ChangeFeedItem<Document> = serde_json::from_value(json!({
        "metadata": {"operationType": "futureOperation"}
    }))
    .unwrap();
    assert_eq!(
        event.operation_type(),
        Some(ChangeFeedOperationType::Unknown)
    );
    let event: ChangeFeedItem<Document> = serde_json::from_value(json!({
        "current": {"id": "a", "value": 2}
    }))
    .unwrap();
    assert!(event.metadata().is_none());
    assert!(event.previous().is_none());
}
