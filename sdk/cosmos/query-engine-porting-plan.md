<!-- cspell:ignore queryengines LALR WCHAR bitflags STARTSWITH ENDSWITH LTRIM RTRIM -->
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
      └── query_info: QueryInfo (unified type for local + gateway plans)
```

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
│   ├── mod.rs        # Query plan generation + unified QueryInfo type
│   └── tests/
│       └── query_plan_comparison.rs  # Exhaustive structural comparison tests
├── eval/mod.rs       # In-memory evaluator (WHERE matching, SELECT projection, JOIN, GROUP BY, ORDER BY)
├── gateway_plan.rs   # Gateway response envelope (GatewayQueryPlan wrapping shared QueryInfo)
├── common.rs         # Shared utilities (root alias extraction)
└── value.rs          # CosmosValue: type-aware comparison semantics
```

### Why Inside the Driver Crate?

- Query plan generation is an internal implementation detail — no external consumer needs the types.
- The driver already has all required dependencies (`serde`, `serde_json`, `azure_core`).
- Keeps the supported public API surface at zero in normal builds; only test/internal feature gates expose validation hooks.
- The unified `QueryInfo` type (with `Serialize + Deserialize`) is shared between local plan generation and gateway response deserialization.

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
- Full structural analysis: `QueryInfo` with distinct, top, offset, limit, order_by, group_by, aggregates, has_join, has_subquery, has_where, has_udf, has_select_value

### Unified QueryInfo Type

`QueryInfo` derives both `Serialize` and `Deserialize` and is used for:
- **Local plans**: fields like `has_join`, `has_subquery`, `has_where`, `has_udf` populated by AST analysis
- **Gateway plans**: fields like `rewritten_query`, `group_by_aliases`, `has_non_streaming_order_by`, `d_count_info` populated by deserialization

Both directions default unknown fields to `None`/`false`/empty.

### In-Memory Evaluator

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

## What Is Explicitly Not Implemented

| Component                                | Reason                                                           |
| ---------------------------------------- | ---------------------------------------------------------------- |
| IL compilation pipeline                  | Direct AST interpretation suffices                               |
| VM runtime / bytecode execution          | Backend-only concern                                             |
| Index plans / physical plans             | Backend-only concern                                             |
| Distributed query coordination               | Gateway's responsibility                                                      |
| KQL / JavaScript query support               | Not needed                                                                    |
| Full ORDER BY / GROUP BY in plan routing     | Plan generation detects these features; execution is server-side              |
| Production query execution using local plans | Still pending; the supported SDK path continues to request Gateway plans      |
