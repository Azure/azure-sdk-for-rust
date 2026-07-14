# ADR-010 — PR sequencing

**Status:** Accepted
**Date:** 2026-07-14

## Context

The work spans a new crate, host feature surface, data-plane hosting for two gateway flavors, a
control-plane API, new store primitives, CI, and eventually auth/HTTPS. It needs to be split into
reviewable increments that each land something coherent.

## Decision

Sequence the work as three PRs:

- **PR1** — the new host crate; per-region h2c hosting of Gateway V1 **and** Gateway V2
  (config-gated); the management REST API over the control-plane actions that map to existing store
  methods; and CI running the existing suites against the hosted emulator in both gateway modes.
- **PR2** — the new store primitives (region offline/online, runtime write-region failover) with
  net-new tests and their management endpoints.
- **PR3** — optional HTTPS and authentication.

## Consequences

PR1 is independently meaningful because the CI leg validates it end-to-end; it does not depend on
the new primitives, which the existing suites do not exercise. PR2 and PR3 are self-contained,
testable increments. Gateway V2 is folded into PR1 so a single CI pass validates both protocol
paths.

## Alternatives

- Splitting hosting and CI into separate PRs was rejected: hosting without the CI validation proves
  nothing, so they ship together.
- Deferring Gateway V2 to its own PR was rejected: hosting both gateways from the start lets one CI
  pass cover both (ADR-006).

## References

- Plan & summary: ../plan.md
