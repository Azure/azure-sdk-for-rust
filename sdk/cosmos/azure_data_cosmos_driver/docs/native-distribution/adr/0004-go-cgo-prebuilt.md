# ADR 0004 — Go consumes via cgo against a prebuilt header + lib from the Go feed

**Status:** Proposed for review — core model firm, **delivery shape WIP**

> **WIP.** The *decision* — Go links a **packaged** prebuilt native library via cgo (`CGO_ENABLED=1`) using the `C.*` FFI stubs from the cbindgen header — is firm. Still being settled: the **delivery shape** (Universal Package vs vendored binaries module, Q3) and the static-vs-dynamic default (Q4). Explicitly **out of scope**: a pure-Go shim that downloads the native lib — the binary is packaged and linked, never fetched by a stub. The Go v2 FFI direction is captured in [ADR 0011](0011-go-v2-uses-ffi.md).

## Context
Go has no NuGet. Go links C libraries through cgo, which needs a C header and a library available at `go build` time. The Go SDK already implements the completion-queue receive loop, `cgo.Handle` correlation, and buffer copy-out; the only distribution question is how header+lib+ABI version reach the Go build.

## Decision
- Go consumes the prebuilt **`include/` header and `lib/` library via cgo** with **`CGO_ENABLED=1`**: `#cgo CFLAGS -I…` parses the header into the `C.*` **FFI stubs**, and `#cgo LDFLAGS -L… -lazurecosmosdriver` links the library. **Not** NuGet, and **not** a pure-Go build.
- The native library is **packaged inside the delivered artifact** and linked at `go build`. It is **not** a pure-Go shim that downloads the library at build/run time — customers link a real binary through the FFI stubs; they are not handed a downloader stub.
- Prefer the **static `.a`** for a self-contained Go binary; dynamic linking is supported as an option.
- The header + lib are delivered through the **azure-sdk-for-go feed** — an Azure Artifacts Universal Package fetched at build, or a vendored "binaries" Go module with per-OS build tags (delivery *shape* is open Q3; in every shape the binary is **packaged**, not fetched by a shim). Either way it derives from the ADR 0001 hand-off artifact.

## Consequences
- Go reuses the exact same signed binaries as .NET — no Go-specific build of the driver.
- cgo + static lib means `CGO_ENABLED=1` and a C toolchain on the Go build host; cross-compilation needs a cross C toolchain.
- Everything resolves at `go build` — no runtime resolver / `runtime.json` / RID probing. The same `ABI_VERSION` feeds Go's handshake (ADR 0005).

## Alternatives considered
- Wrap the lib in NuGet for Go — rejected: Go can't consume NuGet.
- A neutral consumer bundle Go downloads — rejected (ADR 0001/0002): pulls irrelevant formats.
- A pure-Go **shim module** that downloads the native lib at build/run time — rejected: customers can't be handed a downloader stub; the binary must be packaged and linked via cgo.
- **Pure-Go reimplementation of the driver — not selected for Go v2.** A time-boxed spike built a working `CGO_ENABLED=0`, zero-dependency vertical slice and validated it behavior-for-behavior against the real Rust driver with a differential harness. It is valuable as a parity oracle / risk probe, but not the Go v2 delivery path; see [ADR 0011](0011-go-v2-uses-ffi.md).
