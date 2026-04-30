# Cosmos DB Query Engine — Rust Porting Plan

## Executive Summary

Port a subset of the C++ query engine (`Product/Backend/native/queryengines/`) to Rust, enabling:

1. **Client-side query plan generation** — Parse SQL text, extract partition key filters, and determine target FeedRanges without a Gateway roundtrip.
2. **In-memory query evaluation** — Match JSON documents against SQL WHERE clauses and apply SELECT projections, for use in test emulators.

This is **not** a full port of the query engine. We skip the IL compilation pipeline, index plans, distributed query coordination, VM runtime, and the full type system. Instead, we port just enough to achieve the two goals above.

---

## Architecture Overview

### C++ Pipeline (what exists today)

```
SQL Text
  → SqlScanner (lexer)
  → SqlParser (LALR(1) parser → SqlObject AST)
  → SqlBinder (AST → ILProgram, semantic analysis)
  → QueryILCompiler (IL → QueryLogicalPlan)
  → LogicalPlanTranslator (logical plan → filter expressions)
  → FilterExpressionPredicateExtractor (extract PK predicates)
  → FilterExpressionPartitionKeyRangeTranslator (PK predicates → partition key ranges)
  → DistributedQueryCompiler (logical plan → distribution plan for SDK)
```

### Rust Pipeline (what we will build)

```
SQL Text
  → Lexer (hand-crafted, inspired by SqlScanner)
  → Parser (recursive descent, produces SqlAst)
  → QueryPlanAnalyzer (AST → QueryPlanInfo with PK filter extraction)
  → PartitionKeyRangeResolver (PK filters → FeedRange set)
  → [Phase 2] DocumentMatcher (AST WHERE + JSON document → bool)
  → [Phase 2] Projector (AST SELECT + JSON document → projected JSON)
```

Key simplification: We go directly from the SQL AST to partition key extraction. We skip the IL layer entirely — it exists in C++ to support the VM-based execution engine which we don't need.

---

## Module Structure

New crate: **`azure_data_cosmos_query`** under `sdk/cosmos/`

```
sdk/cosmos/azure_data_cosmos_query/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # Public API surface
│   │
│   ├── ast/                      # SQL Abstract Syntax Tree types
│   │   ├── mod.rs                # SqlQuery, SqlSelectClause, SqlFromClause, etc.
│   │   ├── scalar_expression.rs  # All scalar expression variants
│   │   ├── collection.rs         # Collection expressions (FROM clause)
│   │   ├── visitor.rs            # Visitor trait for AST traversal
│   │   └── display.rs            # Display impl for AST → SQL text roundtrip
│   │
│   ├── lexer/                    # Tokenizer
│   │   ├── mod.rs                # Token enum, Lexer struct
│   │   └── keywords.rs           # Keyword lookup table
│   │
│   ├── parser/                   # Recursive descent parser
│   │   ├── mod.rs                # Parser struct, parse() entry point
│   │   └── error.rs              # Parse errors with location info
│   │
│   ├── plan/                     # Query plan analysis
│   │   ├── mod.rs                # QueryPlanInfo struct
│   │   ├── pk_extractor.rs       # Partition key filter extraction from WHERE
│   │   └── query_features.rs     # Feature detection (has aggregates, ORDER BY, etc.)
│   │
│   ├── eval/                     # In-memory evaluation (Phase 2)
│   │   ├── mod.rs                # Public eval API
│   │   ├── matcher.rs            # WHERE clause → JSON document matching
│   │   ├── projector.rs          # SELECT clause → JSON projection
│   │   ├── scalar.rs             # Scalar expression evaluator
│   │   └── functions.rs          # Built-in function implementations
│   │
│   └── value.rs                  # CosmosValue: type-aware JSON value wrapper
│                                   for Cosmos-specific comparison semantics
```

### Why a Separate Crate?

- The query engine is a pure computation library (no I/O, no async, no Azure dependencies).
- It can be used independently by `azure_data_cosmos` (for PK routing) and by test/emulator code.
- Keeps `azure_data_cosmos` focused on the SDK surface.
- Avoids pulling parser/eval code into the driver layer.

### Dependency Direction

```
azure_data_cosmos_query  (no Azure deps, just serde_json)
       ↑
azure_data_cosmos        (uses query crate for PK extraction + routing)
       ↑
azure_data_cosmos_driver (unchanged, schema-agnostic)
```

---

## Phase 1: SQL Parsing + Partition Key Extraction

### 1.1 AST Types (port from `queryLanguages/sql/SqlObject*.h`)

Map the ~60 `SqlObjectKind` variants to Rust enums. Key types:

```rust
/// Top-level parsed SQL program
pub struct SqlProgram {
    pub query: SqlQuery,
}

/// SELECT ... FROM ... WHERE ... GROUP BY ... ORDER BY ... OFFSET ... LIMIT
pub struct SqlQuery {
    pub select: SqlSelectClause,
    pub from: Option<SqlFromClause>,
    pub r#where: Option<SqlWhereClause>,
    pub group_by: Option<SqlGroupByClause>,
    pub order_by: Option<SqlOrderByClause>,
    pub offset_limit: Option<SqlOffsetLimitClause>,
}

pub struct SqlSelectClause {
    pub distinct: bool,
    pub top: Option<SqlTopSpec>,
    pub spec: SqlSelectSpec,
}

pub enum SqlSelectSpec {
    Star,
    List(Vec<SqlSelectItem>),
    Value(Box<SqlScalarExpression>),
}

/// All scalar expression variants (the heart of the AST)
pub enum SqlScalarExpression {
    Literal(SqlLiteral),
    PropertyRef(SqlIdentifier),
    MemberRef { source: Box<SqlScalarExpression>, member: SqlIdentifier },
    MemberIndexer { source: Box<SqlScalarExpression>, index: Box<SqlScalarExpression> },
    Binary { op: SqlBinaryOp, left: Box<SqlScalarExpression>, right: Box<SqlScalarExpression> },
    Unary { op: SqlUnaryOp, operand: Box<SqlScalarExpression> },
    FunctionCall { name: SqlIdentifier, args: Vec<SqlScalarExpression>, is_udf: bool },
    Between { expr: Box<SqlScalarExpression>, low: Box<SqlScalarExpression>, high: Box<SqlScalarExpression>, not: bool },
    In { expr: Box<SqlScalarExpression>, items: Vec<SqlScalarExpression>, not: bool },
    Like { expr: Box<SqlScalarExpression>, pattern: Box<SqlScalarExpression>, escape: Option<String>, not: bool },
    Conditional { condition: Box<SqlScalarExpression>, if_true: Box<SqlScalarExpression>, if_false: Box<SqlScalarExpression> },
    Coalesce { left: Box<SqlScalarExpression>, right: Box<SqlScalarExpression> },
    Exists(Box<SqlQuery>),
    Subquery(Box<SqlQuery>),
    Array(Box<SqlQuery>),
    ArrayCreate(Vec<SqlScalarExpression>),
    ObjectCreate(Vec<SqlObjectProperty>),
    ParameterRef(String),
}

pub enum SqlLiteral {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Null,
    Undefined,
}

pub enum SqlBinaryOp {
    Add, Subtract, Multiply, Divide, Modulo,
    Equal, NotEqual, LessThan, GreaterThan, LessThanOrEqual, GreaterThanOrEqual,
    And, Or,
    BitwiseAnd, BitwiseOr, BitwiseXor, LeftShift, RightShift, ZeroFillRightShift,
    StringConcat,
}

pub enum SqlUnaryOp {
    Not, Minus, Plus, BitwiseNot,
}

pub enum SqlSortOrder {
    None,      // unspecified
    Ascending,
    Descending,
}
```

Source mapping:
| C++ File | Rust Module |
|----------|-------------|
| `SqlObjectKind.h` | `ast/mod.rs` — enum variants become struct/enum types |
| `SqlQuery.h`, `SqlSelectClause.h`, `SqlFromClause.h`, `SqlWhereClause.h`, `SqlGroupByClause.h`, `SqlOrderByClause.h`, `SqlOffsetLimitClause.h` | `ast/mod.rs` |
| `SqlScalarExpression.h`, `SqlBinaryScalarExpression.h`, `SqlUnaryScalarExpression.h`, `SqlLiteralScalarExpression.h`, `SqlFunctionCallScalarExpression.h`, etc. | `ast/scalar_expression.rs` |
| `SqlBinaryScalarOperatorKind.h`, `SqlUnaryScalarOperatorKind.h`, `SqlSortOrder.h` | `ast/mod.rs` — enums |
| `SqlCollectionExpression.h`, `SqlAliasedCollectionExpression.h`, `SqlArrayIteratorCollectionExpression.h`, `SqlJoinCollectionExpression.h`, `SqlInputPathCollection.h` | `ast/collection.rs` |
| `SqlObjectVisitor.h` | `ast/visitor.rs` |

### 1.2 Lexer (port from `SqlScanner.h/.cpp`)

Hand-crafted scanner operating on `&str` (UTF-8, not WCHAR). Key behaviors to preserve:
- Token types: identifiers, keywords, numbers (integer + float), strings (single-quoted), parameters (`@name`), operators, punctuation
- Keyword recognition via lookup table (from `SqlStringTokens.h/.cpp`)
- Location tracking (byte offset start/end for error messages)
- No allocations for most tokens (use string slices into the source)

```rust
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub span: Span,
}

pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub enum TokenKind {
    // Literals
    Identifier,
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    Parameter,         // @name
    
    // Keywords (mapped from SqlStringTokens)
    Select, From, Where, And, Or, Not, As, In, Between, Like, Escape,
    Order, By, Asc, Desc, Top, Distinct, Value, Group, Having,
    Join, Cross, Inner, Exists, Array, Null, True, False, Undefined,
    Offset, Limit, Udf, Is, Let,
    
    // Operators
    Plus, Minus, Star, Slash, Percent, Tilde,
    Ampersand, Pipe, Caret, Eq, NotEq, Lt, Gt, LtEq, GtEq,
    LeftShift, RightShift, ZeroFillRightShift,
    StringConcat,    // ||
    Coalesce,        // ??
    Question, Colon,
    
    // Punctuation
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Dot, Comma, Bang,
    
    // Special
    Eof,
}
```

Source mapping: `SqlScanner.h/.cpp` → `lexer/mod.rs`, `SqlStringTokens.h` → `lexer/keywords.rs`

### 1.3 Parser (port from `sql.y` grammar + `SqlParser.h/.cpp`)

**Recursive descent** rather than LALR. The grammar in `sql.y` is clean enough (~200 production rules) that recursive descent is straightforward and more maintainable in Rust. Operator precedence is handled by the standard Pratt-parsing technique for binary expressions.

Key entry point:
```rust
pub fn parse(sql: &str) -> Result<SqlProgram, ParseError>
```

Grammar coverage (from `sql.y`):
- `program → sql_query`
- `sql_query → select_clause opt_from opt_where opt_group_by opt_order_by opt_offset_limit`
- `select_clause → SELECT [DISTINCT] [TOP n] (star | value_expr | select_list)`
- `collection_expression → aliased_collection | array_iterator | join`
- `scalar_expression` — full expression hierarchy with correct precedence
- Subqueries: `EXISTS(query)`, `ARRAY(query)`, `(query)` in FROM

Nesting depth limit: 128 (matching C++ `MaxAllowedNestingDepth` for client scenarios; the C++ engine uses 20000 for backend but we don't need that).

### 1.4 Partition Key Filter Extraction (port from `QueryLogicalPlan.FilterExpressionPredicateExtractor` + `FilterExpressionPartitionKeyRangeTranslator`)

This is the **critical business logic**. The algorithm:

1. Walk the WHERE clause AST
2. Identify equality comparisons against partition key path(s) e.g., `c.pk = "value"` or `c.pk IN ("a", "b")`
3. For AND expressions, intersect constraints
4. For OR expressions, union constraints (expand to multiple PK ranges)
5. Convert extracted PK values → effective partition key hash → FeedRange

```rust
/// Result of analyzing a query for partition key targeting
pub struct QueryPlanInfo {
    /// Whether the query targets a single partition
    pub is_single_partition: bool,
    
    /// Extracted partition key filters (if any)
    pub pk_filters: Vec<PartitionKeyFilter>,
    
    /// Query features detected
    pub features: QueryFeatures,
}

pub enum PartitionKeyFilter {
    /// Exact equality: pk = <value>
    Equality(Vec<PartitionKeyValue>),
    
    /// IN list: pk IN (v1, v2, ...)  
    InList(Vec<Vec<PartitionKeyValue>>),
    
    /// Range: pk > X AND pk < Y — can't target single partition but can narrow FeedRanges
    Range { min: Option<PartitionKeyValue>, max: Option<PartitionKeyValue> },
    
    /// Full scan needed (no PK filter found)
    None,
}

pub enum PartitionKeyValue {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    Undefined,
}

bitflags! {
    pub struct QueryFeatures: u32 {
        const HAS_AGGREGATE  = 0b0000_0001;
        const HAS_DISTINCT   = 0b0000_0010;
        const HAS_ORDER_BY   = 0b0000_0100;
        const HAS_TOP        = 0b0000_1000;
        const HAS_OFFSET     = 0b0001_0000;
        const HAS_GROUP_BY   = 0b0010_0000;
        const HAS_JOIN       = 0b0100_0000;
        const HAS_SUBQUERY   = 0b1000_0000;
    }
}
```

The extraction walks the WHERE clause looking for patterns like:
- `c.<pk_path> = <literal>` → `Equality`
- `c.<pk_path> = @param` (resolved from query parameters) → `Equality`
- `c.<pk_path> IN (<literal>, ...)` → `InList`
- `<pk_filter> AND <other>` → keep PK filter, pass rest through
- `<pk_filter> OR <pk_filter>` → union of PK ranges
- Anything else on the PK path → falls back to cross-partition

Source mapping:
| C++ | Rust |
|-----|------|
| `FilterExpressionPredicateExtractor` | `plan/pk_extractor.rs` |
| `FilterExpressionPartitionKeyRangeTranslator` | `plan/pk_extractor.rs` (simplified, no logical plan intermediary) |
| `FilterExpressionPartitionKeyRangeOptimizer` | Not needed (optimization for backend execution) |
| `DistributedQueryCompiler` | Not needed (that's the Gateway's job; we replicate only the PK extraction part) |

### 1.5 Integration with azure_data_cosmos

In `azure_data_cosmos`, the query execution path currently always requests a query plan from the Gateway. The integration point:

```rust
// In azure_data_cosmos/src/query/mod.rs or a new routing module:

/// Try to extract partition key targeting from the query text + parameters.
/// Returns None if the query can't be analyzed client-side (unsupported features).
pub fn try_extract_target_ranges(
    query: &Query,
    partition_key_definition: &PartitionKeyDefinition,
    container_routing_map: &CollectionRoutingMap,
) -> Option<Vec<FeedRange>> {
    let program = azure_data_cosmos_query::parse(&query.text).ok()?;
    let plan = azure_data_cosmos_query::analyze(&program, &partition_key_definition.paths)?;
    
    // Resolve parameters in PK filters
    let resolved_plan = plan.resolve_parameters(&query.parameters)?;
    
    match &resolved_plan.pk_filters {
        PartitionKeyFilter::Equality(values) => {
            // Hash the PK values → effective partition key → find FeedRange
            let epk = hash_partition_key(values, &partition_key_definition.kind);
            Some(vec![container_routing_map.get_range_for_epk(&epk)])
        }
        PartitionKeyFilter::InList(value_sets) => {
            // Multiple PK targets → multiple FeedRanges
            let ranges: HashSet<_> = value_sets.iter()
                .map(|values| {
                    let epk = hash_partition_key(values, &partition_key_definition.kind);
                    container_routing_map.get_range_for_epk(&epk)
                })
                .collect();
            Some(ranges.into_iter().collect())
        }
        _ => None, // Cross-partition, fall back to Gateway query plan
    }
}
```

---

## Phase 2: In-Memory Query Evaluation

### 2.1 Document Matching (WHERE clause evaluation)

Evaluate a SQL WHERE clause against a `serde_json::Value`. This is a simplified port of `FilterEvaluator` — we skip the VM/IL pipeline and directly interpret the AST.

```rust
/// Check if a JSON document matches a SQL WHERE clause.
pub fn matches(
    document: &serde_json::Value,
    where_clause: &SqlWhereClause,
    parameters: &[(String, serde_json::Value)],
) -> Result<bool, EvalError>
```

The evaluator recursively evaluates `SqlScalarExpression` against the document:

| Expression Type | Evaluation |
|----------------|------------|
| `PropertyRef("c")` | Returns the document root |
| `MemberRef(source, "name")` | `source["name"]` |
| `MemberIndexer(source, idx)` | `source[idx]` (array index) |
| `Literal(...)` | Returns the literal value |
| `Binary(op, l, r)` | Evaluate both sides, apply operator with Cosmos comparison semantics |
| `Unary(Not, expr)` | `!evaluate(expr)` |
| `FunctionCall("CONTAINS", [s, sub])` | `s.contains(sub)` |
| `In(expr, items)` | `items.any(|i| evaluate(expr) == evaluate(i))` |
| `Between(expr, low, high)` | `low <= expr && expr <= high` |
| `Like(expr, pattern)` | SQL LIKE pattern matching |
| `Exists(subquery)` | Evaluate subquery, check non-empty |
| `ParameterRef("@p")` | Look up in parameter map |

**Cosmos-specific comparison semantics** (important, from the C++ `VMVariantUtils`):
- Type ordering: `null < boolean < number < string < array < object < undefined`
- Cross-type comparisons: always `undefined` (not equal, not less, not greater)
- `null = null` is `true`
- `undefined = undefined` is `undefined` (falsy)

### 2.2 Projection (SELECT clause evaluation)

```rust
/// Apply SELECT clause projection to a document.
pub fn project(
    document: &serde_json::Value,
    select: &SqlSelectClause,
    from_binding: &str,  // e.g., "c"
    parameters: &[(String, serde_json::Value)],
) -> Result<serde_json::Value, EvalError>
```

| SELECT Form | Behavior |
|-------------|----------|
| `SELECT *` | Return full document |
| `SELECT c.name, c.age` | Return `{"name": ..., "age": ...}` |
| `SELECT VALUE c.name` | Return the scalar value directly |
| `SELECT c.name AS n` | Return `{"n": ...}` |
| `SELECT { "x": c.a }` | Construct object from expressions |

### 2.3 Combined Query Function

```rust
/// Evaluate a SQL query against an in-memory collection of documents.
/// Returns matching, projected documents.
pub fn query_documents(
    sql: &str,
    parameters: &[(String, serde_json::Value)],
    documents: &[serde_json::Value],
) -> Result<Vec<serde_json::Value>, QueryError> {
    let program = parse(sql)?;
    let query = &program.query;
    
    let mut results = Vec::new();
    for doc in documents {
        // 1. Evaluate WHERE
        if let Some(where_clause) = &query.r#where {
            if !matches(doc, where_clause, parameters)? {
                continue;
            }
        }
        
        // 2. Apply SELECT projection
        let projected = project(doc, &query.select, "c", parameters)?;
        results.push(projected);
    }
    
    // 3. Apply TOP/OFFSET/LIMIT (simple truncation)
    apply_top_offset_limit(&mut results, &query);
    
    Ok(results)
}
```

### 2.4 Built-in Functions (subset)

Initially support the most commonly used functions:

| Category | Functions |
|----------|-----------|
| Type checking | `IS_DEFINED`, `IS_NULL`, `IS_BOOL`, `IS_NUMBER`, `IS_STRING`, `IS_ARRAY`, `IS_OBJECT` |
| String | `CONTAINS`, `STARTSWITH`, `ENDSWITH`, `UPPER`, `LOWER`, `LENGTH`, `LTRIM`, `RTRIM`, `TRIM`, `CONCAT`, `SUBSTRING`, `REPLACE`, `LEFT`, `RIGHT`, `ToString` |
| Math | `ABS`, `CEILING`, `FLOOR`, `ROUND`, `POWER`, `SQRT`, `LOG`, `LOG10`, `EXP`, `SIGN` |
| Array | `ARRAY_CONTAINS`, `ARRAY_LENGTH`, `ARRAY_SLICE` |
| Conversion | `ToString`, `ToNumber` |

---

## What We Explicitly Skip

| C++ Component | Why Skipped |
|--------------|-------------|
| `queryIL/` (IL representation) | We interpret the AST directly; no need for bytecode |
| `queryRuntime/VM*` (VM execution engine) | Only needed for backend index execution |
| `queryRuntime/IndexPlan*` | Index-level execution plans (backend only) |
| `queryRuntime/QueryPhysicalPlan*` | Physical plan generation (backend only) |
| `queryDistribution/` (most of it) | Distribution planning is Gateway's responsibility |
| `queryEngine/QueryEngine.cpp` | Backend query execution orchestration |
| `queryRuntime/CompilerIL*` | IL → backend assembly compilation |
| `queryLanguages/kql/` | KQL support not needed |
| `queryLanguages/JavaScript/` | JavaScript query support not needed |
| `pgQueryInterop/` | PostgreSQL interop not needed |
| `Tools/` | Query engine tooling not needed |
| ORDER BY evaluation | Phase 2 skip (simple `sort_by` if needed later) |
| GROUP BY / aggregates evaluation | Phase 2 skip (add later if emulator needs it) |
| JOIN evaluation | Phase 2 skip (complex, add later) |

---

## Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
# No async runtime, no azure_core, no network dependencies
```

## Testing Strategy

1. **Parser tests**: Port representative SQL strings from C++ test suites and assert against hardcoded expected AST structures
2. **Formatter tests**: Assert hardcoded expected SQL strings for representative AST inputs and normalized formatting cases; do not use `parse(sql).to_string() ≈ sql` round-trip tests
3. **PK extraction tests**: 
   - `SELECT * FROM c WHERE c.pk = "hello"` → single partition
   - `SELECT * FROM c WHERE c.pk IN ("a", "b")` → two partitions
   - `SELECT * FROM c WHERE c.pk = "x" AND c.other > 5` → single partition
   - `SELECT * FROM c WHERE c.pk = "x" OR c.pk = "y"` → two partitions
   - `SELECT * FROM c WHERE c.other > 5` → cross-partition (no PK filter)
   - Hierarchical PK: `WHERE c.tenant = "t" AND c.user = "u"` → single partition
4. **Matcher tests**: JSON documents × SQL WHERE → expected match/no-match
5. **Projection tests**: JSON documents × SQL SELECT → expected output
6. **Parameter resolution tests**: `@param` references resolved from parameter map

## Implementation Order

1. `ast/` — Define all AST types (straightforward data types, no logic)
2. `lexer/` — Hand-crafted scanner with keyword table
3. `parser/` — Recursive descent from grammar rules
4. `plan/pk_extractor.rs` — Walk WHERE clause, extract PK filters
5. `value.rs` — Cosmos comparison semantics wrapper
6. `eval/scalar.rs` — Scalar expression evaluator
7. `eval/matcher.rs` — WHERE clause document matching
8. `eval/projector.rs` — SELECT clause projection
9. `eval/functions.rs` — Built-in function implementations
10. Integration into `azure_data_cosmos` query path

## Estimated Scope

| Component | Estimated LoC | Complexity |
|-----------|--------------|------------|
| AST types | ~400 | Low (data definitions) |
| Lexer | ~500 | Medium (hand-crafted, thorough) |
| Parser | ~800 | Medium-High (recursive descent, all grammar rules) |
| PK Extractor | ~300 | Medium (AST walking, pattern matching) |
| Value/Comparison | ~200 | Medium (Cosmos type ordering) |
| Evaluator (matcher + projector + scalar + functions) | ~800 | Medium |
| Tests | ~600 | — |
| **Total** | **~3,600** | — |
