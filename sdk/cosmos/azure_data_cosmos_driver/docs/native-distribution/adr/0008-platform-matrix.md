# ADR 0008 — A defined platform matrix; unsupported platforms error clearly

**Status:** Accepted (proposed for review)

## Context
A native library must be built per platform (OS + architecture + libc). The support surface must be bounded and explicit so build, signing, and testing are tractable, and so consumers get a clear answer on an unsupported platform.

## Decision
- The GA matrix is: `win-x64`, `win-arm64`, `linux-x64` (glibc, low floor), `linux-musl-x64`, `linux-arm64`, `osx-x64`, `osx-arm64`.
- `linux-musl-x64` is a **separate target** from `linux-x64` (a glibc build will not load on musl).
- `wasm` is out of scope (no FFI story). An unsupported platform **fails with an actionable error** naming the supported set, never a silent or cryptic failure.

## Consequences
- Bounded, predictable build/sign/test surface.
- Clear consumer experience on unsupported platforms.
- Adding a platform later is an additive ADR + matrix row, not a redesign.

## Alternatives considered
- Build only the most common platforms and let others fail at link/load — rejected: poor experience, no clear message.
- Include `win-x86` / mobile now — deferred (open Q): no demand yet.
