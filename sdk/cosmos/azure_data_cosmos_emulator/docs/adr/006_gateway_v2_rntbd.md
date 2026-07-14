# ADR-006 — Simulate Gateway V2 by promoting the inverse RNTBD codec, config-gated

**Status:** Accepted
**Date:** 2026-07-14

## Context

Gateway V2 (thin-client) uses the RNTBD wire format over HTTP/2 instead of JSON REST. To let the
hosted emulator exercise the driver's Gateway V2 path, the emulator must act as an RNTBD **server**:
answer the connectivity probe, decode inbound request frames, and encode outbound response frames.
The driver already owns the **client** halves (`RntbdRequestFrame::write`, `RntbdResponse::read`);
the inverse halves exist only as `#[cfg(test)]` helpers. Today the emulator advertises no
thin-client endpoints, so the driver always suppresses Gateway V2 against it.

## Decision

Fold Gateway V2 simulation into PR1, gated by config. Promote the test-only inverse codec
(`RntbdRequestFrame::read`, `RntbdResponse::write`) to production, **co-located** in
`src/driver/transport/rntbd/` behind the host feature, reusing the existing token and status
primitives. When a region declares a `thinClientPort`, the emulator binds a thin-client listener
that answers `POST /connectivity-probe` with `200`, decodes RNTBD + thin-client headers into a
logical operation, dispatches through the shared store, and encodes an RNTBD response; the account
topology advertises `thinClient{Readable,Writable}Locations`.

## Consequences

Gateway V1 always works; Gateway V2 is opt-in per region, so CI can run the existing suites in both
modes. Because the request pipeline selects Gateway V2 purely from thin-client advertisement, no
driver routing change is needed. The codec halves stay symmetric and co-located for maximal reuse.

## Alternatives

- Shipping Gateway V2 as a separate later PR was rejected: hosting both from the start lets one CI
  pass validate both protocol paths end-to-end.
- Placing the server-side codec in the emulator module was rejected in favor of co-location with
  the client codec (Option A), which maximizes reuse and keeps the wire format in one place.
- Always advertising thin-client endpoints was rejected: config-gating keeps Gateway V1 as the
  safe default.

## References

- Plan & summary: ../plan.md
- Gateway V2 spec: ../../../azure_data_cosmos_driver/docs/GATEWAY_V2_SPEC.md
