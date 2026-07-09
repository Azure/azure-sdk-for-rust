// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! System property generation and JSON injection.

/// Returns a feed response body with the given service envelope name.
pub(crate) fn feed_to_json(
    envelope_name: &str,
    items: Vec<serde_json::Value>,
    rid: impl Into<String>,
) -> serde_json::Value {
    let count = items.len();
    serde_json::json!({
        envelope_name: items,
        "_rid": rid.into(),
        "_count": count
    })
}

/// Injects system properties (`_rid`, `_self`, `_etag`, `_ts`, `_attachments`)
/// into a document's JSON body.
///
/// Takes the individual values (rather than a [`StoredDocument`]) so callers
/// can mutate the body before owning the doc and avoid cloning the body twice.
pub(crate) fn inject_system_properties(
    rid: &str,
    self_link: &str,
    etag: &str,
    ts: u64,
    body: &mut serde_json::Value,
) {
    if let Some(obj) = body.as_object_mut() {
        obj.insert(
            "_rid".to_string(),
            serde_json::Value::String(rid.to_owned()),
        );
        obj.insert(
            "_self".to_string(),
            serde_json::Value::String(self_link.to_owned()),
        );
        obj.insert(
            "_etag".to_string(),
            serde_json::Value::String(etag.to_owned()),
        );
        obj.insert("_ts".to_string(), serde_json::json!(ts));
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
        "indexingPolicy": {
            "indexingMode": "consistent",
            "automatic": true,
            "includedPaths": [{"path": "/*"}],
            "excludedPaths": [{"path": "/\"_etag\"/?"}]
        },
        "conflictResolutionPolicy": {
            "mode": "LastWriterWins",
            "conflictResolutionPath": "/_ts",
            "conflictResolutionProcedure": ""
        },
        "geospatialConfig": {
            "type": "Geography"
        },
        "_docs": "docs/",
        "_sprocs": "sprocs/",
        "_triggers": "triggers/",
        "_udfs": "udfs/",
        "_conflicts": "conflicts/"
    })
}

/// Returns a JSON representation of throughput offer metadata.
pub(crate) fn offer_to_json(meta: &super::store::OfferMetadata) -> serde_json::Value {
    serde_json::json!({
        "id": meta.id,
        "_rid": meta.rid,
        "_self": meta.self_link,
        "_etag": meta.etag,
        "_ts": meta.ts,
        "resource": meta.offer_resource_id,
        "offerResourceId": meta.offer_resource_id,
        "offerType": "Invalid",
        "offerVersion": "V2",
        "content": {
            "offerThroughput": meta.throughput
        }
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
                "minInclusive": p.epk_min.to_hex(),
                "maxExclusive": p.epk_max.to_hex(),
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
        "_rid": "emulator.documents.azure.com",
        "_self": "",
        "media": "//media/",
        "addresses": "//addresses/",
        "_dbs": "//dbs/",
        "readableLocations": readable,
        "writableLocations": writable,
        "enableMultipleWriteLocations": config.write_mode() == super::config::WriteMode::Multi,
        // Mirrors the real service contract for PPAF dynamic enablement. The
        // driver's background account-refresh loop polls this field; tests
        // that flip `VirtualAccountConfig::set_per_partition_failover(...)`
        // observe the change here on the next refresh tick.
        "enablePerPartitionFailoverBehavior": config.per_partition_failover_enabled(),
        "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
        "userConsistencyPolicy": {
            "defaultConsistencyLevel": config.consistency().as_str()
        },
        "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
        "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
        "queryEngineConfiguration": "{}"
    })
}
