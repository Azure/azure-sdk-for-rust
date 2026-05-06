# [Cosmos] Use `x-ms-cosmos-hub-region-processing-only` header to route `404/1002` retries to the current write (hub) region

## Background

Under session consistency, there is a timing gap between **partition failover** and **partition failback** in which read requests can persistently fail with `404/1002` (`ReadSessionNotAvailable`). This affects **both PPAF-enabled single-master accounts and non-PPAF accounts** — anywhere a session token issued in one region is presented to another region that has not yet caught up to that token's LSN.

Consider a two-region (`R0`, `R1`) single-master session-consistency Cosmos account, with `R0` as the primary write region and two server partitions `P0`/`P1`:

- **T0** — Partition `P0` in `R0` experiences a quorum loss across all 4 replicas, triggering a backend-initiated failover for `P0`.
- **T1** — Backend fails `P0` over to `R1`.
- **T2** — Driver-side flows kick in and start routing `P0` traffic to `R1`:
  - For **PPAF accounts**, writes failover via PPAF; reads failover via PPCB.
  - For **non-PPAF accounts**, reads still failover via PPCB. Writes are not failed over client-side.
- **T3** — A write for `P0` is served from `R1`, returning `session-token#1`.
- **T4** — `R0` comes back online. The backend failover manager begins failback to `R0` (no SLA). At this point `R0` can serve reads but is not yet caught up to `session-token#1`.
- **T5** — `R0` finishes catching up; `P0` is failed back to `R0` and writes resume there.

Between **T4** and **T5** two scenarios cause persistent `404/1002`:

1. A **cold-start client** initializes and the first reads naturally land on the preferred read region (often `R0`). If the session token came from `R1`, those reads fail with `404/1002` until `R0` is caught up.
2. PPCB **fails back reads** to `R0` ahead of the backend completing the catch-up. The driver does not perform cross-region retries on `404/1002`, so reads keep failing from `R0`.

This is **not limited to PPAF**. Any single-master account with session consistency can hit the same pattern any time a session token outlives the lifetime of the writing region — whether the writing region rotated because of PPAF, manual failover, or a backend-initiated emergency failover for a non-PPAF account.

## Is this really an issue?

Yes. Two distinct failover/failback systems run in the driver:

- **Write failover/failback** — driven by PPAF (single-master, PPAF-enabled accounts only).
- **Read failover/failback** — driven by PPCB (any account where PPCB is enabled, including non-PPAF).

When PPAF is enabled, PPCB is enabled by default. PPCB-driven read failback can run independently of any write-side state, so the `R0`-not-yet-caught-up window can be hit by reads on **any** session-consistency account that uses PPCB or has a cold-started client. The driver currently has no retry path that rescues these reads cross-region.

## Acceptance Criteria

- When a session-consistency read receives `404/1002`, the retry policy can opt the next attempt into being processed in the **current write (hub) region** by adding the backend header `x-ms-cosmos-hub-region-processing-only: true`.
- This behavior applies to:
  - Single-master accounts with **PPAF enabled**.
  - Single-master accounts with **PPAF disabled** but **PPCB enabled** (read failback path).
  - Single-master accounts with neither feature enabled but where session tokens can outlive a region's write tenure (cold-start clients, manual failovers).
- The first `404/1002` retry continues to use the existing local-region retry semantics. Only **subsequent** retries (or the second retry, matching the .NET design) carry the hub-region header, to avoid an immediate cross-region round-trip in the common case where the local region simply needs another LSN tick to catch up.
- **The header persists across `403/3` rotations.** Once the hub-region search has begun, every subsequent retry on this operation keeps the header set until the request succeeds or the failover budget is exhausted. Otherwise the driver would ping-pong between `404/1002` (no header) and `403/3` (header) without making progress.
- If a request carrying the hub-region header lands on a non-write region, the backend returns `403/3` (`NotWriteRegion`); the driver treats this as a routing hint analogous to PPAF's `403/3 WriteForbidden` — the failed region is added to the in-flight skip set for this operation, `resolve_endpoint` picks the next candidate, and the next retry carries the header against that region.
- For PPAF accounts, if a secondary loses lease renewal, backend may return `410/1022` (`LeaseNotFound`); the driver routes the next retry to the current hub region using the same header.
- Once the partition is fully recovered, normal routing resumes and subsequent operations no longer carry the header.

## SDK Fix (proposed)

In [retry_evaluation.rs](../../src/driver/pipeline/retry_evaluation.rs):

- Extend the `404/1002` (`ReadSessionNotAvailable`) handler so the chosen `OperationAction::SessionRetry` can carry an additional per-attempt request-mutation hint:
  - First retry: unchanged (local region, no header).
  - Second and subsequent retries: set `x-ms-cosmos-hub-region-processing-only: true` on the outbound request.
- Add a small piece of `OperationRetryState` to track:
  - the count of `404/1002` retries on the current operation, and
  - a `hub_region_search_active: bool` flag that latches to `true` once the second `404/1002` retry adds the header. The flag is the single source of truth for "subsequent retries also carry the header" — including the `403/3` rotation path described below.
- Add a corresponding `403/3` handler arm: when the response was a hub-region-targeted retry (i.e. `hub_region_search_active == true`) and the backend returned `403/3` (`NotWriteRegion`), reissue the next attempt with the header still set, advancing past the rejected region by adding it to the in-flight skip set used by `resolve_endpoint` (mirroring PPAF write failover's `403/3` handling).
- Plumb the header through the request-construction layer (`pipeline/...`) so the retry policy can attach it without leaking into normal request paths.

This logic should run regardless of whether PPAF or PPCB is enabled — the trigger is the response code (`404/1002`), not the account configuration.

## End-to-End Flow

1. **Partition failover (backend-initiated)** — Backend detects quorum loss in `R0` for `P0`; `P0` fails over to `R1`. PPAF/PPCB begin directing traffic to `R1`. (For non-PPAF accounts, only PPCB participates.)
2. **Writes generate session token** — Writes served from `R1` produce `session-token#1`. Session reads also resolve in `R1`.
3. **Region recovery and failback start** — `R0` comes back online and the backend begins the failback. PPCB may interpret `R0` as healthy and route reads back to it.
4. **Early failback window** — Reads against `R0` using `session-token#1` fail with `404/1002` because `R0` has not yet replicated to that token's LSN. The driver currently does not retry cross-region.
5. **SDK retry using hub-region header** — On the second `404/1002` retry, the driver attaches `x-ms-cosmos-hub-region-processing-only: true`. The backend ensures the request is processed in the active hub (write) region — currently `R1` during failback.
6. **Backend validation** — Backend uses the `ReplicationSequencer` to confirm the target region is the write region. If not, it returns `403/3` (`NotWriteRegion`). For PPAF partitions, lease renewal may produce `410/1022` (`LeaseNotFound`). The driver treats both as routing hints, adds the rejected region to the operation's in-flight skip set, and reissues against the next candidate **with the header still set**.
7. **Recovery completion** — Once `R0` catches up and regains write ownership, normal routing resumes; subsequent reads/writes no longer carry the hub-region header.

### Worked Example (matches the .NET design)

| # | Step | Header on request | Counter / state | Backend response | Reasoning |
|---|---|---|---|---|---|
| 1 | Read lands on region A (preferred read region) | — | `404/1002 count = 0` | `404/1002` | Standard `ReadSessionNotAvailable`. Action: `SessionRetry`. |
| 2 | First retry on the **account hub** (from location cache) | **No** | `count = 1` | `404/1002` | First `404/1002` retry uses existing logic; header is intentionally **not** added so the local hub gets one chance to catch up before going cross-region. |
| 3 | Second retry on the **account hub** with the new header | **Yes** | `count = 2`, `hub_region_search_active = true` | `403/3` (`NotWriteRegion`) | Backend's `ReplicationSequencer` says the cached hub is no longer the write region. |
| 4 | Retry on **region C** (next candidate after skip-set rotation) | **Yes** (header persists) | `count = 2`, `hub_region_search_active = true`; skip-set contains the prior hub | `200 OK` | C is the actual hub; session-consistent read succeeds. |

The header staying set on step 4 is critical: dropping it would cause C to potentially return `404/1002` again instead of `403/3`, restarting the chain and burning retry budget. Keeping it set turns the rotation into a deterministic search for the true hub.

## Backend Dependencies

The fix relies on backend support for `x-ms-cosmos-hub-region-processing-only`:

- **Phase 1 — `ReplicationSequencer` check.** Backend validates `WriteStatus_Global` and returns `403/3` when the target is not the current write region. Applies to both PPAF and non-PPAF accounts.
- **Phase 2 — PPAF partitions.** With ungraceful failover, topology may be stale; backend uses `EnableWriteRegionSecondaryLeaseEnforcement` to force secondaries to return `410/1022` if lease renewal fails. `EnableLessThanStrongLeases` validates leases only for requests carrying the header at consistency `< Strong`. Stable non-PPAF regions continue returning `403/3`.
- **Phase 3 — Non-PPAF partitions during offlining.** TODO in the .NET tracking work; mirror it here once the backend semantics are finalized.

## Spec Updates

[PARTITION_LEVEL_FAILOVER_SPEC.md](../PARTITION_LEVEL_FAILOVER_SPEC.md) — add a new subsection under §10 (Status Code Handling Matrix) describing the `404/1002` second-retry behavior, the `403/3` rejection follow-up (with header persistence and skip-set rotation), the `410/1022` PPAF interaction, and that the rule fires for any single-master session-consistency account, not just PPAF-enabled ones.

## Tests

- Unit: `evaluate_transport_result` cases for
  - `404/1002` first vs. second retry (header off, then on),
  - `403/3` after a hub-region-targeted retry (header stays on, region added to skip set),
  - `410/1022` for PPAF (header stays on),
  - the worked example above as a multi-stage scenario test.
- Integration: extend the existing PPAF/PPCB test suite to cover non-PPAF accounts where only PPCB is enabled, plus a cold-start client scenario.

## Cross-References

- .NET tracking issue: [Azure/azure-cosmos-dotnet-v3#5440](https://github.com/Azure/azure-cosmos-dotnet-v3/issues/5440)
- .NET PR: [Azure/azure-cosmos-dotnet-v3#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447)

## Notes

- The .NET implementation scoped this change to single-master accounts only. Multi-master accounts can also exhibit `404/1002` under session reads, but the routing model is different (no single hub region) and the header is not meaningful there. Keep the trigger gated on single-master.
- The driver's `404/1002` path is currently `OperationAction::SessionRetry`. Take care that adding the header does not change the action class or the session-token reuse semantics — it is purely a per-attempt request-header mutation plus a small retry-count counter and a latched `hub_region_search_active` flag.
