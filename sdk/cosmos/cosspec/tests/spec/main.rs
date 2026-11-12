// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB SDK spec tests.
//!
//! Each spec states one SDK behavior in a few lines. See
//! `sdk/cosmos/docs/specs/0030-cosmos-spec-tests.md` for the DSL and
//! `CATALOG.md` next to this crate for the generated index.

// Specs favor plain `.await` over boxing large futures so they read as specifications.
#![allow(clippy::large_futures)]

mod framework;
mod items;
mod query;
mod resilience;

fn main() {
    cosspec::main();
}
