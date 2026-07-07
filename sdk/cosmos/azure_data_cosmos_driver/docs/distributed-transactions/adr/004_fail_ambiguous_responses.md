# ADR-004 — Fail closed on ambiguous coordinator responses

**Status:** Accepted
**Date:** 2026-07-06

## Context

A malformed body, a per-operation parse error, or a result/operation count mismatch
under a **success** envelope would otherwise let a partial or unverifiable result be
reported as success.

## Decision

On any such anomaly under a success envelope, synthesize `500` (`fail_closed`); under
a failure envelope, pad every operation with the envelope status (`padded`). Never
emit a success with unparseable per-operation data.

## Consequences

Callers retry or reconcile rather than trust ambiguous data, upholding "atomic or
nothing." This is slightly more conservative than the raw server bytes.

## Alternatives

Surfacing partial results with a warning was rejected because it violates atomic
guarantees and pushes ambiguity onto every caller.

## References

- Distributed Transactions spec: ../spec.md
