# ADR-008 — Implement emulator rollback fidelity

**Status:** Accepted
**Date:** 2026-07-06

## Context

Offline tests and the live dual-comparison need coordinator-faithful abort/rollback
and read-snapshot semantics, not just sequential point-op execution.

## Decision

The emulator validates all operations (prepare/vote), then applies with per-operation
pre-image capture and reverse-order rollback on runtime failure. Aborts emit `452`
with `453`/`5415` for prepared-then-rolled-back writes; read failures rewrite
successful reads to `424` and promote the surviving codes.

## Consequences

The emulator reproduces the abort/rollback and snapshot contracts, enabling meaningful
parity assertions. Transaction type is inferred from operations to keep the handler
self-contained.

## Alternatives

Sequential point operations with no rollback were rejected because they cannot test
the atomic-abort or snapshot-failure contracts.

## References

- Distributed Transactions spec: ../spec.md
