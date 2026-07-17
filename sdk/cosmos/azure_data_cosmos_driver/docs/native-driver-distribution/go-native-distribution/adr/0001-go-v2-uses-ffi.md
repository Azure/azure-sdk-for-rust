# ADR 0001 — Go v2 uses the Rust driver through FFI

**Status:** Proposed

## Context

Go v2 needs a path that can deliver in the current timeframe while staying close
to the Rust driver's behavior. A time-boxed pure-Go spike proved that a
gateway-mode core path can be ported and validated, but it also clarified the
central trade-off: avoiding cgo and native packaging means re-owning driver
logic in Go and carrying long-term Rust-to-Go drift risk.

## Decision

- Go v2 proceeds with the **Rust-driver FFI path** for the current delivery
  window.
- The Go binding consumes the **prebuilt Rust native driver** through cgo and the
  C ABI surface.
- Packaging mechanics, platform matrix, signing, ABI versioning, and fan-out are
  handled by the native distribution design and ADRs.

## Consequences

- Go v2 gets the same driver implementation as Rust, reducing short-term feature
  drift and implementation risk.
- Go v2 takes a dependency on `CGO_ENABLED=1` for the native driver path.
- The Go packaging model becomes part of the product decision: customers should
  not manually install a Rust toolchain or copy native libraries for the common
  path, but cgo still requires an available C build toolchain.
- Differential validation remains important so Go-visible behavior can be tested
  against the Rust driver.

## Alternatives considered

- **Pure-Go driver port** — technically feasible for the core path tested, but
  not selected for Go v2 because it shifts cost from packaging to long-term Go
  ownership and Rust-to-Go drift.
- **Manual native-library installation** — rejected for the common path because
  it breaks the normal `go get` / `go build` expectation for a first-party Go
  SDK.
- **Pure-Go downloader shim** — rejected by the native distribution ADRs; the
  native driver must be packaged and linked through the FFI path, not fetched by
  a downloader stub.

## Discussion

See
[`go-v2-ffi-exploration.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/native-driver-distribution/go-native-distribution/go-v2-ffi-exploration.md)
for the meeting context, pure-Go spike evidence, packaging concerns, tentative
platform matrix, and market reference points.
