<!--
  Cross-machine handoff document for the `azure_data_cosmos_driver_native`
  implementation. Pick this up on a fresh checkout to resume where the last
  session left off without losing context.

  Read in this order:
  1. §1 (what / why), §2 (where we are), §3 (next steps).
  2. §4 (open questions / decisions) before making any architectural change.
  3. §5 (gotchas) before running anything locally.
-->

# Copilot session memory — `azure_data_cosmos_driver_native`

**Last updated:** 2026-06-02 (Phase 5 committed locally — Phases 2-5 not yet pushed)
**Active branch:** `users/kundadebdatta/4372_cosmos_driver_native_crate_async_impl`
**Remote:** `origin/users/kundadebdatta/4372_cosmos_driver_native_crate_async_impl` (ahead by 4 commits; push at end of session)
**Workspace path on previous machine:** `D:\stash\azure-sdk-for-rust\` (Windows + PowerShell)

If you are an AI assistant picking this up: read this file end-to-end before
making any changes. It captures decisions that are NOT obvious from the spec
or the code alone.

---

## 1. What we are building

A C ABI wrapper crate (`sdk/cosmos/azure_data_cosmos_driver_native`) on top
of `azure_data_cosmos_driver`, designed for cross-language SDK reuse
(.NET, Java, Go, Python, native C/C++). The wrapper exposes a
schema-agnostic, completion-queue-style async FFI.

**Authoritative design docs:**

- [`sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md`](azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md) — full spec
- [`sdk/cosmos/azure_data_cosmos_driver/docs/ASYNC_INVOCATION_ARCHITECTURE.md`](azure_data_cosmos_driver/docs/ASYNC_INVOCATION_ARCHITECTURE.md) — picture-first overview (5 Mermaid diagrams)

The spec calls for **11 phases** (Phase 0 scaffolding through Phase 10
optional advanced surface). See §8 of the spec for the full plan.

---

## 2. Where we are

### 2.1 Completed commits on this branch

| Commit | Phase | Summary |
|---|---|---|
| `be825b8eb` | Phase 0 — scaffolding | New crate + `Cargo.toml` + `build.rs` (cbindgen) + checked-in `azurecosmosdriver.h` + CMake harness + `cosmos_version` + `cosmos_string_free` + `cosmos_bytes_*`. Workspace plumbing (Cargo member, deny.toml MPL-2.0, dict, cspell). |
| `6c57b6775` | Phase 1 — error + async primitives | `src/error.rs` + `src/runtime.rs` + `src/completion.rs`. 34-variant `CosmosErrorCode`, opaque `cosmos_error_t` with 8 accessors + 16 `is_*` predicates, `cosmos_set_backtrace_options`, Tokio-backed `cosmos_runtime_t` (opaque), `cosmos_cq_*` (create/free/wait/try_wait/wait_batch/wait_writable/shutdown/state/runtime), `cosmos_completion_*` (10 accessors + free), `cosmos_operation_handle_*` (cancel/state/free). **23 Rust tests pass.** |
| `2ef559f40` | Phase 2 — runtime builder | New `src/runtime_builder.rs` exposes opaque `cosmos_runtime_builder_t` with `_new` / `_free` / `_build` + 5 primitive setters (`workload_id`, `correlation_id`, `user_agent_suffix`, `wrapping_sdk_identifier`, `cpu_refresh_interval_ms`). `RuntimeContextInner` grows `Arc<CosmosDriverRuntime>` alongside the Tokio runtime. `Cargo.toml` adds the `rustls` driver feature so `build()` actually constructs an HTTP client. C harness `c_tests/runtime_lifecycle.c`. **35 Rust tests pass (23 + 12 new).** |
| `87c12b047` | Phase 3 — account / database refs + driver options + driver `_blocking` | New `src/account_ref.rs` (`with_master_key` + `_clone` + `_free`), `src/database_ref.rs` (`_create` + `_clone` + `_free`), `src/driver_options.rs` (builder + `_with_preferred_regions` + `_build` + `_free`), `src/driver.rs` (`_get_or_create_blocking` + `_free`). `Cargo.toml` adds `azure_core` + `url` runtime deps (needed for `Secret` + `Url::parse`). C harness `c_tests/account_and_driver_options.c`. **55 Rust tests pass (35 + 20 new; 1 ignored network test).** |
| Phase 4 (pending push) | Phase 4 — partition key builder | New `src/partition_key.rs` ships opaque `cosmos_partition_key_builder_t` with `_new` / `_add_string` / `_add_number` / `_add_bool` / `_add_null` / `_add_undefined` / `_build` / `_free`, plus immutable `cosmos_partition_key_t` with `_empty` (cross-partition sentinel) / `_clone` / `_free` / `_component_count` / `_is_empty`. Pre-validates 3-component cap (driver's `From<Vec<...>>` would panic) and non-finite `f64` (driver's `FiniteF64::new_strict` would panic). C harness `c_tests/partition_key.c`. **70 Rust tests pass (55 + 15 new).** || Phase 5 (pending push) | Phase 5 — operation options + factories + mutators | New `src/operation_options.rs` (opaque `cosmos_operation_options_builder_t` mirroring all 16 `OperationOptions` fields, paired `_with_<field>` / `_clear_<field>` setters, `cosmos_operation_options_t` value type) and `src/operation.rs` (opaque `cosmos_operation_t` value type, 11 ContainerReference-free factories + 8 core mutators + lifecycle). Wires Phase 3 deferral `cosmos_driver_options_builder_with_operation_options`. New `CosmosReadConsistency` and `CosmosContentResponseOnWrite` C enums. C harness `c_tests/operation_construction.c`. **102 Rust tests pass (70 + 32 new).** |
Phases 0 and 1 are pushed to `origin`; **Phase 2 is committed locally but not yet pushed** — push at the end of the session (or after Phase 3 commits). Phase 0 separately landed earlier on the spec branch as commit `a7931b1bb` — that branch (`users/kundadebdatta/4372_cosmos_driver_native_crate_spec`) carries the spec + diagrams + the original Phase 0 commit. The current impl branch re-lands Phase 0 cleanly without depending on the spec branch's git ancestry.

### 2.2 Validation status (Phase 0 + Phase 1 + Phase 2)

| Check | Status |
|---|---|
| `cargo build -p azure_data_cosmos_driver_native` | ✅ clean |
| `cargo fmt -p azure_data_cosmos_driver_native` | ✅ clean |
| `cargo clippy -p azure_data_cosmos_driver_native --all-targets` | ✅ zero wrapper-side warnings (3 pre-existing driver-side warnings in `transport/cosmos_transport_client.rs` + `transport/http_client_factory.rs` are not ours) |
| `cargo doc -p azure_data_cosmos_driver_native --no-deps` | ⚠️ 6 'public-doc links to private item' warnings, all matching the Phase 1 pattern (cross-references useful when reading with `--document-private-items`). Not blocking. |
| `cargo test -p azure_data_cosmos_driver_native` | ✅ 35/35 pass |
| C test harness (`cmake -B build && cmake --build build && ctest`) | ⚠️ not exercised locally — CMake is not installed on the previous Windows machine; CI runs this on Linux/macOS |

### 2.3 What an external SDK can do right now (after Phase 5)

- Phase 0–2: stand up a real `cosmos_runtime_t *`, configure it via
  the builder, create completion queues against it.
- Phase 3: stand up a real `cosmos_driver_t *` against a Cosmos
  endpoint via `cosmos_account_ref_with_master_key` →
  `cosmos_driver_get_or_create_blocking`. Name-based database refs are
  constructible.
- Phase 4: construct any single or hierarchical partition key (up to
  3 components, all 5 value kinds).
- Phase 5: **construct fully-populated `cosmos_operation_t` values**
  for the 11 account/database-scope factories (create / read / delete /
  query / read-all on databases and containers, plus query / read /
  replace on throughput offers). Each operation can be decorated with
  `with_body`, `with_session_token`, `with_activity_id`,
  `with_max_item_count`, precondition (if-match / if-none-match),
  populate-index/query-metrics. Per-driver and per-operation default
  options bags are buildable via `cosmos_operation_options_*` (16
  fields with paired `_with_*` / `_clear_*` setters); the Phase 3
  deferral `cosmos_driver_options_builder_with_operation_options` is
  now wired.
- Operations are still NOT submittable — Phase 6 lands
  `cosmos_driver_submit`, the response surface, and the
  container-rooted factories.

**Phase 6** is the first phase that enables end-to-end CRUD from an
external SDK.

---

## 3. Next steps — Phase 6

**Goal:** end-to-end CRUD. Land the generic `tokio::spawn` → `cq_enqueue` submit pipeline (reusable for Phase 3's deferred async driver creation), the response surface, the container-rooted operation factories (which need `cosmos_container_ref_*`), and the patch-only `with_patch_max_attempts` mutator.

### 3.1 Spec section to implement

[`NATIVE_WRAPPER_SPEC.md` §4.4 (async driver creation) + §4.6 (container/item factories + feed range) + §4.7 (response surface) + §8 Phase 6](azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md). Phase 6 done-when criterion: "Full single-item CRUD passes against the emulator; per §6.2, a 404 read-after-delete surfaces as a completion whose `outcome == ERROR` with `cosmos_error_is_from_wire(err) == true`, `cosmos_error_status_code(err) == 404`, and `cosmos_error_is_not_found(err) == true`."

### 3.2 Surface to add

- **`cosmos_container_ref_*`**:
  - Driver's `ContainerReference::new` is `pub(crate)`-only — the
    public path is `CosmosDriver::resolve_container(db_name,
    container_name).await -> ContainerReference`. The FFI needs a
    `cosmos_driver_resolve_container_*` family (sync + async via the
    new submit pipeline) returning a `cosmos_container_ref_t`.
  - Plus the standard `_clone` / `_free`.
- **`cosmos_feed_range_*`** (spec §4.6.4):
  - `_full(out_fr)` — entire EPK key space.
  - `_for_partition_key(container, pk, out_fr)`.
  - `_clone(fr, out_clone)` / `_free(fr)`.
- **Container/item-scope operation factories** (already specified):
  - `read_container`, `replace_container`, `delete_container`.
  - `read_all_items(c, pk)`, `read_all_items_cross_partition(c)`,
    `query_items(c, feed_range)`, `query_plan_for_features(c, mask)`.
  - Item-scope: `create_item`, `read_item`, `upsert_item`,
    `replace_item`, `delete_item`, `patch_item` (each takes
    container + item_id + partition_key).
  - `batch(container, pk)`.
- **`cosmos_operation_with_patch_max_attempts`** — only meaningful
  on patch operations; returns
  `UNSUPPORTED_OPERATION_FOR_MUTATOR` (4009) on other kinds.
- **`cosmos_driver_submit`** (spec §4.7 / §6.1):
  - Binds to `CosmosDriver::execute_singleton_operation`.
  - Delivers `cosmos_response_t` on `OK` completions, rich
    `cosmos_error_t` on `ERROR` completions, per spec §6.2 (404s and
    other final wire errors arrive as `outcome == ERROR`).
  - Takes ownership of the operation handle's inner `Option<CosmosOperation>`
    via `Option::take`. Subsequent mutators / re-submit return
    `OPERATION_CONSUMED` (4005) per the contract already wired into
    `OperationDescHandle::inner.op`.
- **`cosmos_response_*`** accessors:
  - `_body_view(r) -> cosmos_bytes_view_t` (zero-copy borrow into the
    driver-owned buffer).
  - `_into_body(r) -> cosmos_bytes_t *` (take ownership).
  - `_status(r) -> uint16_t`.
  - `_request_charge(r) -> double`.
  - `_activity_id(r) -> cosmos_string_t *`.
  - `_etag(r) -> cosmos_string_t *`.
  - `_session_token(r) -> cosmos_string_t *`.
  - `_continuation_token(r) -> cosmos_string_t *`.
  - `_take_driver(r, out_driver)` for the degenerate response from
    `cosmos_driver_get_or_create_submit`.
  - Header iteration helper.
- **Async driver creation** (Phase 3 deferral):
  - `cosmos_driver_get_or_create_submit(runtime, account, options,
    cq, user_data, out_handle, out_pre_error)`.
  - `cosmos_driver_initialize_submit(driver, cq, user_data,
    out_handle, out_pre_error)`.
  - Reuses the generic submit-helper that Phase 6 builds for
    `cosmos_driver_submit`.

### 3.3 Tactical implementation notes

- **Submit pipeline shape:** factor out a generic
  `submit_oneshot<Fut>(cq, op_handle, user_data, fut)` helper in
  `src/completion.rs` that does `tokio::spawn` → await → wrap result
  in a `Completion` → `cq.enqueue(completion)`. This shape works for
  both `cosmos_driver_submit` and `cosmos_driver_get_or_create_submit`.
- **Take operation ownership at submit:**
  `cosmos_driver_submit` calls `OperationDescHandle::inner_mut(...)?
  .op.take()`. If `take()` returns `None` the op was already
  consumed — surface `OPERATION_CONSUMED` (4005) via
  `out_pre_error` and return NULL handle. The Phase 5 wiring on
  `OperationDescHandle` already supports this.
- **Response surface storage pun:** `cosmos_response_t` wraps
  `Arc<CosmosResponse>` exactly like the existing handle types so
  zero-copy body views can borrow from the Arc's allocation.
- **`cosmos_container_ref_resolve_blocking`** is the most useful
  shape for tests; the async submit variant follows the pattern from
  `cosmos_driver_get_or_create_submit`.
- The 404-as-ERROR-not-OK contract (spec §6.2) is already implicit
  in how `CosmosDriver::execute_singleton_operation` returns
  `Result<CosmosResponse, CosmosError>` — final HTTP errors become
  the `Err` arm. Just route the `Err` through `CosmosErrorHandle`
  on completion.

### 3.4 Test coverage to add in Phase 6

- Rust unit tests for the new helpers (submit-helper happy path, taken
  operation rejects re-submit, response accessors round-trip).
- C harness `c_tests/item_crud.c`: emulator-backed (skipped if
  emulator unreachable). Stand up driver → resolve container →
  create_item / read_item / replace_item / upsert_item / delete_item
  / patch_item against the emulator, driven by a single shared
  `cosmos_cq_t` and a helper thread that converts completions into
  per-call condition-variable signals.
- C harness `c_tests/driver_async_init.c`: re-run Phase 3's
  lifecycle test against the new async submit path.

### 3.5 Phase 5+ deferrals worth tracking (in addition to earlier)

1. **`cosmos_operation_with_request_header`** — the spec calls for
   a per-operation custom-header setter, but the driver's
   `CosmosRequestHeaders` is a typed whitelist with no slot for
   arbitrary headers. Custom headers live on `OperationOptions`
   instead. Either: (a) reconcile the spec (custom headers are
   options-level only), or (b) route well-known names through to the
   typed fields and reject the rest with
   `UNSUPPORTED_OPERATION_FOR_MUTATOR` (4009).
2. **`OperationOptions::priority_level`** and a per-operation
   priority — the spec lists `with_priority_level` on the options
   builder, but the merged driver's `OperationOptions` has no
   `priority_level` field. Either the driver gets one (preferred) or
   the spec drops it.
3. **`cosmos_operation_options_*` per-partition circuit-breaker
   field-count drift** — spec says 8 PPCB knobs; driver actually
   exposes 7. Update the spec.
4. *(Phase 3 and Phase 4 deferrals from prior notes carry forward.)*

### 3.6 Sequenced phase plan after Phase 6

| Phase | Notes |
|---|---|
| 7 | Diagnostics surface |
| 8 | Pagination (read-feeds + query) |
| 9 | Patch & transactional batch (sub-operation builder) |
| 10 | Optional advanced (fault injection, in-memory emulator, tracing) |

---

## 4. Decisions made (do not re-litigate without explicit user sign-off)

### 4.1 Branch model
- Spec lives on `users/kundadebdatta/4372_cosmos_driver_native_crate_spec`.
  Implementation lives on `users/kundadebdatta/4372_cosmos_driver_native_crate_async_impl` (this branch).
- Each phase is a separate commit. **Do not** batch phases.
- The implementation branch re-lands Phase 0 from scratch rather than
  branching off the spec branch. Phase 0 commit IDs differ between branches
  (`a7931b1bb` on spec branch vs `be825b8eb` on impl branch) — this is
  intentional.

### 4.2 cbindgen variant naming
- Enum types are renamed to `*_t` via `export.rename` (e.g. `cq_state_t`).
- **Variant prefixes are baked into Rust variant names**
  (e.g. `CosmosCqStateRunning`) and `rename_variants = "ScreamingSnakeCase"`
  emits the spec-mandated `COSMOS_CQ_STATE_RUNNING`. Switching to
  `QualifiedScreamingSnakeCase` would re-introduce `_T_` infixes.
- The Rust variant identifiers look noisy (`CosmosCqState::CosmosCqStateRunning`).
  Tolerate it — the cost of legibility on the Rust side buys spec-correct
  C variant names without per-enum cbindgen overrides.

### 4.3 Opaque storage pun for handles
- `cosmos_runtime_t`, `cosmos_cq_t`, `cosmos_operation_handle_t`, and
  `cosmos_error_t` all use a **storage pun**: the public `#[repr(C)]`
  struct carries only an `_opaque: [u8; 0]` marker; the real `Arc<...>`
  state lives in a separately-allocated `*Storage` struct accessed by
  pointer cast. This keeps `cosmos_Arc_*` typedefs **out** of the
  generated header — a hard requirement for ABI stability.
- If you add a new handle type, follow this pattern. Do not put
  `Arc<Inner>` directly inside the `#[repr(C)]` public struct.

### 4.4 Phase 1 test-only helpers
- `__test_only_create_default_runtime`, `__test_only_create_operation_handle`,
  and `__test_only_enqueue_completion` are `pub fn` so the Phase 1
  Rust tests can drive the completion-queue pipeline end-to-end without
  the real submit path.
- They will be replaced by the real surface in Phase 2/6. **Keep them**
  for now — the existing 23 Phase 1 tests use them, and so will Phase 2's
  tests until `cosmos_driver_submit` lands.

### 4.5 Error code layout
- `CosmosErrorCode` is `#[repr(i32)]` so it crosses the FFI as a plain
  signed int. The variants are not contiguous — they intentionally
  preserve the legacy band ranges from the old wrapper (see comments in
  `src/error.rs`). Do not collapse to a smaller enum.
- New error codes added in this rollout: `4011 QueueShutdown`,
  `4012 OperationCancelled`, `4013 QueueFull`, `4014 InvalidOptionValue`,
  `4015 RuntimeBuildFailed`.
- Code `4001` is **reserved** — formerly `OPTIONS_IGNORED_ON_CACHE_HIT`,
  moved to `5001` once the `5xxx` warning band was introduced. Do not
  reuse `4001` for anything else without a spec amendment.

### 4.6 Backtrace knobs
- The merged `azure_data_cosmos_driver` only exposes
  `error::set_backtrace_options(BacktraceOptions)` as a **process-global**
  free function. There is no per-runtime or per-driver setter. Earlier
  drafts of the spec described per-runtime knobs; those have been removed.
  See [`NATIVE_WRAPPER_SPEC.md` §6.4](azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md).
- Phase 1 exposes this as `cosmos_set_backtrace_options(captures, resolutions)`.

### 4.7 cargo features
- The wrapper's `Cargo.toml` opts into `azure_data_cosmos_driver/{tokio, rustls}`
  + the wrapper-local `tokio = { workspace = true, features = [...] }`.
- **`rustls` was added in Phase 2** — without it the driver's
  `DefaultHttpClientFactory::new()` can't construct an HTTP client and
  `CosmosDriverRuntimeBuilder::build()` fails. Do NOT switch to
  `native_tls` without testing on Windows first (see §5.2).
- `azure_data_cosmos_driver` is referenced via **path** rather than as a
  workspace dependency, because there is no `[workspace.dependencies.azure_data_cosmos_driver]`
  entry in the root `Cargo.toml`. Phase 3+ can keep this path-dep style.

### 4.8 Phase 2 deferrals (Phase 2+ follow-ups)
- **Complex nested setters on `cosmos_runtime_builder_t`:**
  `with_client_options`, `with_connection_pool`, `with_operation_options`,
  `register_throughput_control_group`, `with_fault_injection_rules`
  (gated on the `fault_injection` driver feature, which the wrapper does
  not currently enable). Each requires its own FFI builder surface that
  would dwarf the Phase 2 commit. Pull them in opportunistically when
  Phase 3's `cosmos_driver_options_builder_*` lands the
  `OperationOptions` surface — both share the same builder pattern.
- **`cosmos_runtime_builder_with_worker_threads` / `_with_thread_name_prefix`:**
  the spec lists these but the merged `CosmosDriverRuntimeBuilder` does
  NOT expose them; the Tokio runtime is built with hard-coded defaults
  inside `RuntimeContext::new_with_builder`. If we ever want to expose
  them at the FFI surface, they'll be Tokio-side knobs only (not
  forwarded to the driver), which is a different shape of API and worth
  thinking through before shipping.

---

## 5. Gotchas — read before running anything locally

### 5.1 Windows + CMake
- The previous machine had **no `cmake` installed**, so the C test
  harness was never exercised locally. The Rust side is fully green.
- The header was regenerated and checked in via `cargo build` (the
  cbindgen call in `build.rs`). If you cargo-clean and rebuild on a new
  machine, the header should round-trip byte-for-byte.

### 5.2 openssl-sys on Windows
- The wrapper uses `azure_data_cosmos_driver`'s default features which
  pull `rustls` (not OpenSSL), so the Windows openssl-sys breakage we hit
  earlier in the session does NOT affect this crate. Don't switch to
  `native_tls` without testing on Windows first.

### 5.3 git CRLF warnings
- The new files trigger git's `CRLF will be replaced by LF` warnings on
  commit. This is normal on Windows — the on-disk content is fine and
  the committed blob is LF.

### 5.4 PowerShell exit-code 1 false alarms
- Many of our `cargo` invocations were piped through PowerShell, which
  often returns exit-code 1 because of the "Blocking waiting for file
  lock on package cache" advisory written to stderr. Inspect the actual
  cargo output — if it says `Finished`, the build succeeded regardless
  of the PowerShell exit code.

### 5.5 Driver-side warnings
- The driver crate (`azure_data_cosmos_driver`) currently emits 3
  pre-existing clippy warnings:
  - `fields method, body, and timeout are never read` in
    `transport/cosmos_transport_client.rs`
  - `fields request_timeout and http2_keep_alive_while_idle are never read`
    in `transport/http_client_factory.rs`
  - `useless conversion to the same type: error::CosmosError` in
    `transport/http_client_factory.rs`
  
  Ignore them when filtering wrapper output. Filter regex:
  `^warning: \`azure_data_cosmos_driver\`|^warning: fields|useless conversion`

### 5.6 Doc-link gotcha
- `cargo doc` will complain about intra-doc links to `cosmos_*` functions
  that aren't valid Rust paths. Use bare backticks for FFI function names
  in docs (e.g. `` `cosmos_set_backtrace_options` ``), not full
  `[\`cosmos_*\`](crate::*)` link syntax.

---

## 6. Resume checklist for a new machine

```pwsh
# 1. Clone + check out the impl branch
git clone https://github.com/Azure/azure-sdk-for-rust.git
cd azure-sdk-for-rust
git checkout users/kundadebdatta/4372_cosmos_driver_native_crate_async_impl

# 2. Verify the last commit matches what this memo expects
git log --oneline -3
# Expected top:
#   6c57b6775 Cosmos: Phase 1 of azure_data_cosmos_driver_native (error + async invocation primitives)
#   be825b8eb Cosmos: Phase 0 of azure_data_cosmos_driver_native (scaffolding)

# 3. Validate the current state
cargo build  -p azure_data_cosmos_driver_native
cargo fmt    -p azure_data_cosmos_driver_native -- --check
cargo clippy -p azure_data_cosmos_driver_native --all-targets
cargo doc    -p azure_data_cosmos_driver_native --no-deps
cargo test   -p azure_data_cosmos_driver_native    # expect 23 passed

# 4. (Optional) C test harness — needs CMake >= 3.20 + a C compiler
cmake -B build sdk/cosmos/azure_data_cosmos_driver_native
cmake --build build
ctest --test-dir build --output-on-failure

# 5. Confirm the generated header matches the checked-in copy. The
#    `cargo build` step regenerates it; `git status` should report no
#    diff under `sdk/cosmos/azure_data_cosmos_driver_native/include/`.
git status -- sdk/cosmos/azure_data_cosmos_driver_native/include/

# 6. Start Phase 2 (see §3 above).
```

---

## 7. Open questions tracked in the spec

Phase 2+ work may touch these — re-read before deciding. From [`NATIVE_WRAPPER_SPEC.md` §9](azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md):

- Q1: Single-runtime-per-process enforcement.
- Q6: Driver-cache scoping vs. credential identity.
- Q12: Multi-consumer (MPMC) `cosmos_cq_t`.
- Q13: Driver-side `CancellationToken` parameter (would let Phase 6
  retire the `tokio::select!` shim and surface mid-flight cancellation
  diagnostics).

None of these block Phase 2; flag them in Phase 6+ design notes.

---

## 8. If something seems wrong

- Diff against `origin/users/kundadebdatta/4372_cosmos_driver_native_crate_async_impl`
  — that's the source of truth. The previous machine's working tree
  should be empty after `git status`.
- The crate doc surface and the NATIVE_WRAPPER_SPEC stay in lock-step.
  If you find them disagreeing, **update the spec first**, then the
  code, then both in a single PR. Do not let them drift.
- The 23 Rust tests in `src/error.rs` and `src/completion.rs` are the
  load-bearing acceptance set for Phase 1. If any of them break in a
  Phase 2 change, that's a regression — fix it before moving on.
