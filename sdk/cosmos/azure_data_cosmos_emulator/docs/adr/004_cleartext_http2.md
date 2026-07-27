# ADR-004 — Support cleartext HTTP/2 for local emulator endpoints

**Status:** Accepted
**Date:** 2026-07-14

## Context

Gateway 2.0 requires HTTP/2. Local emulator scenarios also need to avoid certificate provisioning
and trust-store configuration when transport security is not under test. Cleartext HTTP/2 (h2c)
provides HTTP/2 framing on loopback without TLS, while production Cosmos endpoints remain HTTPS
only.

## Decision

The host accepts h2c prior-knowledge connections on configured loopback endpoints. The Gateway 2.0
listener rejects HTTP/1.x requests. The standard gateway listener may accept either HTTP/1.1 or
HTTP/2 so the driver's normal negotiation and fallback behavior remains observable.

The driver permits `http://` Gateway 2.0 URLs only when the endpoint is recognized as an emulator
host. All other Gateway 2.0 endpoints must use `https://`. The existing `Http2Only` transport uses
`http2_prior_knowledge()` and the existing account probe determines whether standard gateway
traffic uses HTTP/2 or falls back to HTTP/1.1.

## Consequences

Gateway 2.0 can be exercised locally without certificates. Plaintext traffic is limited to
explicit emulator hosts, preserving the production transport boundary. Authentication modes that
carry credentials require TLS, as described by ADR-009.

## Alternatives

- Supporting only HTTP/1.1 was rejected because it cannot model Gateway 2.0.
- Requiring TLS for every local run was rejected because certificate setup would obscure tests
  unrelated to transport security.
- Adding a client option to force prior knowledge was rejected because the existing HTTP/2-only
  transport already provides that behavior.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
- Transport pipeline spec:
  `sdk/cosmos/azure_data_cosmos_driver/docs/TRANSPORT_PIPELINE_SPEC.md`
