# ADR-009 — Keep outer retry in the driver

**Status:** Accepted
**Date:** 2026-07-06

## Context

The driver uses a pure-evaluation, effect-driven pipeline rather than `azure_core`
retry policies. A DTX request is one logical unit whose replay must reuse the same
idempotency token and re-serialized body.

## Decision

Own the outer, body-bearing retry loop inside `execute_distributed_transaction`,
consistent with the driver architecture, and keep only the bodyless classification
inside the transport pipeline.

## Consequences

Retry logic is centralized where the request is assembled; the token and body are
trivially stable across attempts. Two retry sites exist (documented in the main
spec), which is intentional.

## Alternatives

A generic pipeline retry policy was rejected because it does not fit the driver's
effect model and complicates token reuse.

## References

- Distributed Transactions spec: ../spec.md
