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
  a primary read-only key, matching the service), and `entra` (validate a bearer JWT; see trust
  inputs below).
- **HTTPS is required when `key` or `entra` auth is selected.** Enabling either auth mode without
  `--https` is a startup error; the process exits with a descriptive message. This prevents account
  keys or bearer tokens from being transmitted over plaintext h2c.

### Entra JWT trust inputs

An allow-list of object IDs / app IDs is _not_ sufficient to authenticate a JWT. PR3 must define
and validate the following trust inputs before the `entra` mode can be accepted:

| Input | Flag / config key | Purpose |
|---|---|---|
| JWKS source | `--jwks-uri` or `--jwks-file` | Provides the public keys used to verify the JWT signature. The offline goal recommends `--jwks-file`; `--jwks-uri` is used for online (AAD-connected) deployments. |
| Expected issuer | `--issuer` | The `iss` claim value that the emulator will accept; prevents tokens issued by a different authority from being accepted. |
| Expected audience | `--audience` | The `aud` claim value, typically the emulator's resource URI; prevents tokens issued for other apps from being replayed. |
| Allowed OIDs / app IDs | `--allowed-oid` (repeatable) | After the signature, issuer, and audience are validated, restrict access to the listed object or application IDs. |

All four inputs must be provided when `entra` mode is enabled; the process exits with a descriptive
error if any are missing.

## Consequences

The initial hosting and control-plane work ships without certificate or token friction, and can be
validated over h2c immediately. Auth and HTTPS land as a self-contained, reviewable increment, with
a security-sound design: authenticated modes are gated behind HTTPS, and JWT validation is complete
rather than rely on an allow-list alone.

## Alternatives

- Building auth and HTTPS into PR1 was rejected: it enlarges and slows the first deliverable and
  couples unrelated concerns.
- Making auth mandatory was rejected: emulator use cases are predominantly local and offline, where
  `none` over h2c is the least-friction default.

## References

- Plan & summary: ../plan.md
