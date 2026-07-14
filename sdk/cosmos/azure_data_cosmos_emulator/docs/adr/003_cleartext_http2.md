# ADR-003 — Serve cleartext HTTP/2 (h2c) and reuse the existing probe

**Status:** Accepted
**Date:** 2026-07-14

## Context

HTTP/2 is a hard requirement for Gateway V2. PR1 hosts plaintext HTTP (TLS is deferred), so the
data plane must run cleartext HTTP/2 (h2c). The question was whether a driver change is needed to
negotiate h2c.

## Decision

Serve h2c from the host using `axum::serve`, explicitly enabling axum's non-default `http2`
feature so hyper-util's `auto` builder accepts HTTP/2 prior-knowledge connections. Then rely on the
driver's **existing** behavior: the `Http2Only` reqwest client already sets
`http2_prior_knowledge()` (h2c against `http://`), initialization already probes HTTP/2 then falls
back to HTTP/1.1, and `http://` is already permitted for loopback emulator hosts. No mandatory
driver change.

## Consequences

The hosted emulator negotiates HTTP/2 with the unmodified driver. A change to
`has_explicit_http2_incompatibility` is held as a **contingency**, applied only if end-to-end
validation reveals the driver does not cleanly fall back from h2c to HTTP/1.1 against a
cleartext HTTP/1.1-only server.

## Alternatives

- Serving HTTP/1.1 in PR1 and deferring HTTP/2 to the TLS PR was rejected: it would leave the
  Gateway V2 path without validation for longer and understate the HTTP/2 requirement.
- Adding a new client toggle to force prior knowledge was rejected as redundant with existing
  behavior.

## References

- Plan & summary: ../plan.md
- Transport pipeline spec: ../../../azure_data_cosmos_driver/docs/TRANSPORT_PIPELINE_SPEC.md
