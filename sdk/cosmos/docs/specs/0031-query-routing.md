<!-- cspell:ignore partialpartitionkey PKRID -->
# Query Routing and Backend Filtering

**Status:** Draft
**Date:** 2026-11-17
**Crate:** `azure_data_cosmos_driver`

## Purpose

This specification defines which partition-routing and backend-filtering
headers the Rust Cosmos DB driver sends for point operations, queries, and
change feed requests. The primary goal is to avoid backend EPK range filtering
when a logical partition key, partial hierarchical partition key, physical
partition, or SQL predicate already provides the required scope.

Gateway V1 and Gateway V2 are in scope. Direct mode is not implemented by the
Rust SDK. Java and .NET behavior is precedent, not part of this contract.

Related designs are `0007-partition-key-range-cache.md`,
`0011-gateway-v2.md`, `0012-feed-operations-and-dataflow.md`, and
`0013-query-engine.md`.

## Legend

| Term  | Meaning                                                         |
| ----- | --------------------------------------------------------------- |
| PK    | Complete logical partition key                                  |
| PPK   | Non-empty HPK prefix                                            |
| PKRID | Physical partition-key-range ID                                 |
| EPK   | One effective partition key                                     |
| EPK-R | Half-open effective-partition-key range `[min, max)`            |
| SQL   | The query text remains responsible for document filtering       |
| TC-R  | Gateway V2 proxy-routing range; never a backend document filter |
| `—`   | Header must be absent                                           |

## Header ownership

The most important distinction is between **proxy routing** and **backend
filtering**. Removing a backend EPK filter must not remove Gateway V2 routing
metadata.

| Carrier                                     | Wire form                   | Consumer        | Purpose                        | Companion or constraint                                               |
| ------------------------------------------- | --------------------------- | --------------- | ------------------------------ | --------------------------------------------------------------------- |
| `x-ms-documentdb-partitionkey`              | HTTP; RNTBD `0x002B` String | Backend         | Complete PK filter             | Takes precedence over PPK                                             |
| `x-ms-documentdb-partialpartitionkey`       | HTTP; RNTBD `0x0108` String | Backend         | HPK-prefix query filter        | Query only; omitted when PK is present                                |
| `x-ms-documentdb-partitionkeyrangeid`       | HTTP; RNTBD `0x002C` String | Gateway/backend | Physical routing               | May accompany one semantic backend filter                             |
| `x-ms-effective-partition-key`              | HTTP; RNTBD `0x005A` Bytes  | Proxy           | PK-derived EPK routing         | Accompanies PK when no EPK-R is retained; not a backend filter        |
| `x-ms-start-epk`, `x-ms-end-epk`            | HTTP                        | Backend         | EPK-R filter                   | SDK-generated filters use both plus `EffectivePartitionKeyRange`      |
| RNTBD `StartEpk`, `EndEpk`                  | `0x0062`, `0x0063` Bytes    | Backend         | UTF-8 EPK-R filter text        | Derived only from public start/end EPK headers                        |
| `x-ms-read-key-type`                        | HTTP; RNTBD `0x0064` Byte   | Backend         | Selects read-feed key form     | Feed/query filtering uses `3` = `EffectivePartitionKeyRange`          |
| `x-ms-thinclient-range-min`, `-max`         | Outer Gateway V2 HTTP       | Proxy only      | Announces TC-R in frame        | Current Rust value is literal `true`; never backend filtering         |
| RNTBD `StartEpkHash`, `EndEpkHash`          | `0x00D2`, `0x00D3` Bytes    | Proxy only      | TC-R routing bounds            | Hex-decoded; may use physical-range fallback                          |
| `x-ms-thinclient-pkrange-min`, `-max`       | Internal Rust HTTP staging  | V2 serializer   | Supplies fallback TC-R         | Must not become backend `StartEpk`/`EndEpk` tokens                    |

An `EffectivePartitionKey` token routes a PK-scoped Gateway V2 request and does
not require `ReadFeedKeyType`. A feed/query/change-feed filter that covers one
EPK remains a range: start/end plus
`ReadFeedKeyType::EffectivePartitionKeyRange`. This matches current Rust,
Java, and .NET behavior. The protocol value `EffectivePartitionKey` (`2`) is
not used by this contract for feed filtering.

When an EPK-R backend filter must be retained, Gateway V2 uses TC-R routing and
suppresses the proxy `EffectivePartitionKey` token, even if a PK is also known.
This keeps point EPK routing mutually exclusive with range routing hashes.

## Operation capabilities

| Operation   | PK option | PPK option                | FeedRange      | WHERE scope | PKRID per leaf | One-EPK form | Fan-out         |
| ----------- | --------- | ------------------------- | -------------- | ----------- | -------------- | ------------ | --------------- |
| Point       | Required  | Unsupported               | Unsupported    | N/A         | Optional       | N/A          | No              |
| Query       | Optional  | HPK only                  | Optional       | Supported   | Yes            | EPK-R        | Yes             |
| Change feed | Optional  | Scope only; no PPK header | Optional       | N/A         | Yes            | EPK-R        | Yes             |
| Query plan  | Optional  | Optional input            | Optional input | Input SQL   | No data PKRID  | No           | No data fan-out |

A point operation always carries a complete PK. It must not carry PPK, EPK-R,
or `ReadFeedKeyType` backend filtering. Gateway V2 independently derives the
EPK needed by the proxy; complete-PK routing does not use TC-R.

## Range model and normalization

All EPK ranges sent as filters are half-open: `[minInclusive, maxExclusive)`.

| Partitioning model | One-EPK successor rule                                       |
| ------------------ | ------------------------------------------------------------ |
| Hash V1            | Increment the variable-width encoded EPK in big-endian order |
| Hash V2            | Right-pad to 16 bytes with zeroes, then increment            |
| HPK / MultiHash    | Right-pad to `16 * path_count` bytes, then increment         |

A closed query-plan equality range `[X, X]` normalizes to
`[X, successor(X))`. A half-open range is one EPK only when its upper bound is
the container-definition-aware successor of its lower bound. Therefore
`[AA, AB)` is one EPK only when `AA` and `AB` are already comparable at the
correct encoded width. Raw hexadecimal subtraction is not sufficient for
trimmed Hash V2 or HPK bounds.

A partial HPK with prefix EPK `P` covers `[P, P + FF)`. It is not a one-EPK
range.

## Selection algorithm

For every data request, the driver must apply these steps in order:

1. Derive the caller scope: PK, PPK, explicit FeedRange, physical PKRID, or full
   container.
2. Derive query routing ranges from the selected query plan. The routing layer
   must not independently parse SQL.
3. Intersect compatible ranges for physical routing. If request-option scope
   and SQL scope are disjoint, route the request-option scope and let SQL
   evaluate the contradiction. Do not eliminate all leaves: aggregates and
   similar queries can still produce a row over zero matching documents.
4. Resolve each request leaf to one physical partition and retain its PKRID.
5. Choose the most semantic backend filter: PK, then PPK, then EPK-R.
6. Preserve correctness-required EPK-R boundaries for independent fresh or
   resumed leaves, even when SQL also filters the documents.
7. Remove any remaining EPK-R filtering only when another backend constraint
   fully enforces it. Mere overlap is not sufficient.
8. If a required EPK-R filter contains one normalized EPK, retain its
   `[X, successor(X))` start/end range representation.
9. Project physical routing to Gateway V1 or Gateway V2 independently of the
   backend-filter decision.

Correctness-required boundaries include independently continued slices and
fresh gaps reconstructed during query resume or topology recovery. After a
merge, several such leaves can share one PKRID but carry different progress;
their EPK-R filters prevent rereading completed work and overlapping adjacent
leaves.

Fresh query plans can also produce several non-overlapping query ranges on one
PKRID, for example colocated `IN` or `OR` values. The planner must either issue
one execution for the query, semantic scope, and PKRID, or retain a distinct
EPK-R filter on every leaf. It must never issue multiple unfiltered executions
of the same query against one PKRID.

## Range-relation decision table

In this table, `S` is a backend-enforced PK, PPK, or physical-partition scope,
and `F` is an additional explicit EPK filter. A query-plan range used only for
routing is not `F`; SQL already enforces that predicate.

| Relation                            | Physical route       | Backend filter             | Start/end | Key type                     | Result behavior                                             |
| ----------------------------------- | -------------------- | -------------------------- | --------- | ---------------------------- | ----------------------------------------------------------- |
| `F == S`                            | `S` partitions       | PK, PPK, or PKRID          | `—`       | `—`                          | Same scope                                                  |
| `S` strictly inside `F`             | `S` partitions       | PK, PPK, or PKRID          | `—`       | `—`                          | `F` redundant                                               |
| `F` strictly inside `S`             | Overlap              | Semantic filter plus EPK-R | Required  | `EffectivePartitionKeyRange` | Narrower result                                             |
| Partial overlap                     | Overlap              | Semantic filter plus EPK-R | Required  | `EffectivePartitionKeyRange` | Intersection                                                |
| Disjoint                            | `S` partitions       | Semantic filter plus EPK-R | Required  | `EffectivePartitionKeyRange` | Zero matching documents; SQL aggregate may still emit a row |
| `F` is one EPK and is not redundant | Owning partition     | Semantic filter plus EPK-R | Required  | `EffectivePartitionKeyRange` | One-EPK range lookup                                        |

A physical PKRID is sufficient by itself only when the intended scope covers that
whole physical partition. A strict subrange still needs EPK filtering.

Evaluate equal/containing redundancy before the one-EPK row. If `S` fully
enforces a one-EPK `F`, strip `F`; otherwise retain its narrow EPK-R. For a
disjoint pair, route the `S` partitions and send both backend constraints so
the service can preserve zero-input aggregate semantics.

For a WHERE-only contradiction, existing query-planner rules remain valid:
ordinary projections may drain without I/O when emptiness is proven, while
aggregates, GROUP BY, and hybrid search must preserve their zero-input
semantics. The rule above is specifically for contradictions between an
explicit request scope and SQL.

## Query matrix

The table lists backend filters. PKRID and Gateway V2 TC-R routing are added per
physical request and are intentionally omitted from the cells for readability.

| Request option | No PK predicate | Matching PK equality | Matching HPK prefix | Narrower PK predicate | Disjoint PK predicate |
| -------------- | --------------- | -------------------- | ------------------- | --------------------- | --------------------- |
| None           | SQL only        | SQL only             | SQL only            | SQL only              | SQL only              |
| Full PK        | PK              | PK + SQL             | PK + SQL            | PK + SQL              | PK + SQL              |
| Partial HPK    | PPK             | PPK + SQL            | PPK + SQL           | PPK + SQL             | PPK + SQL             |
| EPK FeedRange  | EPK-R           | EPK-R + SQL          | EPK-R + SQL         | EPK-R + SQL           | EPK-R + SQL           |
| Physical PKRID | PKRID only      | PKRID + SQL          | PKRID + SQL         | PKRID + SQL           | PKRID + SQL           |
| Full container | SQL only        | SQL only             | SQL only            | SQL only              | SQL only              |

The query plan may reduce the physical partitions contacted in every row. It
does not require the driver to synthesize PK or PPK headers from SQL values.
When no explicit FeedRange must be enforced, the SQL predicate is preferred to
an EPK-R backend filter so the query engine can use the partition-key path
index. A one-EPK range remains EPK-R only when it is an explicit FeedRange or
is otherwise required as a backend scope. A range derived solely from a WHERE
equality is routing metadata: SQL filters the backend results, so no backend
start/end tokens are required. Gateway V2 uses TC-R—not the PK-derived
`EffectivePartitionKey` token—to route that SQL-only leaf.

If PK or PPK is combined internally with an explicit EPK FeedRange, apply the
range-relation table in addition to the query matrix.

## Change-feed matrix

Change feed has no SQL predicate. PPK is not currently a backend change-feed
filter, so an HPK prefix remains an EPK-R filter.

| Requested scope          | Backend filter           | PKRID route | Gateway V2 proxy route | Efficiency note                   |
| ------------------------ | ------------------------ | ----------- | ---------------------- | --------------------------------- |
| Full container           | None per full PKRID leaf | Required    | TC-R per leaf          | No EPK filtering                  |
| Whole physical partition | None                     | Required    | TC-R                   | No EPK filtering                  |
| Strict EPK subrange      | EPK-R                    | Required    | TC-R                   | Range lookup                      |
| Complete PK              | PK                       | Optional    | Derived EPK            | Logical filter                    |
| Partial HPK prefix       | EPK-R                    | Required    | TC-R                   | Required until PPK support exists |
| One normalized EPK       | EPK-R                    | Required    | TC-R                   | Narrow one-EPK range lookup       |

After a merge, several continuation ranges may share one physical partition.
Each strict saved subrange retains its own EPK filter; PKRID alone would
over-read another continuation range. This rule applies to query and change
feed resume and topology recovery.

## Transport projection

| Logical target           | Gateway V1 backend HTTP            | Gateway V2 outer HTTP  | Gateway V2 routing tokens           | Gateway V2 backend tokens              |
| ------------------------ | ---------------------------------- | ---------------------- | ----------------------------------- | -------------------------------------- |
| Complete PK              | PK                                 | Existing proxy headers | `EffectivePartitionKey`             | `PartitionKey`; no EPK-R               |
| Whole PKRID              | PKRID                              | Range flags for query  | `StartEpkHash`, `EndEpkHash`, PKRID | PKRID; no `StartEpk`/`EndEpk`          |
| EPK-R, including one EPK | Start/end + range key type + PKRID | Range flags            | `StartEpkHash`, `EndEpkHash`, PKRID | `StartEpk`, `EndEpk`, range key type   |

`x-ms-thinclient-range-min/max` continue to be emitted where the existing
Gateway V2 transport requires them. They are proxy flags, not EPK values and not
backend filters.

## Ambiguous or incomplete inputs

| Input state                                | Required SDK behavior                                    | Validation                                  |
| ------------------------------------------ | -------------------------------------------------------- | ------------------------------------------- |
| PK and PPK together                        | Emit PK and omit PPK                                     | PK is more selective                        |
| Staged/wire headers contain only one bound | Preserve current transmission behavior; add no rejection | Pin live service result and status          |
| Staged/wire start/end omit range key type  | Preserve current transmission behavior; add no rejection | Pin live service result and status          |
| PPK on query                               | Emit PPK                                                 | Assert query scope and wire header          |
| PPK on change feed                         | Convert the prefix to EPK-R                              | Assert equivalent results and range headers |
| Data PKRID or TC-R on query-plan request   | Omit                                                     | Query plan is not a data-plane leaf         |

A typed logical range always has both bounds, and SDK-generated filters use the
complete header set. The compatibility rows apply only when an existing staged
or raw wire request is already incomplete; the routing work must not introduce
a new client-side rejection on that path. An empty minimum EPK is a valid
bound, not a missing bound. Gateway V1 may carry it as an empty HTTP value.
Gateway V2 encodes it as a zero-byte `StartEpkHash` for routing and may omit the
empty backend `StartEpk` text token; PKRID disambiguates the physical route.
This wire omission must not be rejected as an incomplete logical range.

## Worked examples

Assume physical partition `7` owns `[20, 80)`.

1. **Full PK inside a containing range:** PK hashes to `40`; candidate EPK-R is
   `[20, 80)`. Send PK (and routing metadata), but omit start/end and key type.
2. **Full PK with a disjoint explicit range:** PK hashes to `40`; explicit range
   is `[90, A0)`. Send PK and the EPK-R. The service returns no documents.
3. **Partial HPK:** PPK `["abc"]` covers `[30, 30FF)`. A containing EPK-R is
   redundant; send PPK only. A strict `[3040, 3050)` subrange requires both PPK
   and EPK-R.
4. **Whole physical partition:** FeedRange `[20, 80)` sends PKRID `7` without
   backend start/end. Gateway V2 still sends TC-R routing hashes.
5. **Strict physical subrange:** FeedRange `[30, 50)` sends PKRID `7`, start/end,
   and range key type.
6. **One EPK:** After width-aware normalization, `[40, successor(40))` sends PKRID
   `7`, start/end, and `EffectivePartitionKeyRange`, like any other strict
   EPK-R.
7. **WHERE-only HPK prefix:** SQL `c.tenantId = "abc"` limits physical routing
   through its query-plan range, but backend filtering is SQL only. Gateway V2
   may carry TC-R without backend `StartEpk`/`EndEpk`.

## Implementation gaps

This document is normative even where the current Rust driver differs:

- Rust does not yet emit `x-ms-documentdb-partialpartitionkey` or RNTBD token
  `0x0108`.
- Query leaves currently emit narrow start/end EPK filters for many
  planner-derived ranges.
- Gateway V2 currently prefers public start/end headers when producing routing
  hashes. The implementation must preserve routing hashes while allowing the
  public backend filters to be removed.

The .NET `RequestInvokerHandler` partial-key and exact-range behavior and the
Java `FeedRangeEpkImpl` / thin-client range tests are the cross-SDK precedents
for these rules.
