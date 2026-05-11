# `cosmos_patch_cross_sdk_parity` Fixture Coverage

This file documents the fixture catalog driven by
`fixtures()` in `driver_patch.rs::cosmos_patch_cross_sdk_parity`.

Each row in the catalog is exercised end-to-end against the emulator: we
seed the document, call `DriverTestClient::patch_item`, and assert either a
matching post-image or an error substring. The expected behavior for each
fixture is derived from running (or reading) the equivalent test in the
.NET and Java Cosmos SDKs, so a green run here means the Rust driver
matches the observable PATCH semantics of those SDKs for the covered cases.

| # | Fixture ID | source_test_id | op_kind | scenario_category | expected_outcome_kind |
|--:|---|---|---|---|---|
|  1 | `set-nested-array-string`        | `CosmosItemTests.ItemPatchSuccessTest`                      | Set       | nested        | post_image |
|  2 | `add-nested-array-string`        | `CosmosItemTests.ItemPatchSuccessTest`                      | Add       | nested        | post_image |
|  3 | `remove-root-scalar`             | `CosmosItemTests.ItemPatchSuccessTest`                      | Remove    | happy_path    | post_image |
|  4 | `replace-int-scalar`             | `CosmosItemTests.ItemPatchSuccessTest`                      | Replace   | happy_path    | post_image |
|  5 | `set-null-object-property`       | `CosmosItemTests.ItemPatchSuccessTest`                      | Set       | null_value    | post_image |
|  6 | `add-int-to-array-child`         | `CosmosItemTests.ItemPatchSuccessTest`                      | Add       | nested        | post_image |
|  7 | `set-null-on-business-key`       | `CosmosItemTests.ItemPatchSuccessTest`                      | Set       | null_value    | post_image |
|  8 | `mixed-set-remove-replace`       | `CosmosItemTests.ItemPatchSuccessTest`                      | Mixed     | multi_op      | post_image |
|  9 | `move-chain-three-steps`         | `CosmosItemTests.ItemPatchSuccessTest`                      | Move      | multi_op      | post_image |
| 10 | `add-nonexistent-parent-fails`   | `CosmosItemTests.ItemPatchFailureTest`                      | Add       | missing_path  | error      |
| 11 | `remove-missing-leaf-fails`      | `CosmosItemTests.ItemPatchFailureTest`                      | Remove    | missing_path  | error      |
| 12 | `replace-missing-path-fails`     | `CosmosItemTests.ItemPatchFailureTest`                      | Replace   | missing_path  | error      |
| 13 | `move-missing-source-fails`      | `CosmosItemTests.ItemPatchFailureTest`                      | Move      | missing_path  | error      |
| 14 | `construct-add-string`           | `PatchOperationTests.ConstructPatchOperationTest`           | Add       | happy_path    | post_image |
| 15 | `construct-add-datetime-as-string` | `PatchOperationTests.ConstructPatchOperationTest`         | Add       | happy_path    | post_image |
| 16 | `construct-add-complex-object`   | `PatchOperationTests.ConstructPatchOperationTest`           | Add       | happy_path    | post_image |
| 17 | `construct-replace-array`        | `PatchOperationTests.ConstructPatchOperationTest`           | Replace   | happy_path    | post_image |
| 18 | `construct-set-guid-string`      | `PatchOperationTests.ConstructPatchOperationTest`           | Set       | happy_path    | post_image |
| 19 | `construct-set-null`             | `PatchOperationTests.ConstructPatchOperationTest`           | Set       | null_value    | post_image |
| 20 | `increment-by-float-7`           | `PatchOperationTTests.CastPatchOperationTest`               | Increment | happy_path    | post_image |
| 21 | `increment-by-int-40`            | `PatchOperationTTests.CastPatchOperationTest`               | Increment | happy_path    | post_image |
| 22 | `java-set-null-scalar`           | `PatchAsyncTest.itemPatchSuccessForNullValue`               | Set       | null_value    | post_image |
| 23 | `java-add-null-scalar`           | `PatchAsyncTest.itemPatchSuccessForNullValue`               | Add       | null_value    | post_image |
| 24 | `java-replace-null-scalar`       | `PatchAsyncTest.itemPatchSuccessForNullValue`               | Replace   | null_value    | post_image |
| 25 | `increment-i64-large-value-fidelity` | rust-derived (R7 i64 fidelity)                          | Increment | i64_fidelity  | post_image |
| 26 | `increment-i64-negative`         | rust-derived (R7 i64 fidelity)                              | Increment | i64_fidelity  | post_image |
| 27 | `array-append-dash`              | rust-derived (RFC 6901 array `-`)                           | Add       | array_append  | post_image |
| 28 | `array-index-out-of-range-fails` | rust-derived (RFC 6902)                                     | Add       | array_idx     | error      |
| 29 | `pointer-escape-tilde-one`       | rust-derived (RFC 6901 ~1)                                  | Set       | pointer_escape| post_image |
| 30 | `pointer-escape-tilde-zero`      | rust-derived (RFC 6901 ~0)                                  | Set       | pointer_escape| post_image |
| 31 | `deep-nesting-five-levels`       | rust-derived (deep nesting)                                 | Set       | deep_nesting  | post_image |
| 32 | `move-scalar-between-paths`      | `CosmosItemTests.ItemPatchSuccessTest` (Move chain)         | Move      | happy_path    | post_image |
| 33 | `add-creates-missing-object-leaf`| rust-derived (Add semantics)                                | Add       | happy_path    | post_image |
| 34 | `set-overwrites-existing-key`    | rust-derived (Set semantics)                                | Set       | happy_path    | post_image |

## Coverage rollup

- **34** total fixtures (≥ 30 required by Phase 4 test plan).
- **By op_kind**: Add ×9, Set ×8, Replace ×5, Remove ×2, Increment ×4,
  Move ×3, Mixed ×1, totalling each row's primary op.
- **By scenario_category**: happy_path ×11, nested ×3, null_value ×7,
  missing_path ×4, multi_op ×2, i64_fidelity ×2, array_append ×1,
  array_idx ×1, pointer_escape ×2, deep_nesting ×1.
- **By expected_outcome_kind**: post_image ×29, error ×5.
- **By source provenance**: .NET `CosmosItemTests` ×13, .NET
  `PatchOperationTests` / `PatchOperationT Tests` ×8, Java
  `PatchAsyncTest` ×3, rust-derived edge-cases ×10.

The "rust-derived" rows cover scenarios the .NET/Java patch surfaces don't
exercise directly (i64 fidelity, RFC 6901 pointer escapes, RFC 6902 array
`-`, deep nesting, semantic edges).

## Verifying against the source SDKs

Each row whose `source_test_id` cites a .NET or Java test can be
re-verified by running that test locally against the Cosmos emulator and
comparing the resulting document body. The Rust fixture's expected
`post_image` (or `error_kind`) reflects what those tests observe today;
updates to the .NET or Java patch surface that change observable behavior
should be mirrored here.
