// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

/// Specifies the mode for reading the change feed.
///
/// The mode determines what types of changes are included in the change feed results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChangeFeedMode {
    /// Returns only the latest version of changed items.
    ///
    /// This is the default mode and returns the current state of items that have been
    /// created or modified. Deleted items are not included.
    #[default]
    LatestVersion,

    /// Returns all versions of items including deletes.
    ///
    /// This mode provides full fidelity change tracking including:
    /// - Item creates (with the created document)
    /// - Item updates (with the updated document and metadata)
    /// - Item deletes (with metadata about the deleted item)
    ///
    /// **Note:** This mode requires the container to be configured with a change feed policy
    /// that enables full fidelity change feed. See the Azure Cosmos DB documentation for
    /// how to enable this feature.
    AllVersionsAndDeletes,
}

impl fmt::Display for ChangeFeedMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChangeFeedMode::LatestVersion => write!(f, "LatestVersion"),
            ChangeFeedMode::AllVersionsAndDeletes => write!(f, "AllVersionsAndDeletes"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mode_is_latest_version() {
        assert_eq!(ChangeFeedMode::default(), ChangeFeedMode::LatestVersion);
    }

    #[test]
    fn display_latest_version() {
        assert_eq!(
            format!("{}", ChangeFeedMode::LatestVersion),
            "LatestVersion"
        );
    }

    #[test]
    fn display_all_versions_and_deletes() {
        assert_eq!(
            format!("{}", ChangeFeedMode::AllVersionsAndDeletes),
            "AllVersionsAndDeletes"
        );
    }
}
