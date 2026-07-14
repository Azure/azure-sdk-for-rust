# ADR-008 — Defer HTTPS and authentication to a dedicated later PR

**Status:** Accepted
**Date:** 2026-07-14

## Context

Cosmos DB supports key-based auth and Entra ID auth, and production endpoints require HTTPS.
Supporting TLS (certificate provisioning and trust) and authentication introduces real friction
that is orthogonal to standing up the hosted emulator and its control plane.

## Decision

PR1 and PR2 host **plaintext HTTP with no authentication**. A dedicated later PR (PR3) adds,
behind an `auth` config block and CLI flags:

- Optional HTTPS via axum + rustls (`--https --cert --key`); h2c remains the default when off.
- Auth modes: `none` (default), `key` (validate the `Authorization` HMAC against a primary key and
  a primary read-only key, matching the service), and `entra` (validate a bearer JWT and check its
  object ID / app ID against an allow-list supplied in config or via `--allowed-oid`).

## Consequences

The initial hosting and control-plane work ships without certificate or token friction, and can be
validated over h2c immediately. Auth and HTTPS land as a self-contained, reviewable increment.

## Alternatives

- Building auth and HTTPS into PR1 was rejected: it enlarges and slows the first deliverable and
  couples unrelated concerns.
- Making auth mandatory was rejected: emulator use cases are predominantly local and offline, where
  `none` over h2c is the least-friction default.

## References

- Plan & summary: ../plan.md
