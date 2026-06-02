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

**Last updated:** 2026-06-02 (Phase 3 committed locally — Phase 2 + 3 not yet pushed)
**Active branch:** `users/kundadebdatta/4372_cosmos_driver_native_crate_async_impl`
**Remote:** `origin/users/kundadebdatta/4372_cosmos_driver_native_crate_async_impl` (ahead by 2 commits; push at end of session)
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
| Phase 3 (pending push) | Phase 3 — account / database refs + driver options + driver `_blocking` | New `src/account_ref.rs` (`with_master_key` + `_clone` + `_free`), `src/database_ref.rs` (`_create` + `_clone` + `_free`), `src/driver_options.rs` (builder + `_with_preferred_regions` + `_build` + `_free`), `src/driver.rs` (`_get_or_create_blocking` + `_free`). `Cargo.toml` adds `azure_core` + `url` runtime deps (needed for `Secret` + `Url::parse`). C harness `c_tests/account_and_driver_options.c`. **55 Rust tests pass (35 + 20 new; 1 ignored network test).** |

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

### 2.3 What an external SDK can do right now (after Phase 3)

- Phase 0: dlopen the .so/.dll, call `cosmos_version()`, exercise the
  null-safe `cosmos_string_free` / `cosmos_bytes_free`.
- Phase 1: internal Rust scaffold only (covered by the 23 lib tests).
- Phase 2: stand up a real `cosmos_runtime_t *` from C via
  `cosmos_runtime_builder_new` → setters → `cosmos_runtime_builder_build`,
  then create completion queues against it.
- Phase 3: **stand up a real `cosmos_driver_t *` against a Cosmos
  endpoint** via `cosmos_account_ref_with_master_key` →
  `cosmos_driver_get_or_create_blocking`. Database refs (name-based)
  are constructible; container refs are not (deferred to Phase 6 —
  driver requires async resolution). The async `_submit` driver
  variant is also deferred to Phase 6 alongside the generic
  submit pipeline. The `5001 OPTIONS_IGNORED_ON_CACHE_HIT` advisory
  is **not** emitted (Phase 3+ follow-up; needs driver-side
  cooperation — see §3.3 Phase 3 deferrals).

**Phase 4** is the first phase that adds partition keys (pure value
type, no network); **Phase 5** is the operation-construction surface;
**Phase 6** is when full CRUD lands.

---

## 3. Next steps — Phase 4

**Goal:** ship the partition-key builder so callers can construct every
shape of partition key the driver supports. All operations gated on a
partition key in Phase 5 will require this surface.

### 3.1 Spec section to implement

[`NATIVE_WRAPPER_SPEC.md` §4.5 + §8 Phase 4](azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md). Phase 4 done-when criterion: "Round-trip: build → debug-print via a Rust-side test helper → assert the wire value matches the gateway baseline `PartitionKeyHashBaselineTest.*.xml` files already in the driver `testdata/`."

### 3.2 Surface to add

- **`cosmos_partition_key_builder_*`** (opaque builder):
  - `cosmos_partition_key_builder_new()` — returns a fresh builder.
  - `cosmos_partition_key_builder_add_string(builder, value)`.
  - `cosmos_partition_key_builder_add_number(builder, value_f64)`.
  - `cosmos_partition_key_builder_add_bool(builder, value)`.
  - `cosmos_partition_key_builder_add_null(builder)`.
  - `cosmos_partition_key_builder_add_undefined(builder)`.
  - `cosmos_partition_key_builder_build(builder, out_pk)` — consumes;
    returns `INVALID_PARTITION_KEY` (4004) on empty builder.
  - `cosmos_partition_key_builder_free(builder)`.
  - `cosmos_partition_key_clone(pk, out_clone)` / `_free(pk)`.
- **Accessors** (for tests + future diagnostics):
  - `cosmos_partition_key_component_count(pk) -> usize`.
  - `cosmos_partition_key_is_empty(pk) -> bool`.

### 3.3 Tactical implementation notes

- The driver's `PartitionKey` lives in `models::partition_key.rs`
  alongside `PartitionKeyValue`. Both are simple value types — no
  builder needed on the driver side, but the FFI benefits from one
  (incrementally populating components across multiple FFI calls is
  the ergonomic shape for cross-language SDKs).
- The `INVALID_PARTITION_KEY` coarse code (4004) already exists in
  `error.rs`; just route the empty-builder case through it.
- For Phase 4, do NOT pull in the hash-baseline test — that requires
  the driver's `testdata/` round-tripping, which is a separate test
  harness. The done-when criterion can be satisfied by Rust-side
  unit tests asserting the constructed `PartitionKey` matches the
  one built directly via the driver API.

### 3.4 Test coverage to add in Phase 4

- Rust unit tests per value kind + empty-builder rejection.
- C harness `c_tests/partition_key.c` covers all 5 value kinds, single
  + hierarchical keys, and the empty-key error path.

### 3.5 Phase 3 deferrals worth tracking

1. **Cache-hit advisory** (`OPTIONS_IGNORED_ON_CACHE_HIT` / `5001`).
   Spec §4.4.1 mandates this; current implementation always returns
   `SUCCESS`. Needs either a driver-side `was_cached` signal or a
   wrapper-side cache shadow. **Recommend**: open an upstream issue to
   add `CosmosDriverRuntime::get_or_create_driver_detailed(...) ->
   Result<(Arc<CosmosDriver>, bool /* was_cached */)>`.
2. **Async submit path for driver creation**
   (`cosmos_driver_get_or_create_submit`,
   `cosmos_driver_initialize_submit`, `cosmos_response_take_driver`).
   Deferred to Phase 6 where the generic
   `tokio::spawn` → `cq_enqueue` plumbing lands once for all
   operations.
3. **Token-credential and resource-token account constructors**
   (`cosmos_account_ref_with_credential`,
   `cosmos_account_ref_with_resource_token`). Bridging
   `Arc<dyn TokenCredential>` (async trait) through FFI is non-trivial
   — needs a callback-driven `TokenCredential` impl on the Rust side
   plus a Tokio bridge that channels the response back. Could be a
   small standalone phase.
4. **Container references** (`cosmos_container_ref_*`). Driver's
   `ContainerReference::new` is `pub(crate)`-only and requires the
   RID + partition-key definition that only `resolve_container`
   produces. Phase 6 lands it alongside the resolved-container
   response.
5. **Driver options operation-options setter**
   (`cosmos_driver_options_builder_with_operation_options`). Requires
   the full `cosmos_operation_options_*` builder; lands in Phase 5
   with the per-call operation options.

### 3.6 Sequenced phase plan after Phase 4

| Phase | Notes |
|---|---|
| 5 | Operation factories + mutators (no execute yet); ~10 factories + ~6 mutators + `cosmos_operation_options_*` builder with 17+ option fields. **Biggest remaining phase.** |
| 6 | `cosmos_driver_submit` + response surface (first **full CRUD** from external SDK) — also lands `cosmos_container_ref_*`, the async driver submit variants, and reuses the generic submit-helper for Phase 3's deferred async driver creation. |
| 7 | Diagnostics surface |
| 8 | Pagination (read-feeds + query) |
| 9 | Patch + transactional batch |
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
