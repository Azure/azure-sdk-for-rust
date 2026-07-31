# ADR-002 — Host in a separate binary crate; keep the emulator in the driver

**Status:** Accepted
**Date:** 2026-07-14

## Context

The in-memory emulator lives in `azure_data_cosmos_driver` and depends heavily on
driver-internal APIs (store, dispatch, EPK routing, RNTBD codec). It must become usable by other
SDKs behind a network port, but the implementation cannot be moved out of the driver without
exposing large swaths of internal surface.

## Decision

Add a new `publish = false` binary crate `azure_data_cosmos_emulator` that hosts the emulator.
The emulator implementation stays in the driver; the driver exposes a small, additional **public**
surface behind the existing `__internal_in_memory_emulator` feature. The host crate enables that
feature automatically through its dependency declaration.

## Consequences

The host crate stays thin (CLI, HTTP listeners, config, management API). The emulator keeps full
access to driver internals. The externally reachable emulator surface remains opt-in and clearly
non-SemVer (the `__internal_` prefix), so stable builds are unaffected.

## Alternatives

- Extracting the emulator into its own library crate was rejected because it would force a large,
  unstable slice of driver internals to become public.
- Adding a second host-specific feature was rejected because the existing emulator feature already
  owns this internal surface and the split would add no useful granularity.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
