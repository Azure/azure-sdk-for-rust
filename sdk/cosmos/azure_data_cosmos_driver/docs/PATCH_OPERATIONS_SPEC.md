# Spec: Cosmos PATCH (Partial Document Update) for Rust SDK

**Issue:** [Azure/azure-sdk-for-rust#4048](https://github.com/Azure/azure-sdk-for-rust/issues/4048)
**Authors:** @NaluTripician
**Reviewers (proposed):** @analogrelay, @FabianMeiswinkel
**Status:** Draft (spec-only PR — no code changes)
**Target branch:** `release/azure_data_cosmos-previews`
**Companion artifacts:**
- `.coding-harness/landscape-report.md` (this PR's landscape analysis)
- `.coding-harness/landscape.json` (machine-readable side-effect catalog)

---

## 1. Problem Statement

Cosmos DB has supported partial document updates (the PATCH verb against
`/dbs/{db}/colls/{coll}/docs/{id}`) for years, and every other Cosmos SDK ships
this capability. The Rust SDK had a `ContainerClient::patch_item` method, but
it was removed in **PR #3765** (Aug 2025) because the design did not adequately
address the interaction between PATCH's *conditional* idempotency and the Rust
SDK's *auto-retry-all-writes* default — a naive retry of a `PATCH` that
contained an `increment` operation could silently double-apply the increment.

The driver-side plumbing (`Method::Patch` mapping in auth signing,
`FaultOperationType::PatchItem` in fault-injection, the SDK-side
`OperationType::Patch` legacy enum variant) was deliberately kept in place
because it was understood that PATCH would return once the design was settled.

This spec defines a **phase-1** PATCH design that:

1. Restores the public PATCH surface for the use cases that are unconditionally
   safe under auto-retry (set / replace / remove).
2. Explicitly rejects (at the SDK boundary, before any wire request is issued)
   patch operations that are NOT safe to silently retry (add / increment / move).
3. Aligns with the new driver/SDK split established by PR #4128 and the
   PPAF/PPCB write-retry semantics established by PR #4156.
4. Is forward-compatible with a phase-2 design that will add Read-Modify-Write
   support for the rejected operation kinds.

---

## 2. Requirements

### 2.1 Must

- **M1.** `azure_data_cosmos::ContainerClient::patch_item` exists with the same
  ergonomic shape as `replace_item` / `upsert_item`.
- **M2.** Public types `PatchDocument` and `PatchOperation` are exposed under
  `azure_data_cosmos::models`. Both are `#[non_exhaustive]`.
- **M3.** `PatchDocument` supports the operations:
  - `set(path, value)`
  - `replace(path, value)`
  - `remove(path)`
- **M4.** `PatchDocument` exposes a builder for an optional **filter predicate**
  (the wire-level `condition` field) — a SQL fragment evaluated server-side
  before the patch is applied.
- **M5.** Serialized wire body matches the existing Cosmos PATCH contract
  exactly. In particular:
  - The top-level wire field is `condition` (NOT `filterPredicate`).
  - Operation `op` tokens are: `set`, `replace`, `remove` (lowercase).
  - The serde representation is preserved from the implementation removed in
    PR #3765, recovered at `.coding-harness/removed_patch_operations.rs.diff`.
- **M6.** `OperationType::Patch` is added to the driver-side canonical enum
  (`azure_data_cosmos_driver/src/models/mod.rs`) with:
  - `http_method() == Method::Patch`
  - `is_read_only() == false`
  - `is_idempotent() == true` (justified by M7)
  - `as_str() == "patch"`
  - `body_content_type() == ContentType::APPLICATION_JSON`
- **M7.** `PatchDocument` cannot be constructed in a way that yields a
  non-idempotent wire request. Specifically, the public builder API exposes
  **only** `set` / `replace` / `remove` in phase 1. Add / increment / move
  cannot be expressed at all (no method exists). This is the mechanism by
  which M6's `is_idempotent() == true` is sound.
- **M8.** Driver-side fault injection: the
  `(OperationType::Patch, ResourceType::Document) => FaultOperationType::PatchItem`
  mapping is added in `azure_data_cosmos_driver/src/fault_injection/mod.rs`,
  resolving the pre-existing TODO at line 191.
- **M9.** Per-call options support `if_match_etag` (existing
  `ItemWriteOptions` pattern) so customers can build classic
  Read-Modify-Replace-with-PATCH flows themselves if they need
  add/increment/move.
- **M10.** A `CHANGELOG.md` entry is added announcing the restored API and
  explicitly documenting the phase-1 limitation (no add/increment/move).
- **M11.** A README example is added back demonstrating a `set`-based
  soft-delete patch — the canonical phase-1 use case. The example should
  also exercise `with_filter_predicate` (e.g., "soft-delete only if not
  already deleted") because the realistic OpenAI-style soft-delete
  workload uses both together; this also documents the `condition` /
  `filter_predicate` interlock, and the example should set
  `enable_content_response_on_write` to `false` to show the RU/bandwidth
  optimization soft-delete callers actually want.

### 2.2 Should

- **S1.** The public Rust name for the wire-level `condition` field is
  `filter_predicate` on the options struct (matching .NET's customer-facing
  name) but the serde wire name remains `condition`. Rationale: customers
  reading Cosmos cross-SDK docs see `FilterPredicate`; we preserve that
  vocabulary while still emitting the correct wire token.
- **S2.** A doc comment on `PatchDocument` (or a sibling type) calls out, in
  the rustdoc itself, that add / increment / move are deferred to a future
  release and links to issue #4048. This keeps the trade-off visible to anyone
  reading docs.rs.
- **S3.** PATCH integration tests are added covering: basic set, set with
  filter predicate matching, set with filter predicate not matching, set with
  if-match etag matching and not matching, remove of an absent path
  (server-side error path), patch of a nonexistent item.

### 2.3 Nice-to-have

- **N1.** A doc-test on `ContainerClient::patch_item` showing the soft-delete
  use case (set `/deleted` to `true`).
- **N2.** README cross-link to the .NET PATCH docs page so users discovering
  PATCH through the .NET docs land on the Rust equivalent.

---

## 3. Acceptance Criteria

A reviewer can confirm "done" by checking:

- [ ] **AC1.** A new test in `tests/emulator_tests/cosmos_items.rs` named
  something like `item_patch_set_succeeds` performs a PATCH with a single
  `set` operation and verifies the document was updated.
- [ ] **AC2.** A test `item_patch_with_filter_predicate_no_match` verifies that
  a PATCH whose `condition` does not match the document returns the documented
  precondition-failed error.
- [ ] **AC3.** A test `item_patch_with_etag_no_match` verifies that
  `if_match_etag` not matching yields a precondition-failed error and the
  document is unchanged.
- [ ] **AC4.** A unit test verifies the serialized wire body of a multi-op
  `PatchDocument` matches the byte-exact expected JSON, including the
  operation order and the `condition` wire field name.
- [ ] **AC5.** `cargo doc -p azure_data_cosmos` succeeds and the rustdoc for
  `PatchDocument` includes the phase-1 limitation note (S2).
- [ ] **AC6.** `OperationType::Patch::is_idempotent()` returns `true` and is
  exercised by an existing or new unit test.
- [ ] **AC7.** `cargo clippy -p azure_data_cosmos -p azure_data_cosmos_driver
  --all-features --all-targets` passes with no new warnings.
- [ ] **AC8.** No public symbol exists by which an `add`, `increment` (`incr`),
  or `move` operation can be expressed by a customer. (Demonstrable by `rg
  with_increment|with_add|with_move sdk/cosmos/azure_data_cosmos/src` returning
  no hits.)
- [ ] **AC9.** Fault-injection: a unit test in the driver fault-injection
  module verifies that an `OperationType::Patch` op against
  `ResourceType::Document` matches a `FaultOperationType::PatchItem` rule.
- [ ] **AC10.** Unit test `patch_document_serializes_remove_operation_without_value`
  asserts byte-exact equality with `{"op":"remove","path":"/foo"}` — the
  `value` key must not appear at all, even as `null`.
- [ ] **AC11.** Compile-time guardrail test
  `all_patch_variants_are_idempotent_or_phase2_required` exists in
  `azure_data_cosmos/src/models/patch_operations.rs` and uses an
  exhaustive `match` over `PatchOperation` (no `_` arm); a comment on the
  match references this spec §4.4.
- [ ] **AC12.** Integration test
  `item_patch_with_etag_and_filter_predicate_both_specified_evaluates_as_conjunction`
  exists, exercising all four combinations of etag and predicate
  match/no-match, pinning the conjunction semantics.
- [ ] **AC13.** Integration test `item_patch_already_soft_deleted_is_idempotent`
  exists, verifying end-to-end idempotency of the headline soft-delete
  use case.

---

## 4. Technical Approach

### 4.1 Type model

```rust
// in azure_data_cosmos/src/models/patch_operations.rs (new file)

/// A set of partial-update operations to apply to a single document.
#[derive(Clone, Debug, Default, serde::Serialize)]
#[non_exhaustive]
pub struct PatchDocument {
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    condition: Option<Cow<'static, str>>,
    operations: Vec<PatchOperation>,
}

/// A single partial-update operation. Phase 1 supports only the
/// idempotent subset of Cosmos PATCH operations.
///
/// Add / increment / move are deferred (issue #4048).
#[derive(Clone, Debug, serde::Serialize)]
#[serde(tag = "op", rename_all = "lowercase")]
#[non_exhaustive]
pub enum PatchOperation {
    Set     { path: Cow<'static, str>, value: serde_json::Value },
    Replace { path: Cow<'static, str>, value: serde_json::Value },
    Remove  { path: Cow<'static, str> },
}

impl PatchDocument {
    pub fn new() -> Self { Self::default() }
    pub fn with_set<P, V>(self, path: P, value: V) -> Result<Self, serde_json::Error>
        where P: Into<Cow<'static, str>>, V: serde::Serialize { ... }
    pub fn with_replace<P, V>(self, path: P, value: V) -> Result<Self, serde_json::Error>
        where P: Into<Cow<'static, str>>, V: serde::Serialize { ... }
    pub fn with_remove<P>(self, path: P) -> Self
        where P: Into<Cow<'static, str>> { ... }
    pub fn with_filter_predicate<C>(self, condition: C) -> Self
        where C: Into<Cow<'static, str>> { ... }
}
```

Notes:
- Serde derives match the implementation removed in PR #3765 — see
  `.coding-harness/removed_patch_operations.rs.diff` for the exact prior shape.
  The prior code used `#[serde(rename_all = "camelCase")]`; this spec uses
  `#[serde(rename_all = "lowercase")]` because for the phase-1 single-word
  variants `Set` / `Replace` / `Remove`, both produce identical wire output
  (`"set"`, `"replace"`, `"remove"`) and `lowercase` is clearer-in-intent
  for the phase-1 subset. **If phase 2 restores `Increment`, neither
  variant of `rename_all` produces the correct wire token `"incr"` —
  phase 2 must add an explicit `#[serde(rename = "incr")]` on the
  `Increment` variant**, exactly as the prior code did.
- `with_set` / `with_replace` return `Result` so the `serde_json::to_value`
  conversion on the user's value can fail without panicking. (This matches the
  prior API's design intent.)
- The trait `ToJsonNumber` from the prior code is **not** restored — it was
  only used by `with_increment`, which is deferred.

### 4.2 Public method on `ContainerClient`

```rust
pub async fn patch_item<T: DeserializeOwned>(
    &self,
    partition_key: impl Into<PartitionKey>,
    item_id: &str,
    patch: PatchDocument,
    options: Option<PatchItemOptions<'_>>,
) -> Result<Response<Item<T>, JsonFormat>>
```

Mirrors `replace_item`'s shape exactly.

**`PatchItemOptions` (concrete shape).** A new type that **inherits from
`ItemWriteOptions` by composition with delegated builders**, mirroring
.NET's `PatchItemRequestOptions : ItemRequestOptions` and Java's
`CosmosPatchItemRequestOptions extends CosmosItemRequestOptions`:

```rust
#[derive(Clone, Debug, Default)]
pub struct PatchItemOptions<'a> {
    /// Underlying write options shared with create_item / replace_item /
    /// upsert_item / delete_item. All fields apply to PATCH unchanged
    /// unless documented otherwise below.
    pub item_options: ItemWriteOptions<'a>,
    // Phase 1 has no patch-only options. The wire-level `condition`
    // (filter predicate) lives on PatchDocument, not on this struct,
    // because it is part of the patch body, not a request modifier.
}

impl<'a> PatchItemOptions<'a> {
    pub fn new() -> Self { Self::default() }

    /// Convenience pass-through. Equivalent to
    /// `self.item_options.if_match_etag = Some(etag)`.
    pub fn with_if_match_etag(mut self, etag: impl Into<Cow<'a, str>>) -> Self {
        self.item_options = self.item_options.with_if_match_etag(etag);
        self
    }

    // ... pass-throughs for the other ItemWriteOptions fields enumerated
    // in the table below as needed for ergonomics.
}
```

Customer use:
```rust
container
    .patch_item(
        partition_key,
        "doc-1",
        PatchDocument::new().with_set("/deleted", true)?,
        Some(PatchItemOptions::new().with_if_match_etag(etag)),
    )
    .await?;
```

**`ItemWriteOptions` fields and their applicability to PATCH.** All
existing fields apply to PATCH with the semantics described in the .NET
`PatchItemRequestOptions` documentation:

| `ItemWriteOptions` field | Applies to PATCH? | Notes |
|---|---|---|
| `if_match_etag` | yes | Optimistic concurrency. Composes with `PatchDocument::filter_predicate` as a server-side conjunction. |
| `if_none_match_etag` | yes | Same as for replace_item. |
| `enable_content_response_on_write` (or its existing Rust equivalent) | yes | High-throughput soft-delete callers should set this to `false` to skip echoing the document body, materially reducing RU/bandwidth. Worth highlighting in the README example. |
| `consistency_level` (per-call override) | yes | Same as replace_item. |
| `session_token` | yes | Patches consume and produce session tokens like any other write. |
| `indexing_directive` | yes | Same as replace_item. |
| `pre_triggers` / `post_triggers` (if exposed) | yes | Patch supports triggers identically to replace. |

If any of these fields does *not* exist on the current `ItemWriteOptions`,
that gap is pre-existing and out of scope for this spec; the table
documents intent so an implementer adding the field later does not
re-litigate the question.

### 4.3 Driver wiring

1. Add `Patch` variant to
   `azure_data_cosmos_driver::models::OperationType` with the property values
   listed in M6.
2. Add a `CosmosOperation::patch_item(reference, patch_document) -> CosmosOperation`
   factory in `azure_data_cosmos_driver::models::cosmos_operation` following the
   existing `replace_item` factory pattern (line ~441).
3. Add the `(OperationType::Patch, ResourceType::Document)` arm in
   `azure_data_cosmos_driver::fault_injection::mod.rs` (currently TODO at L191).

### 4.4 Idempotency soundness argument

The Cosmos PATCH wire request produced by a `PatchDocument` constructed via
the public phase-1 API is **unconditionally idempotent at the document
level**: because `PatchOperation` is a closed `#[non_exhaustive]` enum that
exposes only `Set` / `Replace` / `Remove` and exposes neither a
`Deserialize` impl nor a public constructor for any other variant, every
operation in the body has the property that applying it twice yields the
same final document state as applying it once.

This soundness argument is **load-bearing for retry safety**, and it must be
sound *regardless of which retry pathway in the driver fires*. The driver
retry surface is broader than just `is_idempotent()`. As of HEAD,
`azure_data_cosmos_driver/src/driver/pipeline/retry_evaluation.rs` contains
**at least four** code paths where a write may be retried:

1. **The HTTP retry-trigger group helper** at lines 406–408 evaluates
   ```rust
   let safe_to_retry = operation.is_read_only()
       || operation.is_idempotent()
       || retry_state.ppaf_write_retry_allowed;
   ```
2. **The transport-error path** at lines 523–525 evaluates the *same*
   three-way OR. (These are not two distinct retry policies; they are the
   same gate consulted at two different call sites.)
3. **`try_handle_server_error` at lines 457–486** — fires on any 5xx (and
   408 fallback). Its own doc comment (line 455) reads: *"Cross-region
   retry is attempted for both reads and writes — the assumption is that
   an internal error in one region is unlikely to repeat in another."*
   **There is no `is_idempotent()` check.**
4. **`try_handle_write_forbidden` at lines 289–318** — fires on 403/3
   WriteForbidden. Cross-region retry is unconditional whenever the
   failover budget allows. **There is no `is_idempotent()` check.**

Two consequences fall out of this:

- `ppaf_write_retry_allowed` is **not** "only true when the op is already
  retry-safe." Per its doc comment in `pipeline/components.rs:92`,
  *"PPAF allows non-idempotent write retries on failover"* — it was
  introduced precisely so that non-idempotent writes (`Create`, `Upsert`)
  can be cross-region retried. Reading the OR as "PPAF is gated by
  `is_idempotent()`" inverts the design.
- The `is_idempotent() == true` claim on `OperationType::Patch` is
  **necessary but not sufficient** for retry safety. The 5xx and 403/3
  pathways will retry the request whether `is_idempotent()` returns true
  or false. **The phase-1 design is sound because the wire body itself
  is idempotent under any retry pathway**, not because we believe the
  retry pathways are gated by `is_idempotent()`.

**Why this matters for phase 2:** A phase-2 design that adds non-idempotent
operations cannot make the request safe by simply downgrading
`is_idempotent()` to `false` — the 5xx, 403/3, and PPAF pathways will still
fire. Phase 2 must either (a) make the wire body idempotent regardless of
the operation list (the RMW approach, which produces an `If-Match`-gated
`Replace` rather than a PATCH), or (b) restructure the driver retry surface
itself.

**Maintainer guardrail.** Because `OperationType::is_idempotent` takes
`self` by value with no payload information, the soundness invariant lives
at the `PatchOperation` enum, not at the driver enum. The mitigation is an
**exhaustive in-crate test** (Rust's `#[non_exhaustive]` does NOT suppress
exhaustiveness checks inside the defining crate):

```rust
// in azure_data_cosmos/src/models/patch_operations.rs (defining crate)
#[test]
fn all_patch_variants_are_idempotent_or_phase2_required() {
    fn assert_idempotent(op: &PatchOperation) {
        match op {
            PatchOperation::Set { .. } => (),
            PatchOperation::Replace { .. } => (),
            PatchOperation::Remove { .. } => (),
            // ⛔ Adding a variant here without phase-2 retry redesign breaks
            //   `OperationType::Patch::is_idempotent() == true` in the driver
            //   AND remains unsafe under the 5xx, 403/3, and PPAF retry paths
            //   in retry_evaluation.rs. See PATCH_OPERATIONS_SPEC.md §4.4.
        }
    }
    let _ = assert_idempotent;
}
```

Adding any new variant to `PatchOperation` will fail this test at compile
time, forcing the phase-2 contributor to confront the invariant. This
replaces the weaker "doc comment" mitigation originally proposed for
SE-001.

### 4.5 Two `OperationType` enums

The legacy SDK-side `OperationType::Patch` (in
`azure_data_cosmos/src/operation_context.rs:21`) is left as-is. The
driver-side enum is the canonical one going forward; this spec adds the
`Patch` variant *to the driver enum only*.

The closest precedent — PR #4128, which cut `replace_item` / `upsert_item`
/ `delete_item` over to the driver — established a "**don't touch the
legacy enum**" posture: when an operation moves to the driver, callers are
re-routed through the driver's existing canonical variant, and the legacy
enum's variant is left in place even when call sites become dead. PATCH's
situation is materially different on one axis: PR #4128 added zero new
variants to either enum (Replace / Upsert / Delete already existed in
both), whereas this spec adds `Patch` to the driver enum for the first
time. The "leave the legacy enum alone" half is the precedent we are
following; the "add to the driver enum" half is a new decision being made
here, defensible because (a) every other driver-side write operation
already lives in that enum, and (b) without the variant, the
fault-injection mapping (SE-003) and the `is_idempotent()` retry gate
(§4.4) cannot be expressed.

Reconciliation of the two enums is tracked as a separate follow-up.

---

## 5. Files to Modify / Create

| Path | Action | Notes |
|---|---|---|
| `sdk/cosmos/azure_data_cosmos/src/models/patch_operations.rs` | **create** | `PatchDocument`, `PatchOperation` |
| `sdk/cosmos/azure_data_cosmos/src/models/mod.rs` | modify | re-export |
| `sdk/cosmos/azure_data_cosmos/src/clients/container_client.rs` | modify | `patch_item` method (~near `replace_item` at L424) |
| `sdk/cosmos/azure_data_cosmos/src/options/item_options.rs` (or wherever `ItemWriteOptions` lives) | modify | add `PatchItemOptions` newtype if needed |
| `sdk/cosmos/azure_data_cosmos_driver/src/models/mod.rs` | modify | add `OperationType::Patch` variant + match arms in `body_content_type`, `is_read_only`, `is_idempotent`, `http_method`, `as_str` |
| `sdk/cosmos/azure_data_cosmos_driver/src/models/cosmos_operation.rs` | modify | `CosmosOperation::patch_item` factory |
| `sdk/cosmos/azure_data_cosmos_driver/src/fault_injection/mod.rs` | modify | add `(Patch, Document) => PatchItem` arm — resolves L191 TODO |
| `sdk/cosmos/azure_data_cosmos/CHANGELOG.md` | modify | new entry under Unreleased |
| `sdk/cosmos/azure_data_cosmos/README.md` | modify | add soft-delete patch example |
| `sdk/cosmos/azure_data_cosmos/tests/emulator_tests/cosmos_items.rs` | modify | re-add `item_patch_*` tests |
| `sdk/cosmos/azure_data_cosmos/tests/framework/mod.rs` (if needed) | modify | helpers for filter-predicate tests |

---

## 6. Test Strategy

### 6.1 Unit tests (in `azure_data_cosmos`)

| Test | Scenario | Verifies |
|---|---|---|
| `patch_document_serializes_set_operation_with_lowercase_op_token` | Build a PatchDocument with one Set; serialize | Wire `op` token is `"set"` (SE-004) |
| `patch_document_serializes_filter_predicate_under_condition_field` | Set a filter predicate, serialize | Wire JSON contains `"condition"`, NOT `"filterPredicate"` (SE-005) |
| `patch_document_omits_condition_when_not_set` | No filter predicate set, serialize | `condition` is absent from JSON |
| `patch_document_serializes_remove_operation_without_value` | One Remove op, serialize | **Asserts byte-exact equality** with `{"op":"remove","path":"/foo"}`. The `value` key must not appear at all, even as `null`. (Java's `PatchUtil` emits `"value": null` under default Jackson settings; we explicitly diverge from that.) |
| `patch_document_preserves_operation_order` | Build A then B then C, serialize | Operations array is in insertion order |
| `all_patch_variants_are_idempotent_or_phase2_required` | Exhaustive `match` over `PatchOperation` (no `_` arm); see §4.4 | Compile-time guardrail: adding a non-idempotent variant breaks this test, forcing a phase-2 redesign before the variant can ship. |

### 6.2 Unit tests (in `azure_data_cosmos_driver`)

| Test | Verifies |
|---|---|
| `operation_type_patch_http_method_is_patch` | M6 mapping correctness |
| `operation_type_patch_is_idempotent` | M6 + 4.4 |
| `operation_type_patch_body_content_type_is_json` | M6 |
| `cosmos_operation_patch_item_targets_documents_resource` | Factory routes correctly |
| `fault_injection_patch_op_matches_patch_item_rule` | M8 + AC9 |

### 6.3 Integration tests (emulator)

| Test | Scenario |
|---|---|
| `item_patch_set_succeeds` | AC1 |
| `item_patch_replace_succeeds` | Mirror of set for replace op |
| `item_patch_remove_succeeds` | Mirror for remove op |
| `item_patch_with_filter_predicate_match_succeeds` | Condition holds → patch applies |
| `item_patch_with_filter_predicate_no_match_returns_precondition_error` | AC2 (server returns 412 — confirm in implementation) |
| `item_patch_with_etag_no_match_returns_precondition_error` | AC3 |
| `item_patch_on_nonexistent_item_returns_not_found` | Standard 404 path |
| `item_patch_response_contains_updated_document` | Default behavior returns the patched body |
| `item_patch_with_etag_and_filter_predicate_both_specified_evaluates_as_conjunction` | Pins SDK behavior when both preconditions are set: etag-match + predicate-match should succeed; etag-match + predicate-no-match should fail (412); etag-no-match + predicate-match should fail (412). The Cosmos service evaluates these as a server-side conjunction; this test catches any future divergence. |
| `item_patch_already_soft_deleted_is_idempotent` | Headline OpenAI use case: PATCH-set `/deleted = true` on a document where `/deleted` is already `true`. Verifies end-to-end idempotency of the soft-delete pattern (the case retry-safety is most concerned with). |
| `item_patch_during_partition_split_routes_correctly` | Same blast surface as `replace_item`'s split test. May be covered by parity (i.e., re-using the existing split-test harness); spec calls out the scenario so it is not silently dropped. |
| `item_patch_with_circuit_breaker_open_fails_fast_or_routes_to_alternate_region` | Exercises PPCB interaction (relevant given the PPCB code paths flagged in §4.4). May be deferred to the implementation PR with explicit rationale, but spec calls it out. |

### 6.4 Doc tests

- `ContainerClient::patch_item` doc comment includes a runnable example showing
  the soft-delete pattern (N1).

### 6.5 Negative space (intentional non-tests)

- No test exercises `add`/`increment`/`move` because they cannot be
  constructed. The compile-time absence of those constructors is itself
  the test (AC8).

---

## 7. Alternatives Considered

> Required by overlay §6 — at least two alternatives with explicit rejection
> rationale.

### 7.1 Alternative A — Support all PATCH ops with a Read-Modify-Write loop for non-idempotent ones

**Sketch:** When a `PatchDocument` contains `add`/`increment`/`move`, the
driver issues a `Read` to capture an ETag, applies the patch client-side,
and issues a `Replace` with `If-Match: <etag>` — looping on
`PreconditionFailed` for retries.

**Why rejected for phase 1:**
- **Scope.** This is essentially a second item-write codepath inside the
  driver, with its own retry budget, RU-cost story, ETag-conflict-loop
  semantics, and observability surface. Designing it correctly is at least
  the same magnitude of work as the rest of this spec combined.
- **Wire RU comparison.** A native PATCH consumes server-side RU based on
  the partial update; an RMW consumes RU for a read + a full replace,
  potentially making `increment` materially more expensive than customers
  expect from cross-SDK comparison.
- **Behavioral surprise.** Customers comparing the Rust SDK to .NET would
  see different RU consumption and different retry behavior under
  contention for the *same* user-level call. This is the kind of silent
  divergence overlay §6 explicitly warns about.
- **Phase 2 path is preserved.** Choosing phase-1 disallow does not
  preclude phase 2 RMW; the public enum is `#[non_exhaustive]`, so
  Add/Increment/Move can be added later without breaking any caller.

This is the path we will take in **phase 2**, tracked via a new follow-up
issue. Doing it here would gate the entire restoration on a much larger
design problem.

### 7.2 Alternative B — Support all PATCH ops; downgrade `is_idempotent()` to `false` per-call when a non-idempotent op is present

**Sketch:** Allow Add/Increment/Move in `PatchOperation`. The driver
inspects the operation list and sets a per-`CosmosOperation` "treat as
non-idempotent" flag, threaded into `retry_evaluation.rs`.

**Why rejected:**
- **Loses the auto-retry safety net for the very calls that need it most.**
  Increment-style workloads (counters, telemetry rollups) are typically the
  ones that benefit from automatic recovery from transient transport errors.
  An explicit `false` says "the SDK won't retry — your code must retry on
  every retryable transport class". That's a significantly worse default
  than competing SDKs.
- **At least four retry pathways would need to be gated, not one.** Per
  §4.4, `is_idempotent()` only gates two of the at-least-four retry paths
  in `retry_evaluation.rs`. PPAF (`ppaf_write_retry_allowed`) was
  explicitly designed to enable retry of non-idempotent writes, so it
  bypasses the flag. `try_handle_server_error` (5xx and 408) and
  `try_handle_write_forbidden` (403/3) perform unconditional cross-region
  retry. Defeating all four pathways for non-idempotent patches turns a
  one-line change into a cross-cutting retry-policy redesign — far larger
  than the RMW alternative (A).
- **The `OperationType` enum currently has no per-instance state** — it's
  `Copy`. Threading "is this *specific* patch op idempotent" requires a
  different signature (`fn is_idempotent(&self, payload: &Body) -> bool`)
  that touches every caller.

### 7.3 Alternative C — Support all ops behind an opt-in `unsafe_allow_non_idempotent_patch` flag

**Sketch:** Same as B but the customer must explicitly opt in via a
client-level or call-level flag.

**Why rejected:**
- A `unsafe_*` flag is a smell that the underlying primitive isn't
  designed yet. We'd be making the customer responsible for a correctness
  decision the SDK should be making.
- The flag would need to disable both `is_idempotent()` and the PPAF
  cross-region retry — same complexity as B.
- We have no use case: Ashley confirmed the largest customer (OpenAI) uses
  `set` for soft-delete, which is already covered by phase 1.

### 7.4 Alternative D — Restore the exact API from before #3765 unchanged (with `with_increment`/`with_add`/`with_move`)

**Sketch:** Revert the deletion verbatim and pretend the idempotency
question is solved.

**Why rejected:**
- This is precisely what reviewers in #3765 said was unacceptable. It
  would also make `is_idempotent() == true` unsound, which would silently
  double-apply increments under retry — exactly the failure mode that
  prompted the removal.
- It is not a defensible answer to "what did you do about the original
  concern in #3765?"

---

## 8. Risk Assessment

Pulled directly from `landscape.json`.

| ID | Severity | Risk | Mitigation |
|---|---|---|---|
| **SE-001** | 🟡 potential | Idempotency × auto-retry — naive PATCH retry can double-apply increments | **Phase-1 mitigation:** the public API does not expose `add`/`increment`/`move`. The wire request is therefore unconditionally idempotent at the document level, which is what is required for all four+ retry pathways in `retry_evaluation.rs` (the standard `is_idempotent()` gate, PPAF non-idempotent write retry, `try_handle_server_error` 5xx cross-region, and `try_handle_write_forbidden` 403/3 cross-region). **Maintainer guardrail:** an exhaustive `match` test in the defining crate (§4.4) fails to compile when a new `PatchOperation` variant is added, forcing a phase-2 redesign before any non-idempotent op can ship. Phase-2 RMW design is tracked as a follow-up. |
| **SE-002** | 🟡 potential | Two `OperationType` enums diverge | Spec explicitly adds the `Patch` variant to the driver-side enum (canonical) and leaves the SDK-side legacy enum's existing `Patch` variant as-is. Reconciliation tracked as a separate follow-up; out of scope for this PR. Verified consistent with replace/upsert/delete cutover precedent (#4128). |
| **SE-003** | 🟢 minor | Driver fault-injection PatchItem mapping is TODO | Resolved in this change (M8 / AC9). The pre-existing TODO comment at `fault_injection/mod.rs:191` is removed in the same commit that adds the mapping. |
| **SE-004** | 🟡 potential | Wire op tokens are NOT RFC 6902 (would-be `incr` is irrelevant since increment is gone in phase 1, but `set` vs `add` distinction still matters) | Unit test (`patch_document_serializes_set_operation_with_lowercase_op_token`) asserts byte-exact wire body. Spec §4.1 references the prior implementation as the source of truth for serde attributes. |
| **SE-005** | 🟢 minor | Wire field is `condition`, public Rust name is `filter_predicate` | Documented in M5 + S1. Unit test (`patch_document_serializes_filter_predicate_under_condition_field`) asserts it. |
| **SE-006** | 🟢 minor | Transactional batch PatchItem unsupported | Documented as out of scope. Follow-up issue to be filed. |

**Confidence gap from landscape (one):** server response code on filter-predicate
mismatch is *assumed* to be 412 Precondition Failed. Verified during
implementation phase against the previews emulator. Does not block this spec.

---

## 9. Decision Log

> Filled in during implementation. Empty at spec time.

| # | Decision | Alternatives | Rationale |
|---|---|---|---|
| | | | |

---

## 10. Out of Scope

- **Phase-2 RMW for non-idempotent operations.** Tracked as follow-up.
  Phase 2 must satisfy these constraints inherited from phase 1:
  1. Existing `set` / `replace` / `remove` callers must not break.
  2. `OperationType::Patch::is_idempotent() == true` cannot be flipped
     for phase-1 patches without a major-version bump.
  3. Document-level wire-body idempotency is the property required by
     all four+ retry pathways in `retry_evaluation.rs` (§4.4); a phase-2
     design that simply downgrades `is_idempotent()` will not be safe.
  4. The exhaustive-match guardrail test (§4.4 / §6.1) will fail to
     compile when phase 2 adds new `PatchOperation` variants — this is
     intentional, and forces phase 2 to consciously address (1)–(3)
     rather than silently regressing.
  In practice phase 2 is likely to take one of: (a) per-call dispatcher
  that converts non-idempotent patches into a Read + If-Match Replace
  before they reach the retry surface; (b) a separate
  `OperationType::PatchRmw` variant with its own `is_idempotent` value;
  or (c) a refactor of `OperationType::is_idempotent` to accept
  per-instance state. This spec takes no position among them.
- **Transactional batch PatchItem.** Tracked as follow-up (SE-006).
- **Reconciling SDK-side and driver-side `OperationType` enums** (SE-002).
- **PATCH for stored procedures / UDFs / triggers.** Cosmos PATCH is
  document-only.
- **Microbenchmarks of PATCH RU/latency.** Worth doing once phase 2 lands.

---

## 11. Open Questions for Reviewers

1. *(For @analogrelay)* Confirm the phase-1 disallow approach for
   add/increment/move is the right trade-off, or whether RMW should ship
   in phase 1.
2. *(For @FabianMeiswinkel)* Confirm that for phase-1 `PatchDocument`
   (`set` / `replace` / `remove` only), it is safe for any retry pathway
   in `retry_evaluation.rs` — including the unconditional 5xx / 403/3
   cross-region retries (`try_handle_server_error`, `try_handle_write_forbidden`)
   and the PPAF non-idempotent-write retry path — to re-issue a request
   whose body has already been applied in another region. The spec's
   soundness argument (§4.4) is that document-level wire-body idempotency
   is the load-bearing property; the existing retry pathways are *not*
   gated by `is_idempotent()`. Please flag any phase-1 retry pathway
   where this property is insufficient.
3. *(For Cosmos service team)* Confirm the precise error code returned
   when a filter predicate does not match (assumed 412 Precondition
   Failed).
4. *(Cross-SDK alignment)* Java exposes `CosmosBatchPatchItemRequestOptions`
   for batch PATCH; the Rust spec defers transactional-batch PATCH to
   follow-up (SE-006). Confirm this is acceptable for phase 1.
