# Spec: Decouple azure_data_cosmos_driver from azure_core

## Problem Statement

The `azure_data_cosmos_driver` crate depends on `azure_core` for HTTP infrastructure, credentials, error types, and utilities. Per the multi-crate versioning strategy (AGENTS.md), the driver should be independently versionable from the SDK layer. The `azure_core` dependency creates version coupling — breaking changes in `azure_core` force driver major version bumps even when the driver's own API is stable.

Since the driver is consumed by `azure_data_cosmos_native` (C FFI for Java/.NET/Python SDKs), minimizing heavy Azure-specific dependencies reduces binary size and cross-language complexity.

Reference: <https://github.com/Azure/azure-sdk-for-python/issues/45463>

## Current Crate Layering

The core crates form a dependency chain:

```text
typespec                      # Error, ErrorKind, Result, StatusCode, Bytes
  └── typespec_client_core    # Policy, Transport, Request, Response, Context,
  │                           # Headers, Method, ClientOptions, HttpClient,
  │                           # SafeDebug, time, fmt, base64
  │     └── azure_core        # credentials (Secret, TokenCredential, AccessToken),
  │                           # hmac, Azure-specific headers, ErrorResponse,
  │                           # check_success, Azure pipeline policies
  │           └── azure_data_cosmos_driver  ← TODAY
```

The driver uses types from **all three layers**, but what it gets from `azure_core` specifically (not re-exported from typespec/typespec_client_core) is a small surface:

| azure_core-only type | Used in driver | Purpose |
|----------------------|----------------|---------|
| `credentials::Secret` | account_reference, connection_string | Secret string wrapper with constant-time eq |
| `credentials::TokenCredential` trait | account_reference, authorization_policy | AAD/Entra token acquisition |
| `credentials::AccessToken` | authorization_policy | Token response type |
| `credentials::TokenRequestOptions` | authorization_policy | Token request options |
| `hmac::hmac_sha256` | authorization_policy | Master key HMAC signing |
| `http::headers::MS_DATE` | (via azure_core headers) | Azure-specific header constant |
| `fmt::SafeDebug` derive | connection_string | Debug that redacts secrets |
| `http::ClientOptions` (azure_core re-export) | runtime.rs | Client-level options (but this is actually from typespec_client_core) |

Everything else the driver uses — `Error`, `ErrorKind`, `Result`, `Policy`, `Transport`, `Request`, `Response`, `Context`, `Headers`, `HeaderName`, `HeaderValue`, `Method`, `StatusCode`, `Bytes`, `time::OffsetDateTime`, `base64` — originates in `typespec` or `typespec_client_core` and is merely re-exported through `azure_core`.

## Proposed Approach: Refactor the Core Crates

Instead of the driver duplicating types that already exist in the core stack, we propose changes to the **core crates** that let the driver depend on lighter layers directly.

### Target Architecture

```text
typespec                          # Error, ErrorKind, Result, StatusCode, Bytes
  └── typespec_client_core        # Policy, Transport, Request, Response, Context,
  │     │                         # Headers, Method, ClientOptions, HttpClient,
  │     │                         # SafeDebug, time, fmt, base64
  │     │
  │     ├── azure_core_credentials  ← NEW CRATE (extracted from azure_core)
  │     │   # Secret, TokenCredential, AccessToken, TokenRequestOptions
  │     │
  │     ├── azure_core_hmac         ← NEW CRATE (extracted from azure_core)
  │     │   # hmac_sha256 (feature-gated: hmac_rust / hmac_openssl)
  │     │
  │     ├── azure_data_cosmos_driver  ← AFTER (depends on lighter crates)
  │     │
  │     └── azure_core              ← AFTER (re-exports credentials + hmac + adds Azure policies)
  │           └── azure_data_cosmos   ← unchanged (still depends on azure_core)
```

### What Changes in the Core Package

#### Change 1: Extract `azure_core_credentials` crate

Move `azure_core::credentials` into a new standalone crate.

**New crate**: `sdk/core/azure_core_credentials/`

```rust
// azure_core_credentials/src/lib.rs
pub use typespec_client_core::time::OffsetDateTime;

mod secret;
mod token;

pub use secret::{Secret, SecretBytes};
pub use token::{AccessToken, TokenCredential, TokenRequestOptions};
```

**Contents** (moved from `azure_core/src/credentials.rs`):
- `Secret` — secret string wrapper with constant-time `PartialEq` and redacted `Debug`
- `SecretBytes` — secret bytes wrapper
- `AccessToken` — token + expiry
- `TokenRequestOptions` — options for `get_token()`
- `TokenCredential` trait — async token acquisition

**Dependencies** (minimal):
- `typespec_client_core` (for `ClientMethodOptions`, `SafeDebug`, `time::OffsetDateTime`)
- `async-trait`
- `serde` (for `Secret`/`AccessToken` derives)

**Impact on azure_core**: `azure_core` re-exports the new crate:

```rust
// azure_core/src/credentials.rs (AFTER)
pub use azure_core_credentials::*;
```

No breaking change to existing azure_core consumers.

#### Change 2: Extract `azure_core_hmac` crate

Move `azure_core::hmac` into a new standalone crate.

**New crate**: `sdk/core/azure_core_hmac/`

```rust
// azure_core_hmac/src/lib.rs
pub fn hmac_sha256(data: &str, key: &azure_core_credentials::Secret) -> typespec::error::Result<String> {
    // ... existing implementation using hmac + sha2 or openssl
}
```

**Dependencies**:
- `azure_core_credentials` (for `Secret`)
- `typespec` (for `Error`/`Result`, `base64`)
- `hmac` + `sha2` (feature-gated)
- `openssl` (feature-gated)

**Impact on azure_core**: `azure_core` re-exports the new crate:

```rust
// azure_core/src/hmac.rs (AFTER)
pub use azure_core_hmac::*;
```

No breaking change.

#### Change 3: No changes needed to typespec / typespec_client_core

The driver can depend on `typespec_client_core` directly for everything it currently gets via `azure_core` re-exports:

| Driver needs | Source (today via azure_core) | Source (after, direct) |
|-------------|------------------------------|----------------------|
| `Error`, `ErrorKind`, `Result` | `typespec::error` | `typespec::error` (via `typespec_client_core`) |
| `Policy`, `PolicyResult` | `typespec_client_core::http::policies` | same, direct |
| `Transport` | `typespec_client_core::http::options` | same, direct |
| `Request`, `Body` | `typespec_client_core::http::request` | same, direct |
| `RawResponse`, `AsyncRawResponse` | `typespec_client_core::http::response` | same, direct |
| `Context` | `typespec_client_core::http::context` | same, direct |
| `Headers`, `HeaderName`, `HeaderValue`, `AsHeaders` | `typespec_client_core::http::headers` | same, direct |
| `AUTHORIZATION`, `ACCEPT`, `CONTENT_TYPE`, `USER_AGENT` | `typespec_client_core::http::headers` | same, direct |
| `Method` | `typespec_client_core::http::method` | same, direct |
| `StatusCode` | `typespec::http::StatusCode` | same, direct |
| `Url` | `typespec_client_core::http::Url` (re-export of `url`) | same, direct |
| `ClientOptions` | `typespec_client_core::http::options` | same, direct |
| `Bytes` | `typespec::Bytes` | same, direct |
| `time::OffsetDateTime`, `to_rfc7231` | `typespec_client_core::time` | same, direct |
| `base64` | `typespec_client_core::base64` | same, direct |
| `fmt::SafeDebug` | `typespec_client_core::fmt` | same, direct |

### Driver Dependency Changes

```toml
# Cargo.toml BEFORE
[dependencies]
azure_core = { workspace = true, features = ["reqwest_native_tls", "hmac_rust"] }

# Cargo.toml AFTER
[dependencies]
typespec_client_core = { workspace = true, features = ["reqwest_native_tls"] }
azure_core_credentials = { workspace = true }
azure_core_hmac = { workspace = true, features = ["hmac_rust"] }
# azure_core is REMOVED
```

### Driver Import Migration

The import changes are mechanical find-and-replace:

```rust
// BEFORE
use azure_core::http::policies::{Policy, PolicyResult};
use azure_core::http::{Context, Request, Method, StatusCode};
use azure_core::http::headers::{HeaderName, HeaderValue, Headers};
use azure_core::credentials::{Secret, TokenCredential, AccessToken};
use azure_core::hmac::hmac_sha256;
use azure_core::{Error, Result, Bytes};
use azure_core::time::OffsetDateTime;
use azure_core::fmt::SafeDebug;

// AFTER
use typespec_client_core::http::policies::{Policy, PolicyResult};
use typespec_client_core::http::{Context, Request, Method, StatusCode};
use typespec_client_core::http::headers::{HeaderName, HeaderValue, Headers};
use azure_core_credentials::{Secret, TokenCredential, AccessToken};
use azure_core_hmac::hmac_sha256;
use typespec_client_core::{Error, Result, Bytes};
use typespec_client_core::time::OffsetDateTime;
use typespec_client_core::fmt::SafeDebug;
```

## azure_core Consumers Are Unaffected

Since `azure_core` re-exports the new crates, all existing code that uses `azure_core::credentials::*` or `azure_core::hmac::*` continues to work unchanged. The `azure_data_cosmos` SDK crate (which depends on `azure_core`) needs zero changes.

## Phased Migration Plan

### Phase 1: Extract `azure_core_credentials` crate

1. Create `sdk/core/azure_core_credentials/` with `Cargo.toml` and `src/lib.rs`
2. Move `Secret`, `SecretBytes`, `AccessToken`, `TokenRequestOptions`, `TokenCredential` from `azure_core/src/credentials.rs`
3. Update `azure_core/src/credentials.rs` to `pub use azure_core_credentials::*;`
4. Add `azure_core_credentials` to workspace `Cargo.toml`
5. Add `azure_core_credentials` as dependency of `azure_core`
6. Validate: `cargo build -p azure_core --all-features && cargo test -p azure_core --all-features`

### Phase 2: Extract `azure_core_hmac` crate

7. Create `sdk/core/azure_core_hmac/` with `Cargo.toml` and `src/lib.rs`
8. Move `hmac_sha256` implementations from `azure_core/src/hmac.rs`
9. Update `azure_core/src/hmac.rs` to `pub use azure_core_hmac::*;`
10. Add `azure_core_hmac` to workspace `Cargo.toml`
11. Validate: `cargo build -p azure_core --all-features && cargo test -p azure_core --all-features`

### Phase 3: Switch driver dependencies

12. Replace `azure_core` with `typespec_client_core` + `azure_core_credentials` + `azure_core_hmac` in driver `Cargo.toml`
13. Mechanical find-and-replace of all `use azure_core::*` imports (see table above)
14. Validate: `cargo build -p azure_data_cosmos_driver --all-features && cargo test -p azure_data_cosmos_driver --all-features`

### Phase 4: Validate full stack

15. `cargo build --workspace && cargo test --workspace` to confirm nothing is broken
16. Verify `azure_data_cosmos` (SDK crate) still works unchanged (it depends on `azure_core`, which re-exports everything)

## Scope of Changes by Crate

| Crate | Changes | Breaking? |
|-------|---------|-----------|
| `azure_core_credentials` | **NEW** — extracted from `azure_core::credentials` | N/A (new) |
| `azure_core_hmac` | **NEW** — extracted from `azure_core::hmac` | N/A (new) |
| `azure_core` | Re-exports new crates instead of defining types inline | **No** — public API identical |
| `typespec` | None | No |
| `typespec_client_core` | None | No |
| `azure_data_cosmos_driver` | Swap `azure_core` dep → `typespec_client_core` + new crates; update imports | No (internal crate) |
| `azure_data_cosmos` | None (still uses `azure_core`) | No |
| `azure_data_cosmos_native` | None (uses driver) | No |
| Workspace `Cargo.toml` | Add two new workspace members + deps | No |

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| `azure_identity` must depend on `azure_core_credentials` | Medium — requires coordinated change | `azure_identity` already only uses `TokenCredential` from `azure_core::credentials`. It can depend on the new crate directly or continue via `azure_core` re-export. |
| Circular dependency if `azure_core_credentials` needs `azure_core` types | Low | Credentials only need `ClientMethodOptions` from `typespec_client_core` — no `azure_core` types needed. |
| Workspace CI must build/test new crates | Low | Standard workspace member addition. |
| Other azure SDK crates using `azure_core::credentials` directly | None — re-export preserves API | All existing `use azure_core::credentials::*` continues to work. |

## Benefits

1. **Driver decoupled from azure_core versioning** — driver can rev independently
2. **Smaller driver dependency tree** — `typespec_client_core` is much lighter than `azure_core` (no Azure-specific pipeline policies, retry logic, logging, etc.)
3. **Reusable credential crate** — `azure_core_credentials` can be used by any crate that needs `TokenCredential` without pulling in the full `azure_core`
4. **No breaking changes** — `azure_core` re-exports preserve all existing public APIs
5. **Mechanical migration** — driver changes are pure import path updates, no behavioral changes

## Open Questions

1. **Naming**: `azure_core_credentials` vs `azure_credentials` vs `azure_core_auth`? The `azure_core_*` prefix makes the relationship to `azure_core` clear.

2. **Should `azure_core_hmac` be a separate crate or a feature of `azure_core_credentials`?** HMAC signing is closely tied to master key auth. Combining them reduces crate count but adds optional deps to the credentials crate.

3. **Should the driver depend on `typespec_client_core` directly or through a thin `azure_core_http` extraction?** Direct `typespec_client_core` dependency works today but ties the driver to the typespec crate naming. An `azure_core_http` extraction that re-exports typespec types would add a stable Azure-branded interface layer.

4. **Should `azure_identity` also move to `azure_core_credentials`?** If so, `azure_identity` would no longer need `azure_core` as a dependency — only `azure_core_credentials`. This is a bigger change but aligns with the same decoupling goal.
