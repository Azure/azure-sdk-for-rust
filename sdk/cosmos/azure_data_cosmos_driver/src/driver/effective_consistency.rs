// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Determines whether session consistency is effective for an operation.

use crate::options::ReadConsistencyStrategy;

/// Returns `true` if session consistency is effectively active for an operation.
///
/// Session token capture/resolve should only occur when the effective consistency
/// level is Session. This function combines the per-operation read consistency
/// strategy with the account-level default consistency level to make that
/// determination.
///
/// # Parameters
///
/// - `strategy`: The effective [`ReadConsistencyStrategy`] for this operation
///   (already merged from operation → driver → runtime levels).
/// - `account_default`: The `default_consistency_level` string from the account
///   properties (e.g., `"Session"`, `"Strong"`, `"Eventual"`).
///
/// # Rules
///
/// | Strategy           | Account default | Result  |
/// |--------------------|-----------------|---------|
/// | `Default`          | `"Session"`     | `true`  |
/// | `Default`          | anything else   | `false` |
/// | `Session`          | (any)           | `true`  |
/// | `Eventual`         | (any)           | `false` |
/// | `GlobalStrong`     | (any)           | `false` |
///
/// TODO(read-consistency-strategy): Once full consistency level override pipeline
/// is wired up, revisit this to use the merged consistency level rather than just
/// the runtime strategy + account default.
pub(crate) fn is_session_consistency_effective(
    strategy: ReadConsistencyStrategy,
    account_default: &str,
) -> bool {
    match strategy {
        ReadConsistencyStrategy::Session => true,
        ReadConsistencyStrategy::Default => account_default.eq_ignore_ascii_case("Session"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_strategy_with_session_account() {
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Default,
            "Session",
        ));
    }

    #[test]
    fn default_strategy_with_strong_account() {
        assert!(!is_session_consistency_effective(
            ReadConsistencyStrategy::Default,
            "Strong",
        ));
    }

    #[test]
    fn session_strategy_overrides_account() {
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Session,
            "Strong",
        ));
    }

    #[test]
    fn eventual_strategy_never_session() {
        assert!(!is_session_consistency_effective(
            ReadConsistencyStrategy::Eventual,
            "Session",
        ));
    }

    #[test]
    fn case_insensitive_account_default() {
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Default,
            "session",
        ));
    }
}
