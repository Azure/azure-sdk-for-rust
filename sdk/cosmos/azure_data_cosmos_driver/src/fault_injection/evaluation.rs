// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};

/// Records the evaluation result of a fault injection rule for a single request.
///
/// When fault injection is enabled, each request evaluates all configured rules.
/// This enum captures why each rule was applied, skipped, or missed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FaultInjectionEvaluation {
    /// Rule was applied — fault was injected.
    Applied {
        /// The ID of the applied rule.
        rule_id: String,
    },
    /// Rule matched but the probability check failed — no fault injected.
    ProbabilityMiss {
        /// The ID of the rule.
        rule_id: String,
        /// The configured probability (0.0–1.0).
        probability: f32,
    },
    /// Rule was skipped because it is disabled.
    Disabled {
        /// The ID of the rule.
        rule_id: String,
    },
    /// Rule was skipped because the current time is before its start_time.
    BeforeStartTime {
        /// The ID of the rule.
        rule_id: String,
    },
    /// Rule was skipped because the current time is at or after its end_time.
    AfterEndTime {
        /// The ID of the rule.
        rule_id: String,
    },
    /// Rule was skipped because its hit_limit has been exhausted.
    HitLimitExhausted {
        /// The ID of the rule.
        rule_id: String,
        /// The current hit count.
        hit_count: u32,
        /// The configured hit limit.
        hit_limit: u32,
    },
    /// Rule was skipped because the operation type did not match.
    OperationMismatch {
        /// The ID of the rule.
        rule_id: String,
    },
    /// Rule was skipped because the region did not match.
    RegionMismatch {
        /// The ID of the rule.
        rule_id: String,
    },
    /// Rule was skipped because the container ID did not match.
    ContainerMismatch {
        /// The ID of the rule.
        rule_id: String,
    },
    /// Rule matched but was superseded by a higher-priority rule (first-match-wins).
    Superseded {
        /// The ID of the superseded rule.
        rule_id: String,
    },
}

// Manual `Eq` because `ProbabilityMiss` contains `f32` which doesn't
// implement `Eq`. This is safe because the enum is a diagnostic value
// not used for hashing or ordering — the `f32` probability is always
// a finite value in [0.0, 1.0] (validated by the builder).
impl Eq for FaultInjectionEvaluation {}

impl FaultInjectionEvaluation {
    /// Returns the rule ID associated with this evaluation.
    pub fn rule_id(&self) -> &str {
        match self {
            Self::Applied { rule_id }
            | Self::ProbabilityMiss { rule_id, .. }
            | Self::Disabled { rule_id }
            | Self::BeforeStartTime { rule_id }
            | Self::AfterEndTime { rule_id }
            | Self::HitLimitExhausted { rule_id, .. }
            | Self::OperationMismatch { rule_id }
            | Self::RegionMismatch { rule_id }
            | Self::ContainerMismatch { rule_id }
            | Self::Superseded { rule_id } => rule_id,
        }
    }

    /// Returns true if the rule was applied (fault injected).
    pub fn was_applied(&self) -> bool {
        matches!(self, Self::Applied { .. })
    }
}

impl std::fmt::Display for FaultInjectionEvaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Applied { rule_id } => write!(f, "rule '{rule_id}': applied"),
            Self::ProbabilityMiss {
                rule_id,
                probability,
            } => {
                write!(
                    f,
                    "rule '{rule_id}': skipped (probability miss, p={probability})"
                )
            }
            Self::Disabled { rule_id } => write!(f, "rule '{rule_id}': skipped (disabled)"),
            Self::BeforeStartTime { rule_id } => {
                write!(f, "rule '{rule_id}': skipped (before start time)")
            }
            Self::AfterEndTime { rule_id } => {
                write!(f, "rule '{rule_id}': skipped (after end time)")
            }
            Self::HitLimitExhausted {
                rule_id,
                hit_count,
                hit_limit,
            } => {
                write!(
                    f,
                    "rule '{rule_id}': skipped (hit limit {hit_count}/{hit_limit})"
                )
            }
            Self::OperationMismatch { rule_id } => {
                write!(f, "rule '{rule_id}': skipped (operation mismatch)")
            }
            Self::RegionMismatch { rule_id } => {
                write!(f, "rule '{rule_id}': skipped (region mismatch)")
            }
            Self::ContainerMismatch { rule_id } => {
                write!(f, "rule '{rule_id}': skipped (container mismatch)")
            }
            Self::Superseded { rule_id } => {
                write!(
                    f,
                    "rule '{rule_id}': skipped (superseded by higher priority)"
                )
            }
        }
    }
}
