// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Newtype for partition key range identifiers.

use compact_str::CompactString;

/// Identifies a physical partition key range.
///
/// Newtype wrapper around the raw string ID from the
/// `x-ms-documentdb-partitionkeyrangeid` response header.
/// Using a newtype rather than a bare `String` prevents accidental
/// misuse (e.g., passing an account ID where a partition key range ID
/// is expected).
///
/// Backed by [`CompactString`] so that production-typical short IDs
/// (numeric strings up to `CompactString`'s inline limit) are stored
/// inline without a heap allocation. PPCB and PPAF maps may carry tens
/// of thousands of these in worst-case failover scenarios; eliminating
/// the per-entry `String` heap allocation is a measurable memory win.
//
// `pub` (rather than `pub(crate)`) so that `crate::testing` can surface
// this type for memory benchmarks under the `__internal_testing` feature
// flag. The enclosing `routing` module is `pub(crate)`, so external
// consumers still cannot reach this via `crate::driver::routing::*`; it
// remains accessible only through the `crate::testing::*` re-exports.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PartitionKeyRangeId(CompactString);

impl PartitionKeyRangeId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for PartitionKeyRangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl std::str::FromStr for PartitionKeyRangeId {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(CompactString::from(s)))
    }
}

impl From<String> for PartitionKeyRangeId {
    fn from(s: String) -> Self {
        Self(CompactString::from(s))
    }
}

impl From<&str> for PartitionKeyRangeId {
    fn from(s: &str) -> Self {
        Self(CompactString::from(s))
    }
}

impl std::borrow::Borrow<str> for PartitionKeyRangeId {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}
