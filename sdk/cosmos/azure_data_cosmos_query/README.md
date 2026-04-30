# Azure Cosmos DB Query Engine

SQL query parser, partition key extraction, and in-memory evaluation for Azure Cosmos DB.

## Purpose

`azure_data_cosmos_query` provides:

- **SQL Parser**: A hand-crafted lexer and recursive-descent parser for the Cosmos DB SQL dialect
- **Query Plan Generation**: Extract partition key filters and structural query information (aggregates, ORDER BY, GROUP BY, DISTINCT, etc.) from parsed SQL — equivalent to the Gateway query plan REST endpoint
- **In-Memory Evaluation**: Match JSON documents against SQL WHERE clauses and apply SELECT projections, for use in test emulators

## Usage

```rust no_run
use azure_data_cosmos_query::{parse, plan, eval};

// Parse a SQL query
let program = parse("SELECT * FROM c WHERE c.pk = 'hello' AND c.age > 21").unwrap();

// Generate a query plan (partition targeting + structural info)
let query_plan = plan::generate_query_plan(&program.query, &["/pk"]);

// Evaluate against in-memory documents
let doc = serde_json::json!({"pk": "hello", "age": 30, "name": "Alice"});
assert!(eval::matches_query(&doc, &program.query, &[]).unwrap());
```

## Relationship to Other Crates

- **`azure_data_cosmos`**: The primary Rust SDK uses this crate internally for client-side query plan generation
- **`azure_data_cosmos_driver`**: The transport/routing layer; this crate is independent of the driver

## Support Model

This crate is an internal implementation detail of the Azure Cosmos DB SDK for Rust. It has a public API but is primarily intended for use by `azure_data_cosmos`.