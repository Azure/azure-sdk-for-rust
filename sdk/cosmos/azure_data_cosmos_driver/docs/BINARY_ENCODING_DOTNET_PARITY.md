# Binary Encoding — Rust vs .NET Parity (per operation)

Tracks how Cosmos **binary JSON** encoding behaves across every operation type in
the Rust SDK/driver (`azure_data_cosmos` + `azure_data_cosmos_driver`) compared to
the .NET SDK (`Azure/azure-cosmos-dotnet-v3`).

Cosmos binary JSON is the `0x80`-preamble wire format. "Binary encoding" spans
three independent concerns:

- **Req-encode** — the request body is serialized as Cosmos binary JSON.
- **Resp-negotiate** — the request advertises
  `x-ms-cosmos-supported-serialization-formats: CosmosBinary`, asking the service
  to return a binary body.
- **Resp-decode** — the client decodes a binary response body (auto-detected by
  the `0x80` first byte).

> Last verified: 2026-08-10, against `azure-cosmos-dotnet-v3` `main`.

## Enablement model

| | .NET | Rust |
|---|---|---|
| Opt-in gate | `ConfigurationManager.IsBinaryEncodingEnabled()` (env var) + `ItemRequestOptions.EnableBinaryResponseOnPointOperations` | `BinaryEncodingOptions` (client default + per-op override) |
| Suppressed with custom serializer | Yes — `GetTargetResponseSerializationFormat` returns `Text` | N/A (SDK owns serde) |
| Response decode | Format-agnostic `JsonNavigator` (first-byte detect) | Shared `deserialize_response` / `is_binary` choke point |
| Status | Preview / opt-in | Preview / opt-in |

## Per-operation matrix

Legend: ✅ supported · ❌ not · — not applicable.

| Operation | .NET Req-encode | .NET Resp-negotiate | Rust Req-encode | Rust Resp-negotiate | Rust Resp-decode | Parity |
|---|:--:|:--:|:--:|:--:|:--:|---|
| Create item | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ match |
| Read item | — (no body) | ✅ | ✅ (no-op) | ✅ | ✅ | ✅ match |
| Replace item | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ match |
| Upsert item | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ match |
| Delete item | — | ✅ (in point-binary set) | ❌ | ❌ | — | ⚠️ diff |
| Query — single-partition | text body | ✅ (Query only) | text body | ✅ | ✅ | ✅ match |
| Query — passthrough cross-partition | text body | ✅ | text body | ✅ | ✅ | ✅ match |
| Query — ORDER BY / aggregate / GROUP BY / DISTINCT / TOP / LIMIT | ✅ | ✅ | ❌ (engine absent) | ❌ | ❌ | ⚠️ diff |
| Change feed / ReadFeed | ❌ (backend bug) | ❌ | ❌ | ❌ | (capable, unused) | ✅ match |
| Patch | ❌ (real server op) | ❌ | ✅ via internal Read+Replace | ✅ (sub-ops) | ✅ | ⚠️ diff (mechanism) |
| Transactional batch | ❌ (HybridRow, not binary JSON) | ❌ | ❌ | ❌ | — | ✅ match |
| Bulk | ❌ | ❌ | ❌ | ❌ | — | ✅ match |
| Stored procedure (Execute) | ❌ | ❌ | ❌ | ❌ | — | ✅ match |
| Control-plane (db / container / offer) | ❌ | ❌ | ❌ | ❌ | — | ✅ match |

## Differences that matter

| # | Difference | Detail | Severity |
|---|---|---|---|
| 1 | ORDER BY / aggregate / GROUP BY / DISTINCT / TOP / LIMIT cross-partition | .NET runs them; its cross-partition merge is on the format-agnostic `CosmosElement` model, so binary works for free. Rust's `validate_query_info` (`dataflow/planner.rs`) **rejects them in any encoding** — the ordered/aggregate merge engine does not exist yet, and `SUPPORTED_QUERY_FEATURES = "None"`. | Real capability gap (not binary-specific) |
| 2 | Delete negotiation | .NET's `IsPointOperationSupportedForBinaryEncoding` includes `Delete`; Rust's `supports_binary_encoding` / `supports_binary_response` exclude it. Low impact (delete carries no request body and typically no response body), but Rust does not advertise binary for delete. | Minor |
| 3 | Patch mechanism | .NET Patch is a real server op and is **not** binary-negotiated. Rust Patch is a client-side Read-Modify-Write, so its internal Read/Replace **are** binary-encoded when enabled. Different architecture; both functionally correct. | Cosmetic / architectural |
| 4 | Negotiation header value | .NET query default = `"JsonText,CosmosBinary"`; .NET point ops = `"CosmosBinary"`. Rust = `"CosmosBinary"` for all paths (`BINARY_NEGOTIATION_FORMATS`). Rust always forces binary rather than advertising "either". | Minor wire diff |

## Similarities (matched by design)

- Point item ops (create/read/replace/upsert) encode requests and decode responses identically.
- Single-partition and passthrough cross-partition queries: text request body, negotiated binary response, per-page binary decode.
- Query request body always stays text (`application/query+json` is a query spec, not a document).
- Change feed / ReadFeed excluded from binary negotiation (the backend returns binary for ReadFeed-with-partition-key as a known bug).
- Batch / bulk / stored procedures / control-plane resources never use binary JSON.
- Response decode is a single format-agnostic choke point on both sides (first-byte `0x80` detection).

## Bottom line

For point operations, single-partition queries, and passthrough cross-partition
queries, Rust and .NET are **functionally equivalent** on binary encoding.

Actionable divergences:

- **#1** is the only user-facing gap, and it is a missing query engine, not a
  binary issue — deferred. When that engine is built, use a format-agnostic value
  model (like .NET's `CosmosElement`) so binary support is inherent, not retrofitted.
- **#2 (Delete)** is a genuine small binary-scope divergence: add `Delete` to the
  Rust predicates to match .NET's point-operation binary scope exactly.
- **#3, #4** are mechanism / wire nuances with no functional impact.

## Key source references

Rust:

- `src/models/mod.rs` — `OperationType::supports_binary_encoding` / `supports_binary_response`.
- `src/driver/cosmos_driver.rs` — `binary_encodes_request_body`, `binary_negotiates_response`, `apply_request_binary_encoding`, `apply_response_negotiation`, `BINARY_NEGOTIATION_FORMATS`.
- `src/models/response_body.rs` — `deserialize_response`, `into_single`, `into_items` (text-only splitter note).
- `src/driver/dataflow/planner.rs` — `validate_query_info` (rejects ORDER BY / aggregate / etc.).

.NET (`Azure/azure-cosmos-dotnet-v3`):

- `src/Handler/RequestInvokerHandler.cs` — `IsPointOperationSupportedForBinaryEncoding` (create/replace/delete/read/upsert), sets `SupportedSerializationFormats = CosmosBinary` for point ops.
- `src/RequestOptions/QueryRequestOptions.cs` — `PopulateRequestOptions` sets the header for `OperationType.Query` only (with the ReadFeed backend-bug comment).
- `src/Query/v2Query/DocumentQueryExecutionContextBase.cs` — `DefaultSupportedSerializationFormats = "JsonText,CosmosBinary"`.
- `src/Resource/Container/ContainerCore.Items.cs` — `GetTargetRequestSerializationFormat` / `GetTargetResponseSerializationFormat`.
