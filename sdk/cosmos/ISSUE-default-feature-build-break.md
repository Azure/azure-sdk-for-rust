# `azure_data_cosmos` does not build under its default feature set

**Status:** pre-existing on `main`, unrelated to PR #5040. Not fixed here.
**Found:** 2026-08-20, while linting the `ci.yml` round-trip-budget fix
(Copilot review comment #2 on PR #5040).

## Symptom

Both of these fail:

```pwsh
cargo build  -p azure_data_cosmos
cargo clippy -p azure_data_cosmos --tests
```

```text
error[E0432]: unresolved import `crate::feed::FeedBody`
  --> sdk\cosmos\azure_data_cosmos\src\models\response_body.rs:10:5
   |
10 | use crate::feed::FeedBody;
   |     ^^^^^^^^^^^^^^^^^^^^^ no `FeedBody` in `feed`
   |
note: struct `crate::feed::page::FeedBody` exists but is inaccessible
  --> sdk\cosmos\azure_data_cosmos\src\feed\page.rs:73:1
note: found an item that was configured out
  --> sdk\cosmos\azure_data_cosmos\src\feed\mod.rs:28:22
   |
27 | #[cfg(feature = "control_plane")]
   |       ------------------------- the item is gated behind the `control_plane` feature
28 | pub(crate) use page::FeedBody;
```

This is the **lib**, not just the test targets — the crate as published
cannot be compiled with default features.

## Cause

`feed/mod.rs:27-28` re-exports `FeedBody` only under `control_plane`:

```rust
#[cfg(feature = "control_plane")]
pub(crate) use page::FeedBody;
```

but `models/response_body.rs:10` imports it unconditionally. `ResponseBody`
is not itself feature-gated — it is the return type of
`ItemResponse::into_body`, `ResourceResponse::into_body`, and
`BatchResponse::into_body`, none of which are `control_plane`-only. So the
gate on the re-export is narrower than the gate on the consumer.

The underlying `FeedBody` struct at `feed/page.rs:73` is **not** gated; only
the re-export is. That suggests the `#[cfg]` was attached to the wrong line
rather than the type genuinely being control-plane-specific.

## Why it is pre-existing

- `git log origin/main -1 -- src/models/response_body.rs` → `799438fc86`
  ("Cosmos: Add cross-partition OFFSET/LIMIT/TOP query support (#4750)
  (#4870)"), which is on `main`.
- `git diff --stat origin/main...HEAD -- sdk/cosmos/azure_data_cosmos/src/`
  touches only `clients/container_client.rs` and
  `clients/cosmos_client_builder.rs`. Neither of the three files involved in
  the error is modified by this branch.

## Why CI is green

CI never builds this crate with default features. Every live/test leg that
touches `azure_data_cosmos` passes an explicit feature set — the fuzzer
target, for instance, *requires* `key_auth`, `control_plane`,
`fault_injection`, and cargo refuses to build it otherwise:

```text
error: target `binary_roundtrip_fuzzer` in package `azure_data_cosmos`
requires the features: `key_auth`, `control_plane`, `fault_injection`
```

So the default-feature configuration is untested. Note that `control_plane`
happens to be in that required set, which is exactly why the gap stays
hidden.

## Suggested fix

Remove the `#[cfg(feature = "control_plane")]` from `feed/mod.rs:28` — the
struct it re-exports is ungated and its consumer is ungated. Then add a
default-feature build to CI (`cargo build -p azure_data_cosmos` with no
`--features`) so the configuration users get by default is actually
compiled.

Worth checking whether other `sdk/cosmos` crates have the same blind spot
before adding just the one check.

## Workaround

Pass the features explicitly:

```pwsh
cargo clippy -p azure_data_cosmos --features "key_auth control_plane fault_injection" --test binary_roundtrip_fuzzer
```
