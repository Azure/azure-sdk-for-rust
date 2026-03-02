# Spec: Remove azure_core Dependency from azure_data_cosmos_driver

## Problem Statement

The `azure_data_cosmos_driver` crate currently depends on `azure_core` for HTTP infrastructure, credentials, error types, and utilities. Per the multi-crate versioning strategy (AGENTS.md), the driver should be independently versionable from the SDK layer. The `azure_core` dependency creates version coupling — breaking changes in `azure_core` force driver major version bumps even when the driver's own API is stable.

Additionally, since the driver is consumed by `azure_data_cosmos_native` (C FFI for Java/.NET/Python SDKs), minimizing Rust-ecosystem dependencies reduces binary size and cross-language complexity.

Reference: <https://github.com/Azure/azure-sdk-for-python/issues/45463>

## Current azure_core Usage Inventory (149 references)

### Category 1: HTTP Pipeline Infrastructure (~60% of references)

The driver builds its own custom pipeline (`CosmosPipeline`) but uses `azure_core` types throughout:

| Type | Used In | Purpose |
|------|---------|---------|
| `Policy` trait | authorization_policy, headers_policy, tracked_transport, pipeline | Pipeline policy chain |
| `PolicyResult` | All policy impls | Return type for `Policy::send()` |
| `Transport` | transport/mod.rs, pipeline.rs | Wraps reqwest client |
| `Request` | All policy impls, cosmos_driver | HTTP request builder |
| `RawResponse` / `AsyncRawResponse` | pipeline.rs, tests | HTTP response types |
| `Context` | All policy impls, pipeline, cosmos_driver | Request context bag |
| `Headers`, `HeaderName`, `HeaderValue`, `AsHeaders` | headers_policy, authorization_policy, cosmos_headers, partition_key | Header types |
| Standard headers (`AUTHORIZATION`, `ACCEPT`, etc.) | headers_policy, authorization_policy | Well-known header constants |
| `Method` | authorization_policy, cosmos_driver, tests | HTTP methods |
| `StatusCode` | cosmos_status, cosmos_response, diagnostics, pipeline tests | HTTP status codes |
| `ClientOptions` | runtime.rs | Client-level options struct |
| `Bytes` | tests | Byte buffer type |

### Category 2: Error Types (~25% of references)

| Type | Used In | Purpose |
|------|---------|---------|
| `azure_core::Result<T>` | Almost everywhere | Result type alias |
| `azure_core::Error` | Options, builders, transport | Error construction |
| `azure_core::error::ErrorKind` | tracked_transport, options, builders | Error categorization: `Connection`, `Credential`, `DataConversion`, `Io`, `Other`, `HttpResponse` |

### Category 3: Credentials (~10% of references)

| Type | Used In | Purpose |
|------|---------|---------|
| `TokenCredential` trait | account_reference, authorization_policy | AAD/Entra token acquisition |
| `AccessToken` | authorization_policy tests | Token response type |
| `Secret` | account_reference, connection_string | Secret string wrapper |
| `TokenRequestOptions` | authorization_policy tests | Token request options |

### Category 4: Utilities (~5% of references)

| Type | Used In | Purpose |
|------|---------|---------|
| `hmac::hmac_sha256` | authorization_policy | Master key HMAC signing |
| `time::OffsetDateTime` | authorization_policy | Timestamp for auth header |
| `time::to_rfc7231` | authorization_policy | Date formatting |
| `time::Duration` | authorization_policy tests | Duration type |
| `fmt::SafeDebug` | connection_string | Debug derive that redacts secrets |

## Replacement Strategy by Category

### 1. TokenCredential Trait — KEEP (via lighter dependency)

The driver must acquire AAD tokens. The `TokenCredential` trait is the standard contract that `azure_identity` implements.

**Recommendation**: Depend on `typespec_client_core` (or a future `azure_core_credentials` crate) instead of `azure_core`. The trait is stable and rarely changes. If full decoupling is required, define an identical trait in the driver and provide a blanket adapter.

### 2. Secret Type — REPLACE with own type

`Secret` is a ~10-line newtype around `String` with `Debug` redaction. No reason to pull `azure_core` for this.

```rust
#[derive(Clone, PartialEq, Eq)]
pub struct Secret(String);
impl Secret {
    pub fn new(s: impl Into<String>) -> Self { Self(s.into()) }
    pub fn secret(&self) -> &str { &self.0 }
}
impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("***")
    }
}
```

### 3. HMAC-SHA256 — REPLACE with direct `hmac` + `sha2` crates

`azure_core::hmac::hmac_sha256` delegates to these crates when the `hmac_rust` feature is on. Cut out the middleman.

### 4. Time Utilities — REPLACE with direct `time` crate

`azure_core::time` wraps the `time` crate. Use it directly. For `to_rfc7231`, implement the format string locally (~5 lines).

### 5. HTTP Pipeline Types — REPLACE with `http` crate + driver-owned abstractions

The `http` crate (`hyperium/http`) provides `Method`, `StatusCode`, `HeaderMap`, `HeaderName`, `HeaderValue` — the de facto standard that reqwest uses internally.

| Driver Type to Create | Replaces | Implementation |
|-----------------------|----------|----------------|
| `driver::http::Policy` trait | `azure_core::http::policies::Policy` | Own async trait (~10 lines) |
| `driver::http::Request` | `azure_core::http::Request` | Thin wrapper over `http::request::Builder` or `reqwest::Request` |
| `driver::http::Response` | `azure_core::http::RawResponse` | Thin wrapper over `reqwest::Response` |
| `driver::http::Context` | `azure_core::http::Context` | Type-map bag (~30 lines, or use `typemap-rev` crate) |
| `driver::http::Headers` | `azure_core::http::headers::Headers` | Re-export `http::HeaderMap` |
| `driver::http::HeaderName` / `HeaderValue` | Same | Re-export from `http` crate |
| `driver::http::Method` | Same | Re-export from `http` crate |
| `driver::http::StatusCode` | Same | Re-export from `http` crate |
| `driver::http::Transport` trait | `azure_core::http::Transport` | Own trait (~10 lines), reqwest impl |

### 6. Error Types — REPLACE with driver-owned error

The driver uses only a subset of `ErrorKind` variants. A driver-specific error can carry Cosmos-specific context (status, substatus, activity ID) natively.

```rust
#[derive(Debug)]
pub enum ErrorKind {
    Connection,
    Credential,
    DataConversion,
    Io,
    HttpResponse { status: StatusCode, substatus: Option<u32> },
    Other,
}

pub struct Error {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}
pub type Result<T> = std::result::Result<T, Error>;
```

### 7. ClientOptions — REPLACE with driver's own config

`ClientOptions` is used only in `runtime.rs`. Replace with a driver-specific `TransportOptions` struct.

### 8. SafeDebug — REPLACE with manual Debug impl

Only used on `ConnectionString`. Replace the derive with a hand-written `Debug` impl that redacts the key.

### 9. Bytes — Use `bytes` crate directly

Already a transitive dependency via reqwest.

## Proposed New Dependency Set

```toml
[dependencies]
# Existing (keep)
async-trait = { workspace = true }
base64 = { workspace = true }
futures = { workspace = true }
reqwest = { workspace = true, features = ["native-tls"] }
serde = { workspace = true }
serde_json = { workspace = true }
tracing = { workspace = true }
url = { workspace = true }
uuid = { workspace = true }

# New direct (currently transitive or via azure_core)
bytes = { workspace = true }
hmac = { workspace = true }
sha2 = { workspace = true }
http = { workspace = true }
time = { workspace = true }

# Credentials — pick ONE approach:
# Option A: Keep typespec_client_core for TokenCredential only
typespec_client_core = { workspace = true }
# Option B: Full decoupling — define own trait, no azure/typespec deps
# (nothing added)

# REMOVED
# azure_core = { workspace = true, features = ["reqwest_native_tls", "hmac_rust"] }
```

## New Module Layout

```text
src/
├── credentials/           (NEW)
│   ├── mod.rs             # Re-exports, Credential enum
│   ├── secret.rs          # Secret type
│   └── token.rs           # TokenCredential trait (or re-export from typespec_client_core)
│
├── error.rs               (NEW — driver::Error + driver::ErrorKind + driver::Result)
│
├── http/                  (NEW)
│   ├── mod.rs             # Re-exports from `http` crate + own types
│   ├── context.rs         # Type-map context bag
│   ├── headers.rs         # Cosmos-specific header constants
│   ├── policy.rs          # Policy trait
│   ├── request.rs         # Request builder
│   ├── response.rs        # Response wrapper
│   └── transport.rs       # Transport trait + reqwest impl
│
├── crypto.rs              (NEW — hmac_sha256 using hmac+sha2)
├── time_util.rs           (NEW — RFC 7231 formatting using time crate)
│
├── driver/                (existing, updated imports)
├── models/                (existing, updated imports)
├── options/               (existing, updated imports)
├── diagnostics/           (existing, updated imports)
└── lib.rs                 (updated)
```

## Phased Migration Plan

### Phase 1: Create Internal Abstractions (Non-Breaking)

Create all new modules as thin wrappers/adapters over `azure_core` types. Internal code starts using `crate::*` imports but the underlying implementation still delegates to `azure_core`.

**Todos**:

1. `create-error-module` — Define `driver::error::{Error, ErrorKind, Result}` wrapping azure_core equivalents
2. `create-credentials-module` — Define `driver::credentials::{Secret, Credential, TokenCredential}` wrapping/re-exporting azure_core equivalents
3. `create-http-module` — Define `driver::http::{Policy, Request, Response, Context, Headers, Method, StatusCode, Transport}` wrapping azure_core equivalents
4. `create-crypto-module` — Define `driver::crypto::hmac_sha256` delegating to azure_core::hmac
5. `create-time-module` — Define `driver::time_util` delegating to azure_core::time

### Phase 2: Switch All Imports

Update every `use azure_core::*` in the driver to `use crate::*`. azure_core remains a dependency but is only referenced inside the new wrapper modules.

**Todos**:

6. `migrate-error-imports` — Replace all `azure_core::Error/Result/ErrorKind` with `crate::error::*`
7. `migrate-http-imports` — Replace all `azure_core::http::*` with `crate::http::*`
8. `migrate-credentials-imports` — Replace all `azure_core::credentials::*` with `crate::credentials::*`
9. `migrate-utility-imports` — Replace `azure_core::hmac`, `azure_core::time`, `azure_core::Bytes`, `azure_core::fmt::SafeDebug`
10. `validate-phase2` — Full test suite, clippy, fmt

### Phase 3: Replace Implementations

Swap wrapper implementations with direct dependencies. Remove azure_core.

**Todos**:

11. `impl-error` — Standalone Error/ErrorKind (no azure_core)
12. `impl-http` — Use `http` crate types + own Policy/Transport/Context
13. `impl-credentials` — Own Secret; decide on TokenCredential (own trait vs typespec_client_core)
14. `impl-crypto` — Direct hmac+sha2
15. `impl-time` — Direct time crate + RFC 7231 formatter
16. `remove-azure-core-dep` — Delete azure_core from Cargo.toml
17. `validate-phase3` — Full test suite, clippy, fmt, doc

### Phase 4: SDK Adapter Layer

Update `azure_data_cosmos` (the SDK crate) to bridge between `azure_core` types and `driver` types.

**Todos**:

18. `sdk-error-adapter` — Convert driver::Error ↔ azure_core::Error
19. `sdk-credential-adapter` — Wrap azure_core::TokenCredential for driver
20. `validate-sdk` — Full test suite for azure_data_cosmos

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| TokenCredential incompatibility with azure_identity | High — breaks AAD auth | Phase 1 uses adapter pattern; Phase 3 defines compatible trait |
| Missed azure_core behavior (e.g., header normalization) | Medium — subtle bugs | Comprehensive test suite already exists |
| Increased maintenance burden (own HTTP types) | Medium — more code to maintain | Types are thin wrappers over well-tested ecosystem crates |
| Binary size increase from duplicate types | Low — types are small | Monitor with `cargo bloat` |
| Breaking change for azure_data_cosmos consumers | None — driver is internal | SDK layer adapts between azure_core ↔ driver types |

## Open Questions

1. **`typespec_client_core` vs full decoupling?** Depending on `typespec_client_core` for just `TokenCredential` is lighter than `azure_core` but still couples to the azure crate family. Full decoupling means defining our own trait and requiring an adapter in the SDK.

2. **Should `TokenCredential` be an exact copy of azure_core's trait or simplified?** The driver only needs `get_token(scopes, options) -> Result<AccessToken>`. A simplified trait reduces coupling but requires adapters.

3. **Should we use `http` crate types directly in public API or newtype-wrap them?** Direct use is simpler but couples public API to `http` crate versions. Newtypes add indirection but decouple versioning.

4. **Is `SafeDebug` derive worth replicating?** It's only used on `ConnectionString`. A manual `Debug` impl is simpler than replicating the derive macro.
