// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! System property generation and JSON injection.

use super::store::StoredDocument;

/// Injects system properties (`_rid`, `_self`, `_etag`, `_ts`, `_attachments`)
/// into a document's JSON body.
pub(crate) fn inject_system_properties(doc: &StoredDocument, body: &mut serde_json::Value) {
    if let Some(obj) = body.as_object_mut() {
        obj.insert(
            "_rid".to_string(),
            serde_json::Value::String(doc.rid.clone()),
        );
        obj.insert(
            "_self".to_string(),
            serde_json::Value::String(doc.self_link.clone()),
        );
        obj.insert(
            "_etag".to_string(),
            serde_json::Value::String(doc.etag.clone()),
        );
        obj.insert("_ts".to_string(), serde_json::json!(doc.ts));
        obj.insert(
            "_attachments".to_string(),
            serde_json::Value::String("attachments/".to_string()),
        );
    }
}

/// Returns a JSON representation of database metadata.
pub(crate) fn database_to_json(meta: &super::store::DatabaseMetadata) -> serde_json::Value {
    serde_json::json!({
        "id": meta.id,
        "_rid": meta.rid,
        "_self": meta.self_link,
        "_etag": meta.etag,
        "_ts": meta.ts,
        "_colls": "colls/",
        "_users": "users/"
    })
}

/// Returns a JSON representation of container metadata.
pub(crate) fn container_to_json(meta: &super::store::ContainerMetadata) -> serde_json::Value {
    let pk_paths: Vec<&str> = meta
        .partition_key
        .paths()
        .iter()
        .map(|p| p.as_ref())
        .collect();
    serde_json::json!({
        "id": meta.id,
        "_rid": meta.rid,
        "_self": meta.self_link,
        "_etag": meta.etag,
        "_ts": meta.ts,
        "partitionKey": {
            "paths": pk_paths,
            "kind": format!("{:?}", meta.partition_key.kind()),
            "version": meta.partition_key.version().value()
        },
        "_docs": "docs/",
        "_sprocs": "sprocs/",
        "_triggers": "triggers/",
        "_udfs": "udfs/",
        "_conflicts": "conflicts/"
    })
}

/// Returns a JSON representation of partition key ranges for a container.
pub(crate) fn pkranges_to_json(container: &super::store::ContainerState) -> serde_json::Value {
    let ranges: Vec<serde_json::Value> = container
        .physical_partitions
        .iter()
        .map(|p| {
            let parents: Vec<String> = p.parents.iter().map(|id| id.to_string()).collect();
            serde_json::json!({
                "id": p.id.to_string(),
                "_rid": p.rid,
                "_self": format!("{}pkranges/{}/", container.metadata.self_link, p.id),
                "_etag": container.metadata.etag,
                "_ts": container.metadata.ts,
                "_lsn": p.current_lsn(),
                "minInclusive": p.epk_min.as_str(),
                "maxExclusive": p.epk_max.as_str(),
                "ridPrefix": p.rid_prefix,
                "throughputFraction": p.throughput_fraction,
                "status": "online",
                "parents": parents,
                "vectorClockVersion": p.current_version()
            })
        })
        .collect();

    serde_json::json!({
        "PartitionKeyRanges": ranges,
        "_rid": container.metadata.rid,
        "_count": ranges.len()
    })
}

/// Returns a JSON representation of account properties synthesized from config.
pub(crate) fn account_properties_to_json(
    config: &super::config::VirtualAccountConfig,
) -> serde_json::Value {
    let readable: Vec<serde_json::Value> = config
        .regions()
        .iter()
        .map(|r| {
            serde_json::json!({
                "name": r.name(),
                "databaseAccountEndpoint": r.gateway_url().as_str()
            })
        })
        .collect();

    let writable: Vec<serde_json::Value> = match config.write_mode() {
        super::config::WriteMode::Multi => config
            .regions()
            .iter()
            .map(|r| {
                serde_json::json!({
                    "name": r.name(),
                    "databaseAccountEndpoint": r.gateway_url().as_str()
                })
            })
            .collect(),
        super::config::WriteMode::Single => {
            let r = &config.regions()[0];
            vec![serde_json::json!({
                "name": r.name(),
                "databaseAccountEndpoint": r.gateway_url().as_str()
            })]
        }
    };

    serde_json::json!({
        "id": "emulator-account",
        "_rid": "",
        "readableLocations": readable,
        "writableLocations": writable,
        "enableMultipleWriteLocations": config.write_mode() == super::config::WriteMode::Multi,
        "userConsistencyPolicy": {
            "defaultConsistencyLevel": config.consistency().as_str()
        }
    })
}
