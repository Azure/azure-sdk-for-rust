# ADR-009 — Validate via the existing suites over a new `emulator_inmemory` cfg

**Status:** Accepted
**Date:** 2026-07-14

## Context

The hosted emulator is only meaningful if it faithfully serves the real wire protocol. The most
convincing validation is the existing `emulator` / `emulator_vnext` integration suites passing
against it. The suites are gated at runtime by a `test_category` cfg set via `RUSTFLAGS`.

## Decision

Add a new `test_category = "emulator_inmemory"` cfg (registered in the `build.rs` of both
`azure_data_cosmos` and `azure_data_cosmos_driver`). Extend
`sdk/cosmos/eng/scripts/Invoke-CosmosTestSetup.ps1` to build and start the host binary with a
provisioning config, wait for `GET /health`, and point `AZURE_COSMOS_CONNECTION_STRING` at the
hosted endpoint. Add a `ContinueOnError` matrix leg to `sdk/cosmos/ci.yml` (modeled on
`Cosmos_vnext_emulator`) that runs the existing suites in **both** Gateway V1 and Gateway V2
(thin-client) modes. This CI pass replaces a standalone HTTP/2 validation spike.

## Consequences

The same battle-tested suites validate h2c and RNTBD end-to-end, in both gateway modes, without
new bespoke tests. Starting non-blocking (`ContinueOnError`) surfaces failures as "succeeded with
issues" while the host stabilizes.

## Alternatives

- A separate standalone h2c validation spike was rejected: running the real suites over the hosted
  emulator is stronger and is exactly what the combined PR delivers.
- A bespoke `emulator_inmemory`-only test set was rejected: reusing the existing suites maximizes
  coverage and avoids drift.
- Reusing the `emulator_vnext` cfg was rejected: its behavioral-divergence skips do not match the
  in-memory emulator, so a distinct cfg is cleaner.

## References

- Plan & summary: ../plan.md
