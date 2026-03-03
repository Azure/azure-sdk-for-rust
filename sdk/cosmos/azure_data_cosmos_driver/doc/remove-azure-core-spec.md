# Spec: Decouple azure_data_cosmos_driver from azure_core

## Problem Statement

The `azure_data_cosmos_driver` crate depends on `azure_core` for HTTP infrastructure, credentials, error types, and utilities. Per the multi-crate versioning strategy (AGENTS.md), the driver should be independently versionable from the SDK layer. The `azure_core` dependency creates version coupling — breaking changes in `azure_core` force driver major version bumps even when the driver's own API is stable.

Since the driver is consumed by `azure_data_cosmos_native` (C FFI for Java/.NET/Python SDKs), minimizing heavy Azure-specific dependencies reduces binary size and cross-language complexity.

The previous revision of this spec proposed having the driver depend on `typespec_client_core` directly, but `typespec_client_core` includes significant functionality the driver never uses (retry policies, pipeline builder, logging policies, streaming, sleep abstractions, tracing module, etc.). The driver only needs the HTTP primitive types and a handful of Azure-specific types (credentials + HMAC).

Reference: <https://github.com/Azure/azure-sdk-for-python/issues/45463>

## Current Crate Layering

```text
typespec                      # Error, ErrorKind, Result, StatusCode, Bytes
  └── typespec_client_core    # Policy, Transport, Request, Response, Context,
  │                           # Headers, Method, ClientOptions, HttpClient,
  │                           # SafeDebug, time, fmt, base64
  │                           #
  │                           # ALSO (unused by driver):
  │                           # RetryPolicy trait, ExponentialRetryPolicy,
  │                           # FixedRetryPolicy, RetryOptions, Pipeline struct,
  │                           # TransportPolicy, logging policy, PipelineOptions,
  │                           # LoggingOptions, ClientMethodOptions,
  │                           # sleep, async_runtime, stream/BytesStream,
  │                           # tracing module, json/xml
  │
  │     └── azure_core        # credentials (Secret, TokenCredential, AccessToken),
  │                           # hmac, Azure-specific headers, ErrorResponse,
  │                           # check_success, Azure pipeline policies
  │           └── azure_data_cosmos_driver  ← TODAY
```

## What the Driver Actually Uses

### ✅ Types the driver needs

| Category | Types | Origin |
|----------|-------|--------|
| **Error handling** | `Error`, `ErrorKind`, `Result`, `ResultExt` | `typespec::error` |
| **HTTP request/response** | `Request`, `Body`, `RequestContent`, `RawResponse`, `AsyncRawResponse`, `Response` | `typespec_client_core::http` |
| **HTTP primitives** | `Method`, `StatusCode`, `Url` | `typespec_client_core::http` / `typespec::http` |
| **Headers** | `HeaderName`, `HeaderValue`, `Headers`, `AsHeaders` | `typespec_client_core::http::headers` |
| **Pipeline** | `Policy` trait, `PolicyResult`, `Transport`, `Context` | `typespec_client_core::http::policies` / `typespec_client_core::http` |
| **Client config** | `ClientOptions` | `typespec_client_core::http` |
| **Utilities** | `SafeDebug`, `Bytes`, `Uuid` | `typespec_client_core::fmt`, `typespec`, `uuid` |
| **Time** | `OffsetDateTime`, `to_rfc7231` | `typespec_client_core::time` |
| **Credentials** | `Secret`, `SecretBytes`, `TokenCredential`, `AccessToken`, `TokenRequestOptions` | `azure_core::credentials` |
| **Crypto** | `hmac_sha256` | `azure_core::hmac` |

### ❌ Types the driver does NOT use (from typespec_client_core)

| Module | What it provides | Why driver doesn't need it |
|--------|-----------------|---------------------------|
| `http::policies::retry` | `RetryPolicy` trait, `ExponentialRetryPolicy`, `FixedRetryPolicy` | Driver has its own `ClientRetryPolicy` |
| `http::options::retry` | `RetryOptions`, `ExponentialRetryOptions`, `FixedRetryOptions` | Driver has its own retry configuration |
| `http::options` | `PipelineOptions`, `LoggingOptions`, `ClientMethodOptions` | Driver builds its own pipeline, does its own logging |
| `http::pipeline` | `Pipeline` struct (generic pipeline builder) | Driver has `CosmosPipeline` |
| `http::policies::transport` | `TransportPolicy` | Driver has `TrackedTransportPolicy` |
| `http::policies::logging` | Request/response logging policy | Driver does its own diagnostics |
| `tracing` | OpenTelemetry span/attribute helpers | Driver uses `tracing` crate directly |
| `sleep` | Async sleep abstraction | Driver uses `tokio::time::sleep` directly |
| `async_runtime` | Runtime-agnostic async helpers | Driver uses tokio directly |
| `stream` / `BytesStream` | Streaming response bodies | Not needed — Cosmos responses are buffered (≤16MB) |
| `base64` | Base64 encode/decode | Driver could use `base64` crate directly |
| `json` / `xml` | Serde helpers | Driver does its own JSON (schema-agnostic) |

This is roughly **half** of `typespec_client_core`'s surface area.

## Proposed Approach: New `azure_core_shared` Crate

Instead of depending on either `azure_core` (too heavy, Azure-specific policies) or `typespec_client_core` (too broad, includes unused retry/pipeline/streaming), create a single new crate that contains **exactly** what the driver needs.

### Target Architecture

```text
typespec                              # Error, ErrorKind, Result, StatusCode, Bytes
  └── typespec_client_core            # Full client runtime (policies, retry, pipeline, etc.)
        │
        ├── azure_core_shared   ← NEW (lean subset + credentials + hmac)
        │     │
        │     ├── azure_data_cosmos_driver   ← AFTER (single dep)
        │     │
        │     └── azure_core                 ← AFTER (depends on shared + adds Azure policies)
        │           └── azure_data_cosmos     ← unchanged
        │
        └── (other Azure SDK crates via azure_core)
```

### What `azure_core_shared` Contains

The new crate has three sections:

#### 1. Re-exports from typespec / typespec_client_core (HTTP primitives only)

```rust
// azure_core_shared/src/lib.rs

// === Error types (from typespec) ===
pub mod error {
    pub use typespec::error::{Error, ErrorKind, Result, ResultExt};
}
pub use error::{Error, Result};

// === Binary data ===
pub use typespec::Bytes;
pub use uuid::Uuid;

// === Time ===
pub mod time {
    pub use typespec_client_core::time::*;
}

// === SafeDebug ===
pub mod fmt {
    pub use typespec_client_core::fmt::*;
}

// === HTTP primitives (NO retry, pipeline builder, logging, streaming) ===
pub mod http {
    // Request/Response
    pub use typespec_client_core::http::{
        Request, Body, RequestContent,
        RawResponse, AsyncRawResponse, Response,
    };

    // Method, StatusCode, Url
    pub use typespec_client_core::http::{Method, StatusCode, Url};

    // URL helpers
    pub use typespec_client_core::http::{AppendToUrlQuery, UrlExt};

    // Headers
    pub mod headers {
        pub use typespec_client_core::http::headers::{
            HeaderName, HeaderValue, Headers, AsHeaders,
            // Standard header constants the driver uses
            ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT,
        };
    }

    // Policy trait + Transport + Context (NO retry policies, NO pipeline builder)
    pub mod policies {
        pub use typespec_client_core::http::policies::{Policy, PolicyResult};
    }
    pub use typespec_client_core::http::{Transport, Context};

    // Client options (just the struct, not PipelineOptions/LoggingOptions)
    pub use typespec_client_core::http::ClientOptions;

    // HttpClient trait (needed for Transport construction)
    pub use typespec_client_core::http::HttpClient;
}

// === Credentials (moved from azure_core) ===
pub mod credentials;

// === HMAC (moved from azure_core) ===
pub mod hmac;
```

#### 2. Credentials module (moved from `azure_core/src/credentials.rs`)

```rust
// azure_core_shared/src/credentials.rs
// Contents moved verbatim from azure_core/src/credentials.rs

/// A secret string wrapper with constant-time PartialEq and redacted Debug.
pub struct Secret { ... }
pub struct SecretBytes { ... }

/// A token and its expiry.
pub struct AccessToken { ... }

/// Options for token requests.
pub struct TokenRequestOptions<'a> { ... }

/// Async token acquisition.
#[async_trait]
pub trait TokenCredential: Send + Sync { ... }
```

**Dependencies** needed for credentials:
- `typespec_client_core` (for `ClientMethodOptions` used by `TokenRequestOptions`, `SafeDebug`, `time::OffsetDateTime`)
- `async-trait`
- `serde`

#### 3. HMAC module (moved from `azure_core/src/hmac.rs`)

```rust
// azure_core_shared/src/hmac.rs
// Contents moved verbatim from azure_core/src/hmac.rs

#[cfg(feature = "hmac_rust")]
pub fn hmac_sha256(data: &str, key: &Secret) -> Result<String> { ... }

#[cfg(feature = "hmac_openssl")]
pub fn hmac_sha256(data: &str, key: &Secret) -> Result<String> { ... }
```

**Dependencies** needed for HMAC:
- `hmac` + `sha2` (feature-gated behind `hmac_rust`)
- `openssl` (feature-gated behind `hmac_openssl`)
- `base64`

### `azure_core_shared` Cargo.toml

```toml
[package]
name = "azure_core_shared"
version = "0.1.0"
description = "Shared core types for Azure SDK crates — HTTP primitives, credentials, and HMAC."

[dependencies]
# Core type sources (re-exported selectively)
typespec = { workspace = true, default-features = false }
typespec_client_core = { workspace = true, default-features = false, features = ["http"] }

# For credentials
async-trait = { workspace = true }
serde = { workspace = true }
time = { workspace = true }

# For Uuid
uuid = { workspace = true }

# For HMAC (feature-gated)
base64 = { workspace = true }
hmac = { workspace = true, optional = true }
sha2 = { workspace = true, optional = true }
openssl = { workspace = true, optional = true }

[features]
default = []
hmac_rust = ["dep:hmac", "dep:sha2"]
hmac_openssl = ["dep:openssl"]
# Pass through reqwest features for Transport
reqwest = ["typespec_client_core/reqwest"]
reqwest_native_tls = ["reqwest", "typespec_client_core/reqwest_native_tls"]
reqwest_rustls = ["reqwest", "typespec_client_core/reqwest_rustls"]
```

### What `azure_core_shared` Explicitly EXCLUDES

These types exist in `typespec_client_core` but are **not re-exported** by `azure_core_shared`:

| Excluded | Why |
|----------|-----|
| `RetryPolicy` trait + implementations | Driver has `ClientRetryPolicy` |
| `RetryOptions`, `ExponentialRetryOptions`, `FixedRetryOptions` | Driver has its own retry config |
| `Pipeline` struct | Driver has `CosmosPipeline` |
| `PipelineOptions` | Not used |
| `TransportPolicy` | Driver has `TrackedTransportPolicy` |
| `LoggingOptions`, logging policy | Driver does its own diagnostics |
| `ClientMethodOptions` | Driver passes `Context` directly |
| `sleep` module | Driver uses `tokio::time::sleep` |
| `async_runtime` module | Driver uses tokio directly |
| `stream` / `BytesStream` | Cosmos responses are buffered |
| `tracing` module | Driver uses `tracing` crate directly |
| `base64` module | Only needed internally for HMAC |
| `json` / `xml` modules | Driver does its own JSON |

Note: these types are still **available transitively** since `azure_core_shared` depends on `typespec_client_core`. But they're not part of the `azure_core_shared` API surface, so the driver has no reason to reach for them.

### Impact on `azure_core`

`azure_core` depends on `azure_core_shared` and re-exports credentials + hmac:

```rust
// azure_core/src/credentials.rs (AFTER)
pub use azure_core_shared::credentials::*;

// azure_core/src/hmac.rs (AFTER)
pub use azure_core_shared::hmac::*;
```

No breaking change. All existing `use azure_core::credentials::*` and `use azure_core::hmac::*` continue to work.

### Driver Dependency Changes

```toml
# Cargo.toml BEFORE
[dependencies]
azure_core = { workspace = true, features = ["reqwest_native_tls", "hmac_rust"] }

# Cargo.toml AFTER
[dependencies]
azure_core_shared = { workspace = true, features = ["reqwest_native_tls", "hmac_rust"] }
# azure_core is REMOVED — single dependency
```

### Driver Import Migration

Mechanical find-and-replace — all `azure_core::` → `azure_core_shared::`:

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
use azure_core_shared::http::policies::{Policy, PolicyResult};
use azure_core_shared::http::{Context, Request, Method, StatusCode};
use azure_core_shared::http::headers::{HeaderName, HeaderValue, Headers};
use azure_core_shared::credentials::{Secret, TokenCredential, AccessToken};
use azure_core_shared::hmac::hmac_sha256;
use azure_core_shared::{Error, Result, Bytes};
use azure_core_shared::time::OffsetDateTime;
use azure_core_shared::fmt::SafeDebug;
```

The migration is a single sed command:
```bash
find sdk/cosmos/azure_data_cosmos_driver/src -name '*.rs' \
  -exec sed -i '' 's/azure_core::/azure_core_shared::/g' {} +
```

## Comparison: Previous Approach vs This Approach

| Aspect | Previous (3 deps) | This (1 dep) |
|--------|--------------------|--------------|
| Driver dependencies | `typespec_client_core` + `azure_core_credentials` + `azure_core_hmac` | `azure_core_shared` only |
| New crates | 2 (`azure_core_credentials`, `azure_core_hmac`) | 1 (`azure_core_shared`) |
| Import prefix | Mixed (`typespec_client_core::`, `azure_core_credentials::`, `azure_core_hmac::`) | Uniform (`azure_core_shared::`) |
| Migration complexity | Multi-prefix find-and-replace | Single prefix swap |
| API surface control | No — driver sees all of `typespec_client_core` | Yes — only re-exported types are visible |
| Unused types exposed | Retry policies, pipeline builder, streaming, etc. | None (explicitly excluded) |
| Future additions | Must coordinate which crate to add to | Single place to add what the driver needs |

## Phased Migration Plan

### Phase 1: Create `azure_core_shared` crate

1. Create `sdk/core/azure_core_shared/` with `Cargo.toml` and `src/lib.rs`
2. Add re-exports of HTTP primitive types from `typespec_client_core`
3. Move `Secret`, `SecretBytes`, `AccessToken`, `TokenRequestOptions`, `TokenCredential` from `azure_core/src/credentials.rs` into `azure_core_shared/src/credentials.rs`
4. Move `hmac_sha256` implementations from `azure_core/src/hmac.rs` into `azure_core_shared/src/hmac.rs`
5. Add `azure_core_shared` to workspace `Cargo.toml`
6. Validate: `cargo build -p azure_core_shared --all-features`

### Phase 2: Wire `azure_core` to re-export from `azure_core_shared`

7. Add `azure_core_shared` as dependency of `azure_core`
8. Update `azure_core/src/credentials.rs` to `pub use azure_core_shared::credentials::*;`
9. Update `azure_core/src/hmac.rs` to `pub use azure_core_shared::hmac::*;`
10. Validate: `cargo build -p azure_core --all-features && cargo test -p azure_core --all-features`

### Phase 3: Switch driver dependency

11. Replace `azure_core` with `azure_core_shared` in driver `Cargo.toml`
12. Global find-and-replace: `azure_core::` → `azure_core_shared::` across all driver source files
13. Validate: `cargo fmt -p azure_data_cosmos_driver && cargo clippy -p azure_data_cosmos_driver --all-features --all-targets`
14. Validate: `cargo build -p azure_data_cosmos_driver --all-features && cargo test -p azure_data_cosmos_driver --all-features`

### Phase 4: Validate full stack

15. `cargo build --workspace && cargo test --workspace`
16. Verify `azure_data_cosmos` (SDK crate) still works unchanged
17. Verify `azure_data_cosmos_native` (C FFI crate) still works unchanged

## Scope of Changes by Crate

| Crate | Changes | Breaking? |
|-------|---------|-----------|
| `azure_core_shared` | **NEW** — HTTP primitives + credentials + HMAC | N/A (new) |
| `azure_core` | Re-exports credentials + hmac from `azure_core_shared` | **No** — public API identical |
| `typespec` | None | No |
| `typespec_client_core` | None | No |
| `azure_data_cosmos_driver` | Swap `azure_core` → `azure_core_shared`; update import prefix | No (internal crate) |
| `azure_data_cosmos` | None (still uses `azure_core`) | No |
| `azure_data_cosmos_native` | None (uses driver) | No |
| Workspace `Cargo.toml` | Add one new workspace member + dep | No |

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| `TokenRequestOptions` uses `ClientMethodOptions` from `typespec_client_core` | Low | `azure_core_shared` already depends on `typespec_client_core` — just don't re-export `ClientMethodOptions` at the crate root |
| `azure_identity` wants `TokenCredential` | None | `azure_identity` can depend on `azure_core_shared` directly OR continue via `azure_core` re-export — either works |
| Crate naming (`azure_core_shared` vs alternatives) | Low | Name clearly indicates relationship to `azure_core`; alternatives: `azure_core_base`, `azure_core_essentials` |
| Future driver needs a type not in `azure_core_shared` | Low | Add another re-export to `azure_core_shared` — it's our crate to evolve |
| Transitive dep still includes all of `typespec_client_core` | None | True, but unused code is dead-code-eliminated. The benefit is API surface control, not binary size. |

## Benefits

1. **Single dependency** — driver's `Cargo.toml` has one dep (`azure_core_shared`) instead of `azure_core` or a mix of three crates
2. **Controlled API surface** — driver only sees types it actually uses; unused retry/pipeline/streaming types are not in scope
3. **Uniform import prefix** — all driver imports are `azure_core_shared::*`, simple migration
4. **Driver decoupled from azure_core versioning** — `azure_core_shared` can version independently
5. **No breaking changes** — `azure_core` re-exports preserve all existing public APIs
6. **Reusable** — any crate that needs "just HTTP primitives + credentials" can use `azure_core_shared` instead of the full `azure_core`

## Open Questions

1. **Naming**: `azure_core_shared` vs `azure_core_base` vs `azure_core_essentials`? Should convey "lean subset of azure_core for library authors."

2. **Should HMAC be a feature or always included?** Currently feature-gated (`hmac_rust` / `hmac_openssl`). Could make the whole `hmac` module optional if some consumers of `azure_core_shared` don't need crypto.

3. **Should `azure_identity` also switch to `azure_core_shared`?** It primarily needs `TokenCredential` + HTTP types. Moving it would further validate the crate as a reusable foundation.

4. **Type compatibility guarantee**: Since `azure_core_shared::http::Request` is literally `typespec_client_core::http::Request` (re-export, not wrapper), types are fully compatible across crate boundaries. Worth documenting this explicitly.
