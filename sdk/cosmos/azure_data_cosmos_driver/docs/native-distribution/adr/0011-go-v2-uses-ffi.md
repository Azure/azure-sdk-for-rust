# ADR 0011 — Go v2 uses the Rust driver through FFI

**Status:** Proposed

## Context

Go v2 needs a path that can deliver the next Cosmos DB SDK within the current
timeframe while staying close to the Rust driver's behavior. Two implementation
paths were evaluated:

- **FFI:** Go links the prebuilt Rust driver through cgo and a C ABI.
- **Pure-Go port:** Go reimplements the driver behavior directly.

A time-boxed pure-Go spike proved that a faithful port is technically feasible
for the core gateway-mode path tested. It stayed `CGO_ENABLED=0`, used zero
third-party dependencies, and was validated against Rust behavior through a
scenario-based differential harness backed by the Rust in-memory emulator.

The spike also clarified the trade-off: avoiding cgo and native packaging means
re-owning driver behavior in Go and carrying long-term drift risk from Rust.

## Decision

- Go v2 will proceed with the **FFI approach** for the current delivery window.
- Go v2 will consume the **prebuilt Rust native driver** through cgo and the
  C ABI described by the native wrapper.
- Packaging, platform support, signing, ABI versioning, and fan-out are owned by
  the native distribution ADRs, especially ADR 0004, ADR 0008, ADR 0009, and ADR
  0010.

## Consequences

- Go v2 gets the same driver implementation as Rust, reducing short-term feature
  drift and implementation risk.
- Go v2 takes a dependency on `CGO_ENABLED=1` for the native driver path.
- The packaging bar is high: the Go package must still feel like a normal Go SDK
  as much as possible. Customers should not manually install Rust/C toolchains or
  copy native libraries.
- The native artifacts must be produced, versioned, selected, and loaded for the
  supported Go OS/architecture matrix.
- Testing investment remains required. The differential harness and in-memory
  emulator work should continue to validate Go-visible behavior against the Rust
  driver.

## Alternatives considered

- **Pure-Go driver port.** Feasible for the core path tested, and valuable as a
  validation reference, but not selected for Go v2 because it shifts cost from
  packaging to long-term Go ownership and Rust-to-Go drift.
- **Manual native-library installation.** Rejected for a first-party Go SDK. It
  breaks the normal `go get` / `go build` expectation.
- **Pure-Go downloader shim.** Rejected by ADR 0004. The native driver must be
  packaged and linked through the FFI path, not fetched by a downloader stub.
