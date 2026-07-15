# ADR-008 — Model outages and failover as dynamic account topology

**Status:** Proposed
**Date:** 2026-07-14

## Context

The hosted emulator needs to reproduce region outages and write-region changes so SDK routing,
topology refresh, and retry behavior can be tested without a live account. Replication
pause/resume models lag to a target region, but it does not make that region unreachable or remove
it from account discovery. The configured region list and initial write region are otherwise
static.

## Decision

Represent availability and write ownership as runtime account-topology state shared by account
discovery and request dispatch:

- An offline-region set determines which regions appear in readable and writable locations.
  Requests sent directly to an offline region fail with `503 Service Unavailable`.
- Single-write accounts maintain a current write-region selection. Changing it updates writable
  locations and the write-region guard used by data-plane operations.
- State changes are visible through subsequent account reads; clients observe them through their
  normal metadata refresh path rather than an emulator-specific client hook.

The management REST API mutates this state, while the in-process emulator can use the same store
operations directly.

## Consequences

SDK failover behavior is driven by the same account topology contract used with the service.
Replication lag and endpoint outage remain distinct failure models. Existing static behavior is
preserved until the dynamic state is changed.

## Alternatives

- Treating replication pause as an outage was rejected because requests can still reach the
  region and account discovery still advertises it.
- Rebuilding the emulator for every topology change was rejected because clients could not test
  runtime refresh and recovery behavior.
- Adding SDK-specific failover switches was rejected because they would bypass normal account
  discovery and routing.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
