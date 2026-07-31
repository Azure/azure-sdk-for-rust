# ADR-007 — Keep hosted Gateway 2.0 framing in the driver

**Status:** Accepted
**Date:** 2026-07-14

## Context

Gateway 2.0 uses the RNTBD wire format over HTTP/2 instead of JSON REST. To let the hosted
emulator exercise the driver's Gateway 2.0 path, the emulator must act as an RNTBD **server**:
answer the connectivity probe, decode inbound request frames, and encode outbound response frames.
The driver already owns the **client** halves (`RntbdRequestFrame::write`, `RntbdResponse::read`).
The inverse codec must use the same token IDs, operation mappings, and framing rules as the client
codec. Duplicating those rules in the host crate would create two protocol implementations that
can drift.

## Decision

Keep request decoding and response encoding alongside the existing RNTBD client codec in
`azure_data_cosmos_driver`. Expose one feature-gated, high-level emulator entry point that accepts
an RNTBD-framed request, dispatches it through the existing in-memory operation handlers, and
returns an RNTBD-framed response. Token and frame internals remain private to the driver.

Gateway 2.0 is enabled per region by configuring a Gateway 2.0 endpoint. Only then does account
discovery advertise `thinClientReadableLocations` and `thinClientWritableLocations`. The
Gateway 2.0 listener answers `POST /connectivity-probe` and requires HTTP/2. Unsupported RNTBD
semantics are rejected explicitly instead of being silently discarded.

## Consequences

The driver remains the single owner of RNTBD wire compatibility. The host depends on a small
unstable adapter rather than public token types. Accounts without Gateway 2.0 endpoints retain
standard gateway behavior, while configured regions exercise the same discovery and probe flow as
the service.

## Alternatives

- Implementing a second RNTBD codec in the host crate was rejected because protocol mappings would
  drift from the production driver.
- Exposing frame and token types as a general public API was rejected because they are unstable
  transport internals.
- Always advertising Gateway 2.0 endpoints was rejected: config-gating keeps Gateway V1 as the
  safe default.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
- Gateway 2.0 spec: `sdk/cosmos/azure_data_cosmos_driver/docs/GATEWAY_V2_SPEC.md`
