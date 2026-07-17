# Architecture Decision Records — Go v2 FFI direction

ADRs (Architecture Decision Records) capture **what we decided** and a brief
**why**, in a minimal, easy-to-reference form. They are **numbered and
immutable**: once accepted, an ADR is not edited; a later ADR may supersede it.
Detailed discussion and exploration live in
[`go-v2-ffi-exploration.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/native-driver-distribution/go-native-distribution/go-v2-ffi-exploration.md),
not here.

**Format (template):** Status (immediately under the title) · Context (2-4
sentences) · Decision (1-3 bullets) · Consequences (2-4 bullets) · Alternatives
considered (1 line each).

| # | Title | Status |
|---|-------|--------|
| [0001](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/native-driver-distribution/go-native-distribution/adr/0001-go-v2-uses-ffi.md) | Go v2 uses the Rust driver through FFI | Proposed |

> This ADR is proposed for review. Packaging mechanics are intentionally left to
> the native distribution ADRs; this record captures the Go v2 implementation
> direction.
