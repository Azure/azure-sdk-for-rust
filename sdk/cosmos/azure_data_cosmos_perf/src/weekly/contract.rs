// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::stats::Summary;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Operation {
    ReadItem,
    CreateItem,
    UpsertItem,
    ReplaceItem,
    PatchItem,
    DeleteItem,
    QueryItems,
    FeedRangeQuery,
    ChangeFeed,
    CrossPartitionQuery,
    OrderByQuery,
    AggregateQuery,
    DistinctQuery,
    TransactionalBatch,
}

impl Operation {
    pub(super) const ALL: [Self; 14] = [
        Self::ReadItem,
        Self::CreateItem,
        Self::UpsertItem,
        Self::ReplaceItem,
        Self::PatchItem,
        Self::DeleteItem,
        Self::QueryItems,
        Self::FeedRangeQuery,
        Self::ChangeFeed,
        Self::CrossPartitionQuery,
        Self::OrderByQuery,
        Self::AggregateQuery,
        Self::DistinctQuery,
        Self::TransactionalBatch,
    ];

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::ReadItem => "ReadItem",
            Self::CreateItem => "CreateItem",
            Self::UpsertItem => "UpsertItem",
            Self::ReplaceItem => "ReplaceItem",
            Self::PatchItem => "PatchItem",
            Self::DeleteItem => "DeleteItem",
            Self::QueryItems => "QueryItems",
            Self::FeedRangeQuery => "FeedRangeQuery",
            Self::ChangeFeed => "ChangeFeed",
            Self::CrossPartitionQuery => "CrossPartitionQuery",
            Self::OrderByQuery => "OrderByQuery",
            Self::AggregateQuery => "AggregateQuery",
            Self::DistinctQuery => "DistinctQuery",
            Self::TransactionalBatch => "TransactionalBatch",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct Allocation {
    pub operation: Operation,
    pub workers: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct Profile {
    pub version: u32,
    pub duration_seconds: u64,
    pub warmup_seconds: u64,
    pub report_seconds: u64,
    pub request_timeout_seconds: u64,
    pub seed_count: u32,
    pub payload_bytes: u32,
    pub throughput_per_container: u32,
    pub max_pages: u32,
    pub vm_size: String,
    pub consistency: String,
    pub operations: Vec<Allocation>,
}

impl Profile {
    pub(super) fn validate(&self) -> Result<(), String> {
        if self.version != 1
            || self.duration_seconds <= self.warmup_seconds
            || self.duration_seconds > 2700
            || self.report_seconds == 0
            || self.request_timeout_seconds == 0
            || self.request_timeout_seconds > 60
            || self.seed_count == 0
            || self.payload_bytes == 0
            || self.payload_bytes > 1_000_000
            || self.throughput_per_container < 400
            || self.max_pages == 0
            || self.vm_size.trim().is_empty()
            || self.consistency.trim().is_empty()
        {
            return Err("invalid workload profile bounds or version".into());
        }
        let actual: BTreeSet<_> = self.operations.iter().map(|a| a.operation).collect();
        if actual != Operation::ALL.into_iter().collect()
            || actual.len() != self.operations.len()
            || self
                .operations
                .iter()
                .any(|a| a.workers == 0 || a.workers > 1024)
        {
            return Err(
                "profile must allocate 1..1024 workers to every operation exactly once".into(),
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Threshold {
    pub operation: Operation,
    pub min_successful_ops_per_second: f64,
    pub max_p99_ms: f64,
    pub max_error_rate: f64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Baseline {
    pub version: u32,
    pub approved: bool,
    pub profile: Option<Profile>,
    pub thresholds: Vec<Threshold>,
}

impl Baseline {
    pub(super) fn validate(&self, profile: &Profile) -> Result<(), String> {
        if self.version != 1 || !self.approved || self.profile.as_ref() != Some(profile) {
            return Err("baseline must be reviewed and match the complete workload profile".into());
        }
        let actual: BTreeSet<_> = self.thresholds.iter().map(|t| t.operation).collect();
        if actual != Operation::ALL.into_iter().collect() || actual.len() != self.thresholds.len() {
            return Err("baseline must contain exactly one threshold per operation".into());
        }
        for t in &self.thresholds {
            if !t.min_successful_ops_per_second.is_finite()
                || t.min_successful_ops_per_second <= 0.0
                || !t.max_p99_ms.is_finite()
                || t.max_p99_ms <= 0.0
                || !t.max_error_rate.is_finite()
                || !(0.0..1.0).contains(&t.max_error_rate)
            {
                return Err(format!("invalid threshold for {}", t.operation.name()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Environment {
    pub source_commit: String,
    pub baseline_commit: String,
    pub account_resource_id: String,
    pub endpoint: String,
    pub region: String,
    pub vm_size: String,
    pub consistency: String,
}

#[derive(Serialize)]
pub(super) struct Metric {
    pub operation: Operation,
    pub successes: u64,
    pub errors: u64,
    pub successful_ops_per_second: f64,
    pub p99_ms: f64,
    pub error_rate: f64,
    pub threshold: Option<Threshold>,
    pub failures: Vec<String>,
}

pub(super) fn evaluate(
    operation: Operation,
    summary: Option<&Summary>,
    seconds: f64,
    threshold: Option<&Threshold>,
) -> Metric {
    let successes = summary.map_or(0, |s| s.count);
    let errors = summary.map_or(0, |s| s.errors);
    let rate = successes as f64 / seconds;
    let p99 = summary.map_or(0.0, |s| s.p99.as_secs_f64() * 1000.0);
    let error_rate = if successes + errors == 0 {
        1.0
    } else {
        errors as f64 / (successes + errors) as f64
    };
    let mut failures = Vec::new();
    if successes == 0 {
        failures.push("no successful samples".into());
    }
    if let Some(t) = threshold {
        if rate < t.min_successful_ops_per_second {
            failures.push("successful throughput below minimum".into());
        }
        if p99 > t.max_p99_ms {
            failures.push("p99 latency above maximum".into());
        }
        if error_rate > t.max_error_rate {
            failures.push("error rate above maximum".into());
        }
    }
    Metric {
        operation,
        successes,
        errors,
        successful_ops_per_second: rate,
        p99_ms: p99,
        error_rate,
        threshold: threshold.cloned(),
        failures,
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate, Baseline, Operation, Profile, Threshold};
    use crate::stats::Stats;
    use std::time::Duration;

    fn profile() -> Profile {
        serde_json::from_str(include_str!("../../../eng/pipelines/perf-profile.json"))
            .expect("profile fixture")
    }

    #[test]
    fn equality_passes_and_each_violation_fails() {
        let stats = Stats::new(&["ReadItem"]);
        for _ in 0..9 {
            stats.record_latency("ReadItem", Duration::from_millis(10), None);
        }
        stats.record_error("ReadItem");
        let summaries = stats.drain_summaries();
        let threshold = Threshold {
            operation: Operation::ReadItem,
            min_successful_ops_per_second: 0.9,
            max_p99_ms: summaries[0].p99.as_secs_f64() * 1000.0,
            max_error_rate: 0.1,
        };
        assert!(evaluate(
            Operation::ReadItem,
            summaries.first(),
            10.0,
            Some(&threshold)
        )
        .failures
        .is_empty());
        let strict = Threshold {
            min_successful_ops_per_second: 1.0,
            max_p99_ms: 1.0,
            max_error_rate: 0.0,
            ..threshold
        };
        assert_eq!(
            evaluate(Operation::ReadItem, summaries.first(), 10.0, Some(&strict))
                .failures
                .len(),
            3
        );
        assert!(!evaluate(Operation::ReadItem, None, 10.0, None)
            .failures
            .is_empty());
    }

    #[test]
    fn profiles_and_baselines_reject_missing_and_duplicate_operations() {
        let mut profile = profile();
        profile.validate().unwrap();
        profile.operations[0] = profile.operations[1].clone();
        assert!(profile.validate().is_err());
        let baseline = Baseline {
            version: 1,
            approved: true,
            profile: Some(profile.clone()),
            thresholds: vec![],
        };
        assert!(baseline.validate(&profile).is_err());
    }

    #[test]
    fn baseline_rejects_invalid_values_duplicates_and_profile_drift() {
        let profile = profile();
        let mut baseline = Baseline {
            version: 1,
            approved: true,
            profile: Some(profile.clone()),
            thresholds: Operation::ALL
                .into_iter()
                .map(|operation| Threshold {
                    operation,
                    min_successful_ops_per_second: 1.0,
                    max_p99_ms: 100.0,
                    max_error_rate: 0.01,
                })
                .collect(),
        };
        baseline.validate(&profile).unwrap();
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            baseline.thresholds[0].min_successful_ops_per_second = invalid;
            assert!(baseline.validate(&profile).is_err());
        }
        baseline.thresholds[0].min_successful_ops_per_second = 1.0;
        baseline.thresholds[0].operation = baseline.thresholds[1].operation;
        assert!(baseline.validate(&profile).is_err());
        baseline.thresholds[0].operation = Operation::ReadItem;
        baseline.profile.as_mut().unwrap().payload_bytes += 1;
        assert!(baseline.validate(&profile).is_err());
        assert!(serde_json::from_str::<Threshold>(
            r#"{"operation":"Missing","min_successful_ops_per_second":1,"max_p99_ms":1,"max_error_rate":0}"#
        ).is_err());
    }
}
