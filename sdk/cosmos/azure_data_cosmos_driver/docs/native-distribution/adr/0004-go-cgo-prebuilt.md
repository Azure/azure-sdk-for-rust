# ADR 0004 — Go consumes via cgo against a prebuilt header + lib from the Go feed

**Status:** Accepted (proposed for review)

## Context
Go has no NuGet. Go links C libraries through cgo, which needs a C header and a library available at `go build` time. The Go SDK already implements the completion-queue receive loop, `cgo.Handle` correlation, and buffer copy-out; the only distribution question is how header+lib+ABI version reach the Go build.

## Decision
- Go consumes the prebuilt **`include/` header and `lib/` library via cgo** (`#cgo CFLAGS -I…` to parse the header into `C.*` symbols; `#cgo LDFLAGS -L… -lazurecosmosdriver` to link), **not NuGet**.
- Prefer the **static `.a`** for a self-contained Go binary; dynamic linking is supported as an option.
- The header + lib are delivered through the **azure-sdk-for-go feed** — an Azure Artifacts Universal Package fetched at build, or a vendored "binaries" Go module with per-OS build tags (delivery shape is open Q3). Either way it derives from the ADR 0001 hand-off artifact.

## Consequences
- Go reuses the exact same signed binaries as .NET — no Go-specific build of the driver.
- cgo + static lib means `CGO_ENABLED=1` and a C toolchain on the Go build host; cross-compilation needs a cross C toolchain.
- Everything resolves at `go build` — no runtime resolver / `runtime.json` / RID probing. The same `ABI_VERSION` feeds Go's handshake (ADR 0005).

## Alternatives considered
- Wrap the lib in NuGet for Go — rejected: Go can't consume NuGet.
- A neutral consumer bundle Go downloads — rejected (ADR 0001/0002): pulls irrelevant formats.
- Pure-Go reimplementation of the driver — rejected: defeats the shared-core goal.
