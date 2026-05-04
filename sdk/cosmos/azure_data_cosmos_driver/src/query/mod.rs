// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB SQL query parser, partition key extraction, and in-memory evaluation.
//!
//! This module provides:
//! - A SQL parser for the Cosmos DB SQL dialect
//! - Partition key filter extraction from WHERE clauses (to avoid Gateway query plan calls)
//! - In-memory document matching and projection (for test emulators)

pub(crate) mod ast;
pub(crate) mod common;
pub(crate) mod eval;
pub(crate) mod gateway_plan;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod plan;
mod value;

pub(crate) use parser::parse;

#[cfg(any(test, feature = "__internal_testing"))]
pub use plan::generate_query_plan_for_pk_paths;
