<!-- cspell:ignore azurecosmosdriver cdylib staticlib corrosion cbindgen ctest -->

# Azure Cosmos DB Driver — Native C Bindings (`azure_data_cosmos_driver_native`)

C ABI wrapper around [`azure_data_cosmos_driver`](../azure_data_cosmos_driver),
designed for cross-language SDK reuse (.NET, Java, Go, Python, native C/C++).
The full design is in
[NATIVE_WRAPPER_SPEC.md](../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md);
the picture-first overview is in
[ASYNC_INVOCATION_ARCHITECTURE.md](../azure_data_cosmos_driver/docs/ASYNC_INVOCATION_ARCHITECTURE.md);
this README is a short orientation.

## What this crate ships

- A `cdylib` + `staticlib` named `azurecosmosdriver`
  (`libazurecosmosdriver.{so,dylib,dll}`).
- A C header at [include/azurecosmosdriver.h](include/azurecosmosdriver.h),
  regenerated on every build and **checked in** so language-binding consumers
  can vendor it without a Rust toolchain.
- A small C test harness under [c_tests/](c_tests/) driven by CMake +
  [corrosion](https://github.com/corrosion-rs/corrosion).

## Rollout status

This crate is being built up phase-by-phase per
[§8 of the spec](../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md). **Phase 0**
ships only the scaffolding — `cosmos_version()`, the `c_str!` macro for
static C strings, and the `cosmos_string_free` / `cosmos_bytes_free` allocator
companions. Subsequent phases add error / completion-queue plumbing (Phase 1),
the runtime (Phase 2), driver instances (Phase 3), partition keys (Phase 4),
operation construction (Phase 5), submit + response (Phase 6), and so on.

## Building

```pwsh
# Rust side (produces the cdylib / staticlib and regenerates the header).
cargo build -p azure_data_cosmos_driver_native

# C test harness (requires CMake ≥ 3.20 and a C compiler).
cmake -B build sdk/cosmos/azure_data_cosmos_driver_native
cmake --build build
ctest --test-dir build --output-on-failure
```

## Repository archaeology — files removed by PR #4103

The earlier `azure_data_cosmos_native` crate (removed in
[PR #4103](https://github.com/Azure/azure-sdk-for-rust/pull/4103),
commit `ccf43caae`) shipped a handful of files that have **not** been
reintroduced in this crate; their content now lives elsewhere:

| Old file | New location |
|---|---|
| `azurecosmos.pc.in` (pkg-config template) | This crate ships a sibling `azurecosmosdriver.pc.in` with the same shape but a new package name. |
| `docs/next_generation_sdks_design_principles.md` | Folded into [NATIVE_WRAPPER_SPEC.md §2](../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md). |
| `c_tests/test_common.h` runtime / client / database fixtures | Re-added incrementally as the corresponding C entry points land — Phase 2 brings runtime fixtures back, Phase 3 brings driver/auth, etc. |

If you are spelunking the git history of the old crate looking for a behavior
or test that "should be here", that table is the first place to check.
