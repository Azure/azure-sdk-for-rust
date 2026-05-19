<!-- cspell:ignore queryengines LALR WCHAR bitflags STARTSWITH ENDSWITH LTRIM RTRIM sqlparser -->
# Cosmos DB Query Engine — Rust Implementation

## Summary

A subset of the C++ query engine has been ported to Rust, enabling:

1. **Client-side query plan generation** — Parse SQL text, extract partition key filters, and produce structural query info (aggregates, ORDER BY, GROUP BY, DISTINCT, etc.) without a Gateway roundtrip.
2. **In-memory query evaluation** — Match JSON documents against SQL WHERE clauses and apply SELECT projections, for use in test emulators.

The implementation lives entirely inside the `azure_data_cosmos_driver` crate. In normal builds the query subsystem remains crate-private; test builds and the `__internal_testing` feature expose temporary validation entry points (`query` and `__test_only_generate_query_plan_for_pk_paths`) so parity tests can exercise the local planner without making it part of the supported surface.

The supported SDK query path still uses Gateway query plans today. The local planner and evaluator are scaffolding that is validated in isolation, but they are not yet wired into production query execution.

---

## Architecture

```
SQL Text
  → Lexer (hand-crafted tokenizer)
  → Parser (recursive descent with Pratt precedence)
  → QueryPlan { pk_filters, query_info }
      ├── pk_filters: PartitionKeyFilter (Equality / InList / Unconstrained / Contradictory / NotEvaluated)
      └── query_info: LocalQueryInfo (structural analysis from the AST)

Gateway response (when issued)
  → GatewayQueryPlan { partition_key_ranges, query_info: GatewayQueryInfo }
```

The `LocalQueryInfo` and `GatewayQueryInfo` types are intentionally **not**
unified (see commit marker `F21`). `LocalQueryInfo` carries fields the AST
can populate (`has_join`, `has_subquery`, `has_where`, `has_udf`,
`has_select_value`, …). `GatewayQueryInfo` carries fields only the Gateway
can populate (`rewritten_query`, `group_by_aliases`, `d_count_info`,
`has_non_streaming_order_by`, …). The fields they share are compared by
`gateway_plan::shared_fields_match`, which is the parity surface the
`tests/gateway_query_plan_comparison.rs` suite asserts against. Splitting
the types avoids silently fabricating `false` for local-only booleans on
Gateway responses (and vice versa).

The pipeline goes directly from SQL AST to partition key extraction and structural analysis. No IL layer, no VM — direct AST interpretation.

---

## Module Structure

All modules live under `azure_data_cosmos_driver::query`. The module is `pub(crate)` in normal builds and exposed only for tests / `__internal_testing` validation:

```
sdk/cosmos/azure_data_cosmos_driver/src/query/
├── mod.rs            # Module root, re-exports parse()
├── ast/mod.rs        # SQL AST types (SqlProgram, SqlQuery, SqlScalarExpression, etc.)
├── lexer/mod.rs      # Hand-crafted tokenizer (TokenKind, Lexer, keyword lookup)
├── parser/mod.rs     # Recursive descent parser, Pratt precedence for expressions
├── plan/
│   ├── mod.rs        # Query plan generation + LocalQueryInfo type
│   └── tests/
│       └── query_plan_comparison.rs  # Exhaustive structural comparison tests
├── eval/mod.rs       # In-memory evaluator (gated on `__internal_in_memory_emulator`)
├── gateway_plan.rs   # Gateway response envelope (GatewayQueryPlan / GatewayQueryInfo + shared_fields_match)
├── common.rs         # Shared utilities (root alias extraction)
└── value.rs          # CosmosValue: type-aware comparison semantics (gated on `__internal_in_memory_emulator`)
```

### Why Inside the Driver Crate?

- Query plan generation is an internal implementation detail — no external consumer needs the types.
- The driver already has all required dependencies (`serde`, `serde_json`, `azure_core`).
- Keeps the supported public API surface at zero in normal builds; only test/internal feature gates expose validation hooks.
- The split `LocalQueryInfo` / `GatewayQueryInfo` types live next to the
  pieces that produce them (plan generator vs. response deserialization)
  while `gateway_plan::shared_fields_match` keeps the parity contract in
  one place.

---

## Implemented Features

### SQL Parser

Full recursive descent parser for the Cosmos DB SQL dialect:
- SELECT (star, list, VALUE), DISTINCT, TOP
- FROM with aliases, JOINs, array iterators, subqueries
- WHERE with all scalar expression types
- GROUP BY, ORDER BY, OFFSET/LIMIT
- Operators: arithmetic, comparison, logical, bitwise, string concat, coalesce, ternary
- IN, BETWEEN, LIKE (with ESCAPE), IS NULL / IS NOT NULL
- EXISTS, ARRAY, scalar subqueries
- UDF calls (`udf.name(args)`)
- Parameters (`@name`)
- Max nesting depth: 128

### Query Plan Generation

- Partition key filter extraction from WHERE clauses
- Single PK equality, IN lists, hierarchical PK (2 and 3 components)
- AND intersection logic (contradictory, redundant, narrowing)
- OR union logic (equality + equality, equality + IN, IN + IN) with duplicate-value deduplication
- Nested PK paths (e.g., `/address/city`)
- FROM alias resolution
- Full structural analysis populated by the AST: `LocalQueryInfo` with
  `distinct`, `top`, `offset`, `limit`, `order_by`, `group_by`,
  `aggregates`, `has_join`, `has_subquery`, `has_where`, `has_udf`,
  `has_select_value`.

### LocalQueryInfo / GatewayQueryInfo split

- **`LocalQueryInfo`** is produced by the local plan generator from the
  AST. Only fields the AST can populate are present (`has_join`,
  `has_subquery`, `has_where`, `has_udf`, `has_select_value`, plus the
  structural fields above).
- **`GatewayQueryInfo`** is what the Gateway returns over the wire. It
  carries Gateway-only fields (`rewritten_query`, `group_by_aliases`,
  `d_count_info`, `has_non_streaming_order_by`, …) in addition to the
  shared fields.
- `gateway_plan::shared_fields_match(&LocalQueryInfo)` is the comparison
  surface: it explicitly ignores the disjoint Gateway-only / local-only
  fields and only compares the fields both sides can populate. The
  parity test suite (`tests/gateway_query_plan_comparison.rs`) asserts
  through this contract so future divergences are caught early.

### In-Memory Evaluator

Gated behind the `__internal_in_memory_emulator` feature flag. Used by the
in-memory Cosmos DB emulator and inline unit tests. The evaluator
intentionally trades full Cosmos parity for emulator usability — see
`docs/IN_MEMORY_EMULATOR_SPEC.md` for the documented trade-offs.

- `matches_query()`: WHERE clause evaluation against JSON documents
- `project()`: SELECT clause projection
- `query_documents()`: Full query execution (WHERE + SELECT + JOIN + GROUP BY + ORDER BY + TOP + OFFSET/LIMIT)
- 30+ built-in functions (CONTAINS, UPPER, ABS, ARRAY_CONTAINS, etc.)
- SQL LIKE with DP-based pattern matching
- Three-valued logic (undefined AND/OR semantics)
- Cosmos DB comparison semantics (type ordering, cross-type = undefined)
- JOIN expansion with multiple iterator bindings
- GROUP BY with aggregate evaluation (COUNT, SUM, AVG, MIN, MAX)

---

## Testing

- **Exhaustive structural plan comparison tests** covering every `QueryInfo` field, PK extraction pattern, hierarchical PK, AND/OR intersection, nested paths, aliases, and edge cases
- **Inline unit tests** in each module (lexer, parser, plan, eval, value), including typed `GatewayQueryPlan` deserialization coverage in `gateway_plan.rs`
- **Live Gateway validation tests** in `tests/gateway_query_plan_comparison.rs`, behind `__internal_testing`, comparing local plans against Gateway responses using `CosmosOperation::query_plan`

---

## Known Limitations / Parity Gaps

These are deliberate (and small) divergences from the Gateway, tracked here so
a future PR can close the gap without re-discovering it from scratch.

| Area                              | Gap                                                                                                                                                                                               | Notes                        |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| PK extraction                     | `c["pk"]` and `c.address["city"]` style indexer references are not extracted as PK references; the local plan falls back to cross-partition routing for them. The Gateway recognizes these forms. | F5 in the post-review notes. |
| `LENGTH` builtin                  | The local evaluator counts Unicode scalar values; the Gateway returns UTF-16 code-unit count (matching JS / .NET `string.Length`). Surrogate-pair characters diverge.                             | F35.                         |
| Bitwise ops on `f64`              | `&` `\|` `^` `<<` `>>` use `f64 as i64` saturating cast; the Gateway uses C++/JS int32 truncation. Documented inline in `eval::int_op`.                                                           | F23.                         |
| Parameterized `TOP @n`            | Locally accepted when the parameter is bound; the Gateway rejects parameterized `TOP` with HTTP 400 even when bound. The integration layer must avoid sending such queries to the Gateway.        | F14.                         |
| `LIKE … ESCAPE 'xy'` (multi-char) | Local evaluator returns `Undefined` (row does not match); the Gateway rejects the query. Plan-level shape is unaffected.                                                                          | F15.                         |
| `~` on fractional `Number`        | Local evaluator returns `Undefined`; the Gateway rejects non-integral bitwise input.                                                                                                              | F22.                         |

## What Is Explicitly Not Implemented

| Component                                    | Reason                                                                   |
| -------------------------------------------- | ------------------------------------------------------------------------ |
| IL compilation pipeline                      | Direct AST interpretation suffices                                       |
| VM runtime / bytecode execution              | Backend-only concern                                                     |
| Index plans / physical plans                 | Backend-only concern                                                     |
| Distributed query coordination               | Gateway's responsibility                                                 |
| KQL / JavaScript query support               | Not needed                                                               |
| Full ORDER BY / GROUP BY in plan routing     | Plan generation detects these features; execution is server-side         |
| Production query execution using local plans | Still pending; the supported SDK path continues to request Gateway plans |

## Alternatives considered

This implementation is a port of the Cosmos SQL native engine; an off-the-shelf parser like
[`sqlparser-rs`](https://crates.io/crates/sqlparser) was not adopted because (a) Cosmos SQL
has dialect-specific JSON-path syntax (`c.address.city`, array subscripts, `IN` over arrays)
and operators (`??`, ternary, `EXISTS`/`ARRAY` subqueries) that don't map cleanly onto a
generic SQL parser's AST, (b) the porting strategy validates correctness against the
Gateway via `tests/gateway_query_plan_comparison.rs`
for end-to-end parity, and (c) hand-written parsing keeps the AST under tight control for
the partition-key extraction and plan-generation passes that are the main reason the local
plan generator exists.
