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

/// Returns a JSON representation of a page of partition key ranges.
pub(crate) fn pkranges_to_json(
    container: &super::store::ContainerState,
    start: usize,
    end: usize,
) -> serde_json::Value {
    let ranges: Vec<serde_json::Value> = container
        .physical_partitions
        .get(start..end)
        .unwrap_or_default()
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
///
/// `request_host` is the host the account read arrived on; the real gateway
/// derives the account `id` from it rather than reporting a fixed name.
pub(crate) fn account_properties_to_json(
    config: &super::config::VirtualAccountConfig,
    request_host: Option<&str>,
) -> serde_json::Value {
    let location_json = |r: &super::config::VirtualRegion| {
        serde_json::json!({
            "name": r.name(),
            "databaseAccountEndpoint": r.gateway_url().as_str()
        })
    };

    // One consistent view of the topology: reading `write_mode` under one guard
    // for `writableLocations` and again under another for
    // `enableMultipleWriteLocations` could straddle a concurrent
    // `set_write_mode` and emit a payload the real gateway never produces (the
    // service flaps *between* payloads, but each individual payload is
    // internally consistent).
    let topology = config.topology_snapshot();
    // Offline regions are filtered out and the write region hoisted to the
    // front; during a failover `writable` carries both the outgoing and
    // incoming write regions, which is what the service advertises.
    let advertised = topology.advertised();
    let readable: Vec<serde_json::Value> = advertised.iter().map(|r| location_json(r)).collect();

    let is_multi_write = topology.write_mode == super::config::WriteMode::Multi;
    // The service emits enableMultipleWriteLocations=true under Strong, but
    // separately gates expansion of writableLocations on consistency != Strong.
    let allow_multiple_write_locations =
        is_multi_write && config.consistency() != super::config::ConsistencyLevel::Strong;
    let writable: Vec<serde_json::Value> = topology
        .writable(allow_multiple_write_locations)
        .iter()
        .map(|r| location_json(r))
        .collect();

    // The real gateway sets the account id from the tenant portion of the
    // request host (everything before the first '.'), so a client that called a
    // regional endpoint sees that regional id echoed back. Verified against a
    // live account: the global endpoint reports `{account}` and the regional
    // endpoint reports `{account}-{region}`. Falls back to the configured
    // account id when no host is available.
    let account_id = request_host
        .map(|host| host.split('.').next().unwrap_or(host).to_string())
        .unwrap_or_else(|| config.account_id().to_string());

    // The live service reports `_rid` as
    // `{account}-{write-region}.sql.cosmos.azure.com`. The emulator's synthetic
    // hosts carry no `{account}-{region}` structure to recover a base account
    // name from, so encoding the write region here would make `_rid` name a
    // different account than `id` on the same payload -- something the service
    // never does. Reporting the same account keeps the payload self-consistent;
    // the write-region component is the one part of `_rid` the emulator does not
    // reproduce.
    let rid = format!("{account_id}.sql.cosmos.azure.com");

    // NOTE: no `_etag`. Verified against live accounts (two accounts, global and
    // regional endpoints, `x-ms-version` 2018-12-31 and 2020-07-15): the account
    // read carries no etag in the body and none in the response headers. The
    // driver's unchanged-etag short-circuit in `sync_account_properties` is
    // therefore inert in production -- it is guarded by `!etag.is_empty()` -- and
    // emitting one here would make the emulator exercise a path the service can
    // never trigger. That short-circuit is covered by unit tests instead.
    let mut response = serde_json::json!({
        "id": account_id,
        "_rid": rid,
        "_self": "",
        "media": "//media/",
        "addresses": "//addresses/",
        "_dbs": "//dbs/",
        "readableLocations": readable,
        "writableLocations": writable,
        "enableMultipleWriteLocations": is_multi_write,
        "continuousBackupEnabled": false,
        "enableNRegionSynchronousCommit": false,
        // Mirrors the real service contract for PPAF dynamic enablement. The
        // driver's background account-refresh loop polls this field; tests
        // that flip `VirtualAccountConfig::set_per_partition_failover(...)`
        // observe the change here on the next refresh tick.
        "enablePerPartitionFailoverBehavior": config.per_partition_failover_enabled(),
        "disableCrossRegionalHedging": false,
        "userReplicationPolicy": {
            "asyncReplication": false,
            "minReplicaSetSize": 3,
            "maxReplicasetSize": 4
        },
        "userConsistencyPolicy": {
            "defaultConsistencyLevel": config.consistency().as_str()
        },
        "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
        "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
        "queryEngineConfiguration": "{}"
    });

    #[cfg(feature = "__internal_in_memory_emulator")]
    {
        let thin_client_readable: Vec<_> = advertised
            .iter()
            .filter_map(|region| {
                region.gateway_v2_url().map(|url| {
                    serde_json::json!({
                        "name": region.name(),
                        "databaseAccountEndpoint": url.as_str()
                    })
                })
            })
            .collect();
        let thin_client_writable: Vec<_> = topology
            .writable(allow_multiple_write_locations)
            .iter()
            .filter_map(|region| {
                region.gateway_v2_url().map(|url| {
                    serde_json::json!({
                        "name": region.name(),
                        "databaseAccountEndpoint": url.as_str()
                    })
                })
            })
            .collect();
        if !thin_client_readable.is_empty() {
            let object = response
                .as_object_mut()
                .expect("account properties response is a JSON object");
            object.insert(
                "thinClientReadableLocations".to_owned(),
                serde_json::Value::Array(thin_client_readable),
            );
            object.insert(
                "thinClientWritableLocations".to_owned(),
                serde_json::Value::Array(thin_client_writable),
            );
        }
    }

    response
}
