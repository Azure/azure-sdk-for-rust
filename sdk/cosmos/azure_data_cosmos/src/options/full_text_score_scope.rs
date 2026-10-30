// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Selects the partitions used to calculate full-text ranking statistics.
///
/// This option applies to ranked full-text and hybrid queries. It does not
/// change which partitions return matching documents.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum FullTextScoreScope {
    /// Calculate statistics across the entire container, even for a scoped query.
    #[default]
    Global,
    /// Calculate statistics only across the partitions targeted by the query.
    Local,
}
