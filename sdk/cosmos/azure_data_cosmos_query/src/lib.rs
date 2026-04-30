// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB SQL query parser, partition key extraction, and in-memory evaluation.
//!
//! This crate provides:
//! - A SQL parser for the Cosmos DB SQL dialect
//! - Partition key filter extraction from WHERE clauses (to avoid Gateway query plan calls)
//! - In-memory document matching and projection (for test emulators)
//!
//! # Examples
//!
//! Parse a SQL query:
//! ```rust
//! use azure_data_cosmos_query::parse;
//! let program = parse("SELECT * FROM c WHERE c.pk = 'hello'").unwrap();
//! ```
//!
//! Generate a query plan:
//! ```rust
//! use azure_data_cosmos_query::{parse, plan::generate_query_plan};
//! let program = parse("SELECT * FROM c WHERE c.pk = 'hello' AND c.x > 5").unwrap();
//! let query_plan = generate_query_plan(&program.query, &["/pk"]);
//! // query_plan.pk_filters — partition targeting
//! // query_plan.query_info — structural info (aggregates, ORDER BY, etc.)
//! ```
//!
//! Match a document against a query:
//! ```rust
//! use azure_data_cosmos_query::{parse, eval};
//! let program = parse("SELECT * FROM c WHERE c.age > 21").unwrap();
//! let doc = serde_json::json!({"age": 30, "name": "Alice"});
//! assert!(eval::matches_query(&doc, &program.query, &[]).unwrap());
//! ```

pub mod ast;
pub(crate) mod common;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod plan;
mod value;

pub use parser::parse;
