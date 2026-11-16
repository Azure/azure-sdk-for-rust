// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Deterministic documents and expected query results for live ORDER BY checks.

use super::{require, TestResult};
use crate::framework::seeded_json::SplitMix64;
use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub(super) const FIXTURE_SEED: u64 = 0x5283_5409;

#[derive(Clone, SafeDebug, Serialize, Deserialize, PartialEq)]
pub(super) struct LiveItem {
    pub id: String,
    pub pk: String,
    pub rank: i64,
    pub bucket: i64,
    pub label: String,
    pub payload: String,
    pub number: f64,
    pub nested: Value,
    pub sort_key: String,
}

pub(super) struct QueryCase {
    pub name: &'static str,
    pub query: &'static str,
    pub expected: Vec<Value>,
    pub tied: bool,
    pub resume: bool,
    pub whole_document: bool,
}

impl QueryCase {
    fn documents(name: &'static str, query: &'static str, items: &[LiveItem]) -> Self {
        Self {
            name,
            query,
            expected: values(items),
            tied: false,
            resume: true,
            whole_document: true,
        }
    }

    fn without_resume(mut self) -> Self {
        self.resume = false;
        self
    }
}

pub(super) fn items_with_count(count: i64) -> Vec<LiveItem> {
    let mut rng = SplitMix64::new(FIXTURE_SEED);
    (0..count)
        .map(|rank| LiveItem {
            id: format!("item-{rank:04}"),
            pk: format!("pk-{}", rank % 64),
            rank,
            bucket: rank % 7,
            label: "repeated label with quotes \" and backslash \\".to_owned(),
            payload: "repeated-string-".repeat(if rank % 16 == 0 { 2048 } else { 128 }),
            number: 9_000_000_000.0 + rank as f64 / 4.0,
            sort_key: format!("{rank:04}-quote \" slash \\ \u{e9}\u{2603}\u{1d11e}"),
            nested: json!({
                "same": ["repeated label", "repeated label", ""],
                "active": rank % 2 == 0,
                "optional": null,
                "unicode": "\u{e9}\u{2603}\u{1d11e}",
                "numbers": [0, 31, 32, 255, 256, -1],
                "generated": rng.document(),
            }),
        })
        .collect()
}

pub(super) fn values(items: &[LiveItem]) -> Vec<Value> {
    items
        .iter()
        .map(|item| serde_json::to_value(item).unwrap())
        .collect()
}

pub(super) fn cases(items: &[LiveItem]) -> Vec<QueryCase> {
    let mut descending = items.to_vec();
    descending.reverse();
    let mut composite = items.to_vec();
    composite.sort_by_key(|item| (item.bucket, std::cmp::Reverse(item.rank)));
    let mut tied = items.to_vec();
    tied.sort_by_key(|item| (item.bucket, item.rank));
    let mut cases = vec![
        QueryCase::documents("ascending", "SELECT * FROM c ORDER BY c.rank ASC", items),
        QueryCase::documents(
            "descending",
            "SELECT * FROM c ORDER BY c.rank DESC",
            &descending,
        ),
        QueryCase::documents(
            "top",
            "SELECT TOP 11 * FROM c ORDER BY c.rank ASC",
            &items[..11],
        ),
        QueryCase::documents(
            "offset-limit",
            "SELECT * FROM c ORDER BY c.rank ASC OFFSET 17 LIMIT 23",
            &items[17..40],
        ),
        QueryCase::documents(
            "descending-offset-limit",
            "SELECT * FROM c ORDER BY c.rank DESC OFFSET 9 LIMIT 13",
            &descending[9..22],
        ),
        QueryCase::documents(
            "projection",
            "SELECT c.id, c.pk, c.rank, c.bucket, c.label, c.payload, c.number, c.nested, c.sort_key FROM c ORDER BY c.rank",
            items,
        )
        .without_resume(),
        QueryCase::documents("value-projection", "SELECT VALUE c FROM c ORDER BY c.rank", items)
            .without_resume(),
        QueryCase::documents(
            "filtered",
            "SELECT * FROM c WHERE c.rank >= 80 ORDER BY c.rank",
            &items[80..],
        )
        .without_resume(),
        QueryCase::documents(
            "mixed-direction-composite",
            "SELECT * FROM c ORDER BY c.bucket ASC, c.rank DESC",
            &composite,
        ),
        QueryCase {
            tied: true,
            ..QueryCase::documents("tied-keys", "SELECT * FROM c ORDER BY c.bucket", &tied)
        },
        QueryCase::documents(
            "ordered-distinct",
            "SELECT DISTINCT TOP 11 * FROM c ORDER BY c.rank",
            &items[..11],
        )
        .without_resume(),
        QueryCase::documents("empty", "SELECT * FROM c WHERE c.rank < 0 ORDER BY c.rank", &[])
            .without_resume(),
        QueryCase::documents(
            "offset-beyond-end",
            "SELECT * FROM c ORDER BY c.rank OFFSET 200 LIMIT 5",
            &[],
        )
        .without_resume(),
    ];
    for (name, query, expected) in [
        (
            "scalar-number",
            "SELECT VALUE c.rank FROM c ORDER BY c.rank",
            items.iter().map(|item| json!(item.rank)).collect(),
        ),
        (
            "scalar-string",
            "SELECT VALUE c.sort_key FROM c ORDER BY c.sort_key",
            items.iter().map(|item| json!(item.sort_key)).collect(),
        ),
        (
            "scalar-boolean",
            "SELECT VALUE c.nested.active FROM c ORDER BY c.rank",
            items.iter().map(|item| json!(item.rank % 2 == 0)).collect(),
        ),
        (
            "scalar-null",
            "SELECT VALUE c.nested.optional FROM c ORDER BY c.rank",
            vec![Value::Null; items.len()],
        ),
        (
            "array-projection",
            "SELECT VALUE c.nested.same FROM c ORDER BY c.rank",
            items
                .iter()
                .map(|item| item.nested["same"].clone())
                .collect(),
        ),
        (
            "generated-projection",
            "SELECT VALUE c.nested.generated FROM c ORDER BY c.rank",
            items
                .iter()
                .map(|item| item.nested["generated"].clone())
                .collect(),
        ),
        (
            "distinct-buckets",
            "SELECT DISTINCT VALUE c.bucket FROM c ORDER BY c.bucket",
            (0..7).map(|bucket| json!(bucket)).collect(),
        ),
        (
            "distinct-window",
            "SELECT DISTINCT VALUE c.bucket FROM c ORDER BY c.bucket OFFSET 2 LIMIT 3",
            (2..5).map(|bucket| json!(bucket)).collect(),
        ),
    ] {
        cases.push(QueryCase {
            name,
            query,
            expected,
            tied: false,
            resume: true,
            whole_document: false,
        });
    }
    cases
}

pub(super) fn compare(mut actual: Vec<Value>, case: &QueryCase, source: &str) -> TestResult {
    if case.tied {
        require(
            actual
                .windows(2)
                .all(|pair| pair[0]["bucket"].as_i64() <= pair[1]["bucket"].as_i64()),
            format!("{source}/{}: tied keys are not globally ordered", case.name),
        )?;
        // Separate containers have different RIDs, so equal-key order can differ.
        actual.sort_by_key(|item| (item["bucket"].as_i64(), item["rank"].as_i64()));
    }
    require(
        actual == case.expected,
        format!(
            "{source}/{}: item contents, ordering, or count differ",
            case.name
        ),
    )
}
