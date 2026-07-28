# ADR-009 — Enforce transport security and authentication at the host boundary

**Status:** Proposed
**Date:** 2026-07-14

## Context

Cosmos DB supports key-based and Microsoft Entra ID authentication, while many local emulator
tests need an authentication-free mode. The in-memory store and operation handlers should not
contain listener-specific TLS, certificate, or token-validation logic. At the same time,
credentials must never be accepted over plaintext transport.

## Decision

The host owns transport security and authentication before a request enters the emulator core.
It supports three authentication modes:

- `none`: no credential validation; allowed over loopback HTTP or HTTPS.
- `key`: validate the Cosmos authorization signature against a primary key or primary read-only
  key; HTTPS is required.
- `entra`: validate a bearer token; HTTPS is required.

Entra validation requires all of the following trust inputs:

| Input                             | Purpose                                                           |
| --------------------------------- | ----------------------------------------------------------------- |
| JWKS URI or local JWKS file       | Verify the token signature.                                       |
| Expected issuer                   | Restrict the accepted token authority.                            |
| Expected audience                 | Prevent tokens issued for another resource from being replayed.   |
| Allowed object or application IDs | Restrict authenticated principals after cryptographic validation. |

Selecting an authenticated mode without HTTPS, or omitting required trust inputs, is a startup
error. The emulator core receives only requests that have passed the host policy and remains
independent of certificate and identity libraries.

## Consequences

Local tests retain a low-friction no-auth mode. Authenticated scenarios use a security boundary
that is reusable across Gateway V1, Gateway 2.0, and the management API. Offline Entra tests can
use a local JWKS file without weakening signature, issuer, or audience validation.

## Alternatives

- Implementing authentication inside operation handlers was rejected because it would couple the
  in-process emulator to network-hosting concerns.
- Allowing key or bearer authentication over plaintext was rejected because credentials could be
  exposed in transit.
- Checking only an OID/application allow-list was rejected because claims are not trustworthy
  until the signature, issuer, and audience have been validated.
- Making authentication mandatory was rejected because many deterministic local tests do not test
  identity behavior.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
