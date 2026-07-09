# ADR-001 — Ship DTX behind a disabled-by-default `preview_dtx` feature

**Status:** Accepted
**Date:** 2026-07-06

## Context

DTX is a preview capability whose wire contract and API surface are still evolving.
It must not affect stable builds or the public API.

## Decision

Gate every DTX type, the driver entry point, the pipeline classifiers, and the
emulator handler behind a single `preview_dtx` Cargo feature, off by default. The
SDK feature re-exports the driver feature transitively.

## Consequences

Default builds contain no DTX surface; preview consumers opt in explicitly. A coarse
single gate prevents a half-wired feature from compiling.

## Alternatives

Per-module features were rejected because they are too granular and easy to mis-wire.
A runtime flag was rejected because it would still ship preview types in the stable API.

## References

- Distributed Transactions spec: ../spec.md
