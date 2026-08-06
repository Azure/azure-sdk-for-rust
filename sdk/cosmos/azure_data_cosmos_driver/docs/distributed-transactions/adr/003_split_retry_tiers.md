# ADR-003 — Split body-bearing and bodyless retry tiers

**Status:** Accepted
**Date:** 2026-07-06

## Context

Some DTX failures reach the coordinator's transaction logic and come back with a body
plus an authoritative `isRetriable`, while others fail in transport/dispatch before
any transaction state exists and return bodyless envelopes. Conflating them would
either over-retry non-idempotent-looking states or under-retry safe transport blips.

## Decision

Split retries by **body presence**. The bodyless inner classifier
(`evaluate_dtx_http_outcome`) owns `408` and specific `449`/`500` sub-statuses with
their own budgets; the body-bearing outer loop obeys the coordinator's `isRetriable`.
A body-bearing `408`/`449`/`500` is intentionally left to the outer loop.

## Consequences

Safe, deterministic replay; each tier has an independent budget that matches the
.NET client. Requires the pipeline to route `DistributedTransactionBatch` specially.

## Alternatives

A single unified retry policy was rejected because it cannot distinguish "never
reached the coordinator" from "coordinator says retriable".

## References

- Distributed Transactions spec: ../spec.md
