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
surface behind a feature flag (`__internal_in_memory_emulator_host`) that the host crate enables
automatically through its dependency declaration.

## Consequences

The host crate stays thin (CLI, HTTP listeners, config, management API). The emulator keeps full
access to driver internals. The extra surface is opt-in and clearly non-SemVer (the `__internal_`
prefix), so stable builds and the public API are unaffected.

## Alternatives

- Extracting the emulator into its own library crate was rejected: it would force a large,
  unstable slice of driver internals to become public.
- Growing the existing test-only feature to cover hosting was rejected: it would entangle
  in-process test wiring with server-only concerns.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
