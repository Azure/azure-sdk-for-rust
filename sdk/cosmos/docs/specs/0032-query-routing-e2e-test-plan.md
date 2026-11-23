<!-- cspell:ignore PKRID -->
# Query Routing E2E Test Plan

**Status:** Draft
**Date:** 2026-11-20
**Crate:** `azure_data_cosmos`

## Purpose

This plan validates the routing and backend-filtering contract in
`0031-query-routing.md`. It separates fast wire-correctness tests from manual
live-account efficiency tests because Gateway V2 enablement and the EPK index
require prepared service resources.

The general E2E architecture remains defined by
`0028-functional-e2e-testing.md`. This plan adds routing-specific fixtures,
observations, and acceptance criteria. Direct mode is out of scope for Rust.

## Test tiers

| Tier | Backend                                             | Data scale                      | Primary evidence                    | Intended execution         |
| ---- | --------------------------------------------------- | ------------------------------- | ----------------------------------- | -------------------------- |
| A    | Hosted in-memory emulator, Gateway V1 and V2        | Small                           | Exact headers/tokens and results    | Pull request and scheduled |
| B    | Prepared Azure Cosmos DB account, Gateway V1 and V2 | 200,000 documents per container | Results, RU, latency, request count | Manual qualification       |

Tier A proves what the SDK sends. Tier B proves that removing unnecessary EPK
range filters preserves correctness and restores the expected index-only query
behavior.

## Implementation prerequisites

The current Rust SDK rejects cross-partition aggregates. Complete-PK aggregate
queries are executable, but partial-HPK, FeedRange, and full-container `COUNT`
qualification is blocked until the query pipeline supports distributed
aggregates or a service-side aggregate execution path. The blocked cells must
be reported as prerequisites, not skipped and counted as passes.

Rust also does not yet emit the PPK HTTP header or RNTBD token. Tier A PPK
carrier tests and Q2–Q4 are blocked until public partial-HPK query scope is
projected to those wire carriers. This prerequisite is independent of
distributed aggregate support.

Malformed wire combinations and internal scope-plus-range combinations that
cannot be constructed through the public SDK belong in driver component or raw
protocol tests. PPK plus an EPK range is valid on HPK containers; complete PK
plus backend start/end EPK is rejected by the backend, not a successful
combined-scope case. Public E2E tests continue to use only public
`azure_data_cosmos` APIs.

## Environment matrix

Prepare one equivalent container for every matrix cell.

| Container shape | Partition key      | EPK index off | EPK index on | Minimum physical partitions |
| --------------- | ------------------ | ------------- | ------------ | --------------------------- |
| Hash V1         | `/tenantId`        | Required      | Required     | 3                           |
| Hash V2         | `/tenantId`        | Required      | Required     | 3                           |
| HPK / MultiHash | `/tenantId`, `/id` | Required      | Required     | 3                           |

Run every applicable cell through Gateway V1 and Gateway V2. This produces
$3 \times 2 \times 2 = 12$ primary live cells before query/scope expansion.

The paired containers must have identical account, throughput, partition
layout, ordinary indexing policy, and data. Only the EPK-index setting may
differ. Record these values with every result:

- account and region;
- container RID and partition-key definition;
- indexing policy and EPK-index state;
- provisioned throughput;
- current physical partition ranges;
- Gateway V2 capability state; and
- SDK commit and build profile.

Before each run, compare the exact physical EPK boundaries of every paired
container. Fail setup if the EPK-index pair does not have equivalent topology
or if the selected landmarks do not realize every required relation. The
`abc` HPK prefix must span more than one physical partition for prefix fan-out
tests; otherwise that fixture does not qualify.

Account and container creation are manual prerequisites. This plan does not add
root engineering-system provisioning.

## Indexing policy

Keep `/tenantId/?` and `/id/?` indexed in every container. Keep all other
included and excluded paths identical between the EPK-index pair. Do not use a
policy difference that independently prevents the aggregate from being served
from the ordinary path index.

Before calibration, wait for index transformation to complete and record the
reported transformation progress. A run against a transforming index is
invalid.

## Seed data

Populate every container with the same deterministic logical data.

| Cohort     |   Count | `tenantId`                      | `id`                       | Purpose                                          |
| ---------- | ------: | ------------------------------- | -------------------------- | ------------------------------------------------ |
| Target     | 100,000 | `abc`                           | Unique deterministic value | High-cardinality HPK suffix and aggregate target |
| Background | 100,000 | Deterministic non-`abc` tenants | Unique deterministic value | Cross-tenant and FeedRange controls              |

Use small fixed-shape documents, for example:

- `id`;
- `tenantId`;
- fixed-width numeric `sequence`;
- fixed-size padding field; and
- deterministic cohort marker.

Use a fixed generator seed. The background cohort should contain many tenants
with low, repeatable document counts per tenant. Do not use nondeterministic
random data in qualification runs.

The loader must be resumable and idempotent. After loading, verify:

1. total count is 200,000;
2. target count is 100,000;
3. IDs are unique;
4. cohort checksums match across all containers; and
5. no background document uses tenant `abc`.

Persist EPK landmarks for documents and prefixes that produce these test
ranges:

- exact physical partition;
- strict subrange;
- containing or superset range;
- partial overlap; and
- disjoint range.

Do not infer landmarks from physical range IDs alone; IDs can change after a
split or merge.

## Tier A: automated correctness and wire tests

### Observation points

| Transport                | Authoritative observation          | Required fields                                   |
| ------------------------ | ---------------------------------- | ------------------------------------------------- |
| Gateway V1               | Hosted emulator request capture    | Backend HTTP partition headers                    |
| Gateway V2 outer request | Hosted proxy capture               | `x-ms-thinclient-range-*`, operation, resource    |
| Gateway V2 frame         | Hosted RNTBD decoder capture       | EPK, PKRID, routing hashes, backend filter tokens |
| Pre-send SDK             | Test-only observer where practical | Logical target and serialized carrier set         |

The hosted emulator capture is authoritative because it observes what each
consumer receives. A test-only SDK observer is supplemental and must not mutate
or serialize the request again.

Each test must assert required **absence** as well as presence. In particular,
a test that strips backend `StartEpk`/`EndEpk` must still assert the expected
Gateway V2 routing hashes and outer range flags.

Assert physical PKRID presence on every point-operation attempt and every
feed-execution leaf, page, resume, and retry, including queries and change
feed. Gateway V1 must
carry `x-ms-documentdb-partitionkeyrangeid`; Gateway V2 must carry RNTBD
`PartitionKeyRangeId` token `0x002C`. Include complete-PK cases even when the
proxy EPK or backend PK would locate the data without PKRID. An otherwise
successful response does not excuse a missing PKRID. This preserves client
topology-cache correctness independently of backend presence validation.
Query-plan requests remain the no-data-route exception below.

Capture the proxy's backend-bound request as well as its incoming RNTBD frame.
For SDK-originated and Compute-Gateway-originated requests, assert that the
proxy forwards all supplied backend metadata without adding, removing, or
rewriting it, including filters, page size, continuation, and consistency
metadata. Routing hashes alone must not create backend start/end EPK filters;
explicit PPK and EPK-R filters must survive unchanged. Consumed proxy-only
routing metadata must not become backend filter metadata.

The RNTBD oracle must decode raw frame bytes with an independent, test-owned
token catalog rather than importing production token constants. Pin these IDs,
types, and encodings:

| Token                    | ID                 | Type   | Encoding                                            |
| ------------------------ | ------------------ | ------ | --------------------------------------------------- |
| PartitionKey             | `0x002B`           | String | JSON PK array                                       |
| PartialPartitionKey      | `0x0108`           | String | JSON HPK prefix array                               |
| PartitionKeyRangeId      | `0x002C`           | String | Physical range ID                                   |
| EffectivePartitionKey    | `0x005A`           | Bytes  | Binary PK-derived proxy routing EPK                 |
| StartEpk, EndEpk         | `0x0062`, `0x0063` | Bytes  | UTF-8 hexadecimal filter text                       |
| ReadFeedKeyType          | `0x0064`           | Byte   | Feed filtering uses `3` = EPK-R; `2` must be absent |
| StartEpkHash, EndEpkHash | `0x00D2`, `0x00D3` | Bytes  | Hex-decoded proxy routing bounds                    |

### Wire matrix

Use the abbreviations from `0031-query-routing.md`.

| Scenario                        | Backend semantic header | Backend EPK-R                  | PKRID             | Proxy EPK `0x005A` | Gateway V2 TC-R |
| ------------------------------- | ----------------------- | ------------------------------ | ----------------- | ------------------ | --------------- |
| Point with complete PK          | PK                      | Absent                         | Required          | Required           | Absent          |
| Query with complete PK          | PK                      | Absent                         | Required          | Required           | Absent          |
| Query with partial HPK          | PPK                     | Absent for planner-derived SQL | Required per leaf | Absent             | Required        |
| Query over whole physical range | None                    | Absent                         | Required          | Absent             | Required        |
| Query over strict FeedRange     | None                    | Present                        | Required          | Absent             | Required        |
| Query over one EPK              | None                    | Present                        | Required          | Absent             | Required        |
| WHERE-only PK equality          | SQL                     | Absent                         | Required          | Absent             | Required        |
| WHERE-only HPK prefix           | SQL                     | Absent                         | Required per leaf | Absent             | Required        |
| Change feed with complete PK    | PK                      | Absent                         | Required          | Required           | Absent          |
| Change feed over HPK prefix     | None                    | Present                        | Required per leaf | Absent             | Required        |
| Change feed over whole PKRID    | None                    | Absent                         | Required          | Absent             | Required        |
| QueryPlan request               | Input options only      | Absent                         | Absent            | Absent             | Absent          |

For QueryPlan, assert that Gateway V1 and Gateway V2 omit data-plane PKRID,
thin-client range flags, routing hashes, backend start/end, and key type. Then
assert that the returned ranges affect only the subsequent data-leaf routing.

Add a component test for simultaneous PK and PPK that asserts PK is emitted and
PPK is omitted, with PKRID retained. Do not introduce new client-side rejection
for incomplete range headers. Tier A raw-protocol tests assert that a missing start or end,
absent key type, mismatched key type, and backend key type `2` combined with
EPK-R filtering are serialized without a newly introduced client rejection.
Tier B pins the corresponding live service outcomes.

Separately, add a positive `[MIN, X)` test proving that a logical empty minimum
is preserved as a zero-byte routing hash even when the empty backend text token
is omitted.

Add operation-specific PPK tests: query emits PPK, while change feed converts
the same HPK prefix to EPK-R and returns equivalent scoped results.

### Explicit semantic-scope and EPK-R component matrix

WHERE-derived ranges alone are routing metadata, not a reason to add backend
EPK-R. Correctness-required independent work boundaries remain protected. Test
the separate explicit `S` plus `F` relation rules at driver planning and
serialization boundaries. PKRID is mandatory for every valid combination below:

| Semantic scope `S` | Explicit `F` relation | Backend carriers | Proxy EPK `0x005A` | Gateway V2 TC-R |
| ------------------ | --------------------- | ---------------- | ------------------ | --------------- |
| PK                 | Equal or containing   | PK only          | Required           | Absent          |
| PPK                | Equal or containing   | PPK only         | Absent             | Required        |
| PPK                | Strict subset         | PPK + EPK-R      | Absent             | Required        |
| PPK                | Partial overlap       | PPK + EPK-R      | Absent             | Required        |
| PPK                | Disjoint              | PPK + EPK-R      | Absent             | Required        |

For disjoint PPK-plus-range cases, assert that physical routing follows `S`
and the backend receives both constraints. Do not substitute PK for PPK in
these valid intersection cases.

Test raw complete-PK-plus-EPK-R combinations separately for equal, containing,
strict-subset, overlapping, and disjoint ranges. They must be rejected by the
backend through both gateways, including when PKRID is present; do not assert
a successful intersection or an empty aggregate result. Record the exact
service status and substatus. This is coverage of existing backend validation,
not a requirement for new client-side rejection or a feature flag.

Add a raw PPK request against a non-HPK container and assert backend rejection
through both gateways. Keep the corresponding positive HPK PPK and
PPK-plus-EPK-R cases so this negative test cannot disable valid prefix scopes.

### One-EPK normalization cases

One-EPK ranges still use EPK-R filtering, but their successor normalization
needs independent cases rather than one generic example.

| Case                                                               | Expected classification   |
| ------------------------------------------------------------------ | ------------------------- |
| Hash V1 variable-width immediate successor                         | One EPK, encoded as EPK-R |
| Hash V2 full-width immediate successor                             | One EPK, encoded as EPK-R |
| Hash V2 trailing-zero-trimmed lower bound, padded before increment | One EPK, encoded as EPK-R |
| HPK full key padded to `16 * path_count` bytes                     | One EPK, encoded as EPK-R |
| Carry from one or more trailing `FF` bytes                         | One EPK, encoded as EPK-R |
| Raw `[AA, AB)` with the wrong normalized width                     | Not sufficient evidence   |
| HPK prefix `[P, P + FF)`                                           | EPK-R, never one EPK      |

For every positive one-EPK case, assert both backend start/end bounds through
Gateway V1 and Gateway V2. Assert HTTP
`x-ms-read-key-type: EffectivePartitionKeyRange` on Gateway V1 and byte `3`
in Gateway V2 RNTBD token `0x0064`. Assert that HTTP value
`EffectivePartitionKey` and RNTBD byte `2` are absent. Gateway V2 may
independently emit proxy-routing hashes; those do not replace the backend
range-filter tokens. For SQL-only or FeedRange-derived leaves, also assert that
the PK-derived proxy EPK token `0x005A` is absent. A complete-PK request remains
the separate positive case for `0x005A`.

### Result assertions

For every automated scenario, assert:

- exact item identities or exact aggregate value;
- no item outside the requested scope;
- no duplicate or missing item across pages;
- continuation resume equivalence;
- Gateway V1 and Gateway V2 result equivalence; and
- stable behavior after rebuilding the request for a retry.

### Colocated and topology-change query cases

These cases protect the correctness exceptions to EPK-filter removal.

| Case                     | Setup                                                                  | Required wire behavior                                                                        | Result assertion                                       |
| ------------------------ | ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| Colocated `IN`           | At least two planned point ranges on one PKRID                         | One unfiltered execution for the query/scope/PKRID, or one non-overlapping EPK-R per leaf     | No duplicate or missing rows across pages and resume   |
| Colocated `OR`           | At least two disjoint planned ranges on one PKRID                      | Same as colocated `IN`                                                                        | Exact identity set across small pages and resume       |
| Query resume after split | Save continuation, split the active range, then resume                 | Each child retains the saved range intersection and continuation boundary                     | No reread, loss, or duplicate rows                     |
| Query resume after merge | Advance sibling ranges to different positions, merge them, then resume | Independently continued slices and fresh gaps keep distinct EPK-R filters on the merged PKRID | No overlap between leaves and no completed work reread |
| Topology recovery retry  | Trigger stale topology while several slices share one PKRID            | Rebuilt leaves preserve the original non-overlapping range ledger                             | Same result as an uninterrupted execution              |

Use page size `1` or another deliberately small size so every case crosses a
continuation boundary. The hosted emulator must expose the captured range and
continuation for each request leaf.

The in-memory emulator is not the RU oracle. Its request capture validates the
wire contract; live service runs validate the performance consequence.

Include topology-validation cases for PK, PPK, and EPK-R data requests. Assert
that the resolved PKRID accompanies each filter set, including on retry.
Use a stale PKRID to verify routing feedback, including `410` where required
by the topology transition. After refreshing topology, assert that the rebuilt
request uses the refreshed PKRID and preserves the semantic scope. Do not
mistake a rejected physical route for disjoint SQL or an empty logical scope.

## Tier B: live correctness and efficiency

### Live incomplete-header compatibility matrix

Issue raw qualification requests through Gateway V1 and Gateway V2. These are
compatibility probes, not supported public option combinations. Assert and
record the exact service status, substatus, response body classification, and
whether the request reached the service.

| Case                               | Required client behavior        | Live assertion                      |
| ---------------------------------- | ------------------------------- | ----------------------------------- |
| Start without end                  | No newly added client rejection | Pin exact V1 and V2 service outcome |
| End without start                  | No newly added client rejection | Pin exact V1 and V2 service outcome |
| Start/end without key type         | No newly added client rejection | Pin exact V1 and V2 service outcome |
| Start/end with mismatched key type | No newly added client rejection | Pin exact V1 and V2 service outcome |
| EPK-R with key type `2`            | No newly added client rejection | Pin exact V1 and V2 service outcome |

Treat any difference between Gateway V1 and Gateway V2 as an explicit
transport result, not a reason to normalize behavior client-side without a
separate design decision.

### Concrete query scenarios

Use the seed manifest to compute exact expected counts for range-based rows.
Do not treat this as an unconstrained cross-product.

| ID  | Query                                                 | Public request scope       | Range relation             | Expected result              | Expected backend carrier |
| --- | ----------------------------------------------------- | -------------------------- | -------------------------- | ---------------------------- | ------------------------ |
| Q1  | `SELECT COUNT(1) FROM c WHERE c.tenantId = "abc"`     | Full container             | N/A                        | `100000`                     | SQL only                 |
| Q2  | `SELECT COUNT(1) FROM c`                              | PPK `["abc"]`              | Exact prefix               | `100000`                     | PPK                      |
| Q3  | `SELECT COUNT(1) FROM c WHERE c.tenantId = "abc"`     | PPK `["abc"]`              | Matching                   | `100000`                     | PPK + SQL                |
| Q4  | `SELECT COUNT(1) FROM c WHERE c.tenantId = "missing"` | PPK `["abc"]`              | Disjoint SQL               | One row with value `0`       | PPK + SQL                |
| Q5  | `SELECT * FROM c WHERE c.tenantId = "abc"`            | Full container             | N/A                        | Exact target IDs             | SQL only                 |
| Q6a | `SELECT COUNT(1) FROM c`                              | Complete HPK `["abc", id]` | One logical partition      | `1`                          | PK                       |
| Q6b | `SELECT COUNT(1) FROM c`                              | Hash PK `["abc"]`          | One logical partition      | `100000`                     | PK                       |
| Q7  | `SELECT * FROM c`                                     | Whole physical PKRID       | Exact physical range       | Manifest PKRID count         | PKRID only               |
| Q8  | `SELECT * FROM c`                                     | Strict FeedRange           | Subset                     | Manifest range IDs           | EPK-R                    |
| Q9  | `SELECT * FROM c`                                     | Overlapping FeedRange      | Partial overlap            | Manifest range IDs           | EPK-R                    |
| Q10 | `SELECT * FROM c`                                     | Disjoint FeedRange         | Disjoint from `abc` cohort | Manifest range IDs; no `abc` | EPK-R                    |
| Q11 | `SELECT * FROM c`                                     | One-EPK FeedRange          | Immediate successor        | Exact EPK IDs                | EPK-R                    |

Q1–Q4 are blocked where they require cross-partition aggregate support. Q2–Q4
also require PPK carrier support. Q5 and Q7–Q11 provide immediate
routing/result coverage without client-side aggregation. Q6a is the HPK
complete-key case; Q6b is the Hash V1/V2 complete-key case.

For a FeedRange spanning several physical partitions, validate each generated
leaf. Interior leaves that cover a whole physical partition must omit backend
EPK-R filtering; strict edge leaves must retain it.

Public PK/PPK plus WHERE scenarios cover planner-derived routing and verify
that SQL ranges alone do not become backend EPK-R filters. Explicit EPK-R
relationships belong in the component matrix above because the public API
cannot express arbitrary PK/PPK-plus-FeedRange pairs. PPK-plus-range cases test
valid scope intersections; raw PK-plus-range cases test backend rejection.

### Change-feed set

Seed a fixed batch after recording a known start position. Read and resume:

| Scope                    | Expected behavior                                      |
| ------------------------ | ------------------------------------------------------ |
| Full container           | All seeded changes exactly once                        |
| Whole physical partition | Only that partition; no backend EPK-R                  |
| Strict EPK subrange      | Only the subrange; backend EPK-R required              |
| Complete PK              | Only that logical partition                            |
| Partial HPK prefix       | All `abc` changes; EPK-R required today                |
| One EPK                  | Tight EPK-R with key type `EffectivePartitionKeyRange` |

Run both LatestVersion and AllVersionsAndDeletes where the selected transport
supports the mode. Transport fallback is recorded, not silently treated as a
Gateway V2 pass.

## Performance method

Performance qualification uses fixed thresholds, calibrated once and then
frozen.

1. Use a dedicated account with no competing workload.
2. Confirm indexing and replication are complete.
3. Run each cell once to populate metadata, query-plan, connection, and index
   caches.
4. Run a fixed warm-up count, excluded from measurements.
5. Run a fixed measured count serially with identical page-size and consistency
   settings.
6. Record every request, not only the final aggregate page.
7. Reject runs containing throttling, transport retry, topology refresh, or
   account reconfiguration unless that behavior is the test subject.

Capture:

- total and per-request RU;
- end-to-end latency and server duration;
- request and page count;
- query and index metrics;
- result value;
- transport actually used;
- emitted backend filter class;
- physical ranges contacted; and
- continuation behavior.

### Calibration record

Freeze one threshold record per environment and scenario:

| Field                           | Required value                                   |
| ------------------------------- | ------------------------------------------------ |
| Calibration date                | UTC date                                         |
| Account/container configuration | Immutable identifier or checked-in manifest hash |
| Dataset checksum                | Seed and checksum                                |
| Samples                         | Warm-up and measured counts                      |
| Baseline statistics             | Median and p95 RU/latency                        |
| Threshold                       | Explicit maximum RU and latency                  |
| Margin                          | Documented fixed safety margin                   |

Later qualification runs compare against the frozen maxima. They must not
recompute a relative baseline and move the threshold automatically.

The intentionally EPK-R-filtered controls must demonstrate the expensive path
on the prepared dataset. If a control no longer does so, mark the calibration
invalid and investigate service/index behavior rather than weakening the
optimized threshold.

### Optimized/control performance pairs

| Pair | Optimized cell            | Control cell                                 | Equal result         | Required evidence                                                    |
| ---- | ------------------------- | -------------------------------------------- | -------------------- | -------------------------------------------------------------------- |
| P1   | Q1, SQL-only HPK prefix   | Same SQL forced through exact prefix EPK-R   | `100000`             | Optimized fixed RU/latency maxima; control remains measurably higher |
| P2   | Q2, PPK-only aggregate    | Same scope forced through prefix EPK-R       | `100000`             | PPK carrier versus EPK-R carrier                                     |
| P3   | Q3, PPK plus matching SQL | Same query and scope forced through EPK-R    | `100000`             | PPK + SQL versus PPK + SQL + EPK-R                                   |
| P4   | Q7, whole PKRID           | Same physical range forced through start/end | Manifest PKRID count | PKRID-only versus redundant EPK-R                                    |

If the public API cannot force a control carrier without changing semantics,
run that control through a test-only driver operation in the manual
qualification harness and label it non-customer-surface. Do not expose raw
headers as public SDK options.

## Acceptance criteria

A qualification passes only when:

1. every applicable correctness cell returns the exact expected result;
2. no scope leaks documents;
3. Gateway V1 and Gateway V2 agree on results and continuation behavior;
4. EPK-index on/off containers agree on results;
5. captured carriers match `0031-query-routing.md` for Tier A;
6. optimized aggregate cells remain below their frozen RU and latency maxima;
7. required control cells still expose the expected higher-cost path; and
8. no required scenario is silently skipped.

On failure, report the operation, request option, SQL predicate, normalized
range relation, physical topology, backend carriers, Gateway V2 routing
carriers, index policy, transport, result, RU, latency, request count, query
metrics, and continuation.

## Proposed repository integration

Future implementation should add, without making this document executable:

- routing scenario metadata under `sdk/cosmos/e2e_tests/scenarios`;
- setup-profile capability fields for prepared EPK-index resources;
- a source-native Rust test module under
  `sdk/cosmos/azure_data_cosmos/tests/e2e_test_cases`;
- hosted-emulator request capture for backend and Gateway V2 proxy views;
- a non-mutating SDK request/frame observer for tests;
- an idempotent seed utility; and
- checked-in calibration metadata containing no credentials or account keys.

Tests continue to use public `azure_data_cosmos` APIs for product operations.
Hosted-emulator management and capture use its external management surface.
