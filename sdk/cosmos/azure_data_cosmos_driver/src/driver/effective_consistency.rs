// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Effective consistency resolution for session token management.

use crate::options::ReadConsistencyStrategy;

/// Resolves whether session tokens should be captured for the given consistency configuration.
///
/// Returns `true` if the effective consistency is `Session`, meaning the driver should
/// actively resolve and capture session tokens.
///
/// The resolution logic:
/// - If `strategy` is `ReadConsistencyStrategy::Session`, always returns `true`.
/// - If `strategy` is `ReadConsistencyStrategy::Default`, checks `account_default_consistency`
///   to see if it equals "Session" (case-insensitive).
/// - For all other strategies (`Eventual`, `GlobalStrong`), returns `false`.
///
/// # TODO(read-consistency-strategy)
///
/// When the full `ReadConsistencyStrategy` pipeline is wired up (consistency level override
/// at operation/client level), this function should be revisited to ensure it correctly
/// handles the merged consistency level. Currently it only considers the operation-level
/// strategy and the account default.
pub(crate) fn is_session_consistency_effective(
    strategy: ReadConsistencyStrategy,
    account_default_consistency: &str,
) -> bool {
    match strategy {
        ReadConsistencyStrategy::Session => true,
        ReadConsistencyStrategy::Default => {
            account_default_consistency.eq_ignore_ascii_case("Session")
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_strategy_always_true() {
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Session,
            "Strong"
        ));
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Session,
            "Eventual"
        ));
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Session,
            "Session"
        ));
    }

    #[test]
    fn default_strategy_with_session_account() {
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Default,
            "Session"
        ));
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Default,
            "session"
        ));
        assert!(is_session_consistency_effective(
            ReadConsistencyStrategy::Default,
            "SESSION"
        ));
    }

    #[test]
    fn default_strategy_with_strong_account() {
        assert!(!is_session_consistency_effective(
            ReadConsistencyStrategy::Default,
            "Strong"
        ));
    }

    #[test]
    fn eventual_strategy_always_false() {
        assert!(!is_session_consistency_effective(
            ReadConsistencyStrategy::Eventual,
            "Session"
        ));
        assert!(!is_session_consistency_effective(
            ReadConsistencyStrategy::Eventual,
            "Eventual"
        ));
    }

    #[test]
    fn global_strong_strategy_always_false() {
        assert!(!is_session_consistency_effective(
            ReadConsistencyStrategy::GlobalStrong,
            "Session"
        ));
        assert!(!is_session_consistency_effective(
            ReadConsistencyStrategy::GlobalStrong,
            "Strong"
        ));
    }
}
