# Go v2 SDK decision: use the Rust driver through FFI

**Status:** Proposed  
**Decision:** Go v2 proceeds with the Rust-driver FFI path for the current delivery window.

## Context

The team evaluated two implementation paths for the next-generation Go SDK:

- **FFI path:** Go calls the prebuilt Rust driver through cgo and a C ABI.
- **Pure-Go port:** Go reimplements the driver behavior directly in Go.

Both paths were shown to be technically feasible. The pure-Go spike proved that a
gateway-mode core path can be ported in idiomatic Go: `CGO_ENABLED=0`, zero
third-party dependencies, point create/read, retry/routing behavior, basic
failover behavior, cross-partition unordered query fan-out, and unsupported-query
rejection parity.

The spike was validated with a scenario-based differential harness. The harness
put an HTTP endpoint in front of the Rust driver's in-memory emulator so the Go
port and the Rust driver could run against the same emulator-backed state and
compare caller-visible outcomes.

That result is important: the Go port is credible. The decision is therefore not
"can we port it?" The real trade-off is:

> Is avoiding cgo and native packaging worth re-owning driver logic that Rust
> already implements?

For the current Go v2 timeframe, the answer is **no**. FFI gives faster delivery,
stronger parity with the Rust driver, and lower short-term implementation risk.

## Decision

- Go v2 will use the **Rust driver through FFI** for the current delivery window.
- The Go binding will consume the **prebuilt native driver** through cgo and the
  C ABI surface.
- The pure-Go port remains useful as validation evidence and as a reference for
  Go-visible behavior, but it is not the Go v2 delivery path.

## Why FFI for Go v2

FFI has the better short-term delivery shape:

- **Parity:** Go consumes the same driver implementation as Rust.
- **Velocity:** new Rust-driver behavior does not need to be re-ported into Go
  before Go can benefit.
- **Risk containment:** the hard driver logic stays in one implementation.
- **Testing leverage:** the Go binding can validate language-level behavior
  against the Rust driver's known behavior instead of re-proving the full driver.

The main cost is not binary size. The main cost is **Go packaging and customer
deployment experience**.

## Packaging implications

Cosmos Go v1 is consumed like a normal Go SDK:

```bash
go get github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos
go build ./...
```

There is no Rust compiler, C compiler, native shared library, or post-install
step in the customer path. If Go v2 uses FFI, the packaging bar is therefore:

> Can the Go SDK still feel like one normal Go module across the supported
> Windows, macOS, and Linux targets?

That means native Rust driver artifacts must be produced, versioned, selected,
and linked for the supported Go OS/architecture matrix.

The platform discussion quickly reaches roughly ten target combinations:

| OS family | Target combinations discussed |
|---|---|
| Windows | x64/amd64, ARM64, possibly x86 as a consideration |
| macOS | ARM64/Apple Silicon, x64/Intel |
| Linux glibc | x64/amd64, ARM64 |
| Linux musl/Alpine | x64/amd64, ARM64 |

The Linux callout is especially important: supporting both glibc and musl means
likely needing **four Linux binaries**: amd64/arm64 × glibc/musl.

With an estimated **~5 MB optimized native binary per platform**, the
all-platform packaging footprint can reach roughly **~50 MB** before compression
or packaging refinements. This is manageable for an initial release, but it is a
real packaging design point and should not be treated as an implementation
detail.

## Packaging models to evaluate

Two practical models are on the table:

| Model | Customer experience | Trade-off |
|---|---|---|
| **Bundle the full platform matrix in the Go package** | Closest to `go get` / `go build` just working everywhere | Larger module/package footprint; all platform binaries travel together |
| **Per-platform native dependency** | Customer obtains only the platform-specific binary they need | Smaller per-customer footprint, but more setup burden and more room for install mistakes |

The second model has industry precedent. For example,
[`confluent-kafka-go`](https://github.com/confluentinc/confluent-kafka-go/blob/master/README.md#librdkafka)
bundles prebuilt `librdkafka` binaries for common platforms, requires
`CGO_ENABLED` to remain enabled, uses a `musl` build tag for Alpine, and asks
customers to install `librdkafka` manually when they need unsupported platforms
or special features such as GSSAPI/Kerberos.

That reference shows a viable market pattern for a Go library backed by native
code: **bundle the common path, document unsupported/special paths clearly, and
make native requirements explicit**. It also confirms that this is a materially
different customer experience from a pure-Go Azure SDK.

## Pure-Go port trade-off

The pure-Go path has a strong developer-experience story:

- `CGO_ENABLED=0`
- normal Go cross-compilation
- no native runtime dependency
- Go standard library transport/TLS/runtime

But it moves the cost into ownership:

- driver behavior must be reimplemented in Go
- every Rust-driver change has to be tracked, ported, and revalidated
- long-term drift becomes the main risk
- the hardest remaining areas still need evidence: richer query execution,
  continuation/split-resume behavior, broader multi-region behavior, change feed,
  and bulk/batch

The pure-Go spike remains valuable because it proved the fallback is not
fictional. It also produced a useful differential-validation approach. But for
Go v2, it is better treated as evidence and risk retirement, not the shipping
plan.

## Consequences

- Go v2 takes a dependency on cgo for the Rust-driver path.
- The Go SDK packaging design must be treated as part of the product decision,
  not as an afterthought.
- The native distribution plan must explicitly cover Windows, macOS, Linux
  glibc, Linux musl/Alpine, amd64, and ARM64 targets.
- Customer-facing docs must clearly state the native dependency model,
  `CGO_ENABLED` expectations, supported platform matrix, and unsupported-platform
  behavior.
- Differential validation should continue so Go-visible behavior can be tested
  against the Rust driver.

## Non-goals

- This decision does not define the final Go packaging shape.
- This decision does not require customers to manually install native libraries.
- This decision does not introduce direct mode; both the Rust and Go v2 paths are
  gateway-mode SDKs.
- This decision does not remove the pure-Go spike as a reference or validation
  asset.
