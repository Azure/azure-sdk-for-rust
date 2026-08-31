# Implementation Plan — Binary Encoding for Query

**Goal:** enable Cosmos binary-JSON encoding for the **query** path (a deferred item in
[`BINARY_ENCODING_HLD.md`](azure_data_cosmos_driver/docs/BINARY_ENCODING_HLD.md) → "Deferred work").
Point item operations already shipped in #4671; queries currently negotiate text only.

**Branch:** `users/kundadebdatta/4305_implement_binary_encoding_for_query`

---

## Current state (verified against code + spec)

| Piece | Status | Evidence |
|---|---|---|
| **Query response decode** | ✅ already works | SDK query decodes a page as `ResponseBody::Bytes` → `into_single::<FeedBody<T>>` → shared `deserialize_response` choke point, which checks the `0x80` first byte and routes a binary `{"Documents":[...]}` envelope to `binary_json::from_slice::<FeedBody<T>>` (walks the nested array + decodes each item). |
| **Query request negotiation** | ❌ missing | `query_items` does **not** call `resolve_binary_encoding`; driver gate `supports_binary_encoding()` covers only `Create/Read/Replace/Upsert`, so no `x-ms-cosmos-supported-serialization-formats: CosmosBinary` header is sent → service always returns text. |
| **Query request *body*** | ⚠️ must stay text | Query body is `{"query":"...","parameters":[...]}` (`application/query+json`), not a document — transcoding it to binary would send a malformed query. |
| **"Feed splitter scans text" blocker** (HLD) | not a blocker for SDK query | Applies only to `ResponseBody::Items` → `into_items`; `from_items` has **no production caller**. SDK query uses `into_single`, not `into_items`. |
| **ORDER BY / cross-partition merge** | ⚠️ separate real blocker | `parse_envelope_page` in `dataflow/query_response.rs` uses `serde_json::from_slice` + `RawValue` (text-only). Single-partition + passthrough cross-partition queries are fine; ORDER BY / aggregate / merge queries are not. |

## Key design decision (answers the HLD's open question)

`apply_request_binary_encoding` currently does **two coupled things**: (1) transcode the text
body → binary, and (2) set the negotiation header. For query these must be **decoupled**:

- **DO** advertise binary responses (set the header) so the service returns the `Documents`
  envelope in binary.
- **DO NOT** transcode the query body (keep `application/query+json` text).

This matches .NET behavior and the shared-choke-point design.

---

## .NET reference (validated against `Azure/azure-cosmos-dotnet-v3`)

The .NET SDK confirms this exact model. Key findings:

- **Header set for `OperationType.Query` only** — `QueryRequestOptions.PopulateRequestOptions`:
  ```csharp
  if (request.OperationType == OperationType.Query)
  {
      request.Headers.CosmosMessageHeaders.SupportedSerializationFormats =
          this.SupportedSerializationFormats?.ToString() ?? DefaultSupportedSerializationFormats;
  }
  ```
  with the comment: *"backend only honors SupportedSerializationFormats for OperationType Query
  but has a bug where it returns a binary response for ReadFeed API when partition key is also
  specified."* → **ReadFeed / change feed must NOT advertise binary** (not just a scoping
  preference — it triggers a backend bug).
- **Default header value** = `"JsonText,CosmosBinary"`
  (`DocumentQueryExecutionContextBase.DefaultSupportedSerializationFormats`), i.e. the client
  advertises it accepts either and the backend picks binary. This matches Rust's
  `BINARY_NEGOTIATION_FORMATS`.
- **Query body stays text** — the `SqlQuerySpec` is serialized by the (text) `sqlQuerySpecSerializer`;
  nothing transcodes the query body. The header is purely **response** negotiation.
- **Response is format-agnostic** — .NET reads results through a `JsonNavigator` that auto-detects
  text vs. binary by the first byte (`JsonSerializationFormat.Binary = 128 = 0x80`, the same
  preamble). A binary `Documents` envelope is navigated transparently — the analogue of Rust's
  shared `deserialize_response` / `is_binary` choke point.
- **ORDER BY works in .NET for free** because its cross-partition merge operates on the
  format-agnostic `CosmosElement` model (`CosmosArray`/`CosmosBinary` visitors in
  `OrderByCrossPartitionQueryPipelineStage`). Rust's merge (`parse_envelope_page`) uses
  `serde_json::from_slice` + `RawValue` (text-only), which is precisely **why the ORDER BY
  divergence is a Rust-specific blocker** and is deferred here.

**Side-by-side:**

| Design point | .NET | This plan |
|---|---|---|
| Query **body** encoding | text only (never binary) | text only ✅ |
| Negotiation mechanism | `x-ms-cosmos-supported-serialization-formats` header | same header ✅ |
| Header value | `JsonText,CosmosBinary` | `JsonText,CosmosBinary` ✅ |
| Header scope | **Query only** (explicitly not ReadFeed/change-feed) | Query (+SqlQuery); **not** feed ✅ |
| Response decode | format-agnostic navigator (first-byte detect) | shared `deserialize_response` (`is_binary`) ✅ |
| ORDER BY merge | format-agnostic `CosmosElement` → works | text-only `RawValue` → **deferred** (Rust-specific) |

---

## Key file locations (verified)

- **Gate:** `azure_data_cosmos_driver/src/driver/cosmos_driver.rs`
  - `binary_encoding_applies(resource_type, operation_type)` (~L2654):
    `resource_type == Document && operation_type.supports_binary_encoding()`
  - `apply_request_binary_encoding(operation)` (~L2670): transcodes text body → binary via
    `binary_json::transcode_to_binary`, **and** sets header via
    `with_supported_serialization_formats(BINARY_NEGOTIATION_FORMATS)`. Already-binary/empty
    body passes through.
- **Op gate:** `supports_binary_encoding()` in `azure_data_cosmos_driver/src/models/mod.rs`
  (~L645): matches `Create|Read|Replace|Upsert` only.
- **Response choke point:** `azure_data_cosmos_driver/src/models/response_body.rs`
  - `deserialize_response` (~L245): `is_binary ? from_slice : serde_json::from_slice`
  - `into_single` (~L128), `into_items` (~L139, has the text-only NOTE), `from_items` (~L69, tests-only)
- **ORDER BY merge:** `azure_data_cosmos_driver/src/driver/dataflow/query_response.rs`
  `parse_envelope_page` (~L295) — text-only `RawFeedBody`/`RawValue`.
- **SDK query entry:** `azure_data_cosmos/src/clients/container_client.rs` `query_items` (~L918)
  — builds `CosmosOperation::query_items(...).with_body(serde_json::to_vec(&query))`, then
  `driver.plan_operation(...)`; passes `options.operation` **unmodified** (no binary resolve).
- **SDK page decode:** `azure_data_cosmos/src/feed/query_page.rs` (~L100)
  `let body: FeedBody<T> = response.into_model()?;` → `into_single`.
  `FeedBody<T>` in `azure_data_cosmos/src/feed/page.rs` (serde alias `"Documents"`).

---

## Implementation task order (the actual work)

Concrete, ordered checklist. Each item is small; the whole change is a **request-negotiation
decoupling + one SDK wire-up**, since response decode already works.

1. **[driver] `models/mod.rs`** — add `OperationType::supports_binary_response()` returning
   `true` for `Create | Read | Replace | Upsert | Query | SqlQuery` (and **not** `ReadFeed`).
   Keep the existing `supports_binary_encoding()` (request-body encode) unchanged.
2. **[driver] `cosmos_driver.rs`** — split the gate:
   - `binary_encodes_request_body(resource, op)` = `Document && op.supports_binary_encoding()`
     (unchanged behavior, renamed).
   - `binary_negotiates_response(resource, op)` = `Document && op.supports_binary_response()`.
3. **[driver] `cosmos_driver.rs` — refactor `apply_request_binary_encoding`** into two steps so
   the caller can invoke them independently:
   - body transcode → only when `binary_encodes_request_body` **and** `binary.enabled`.
   - `with_supported_serialization_formats(BINARY_NEGOTIATION_FORMATS)` → when
     `binary_negotiates_response` **and** `binary.enabled`.
   Update the `execute_operation_direct` call site (~L2609–2621) to use the two predicates.
4. **[sdk] `container_client.rs::query_items`** — resolve binary encoding (mirror
   `resolve_binary_encoding` used by point ops) and store the effective options into
   `options.operation` before constructing `QueryItemIterator`. No other SDK change (flow-through
   verified — Phase 2).
5. **[driver] unit tests** in `cosmos_driver.rs` — a `Query` op with `binary.enabled`:
   assert the request **body is unchanged** (not transcoded, still `application/query+json`) and
   the `x-ms-cosmos-supported-serialization-formats` header **is** set. Mirror the existing
   `apply_request_binary_encoding_*` tests.
6. **[sdk/emulator] in-memory-emulator test** — query returns a binary `Documents` envelope; items
   decode into `T`. Extend the emulator's query response path to honor the negotiation header (emit
   binary when advertised). Mirror `tests/in_memory_emulator_tests/binary_response_format.rs`.
7. **[docs] SPEC + HLD** — flip `query_items` to "response negotiate: done"; move the ORDER BY
   merge (`parse_envelope_page`) + `into_items` splitter into an explicit Rust-specific deferred
   item; note ReadFeed exclusion.

**Files touched:** `azure_data_cosmos_driver/src/models/mod.rs`,
`azure_data_cosmos_driver/src/driver/cosmos_driver.rs`,
`azure_data_cosmos/src/clients/container_client.rs`, an in-memory-emulator test +
its response path, `BINARY_ENCODING_SPEC.md`, `BINARY_ENCODING_HLD.md`. No changes to
`response_body.rs` (decode already works), `feed/query_page.rs`, or `feed/iterator.rs`.

---

## Phase 0 — Confirm scope (decision needed)

- **DR1:** target **single-partition + passthrough cross-partition** queries first (common case;
  response decode already works), or also **ORDER BY / aggregate / merge** queries (which
  additionally need `parse_envelope_page` made binary-aware)?
- *Recommendation:* land the common case first (small, high-value); defer the merge-path
  binary-awareness as a separate follow-up.

## Phase 1 — Split the driver gate (decouple body-encode from response-negotiate)

- [ ] In `cosmos_driver.rs`, split the single gate into two predicates:
  - `binary_encodes_request_body()` → point ops only (`Create/Read/Replace/Upsert`).
  - `binary_negotiates_response()` → point ops **+ Query (+ SqlQuery)**.
- [ ] Refactor `apply_request_binary_encoding`: guard the **body transcode** behind predicate #1;
  set the **negotiation header** under predicate #2. Query gets the header, keeps its text body.
- [ ] Add `supports_binary_response()` in `models/mod.rs` (includes `Query`/`SqlQuery`) alongside
  the existing `supports_binary_encoding()`. **Explicitly exclude `ReadFeed` / change feed** —
  .NET confirms the backend returns binary for ReadFeed-with-partition-key as a *bug*, so change
  feed must not advertise binary.

## Phase 2 — Wire it through the SDK query entry

- [x] **VERIFIED — flow-through works.** Every query page request goes:
  `query_items` → `QueryItemIterator` (stores `OperationOptions`) →
  `execute_plan(self.options.clone())` → `DriverRequestExecutor::execute_request` →
  `execute_operation_direct(operation, overrides, options)` (`cosmos_driver.rs` ~L221) — which is
  exactly where the binary gate runs. So resolving binary into `options.operation` at the SDK
  entry is **sufficient**; no per-page plumbing is needed. (This also confirms Phase 1 is
  mandatory: the page op is `OperationType::Query` with a query-spec body, so today's
  `apply_request_binary_encoding` would wrongly transcode that body.)
- [ ] In `query_items` (`container_client.rs` ~L918), resolve binary encoding (mirror the point-op
  `resolve_binary_encoding` pattern) and thread the effective `BinaryEncodingOptions` into
  `options.operation` before building the iterator, so `execute_plan` carries it onto each page.

## Phase 3 — Response decode (verify, no code expected)

- [ ] Confirm `into_single::<FeedBody<T>>` routes a binary envelope via the choke point.
  Add a targeted assertion/test rather than code.

## Phase 4 — Tests

- [ ] **In-memory emulator** binary query test: service returns the `Documents` envelope in
  binary, items decode into `T`. Mirror `tests/in_memory_emulator_tests/binary_response_format.rs`
  / `binary_round_trip.rs`; emulator binary bits live in
  `in_memory_emulator/{dispatch,response,operations}.rs`.
- [ ] **Driver unit test:** for a Query op with binary enabled, assert the request body is **NOT**
  transcoded (still `application/query+json` text) but the negotiation header **IS** set.

## Phase 5 — Docs

- [ ] Update `BINARY_ENCODING_SPEC.md` scope table (`query_items`: response-negotiate → done).
- [ ] Update `BINARY_ENCODING_HLD.md` deferred-work list: query enabled; the `Items` splitter +
  ORDER BY merge (`parse_envelope_page`) remain deferred for a future driver-side cross-partition
  binary merge.

## Deferred (explicitly out of this change)

- Binary-aware `into_items` feed splitter and the **ORDER BY / cross-partition merge**
  (`parse_envelope_page`) — needed only for the driver-side merge engine, not the common SDK
  query path. **Rust-specific:** .NET does not have this blocker because its merge runs on the
  format-agnostic `CosmosElement` model; Rust's `parse_envelope_page` is `serde_json`/`RawValue`
  text-only, so making it binary-aware is a separate follow-up.
- **`ReadFeed` / change feed** binary negotiation — excluded deliberately (the backend returns
  binary for ReadFeed-with-partition-key as a bug; .NET scopes the header to `Query` only).
- `patch` / transactional `batch` / `bulk` (already deferred by spec).

---

## Validation sweep (per `sdk/cosmos/AGENTS.md`)

```
cargo fmt -p azure_data_cosmos_driver -p azure_data_cosmos
cargo build -p azure_data_cosmos_driver -p azure_data_cosmos
cargo clippy -p azure_data_cosmos_driver -p azure_data_cosmos --all-features --all-targets
cargo doc -p azure_data_cosmos_driver -p azure_data_cosmos --no-deps --all-features
cargo test -p azure_data_cosmos --features __internal_in_memory_emulator --test in_memory_emulator binary
pwsh eng/common/scripts/check-spelling-in-changed-files.ps1 -TargetCommittish "upstream/main"
```

## Open decisions

1. **DR1** — common queries first (recommended) vs. include ORDER BY/merge now.
2. **Branch** — use existing `4305_implement_binary_encoding_for_query`, or branch fresh.
