// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Emulator-backed E2E tests for the driver's PATCH (RMW) pipeline.
//!
//! Coverage:
//!
//! - `cosmos_patch_basic_set` — round-trip a single `Set` PATCH and observe
//!   the locally-merged post-image.
//! - `cosmos_patch_pk_guard{,_hierarchical}` — preflight rejects ops that
//!   target a partition-key path, on both flat and hierarchical (HPK)
//!   containers; no network round-trip is issued.
//! - `cosmos_patch_412_retry` — fault-inject a 412 on the first
//!   `ReplaceItem` and verify the handler retries and ultimately succeeds.
//! - `cosmos_patch_412_exhaustion` — fault-inject 412 on every
//!   `ReplaceItem` and verify the handler surfaces a `PreconditionFailed`
//!   error after `max_attempts` is exhausted.
//! - `cosmos_patch_semantics` — iterate a fixture catalog of driver-side
//!   PATCH cases and assert each case's post-image (or error substring)
//!   against the live emulator. The fixtures cover the cross product of op
//!   kinds × scenario categories (happy_path, nested, missing_path,
//!   null_value, multi_op, etc.).
//! - `cosmos_patch_read_missing_item_returns_not_found` — 404 on the read
//!   leg propagates immediately; no replace is issued.
//!
//! The missing-ETag and end-to-end latency-budget contracts are
//! unit-tested in `driver/pipeline/patch_handler.rs`.

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::models::{
    CosmosNumber, PartitionKey, PatchInstructions, PatchOperation,
};
use serde_json::{json, Value};
use std::error::Error;

// ---------------------------------------------------------------------------
// A5: basic Set returns the locally-merged post-image
// ---------------------------------------------------------------------------

/// Insert a small document, `Set("/deleted", true)`, then read it back and
/// confirm the patched field is reflected in both the synthetic patch
/// response body and the subsequent read.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cosmos_patch_basic_set() -> Result<(), Box<dyn Error>> {
    Box::pin(DriverTestClient::run_with_unique_db(
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_id = "patch-basic-set-001";
            let pk = "p1";
            let initial = json!({
                "id": item_id,
                "pk": pk,
                "deleted": false,
                "counter": 0,
            });
            let initial_bytes = serde_json::to_vec(&initial)?;

            context
                .create_seed_item(&container, item_id, pk, &initial_bytes)
                .await?;

            let spec = PatchInstructions::from(vec![PatchOperation::set("/deleted", json!(true))]);
            let patch_response = context
                .patch_item(&container, item_id, pk, &spec, None)
                .await?;

            let patched: Value = patch_response.into_body().into_single()?;
            assert_eq!(
                patched.get("deleted"),
                Some(&Value::Bool(true)),
                "patch response body should reflect Set(/deleted = true); got {patched}",
            );
            assert_eq!(patched.get("id"), Some(&json!(item_id)));
            assert_eq!(patched.get("pk"), Some(&json!(pk)));

            let read_response = context.read_item(&container, item_id, pk).await?;
            let read_body: Value = read_response.into_body().into_single()?;
            assert_eq!(
                read_body.get("deleted"),
                Some(&Value::Bool(true)),
                "post-read should observe the same merged document; got {read_body}",
            );

            Ok(())
        },
    ))
    .await
}

// ---------------------------------------------------------------------------
// A6: PK-guard preflight rejects ops on partition-key paths (flat + HPK)
// ---------------------------------------------------------------------------

/// Flat (single-path) PK: ops targeting `/pk` are rejected by the preflight
/// guard with no network call.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cosmos_patch_pk_guard() -> Result<(), Box<dyn Error>> {
    Box::pin(DriverTestClient::run_with_unique_db(
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_id = "patch-pk-guard-001";
            let pk = "tenant-a";
            let initial = json!({ "id": item_id, "pk": pk, "name": "n" });
            context
                .create_seed_item(&container, item_id, pk, &serde_json::to_vec(&initial)?)
                .await?;

            // Each op below targets the PK path directly.
            let guard_cases: Vec<(&'static str, PatchInstructions)> = vec![
                (
                    "Set /pk",
                    PatchInstructions::from(vec![PatchOperation::set("/pk", json!("other"))]),
                ),
                (
                    "Replace /pk",
                    PatchInstructions::from(vec![PatchOperation::replace("/pk", json!("other"))]),
                ),
                (
                    "Remove /pk",
                    PatchInstructions::from(vec![PatchOperation::remove("/pk")]),
                ),
                (
                    "Add /pk",
                    PatchInstructions::from(vec![PatchOperation::add("/pk", json!("other"))]),
                ),
                (
                    "Move to /pk",
                    PatchInstructions::from(vec![PatchOperation::move_value("/name", "/pk")]),
                ),
                // moving FROM a PK path also mutates the partition key
                // (the field is removed at the source after being copied to
                // the destination). The guard must reject this too.
                (
                    "Move from /pk",
                    PatchInstructions::from(vec![PatchOperation::move_value(
                        "/pk",
                        "/somewhere_else",
                    )]),
                ),
            ];

            for (label, spec) in guard_cases {
                let err = context
                    .patch_item(&container, item_id, pk, &spec, None)
                    .await
                    .err()
                    .unwrap_or_else(|| panic!("{label}: expected PK-guard rejection"));
                let msg = format!("{err}").to_ascii_lowercase();
                assert!(
                    msg.contains("partition key") || msg.contains("partitionkey"),
                    "{label}: error should mention partition key; got: {err}",
                );
            }

            Ok(())
        },
    ))
    .await
}

/// Hierarchical (multi-path) PK: ops targeting **any** of the PK paths are
/// rejected. Exercises the `MultiHash` branch of `validate_partition_key_paths`.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cosmos_patch_pk_guard_hierarchical() -> Result<(), Box<dyn Error>> {
    Box::pin(DriverTestClient::run_with_unique_db(
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container_with_pk_paths(
                    &database,
                    &container_name,
                    &["/tenantId", "/userId"],
                )
                .await?;

            let item_id = "patch-hpk-001";
            let tenant = "t1";
            let user = "u1";
            let initial = json!({
                "id": item_id,
                "tenantId": tenant,
                "userId": user,
                "payload": "hello",
            });
            let pk: PartitionKey = (tenant, user).into();
            context
                .create_seed_item(
                    &container,
                    item_id,
                    pk.clone(),
                    &serde_json::to_vec(&initial)?,
                )
                .await?;

            // Each row targets one of the two PK paths.
            let guard_cases: Vec<(&'static str, PatchInstructions)> = vec![
                (
                    "Set /tenantId",
                    PatchInstructions::from(vec![PatchOperation::set("/tenantId", json!("t2"))]),
                ),
                (
                    "Set /userId",
                    PatchInstructions::from(vec![PatchOperation::set("/userId", json!("u2"))]),
                ),
                (
                    "Replace /tenantId",
                    PatchInstructions::from(vec![PatchOperation::replace(
                        "/tenantId",
                        json!("t2"),
                    )]),
                ),
                (
                    "Remove /userId",
                    PatchInstructions::from(vec![PatchOperation::remove("/userId")]),
                ),
                // moving FROM one of the hierarchical PK paths also
                // mutates that PK component. Reject pre-flight.
                (
                    "Move from /tenantId",
                    PatchInstructions::from(vec![PatchOperation::move_value(
                        "/tenantId",
                        "/somewhere_else",
                    )]),
                ),
            ];

            for (label, spec) in guard_cases {
                let err = context
                    .patch_item(&container, item_id, pk.clone(), &spec, None)
                    .await
                    .err()
                    .unwrap_or_else(|| panic!("{label}: expected HPK-guard rejection"));
                let msg = format!("{err}").to_ascii_lowercase();
                assert!(
                    msg.contains("partition key") || msg.contains("partitionkey"),
                    "{label}: error should mention partition key; got: {err}",
                );
            }

            Ok(())
        },
    ))
    .await
}

// ---------------------------------------------------------------------------
// A9: comparison-harness fixture catalog (≥30 rows)
// ---------------------------------------------------------------------------

/// Outcome a [`PatchCompareCase`] expects when its `ops` are applied to its
/// `initial_doc`.
#[derive(Debug, Clone)]
enum Expected {
    /// The patch must succeed; the post-image must equal these JSON
    /// properties (compared after merging into the seed `id` / pk fields).
    PostImageProps(Value),
    /// The patch must fail; the error message (lowercased) must contain
    /// this substring.
    ErrorContains(&'static str),
}

/// A single fixture row in the patch-fixture catalog. Each row exercises a
/// driver-side PATCH case against the live emulator and asserts the
/// observable post-image (or surfaced error substring).
///
/// `source_test_id` is informational provenance only: it records which
/// upstream SDK test method inspired the fixture but the assertion here is
/// against the Rust driver's emulator response, not a cross-SDK comparison.
#[derive(Debug, Clone)]
struct PatchCompareCase {
    /// Stable kebab-case fixture id.
    id: &'static str,
    /// Informational provenance: name of an upstream test method this
    /// fixture was inspired by. Not used by the harness for assertions.
    source_test_id: &'static str,
    /// One of: `Add` | `Set` | `Replace` | `Remove` | `Increment` | `Move`
    /// | `Mixed`.
    op_kind: &'static str,
    /// Coarse scenario bucket used to balance coverage.
    scenario_category: &'static str,
    /// JSON properties merged onto the seed `{id, pk}` to form the initial
    /// document.
    initial_props: Value,
    /// The patch ops to apply.
    ops: Vec<PatchOperation>,
    /// What the harness expects after applying `ops`.
    expected: Expected,
    /// Free-form provenance / notes.
    notes: &'static str,
}

fn fixtures() -> Vec<PatchCompareCase> {
    vec![
        // ---- Direct ports of CosmosItemTests.ItemPatchSuccessTest ops ----
        PatchCompareCase {
            id: "set-nested-array-string",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Set",
            scenario_category: "nested",
            initial_props: json!({
                "description": "orig",
                "children": [
                    { "description": "c0" },
                    { "description": "c1" },
                ],
            }),
            ops: vec![PatchOperation::set("/children/0/description", json!("testSet"))],
            expected: Expected::PostImageProps(json!({
                "description": "orig",
                "children": [
                    { "description": "testSet" },
                    { "description": "c1" },
                ],
            })),
            notes: "Set on nested array child mirrors .NET Set(\"/children/0/description\", \"testSet\")",
        },
        PatchCompareCase {
            id: "add-nested-array-string",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Add",
            scenario_category: "nested",
            initial_props: json!({
                "description": "orig",
                "children": [
                    { "description": "c0" },
                    {},
                ],
            }),
            ops: vec![PatchOperation::add("/children/1/extra", json!("patched"))],
            expected: Expected::PostImageProps(json!({
                "description": "orig",
                "children": [
                    { "description": "c0" },
                    { "extra": "patched" },
                ],
            })),
            notes: ".NET Add(\"/children/1/pk\", \"patched\") — we avoid /pk to keep PK-guard out of A9",
        },
        PatchCompareCase {
            id: "remove-root-scalar",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Remove",
            scenario_category: "happy_path",
            initial_props: json!({ "description": "orig", "leftover": "stay" }),
            ops: vec![PatchOperation::remove("/description")],
            expected: Expected::PostImageProps(json!({ "leftover": "stay" })),
            notes: "Mirrors .NET Remove(\"/description\")",
        },
        PatchCompareCase {
            id: "replace-int-scalar",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Replace",
            scenario_category: "happy_path",
            initial_props: json!({ "taskNum": 1 }),
            ops: vec![PatchOperation::replace("/taskNum", json!(42))],
            expected: Expected::PostImageProps(json!({ "taskNum": 42 })),
            notes: "Mirrors .NET Replace(\"/taskNum\", newTaskNum)",
        },
        PatchCompareCase {
            id: "set-null-object-property",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Set",
            scenario_category: "null_value",
            initial_props: json!({
                "children": [
                    { "description": "c0" },
                    { "nullableInt": 7 },
                ],
            }),
            ops: vec![PatchOperation::set("/children/1/nullableInt", Value::Null)],
            expected: Expected::PostImageProps(json!({
                "children": [
                    { "description": "c0" },
                    { "nullableInt": null },
                ],
            })),
            notes: "Mirrors .NET Set<object>(\"/children/1/nullableInt\", null)",
        },
        PatchCompareCase {
            id: "add-int-to-array-child",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Add",
            scenario_category: "nested",
            initial_props: json!({ "children": [ {}, {} ] }),
            ops: vec![PatchOperation::add("/children/0/cost", json!(1))],
            expected: Expected::PostImageProps(json!({
                "children": [ { "cost": 1 }, {} ],
            })),
            notes: "Mirrors .NET Add(\"/children/0/cost\", 1)",
        },
        PatchCompareCase {
            id: "set-null-on-business-key",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Set",
            scenario_category: "null_value",
            initial_props: json!({
                "children": [
                    { "name": "c0" },
                    { "name": "c1" },
                ],
            }),
            ops: vec![PatchOperation::set("/children/0/name", Value::Null)],
            expected: Expected::PostImageProps(json!({
                "children": [
                    { "name": null },
                    { "name": "c1" },
                ],
            })),
            notes: "Adapted from .NET Set<object>(\"/children/0/id\", null) — we avoid /id to keep the item readable",
        },
        PatchCompareCase {
            id: "mixed-set-remove-replace",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Mixed",
            scenario_category: "multi_op",
            initial_props: json!({
                "description": "orig",
                "taskNum": 1,
                "children": [
                    { "description": "c0" },
                    {},
                ],
            }),
            ops: vec![
                PatchOperation::set("/children/0/description", json!("testSet")),
                PatchOperation::add("/children/1/extra", json!("patched")),
                PatchOperation::remove("/description"),
                PatchOperation::replace("/taskNum", json!(99)),
            ],
            expected: Expected::PostImageProps(json!({
                "taskNum": 99,
                "children": [
                    { "description": "testSet" },
                    { "extra": "patched" },
                ],
            })),
            notes: "Multi-op composition mirrors .NET ItemPatchSuccessTest body",
        },
        PatchCompareCase {
            id: "move-chain-three-steps",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest",
            op_kind: "Move",
            scenario_category: "multi_op",
            initial_props: json!({
                "description": "orig",
                "children": [
                    { "description": "Child#0" },
                    {},
                ],
            }),
            ops: vec![
                PatchOperation::add("/children/1/description", json!("Child#1")),
                PatchOperation::move_value("/children/0/description", "/description"),
                PatchOperation::move_value("/children/1/description", "/children/0/description"),
            ],
            expected: Expected::PostImageProps(json!({
                "description": "Child#0",
                "children": [
                    { "description": "Child#1" },
                    {},
                ],
            })),
            notes: "Mirrors .NET ItemPatchSuccessTest Move chain; final state has description=Child#0, children[0].description=Child#1",
        },
        // ---- Failure cases from CosmosItemTests.ItemPatchFailureTest ----
        PatchCompareCase {
            id: "add-nonexistent-parent-fails",
            source_test_id: "CosmosItemTests.ItemPatchFailureTest",
            op_kind: "Add",
            scenario_category: "missing_path",
            initial_props: json!({ "description": "orig" }),
            ops: vec![PatchOperation::add("/nonExistentParent/Child", json!("bar"))],
            expected: Expected::ErrorContains("nonExistentParent"),
            notes: "Mirrors .NET Add(\"/nonExistentParent/Child\", \"bar\") — expect bad-request from evaluator",
        },
        PatchCompareCase {
            id: "remove-missing-leaf-fails",
            source_test_id: "CosmosItemTests.ItemPatchFailureTest",
            op_kind: "Remove",
            scenario_category: "missing_path",
            initial_props: json!({ "description": "orig" }),
            ops: vec![PatchOperation::remove("/cost")],
            expected: Expected::ErrorContains("cost"),
            notes: "Mirrors .NET Remove(\"/cost\") on a doc without /cost",
        },
        PatchCompareCase {
            id: "replace-missing-path-fails",
            source_test_id: "CosmosItemTests.ItemPatchFailureTest",
            op_kind: "Replace",
            scenario_category: "missing_path",
            initial_props: json!({ "description": "orig" }),
            ops: vec![PatchOperation::replace("/missing", json!(7))],
            expected: Expected::ErrorContains("missing"),
            notes: "Replace on a path that doesn't exist must fail per JSON Patch semantics",
        },
        PatchCompareCase {
            id: "move-missing-source-fails",
            source_test_id: "CosmosItemTests.ItemPatchFailureTest",
            op_kind: "Move",
            scenario_category: "missing_path",
            initial_props: json!({ "description": "orig" }),
            ops: vec![PatchOperation::move_value("/missing", "/dest")],
            expected: Expected::ErrorContains("missing"),
            notes: "Move with absent source path must fail",
        },
        // ---- Direct ports of PatchOperationTests.ConstructPatchOperationTest ----
        PatchCompareCase {
            id: "construct-add-string",
            source_test_id: "PatchOperationTests.ConstructPatchOperationTest",
            op_kind: "Add",
            scenario_category: "happy_path",
            initial_props: json!({}),
            ops: vec![PatchOperation::add("/name", json!("alice"))],
            expected: Expected::PostImageProps(json!({ "name": "alice" })),
            notes: "Construct + apply Add(string)",
        },
        PatchCompareCase {
            id: "construct-add-datetime-as-string",
            source_test_id: "PatchOperationTests.ConstructPatchOperationTest",
            op_kind: "Add",
            scenario_category: "happy_path",
            initial_props: json!({}),
            ops: vec![PatchOperation::add("/createdAt", json!("2024-04-15T12:34:56Z"))],
            expected: Expected::PostImageProps(json!({
                "createdAt": "2024-04-15T12:34:56Z",
            })),
            notes: "Mirrors .NET DateTime-as-string Add",
        },
        PatchCompareCase {
            id: "construct-add-complex-object",
            source_test_id: "PatchOperationTests.ConstructPatchOperationTest",
            op_kind: "Add",
            scenario_category: "happy_path",
            initial_props: json!({}),
            ops: vec![PatchOperation::add(
                "/profile",
                json!({ "city": "Seattle", "zip": 98052 }),
            )],
            expected: Expected::PostImageProps(json!({
                "profile": { "city": "Seattle", "zip": 98052 },
            })),
            notes: "Mirrors .NET Add(complex object)",
        },
        PatchCompareCase {
            id: "construct-replace-array",
            source_test_id: "PatchOperationTests.ConstructPatchOperationTest",
            op_kind: "Replace",
            scenario_category: "happy_path",
            initial_props: json!({ "tags": ["a", "b"] }),
            ops: vec![PatchOperation::replace("/tags", json!(["x", "y", "z"]))],
            expected: Expected::PostImageProps(json!({ "tags": ["x", "y", "z"] })),
            notes: "Replace with array payload",
        },
        PatchCompareCase {
            id: "construct-set-guid-string",
            source_test_id: "PatchOperationTests.ConstructPatchOperationTest",
            op_kind: "Set",
            scenario_category: "happy_path",
            initial_props: json!({}),
            ops: vec![PatchOperation::set(
                "/tenantGuid",
                json!("11111111-2222-3333-4444-555555555555"),
            )],
            expected: Expected::PostImageProps(json!({
                "tenantGuid": "11111111-2222-3333-4444-555555555555",
            })),
            notes: "Mirrors .NET Set(GUID-as-string)",
        },
        PatchCompareCase {
            id: "construct-set-null",
            source_test_id: "PatchOperationTests.ConstructPatchOperationTest",
            op_kind: "Set",
            scenario_category: "null_value",
            initial_props: json!({ "optional": "v" }),
            ops: vec![PatchOperation::set("/optional", Value::Null)],
            expected: Expected::PostImageProps(json!({ "optional": null })),
            notes: "Mirrors .NET Set<object>(path, null)",
        },
        // ---- Direct ports of PatchOperationTTests.CastPatchOperationTest ----
        PatchCompareCase {
            id: "increment-by-float-7",
            source_test_id: "PatchOperationTTests.CastPatchOperationTest",
            op_kind: "Increment",
            scenario_category: "happy_path",
            initial_props: json!({ "score": 1.5 }),
            ops: vec![PatchOperation::increment("/score", CosmosNumber::Float(7.0))],
            expected: Expected::PostImageProps(json!({ "score": 8.5 })),
            notes: "Mirrors .NET Increment(double 7.0)",
        },
        PatchCompareCase {
            id: "increment-by-int-40",
            source_test_id: "PatchOperationTTests.CastPatchOperationTest",
            op_kind: "Increment",
            scenario_category: "happy_path",
            initial_props: json!({ "count": 2 }),
            ops: vec![PatchOperation::increment("/count", CosmosNumber::Int(40))],
            expected: Expected::PostImageProps(json!({ "count": 42 })),
            notes: "Mirrors .NET Increment(long 40)",
        },
        // ---- Direct ports of Java PatchAsyncTest.itemPatchSuccessForNullValue ----
        PatchCompareCase {
            id: "java-set-null-scalar",
            source_test_id: "PatchAsyncTest.itemPatchSuccessForNullValue",
            op_kind: "Set",
            scenario_category: "null_value",
            initial_props: json!({ "uuidField": "abc" }),
            ops: vec![PatchOperation::set("/uuidField", Value::Null)],
            expected: Expected::PostImageProps(json!({ "uuidField": null })),
            notes: "Mirrors Java Set null on UUID-valued field",
        },
        PatchCompareCase {
            id: "java-add-null-scalar",
            source_test_id: "PatchAsyncTest.itemPatchSuccessForNullValue",
            op_kind: "Add",
            scenario_category: "null_value",
            initial_props: json!({}),
            ops: vec![PatchOperation::add("/uuidField", Value::Null)],
            expected: Expected::PostImageProps(json!({ "uuidField": null })),
            notes: "Mirrors Java Add null",
        },
        PatchCompareCase {
            id: "java-replace-null-scalar",
            source_test_id: "PatchAsyncTest.itemPatchSuccessForNullValue",
            op_kind: "Replace",
            scenario_category: "null_value",
            initial_props: json!({ "uuidField": "abc" }),
            ops: vec![PatchOperation::replace("/uuidField", Value::Null)],
            expected: Expected::PostImageProps(json!({ "uuidField": null })),
            notes: "Mirrors Java Replace null",
        },
        // ---- Rust-derived edge cases not covered by .NET/Java surface ----
        PatchCompareCase {
            id: "increment-i64-large-value-fidelity",
            source_test_id: "rust-derived (R7 i64 fidelity)",
            op_kind: "Increment",
            scenario_category: "i64_fidelity",
            initial_props: json!({ "balance": 9_007_199_254_740_991i64 }),
            ops: vec![PatchOperation::increment("/balance", CosmosNumber::Int(2))],
            expected: Expected::PostImageProps(json!({ "balance": 9_007_199_254_740_993i64 })),
            notes: "Past 2^53: must NOT be demoted to f64",
        },
        PatchCompareCase {
            id: "increment-i64-negative",
            source_test_id: "rust-derived (R7 i64 fidelity)",
            op_kind: "Increment",
            scenario_category: "i64_fidelity",
            initial_props: json!({ "balance": 100i64 }),
            ops: vec![PatchOperation::increment("/balance", CosmosNumber::Int(-25))],
            expected: Expected::PostImageProps(json!({ "balance": 75i64 })),
            notes: "Negative integer delta on i64 target",
        },
        PatchCompareCase {
            id: "array-append-dash",
            source_test_id: "rust-derived (RFC 6901 array `-`)",
            op_kind: "Add",
            scenario_category: "array_append",
            initial_props: json!({ "tags": ["a", "b"] }),
            ops: vec![PatchOperation::add("/tags/-", json!("c"))],
            expected: Expected::PostImageProps(json!({ "tags": ["a", "b", "c"] })),
            notes: "RFC 6901 array-append marker",
        },
        PatchCompareCase {
            id: "array-index-out-of-range-fails",
            source_test_id: "rust-derived (RFC 6902)",
            op_kind: "Add",
            scenario_category: "array_idx",
            initial_props: json!({ "tags": ["a"] }),
            ops: vec![PatchOperation::add("/tags/99", json!("z"))],
            expected: Expected::ErrorContains("99"),
            notes: "Index past end of array must fail",
        },
        PatchCompareCase {
            id: "pointer-escape-tilde-one",
            source_test_id: "rust-derived (RFC 6901 ~1)",
            op_kind: "Set",
            scenario_category: "pointer_escape",
            initial_props: json!({ "a/b": "orig" }),
            ops: vec![PatchOperation::set("/a~1b", json!("new"))],
            expected: Expected::PostImageProps(json!({ "a/b": "new" })),
            notes: "JSON Pointer escape ~1 -> /",
        },
        PatchCompareCase {
            id: "pointer-escape-tilde-zero",
            source_test_id: "rust-derived (RFC 6901 ~0)",
            op_kind: "Set",
            scenario_category: "pointer_escape",
            initial_props: json!({ "a~b": "orig" }),
            ops: vec![PatchOperation::set("/a~0b", json!("new"))],
            expected: Expected::PostImageProps(json!({ "a~b": "new" })),
            notes: "JSON Pointer escape ~0 -> ~",
        },
        PatchCompareCase {
            id: "deep-nesting-five-levels",
            source_test_id: "rust-derived (deep nesting)",
            op_kind: "Set",
            scenario_category: "deep_nesting",
            initial_props: json!({
                "a": { "b": { "c": { "d": { "e": "orig" } } } },
            }),
            ops: vec![PatchOperation::set("/a/b/c/d/e", json!("new"))],
            expected: Expected::PostImageProps(json!({
                "a": { "b": { "c": { "d": { "e": "new" } } } },
            })),
            notes: "Five-level nested path",
        },
        PatchCompareCase {
            id: "move-scalar-between-paths",
            source_test_id: "CosmosItemTests.ItemPatchSuccessTest (Move chain)",
            op_kind: "Move",
            scenario_category: "happy_path",
            initial_props: json!({ "src": "value", "dst": null }),
            ops: vec![PatchOperation::move_value("/src", "/dst")],
            expected: Expected::PostImageProps(json!({ "dst": "value" })),
            notes: "Single Move between two scalar fields",
        },
        PatchCompareCase {
            id: "add-creates-missing-object-leaf",
            source_test_id: "rust-derived (Add semantics)",
            op_kind: "Add",
            scenario_category: "happy_path",
            initial_props: json!({ "obj": { "existing": 1 } }),
            ops: vec![PatchOperation::add("/obj/newKey", json!("v"))],
            expected: Expected::PostImageProps(json!({
                "obj": { "existing": 1, "newKey": "v" },
            })),
            notes: "Add on missing leaf of existing object creates the key",
        },
        PatchCompareCase {
            id: "set-overwrites-existing-key",
            source_test_id: "rust-derived (Set semantics)",
            op_kind: "Set",
            scenario_category: "happy_path",
            initial_props: json!({ "key": "old" }),
            ops: vec![PatchOperation::set("/key", json!("new"))],
            expected: Expected::PostImageProps(json!({ "key": "new" })),
            notes: "Set replaces an existing key",
        },
    ]
}

/// Apply the seed `id`/pk fields onto a fixture's [`initial_props`] to form
/// the document we send to `create_item`.
///
/// [`initial_props`]: PatchCompareCase::initial_props
fn seed_document(case: &PatchCompareCase, item_id: &str, pk: &str) -> Value {
    let mut doc = case.initial_props.clone();
    let map = doc
        .as_object_mut()
        .expect("initial_props must be an object");
    map.insert("id".into(), json!(item_id));
    map.insert("pk".into(), json!(pk));
    doc
}

/// Assert that every key in `expected_props` is reflected in `actual` with
/// the exact same value. Extra keys (like `id`, `pk`, `_etag`) in `actual`
/// are ignored.
fn assert_post_image_props(actual: &Value, expected_props: &Value, case_id: &str) {
    let actual_obj = actual
        .as_object()
        .unwrap_or_else(|| panic!("[{case_id}] post-image must be an object; got {actual}"));
    let expected_obj = expected_props
        .as_object()
        .unwrap_or_else(|| panic!("[{case_id}] expected_props must be an object"));
    for (k, v) in expected_obj {
        let actual_v = actual_obj.get(k).unwrap_or_else(|| {
            panic!("[{case_id}] expected key {k:?} missing from post-image {actual}",)
        });
        assert_eq!(
            actual_v, v,
            "[{case_id}] key {k:?} mismatch: expected {v}, got {actual_v}",
        );
    }
}

/// Iterates the fixture catalog, applying each case against the emulator and
/// asserting its expected outcome. This is the workhorse of A9 — it
/// exercises driver-side PATCH semantics (post-image merging, evaluator
/// error surfacing) against the live emulator.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cosmos_patch_semantics() -> Result<(), Box<dyn Error>> {
    Box::pin(DriverTestClient::run_with_unique_db(
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // The fixture catalog tests driver-side PATCH semantics
            // directly against the emulator. Each row's expected post-image
            // (or error substring) is the contract for the Rust driver; if
            // a row needs adjusting after a behavior change, update the
            // expected value here and let the emulator confirm. Some
            // fixtures were originally inspired by upstream .NET/Java PATCH
            // tests (see each row's `notes`) but the assertion is against
            // the emulator response, not a cross-SDK comparison.
            let cases = fixtures();
            assert!(
                cases.len() >= 30,
                "A9 fixture catalog must have ≥30 rows; has {}",
                cases.len()
            );

            for (idx, case) in cases.iter().enumerate() {
                let item_id = format!("a9-{:03}-{}", idx, case.id);
                let pk = format!("pk-{idx:03}");
                let initial = seed_document(case, &item_id, &pk);

                let initial_body = serde_json::to_vec(&initial)?;
                context
                    .create_seed_item(&container, &item_id, pk.clone(), &initial_body)
                    .await
                    .unwrap_or_else(|e| panic!("[{}] seed failed: {e}", case.id));

                let spec = PatchInstructions::from(case.ops.clone());
                let result = context
                    .retry_transient_transport(&format!("[{}] patch", case.id), || {
                        context.patch_item(&container, &item_id, pk.clone(), &spec, None)
                    })
                    .await;

                match (&case.expected, result) {
                    (Expected::PostImageProps(expected_props), Ok(response)) => {
                        let body: Value = response
                            .into_body()
                            .into_single()
                            .unwrap_or_else(|e| panic!("[{}] body parse: {e}", case.id));
                        assert_post_image_props(&body, expected_props, case.id);
                    }
                    (Expected::PostImageProps(_), Err(e)) => {
                        panic!("[{}] expected success but got error: {e}", case.id);
                    }
                    (Expected::ErrorContains(needle), Err(e)) => {
                        let msg = format!("{e}").to_ascii_lowercase();
                        assert!(
                            msg.contains(&needle.to_ascii_lowercase()),
                            "[{}] error did not contain {needle:?}; got: {e}",
                            case.id,
                        );
                    }
                    (Expected::ErrorContains(_), Ok(response)) => {
                        let body: Value = response.into_body().into_single().unwrap_or_default();
                        panic!(
                            "[{}] expected error but patch succeeded; body={body}",
                            case.id,
                        );
                    }
                }
            }

            Ok(())
        },
    ))
    .await
}

// ---------------------------------------------------------------------------
// 404 on the read leg propagates: a never-created item surfaces a typed
// NotFound error from the handler without entering the replace leg.
// ---------------------------------------------------------------------------

/// Patching a never-created item id must surface a `NotFound` from the
/// read leg without retries or replace attempts. This pins the handler's
/// "non-412 read error propagates immediately" branch end-to-end against
/// the live emulator (the unit test `rmw_propagates_read_error_immediately`
/// covers the same branch with a scripted dispatcher).
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cosmos_patch_read_missing_item_returns_not_found() -> Result<(), Box<dyn Error>> {
    Box::pin(DriverTestClient::run_with_unique_db(
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Patch a freshly-named item that was never created.
            let missing_id = "patch-missing-item-001";
            let pk = "tenant-a";
            let spec = PatchInstructions::from(vec![PatchOperation::set("/deleted", json!(true))]);
            let err = context
                .patch_item(&container, missing_id, pk, &spec, None)
                .await
                .expect_err("expected NotFound from read leg, got Ok");
            let msg = format!("{err}");
            let lower = msg.to_ascii_lowercase();
            assert!(
                lower.contains("404") || lower.contains("notfound") || lower.contains("not found"),
                "error should be NotFound-shaped (404/NotFound); got: {msg}",
            );

            Ok(())
        },
    ))
    .await
}

// ---------------------------------------------------------------------------
// Fault-injected 412 retry + exhaustion: drive the RMW loop's
// PreconditionFailed branches end-to-end using
// `CustomResponseBuilder::new(StatusCode::PreconditionFailed)` against the
// internal ReplaceItem sub-operation.
// ---------------------------------------------------------------------------

#[cfg(feature = "fault_injection")]
use azure_data_cosmos_driver::fault_injection::{
    CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
#[cfg(feature = "fault_injection")]
use std::sync::Arc;

/// The handler retries on a 412 produced by a concurrent writer and
/// eventually succeeds.
///
/// We force a single synthetic 412 on the internal `ReplaceItem` sub-op via
/// fault injection (hit_limit=1). The handler must observe the 412, re-Read,
/// re-merge, and re-Replace; the second Replace goes to the real emulator
/// and succeeds, so the overall PATCH returns the locally-merged post-image.
#[cfg(feature = "fault_injection")]
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cosmos_patch_412_retry() -> Result<(), Box<dyn Error>> {
    let custom_412 = CustomResponseBuilder::new(azure_core::http::StatusCode::PreconditionFailed)
        .with_body(br#"{"code":"PreconditionFailed","message":"injected 412"}"#.to_vec())
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_custom_response(custom_412)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReplaceItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("patch-412-once", result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    Box::pin(DriverTestClient::run_with_unique_db_and_fault_injection(
        rules,
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_id = "patch-412-retry-001";
            let pk = "p1";
            let initial = json!({ "id": item_id, "pk": pk, "value": 0 });
            context
                .create_seed_item(&container, item_id, pk, &serde_json::to_vec(&initial)?)
                .await?;

            let spec = PatchInstructions::from(vec![PatchOperation::increment("/value", 1i64)]);
            let response = context
                .patch_item(&container, item_id, pk, &spec, None)
                .await?;

            // Post-image reflects the merged increment.
            let body: Value = response.into_body().into_single()?;
            assert_eq!(
                body.get("value"),
                Some(&json!(1)),
                "PATCH post-image should reflect the increment; got {body}",
            );

            // Fault rule was hit exactly once (one 412 was injected, one
            // retry succeeded).
            assert_eq!(
                rule.hit_count(),
                1,
                "fault rule should fire exactly once; hit_count={}",
                rule.hit_count()
            );

            Ok(())
        },
    ))
    .await
}

/// The handler surfaces a `PreconditionFailed` error after exhausting
/// `max_attempts` against a persistently-412'ing backend.
///
/// Fault injection returns a synthetic 412 on every `ReplaceItem`. With
/// `max_attempts(2)` the handler dispatches Read1 -> Replace1 (412) ->
/// Read2 -> Replace2 (412) -> CosmosError.
#[cfg(feature = "fault_injection")]
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cosmos_patch_412_exhaustion() -> Result<(), Box<dyn Error>> {
    let custom_412 = CustomResponseBuilder::new(azure_core::http::StatusCode::PreconditionFailed)
        .with_body(br#"{"code":"PreconditionFailed","message":"injected 412"}"#.to_vec())
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_custom_response(custom_412)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReplaceItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("patch-412-always", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    Box::pin(DriverTestClient::run_with_unique_db_and_fault_injection(
        rules,
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_id = "patch-412-exhaust-001";
            let pk = "p1";
            let initial = json!({ "id": item_id, "pk": pk, "value": 0 });
            context
                .create_seed_item(&container, item_id, pk, &serde_json::to_vec(&initial)?)
                .await?;

            let max_attempts = std::num::NonZeroU8::new(2).unwrap();
            let spec = PatchInstructions::from(vec![PatchOperation::increment("/value", 1i64)]);
            let err = context
                .patch_item(&container, item_id, pk, &spec, Some(max_attempts))
                .await
                .expect_err("PATCH should fail with 412 after exhausting max_attempts");

            // Check the typed status code rather than the message string:
            // the exhaustion error is constructed with status
            // `PreconditionFailed` but its `Display` is the human-readable
            // attempts-count message (not "412" / "PreconditionFailed"), so
            // callers identify the 412 via `CosmosError::status_code()`. The
            // framework wraps the driver's `crate::error::Error` in a
            // `Box<dyn Error>` via `?`, so downcast to recover the typed
            // accessor.
            let cosmos_err = err
                .downcast_ref::<azure_data_cosmos_driver::error::CosmosError>()
                .expect("framework wraps an azure_data_cosmos_driver::error::CosmosError from execute_operation");
            assert_eq!(
                cosmos_err.status().status_code(),
                azure_core::http::StatusCode::PreconditionFailed,
                "exhausted error should be a 412 / PreconditionFailed; got: {err}",
            );

            // Fault rule fired once per attempt (`max_attempts` total).
            assert_eq!(
                rule.hit_count(),
                u32::from(max_attempts.get()),
                "fault rule should fire once per attempt; hit_count={} max_attempts={}",
                rule.hit_count(),
                max_attempts.get()
            );

            Ok(())
        },
    ))
    .await
}
