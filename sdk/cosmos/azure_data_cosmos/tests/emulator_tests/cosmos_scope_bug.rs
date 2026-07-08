// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end regression test for the cross-partition query **scope bug**: the
//! planner used to ignore the caller's `FeedScope::range(..)` feed range.
//!
//! Before the fix, `plan_fresh` built its request ranges purely from the query
//! plan (`query_plan.query_ranges`) and never intersected them with the
//! operation's scope feed range (`operation.target()`). For a plain `SELECT *`
//! the query plan reports the whole container, so a `FeedScope::range([X, Y))`
//! window was silently dropped and the query did a **full scan** — returning
//! documents outside the requested window.
//!
//! This test seeds one document per partition key, computes each document's
//! effective partition key locally, and issues cross-partition queries scoped
//! to interior EPK windows. It asserts the result set is exactly the windowed
//! subset. Without the planner fix these assertions fail with
//! `SCOPE BUG` (too many documents returned); with the fix — plus the
//! `x-ms-read-key-type: EffectivePartitionKeyRange` header correction that lets
//! the gateway accept the now-emitted interior EPK window — they pass.
//!
//! Run against the emulator:
//!   RUSTFLAGS='--cfg test_category="emulator"' \
//!     cargo test -p azure_data_cosmos --test emulator \
//!     --features "key_auth fault_injection" feed_range_scope

use super::framework;

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::error::Error;

use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::feed::{FeedRange, FeedScope};
use azure_data_cosmos::models::{ContainerProperties, PartitionKeyDefinition};
use azure_data_cosmos::{PartitionKey, Query};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use framework::{TestClient, TestOptions};

const DOC_COUNT: usize = 40;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Doc {
    id: String,
    pk: String,
}

fn pk_definition() -> PartitionKeyDefinition {
    PartitionKeyDefinition::new(vec![Cow::Borrowed("/pk")])
}

/// Runs a feed-range-scoped `SELECT *` and returns the set of returned ids.
async fn drain_ids(
    container: &ContainerClient,
    scope: FeedScope,
) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let mut iter = container
        .query_items::<Doc>(Query::from("SELECT * FROM c"), scope, None)
        .await?;
    let mut got = BTreeSet::new();
    while let Some(doc) = iter.try_next().await? {
        got.insert(doc.id);
    }
    Ok(got)
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn feed_range_scope_restricts_cross_partition_query() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("ScopeBugContainer", pk_definition());
            let container = run_context
                .create_container(db_client, properties, None)
                .await?;

            // Seed DOC_COUNT documents, each on its own partition key.
            let mut docs: Vec<Doc> = Vec::with_capacity(DOC_COUNT);
            for i in 0..DOC_COUNT {
                let id = format!("doc-{i:03}");
                let doc = Doc {
                    id: id.clone(),
                    pk: id.clone(),
                };
                container
                    .create_item(PartitionKey::from(doc.pk.clone()), &id, doc.clone(), None)
                    .await?;
                docs.push(doc);
            }

            // Compute each document's effective partition key locally (same hash
            // the service uses), then sort ascending by EPK.
            let pk_def = pk_definition();
            let mut points: Vec<(String, FeedRange)> = docs
                .iter()
                .map(|d| {
                    let fr = FeedRange::for_partition(PartitionKey::from(d.pk.clone()), &pk_def);
                    (d.id.clone(), fr)
                })
                .collect();
            points.sort_by(|a, b| {
                a.1.min_inclusive()
                    .as_str()
                    .cmp(b.1.min_inclusive().as_str())
            });
            let k = points.len();

            // Control: a full-container scan must return every seeded document.
            let all: BTreeSet<String> = points.iter().map(|(id, _)| id.clone()).collect();
            let control = drain_ids(&container, FeedScope::full_container()).await?;
            assert_eq!(
                control, all,
                "control full-container scan should return all {DOC_COUNT} docs"
            );

            // Test B: a WIDE interior window [X1, X_{k-1}) must exclude the
            // globally smallest and largest EPKs => expect k-2 documents.
            let window_b = FeedRange::new(
                points[1].1.min_inclusive().clone(),
                points[k - 1].1.min_inclusive().clone(),
            )?;
            let expected_b: BTreeSet<String> =
                points[1..k - 1].iter().map(|(id, _)| id.clone()).collect();
            let got_b = drain_ids(&container, FeedScope::range(window_b)).await?;
            assert_eq!(
                got_b,
                expected_b,
                "SCOPE BUG (wide window): FeedScope::range was ignored and the \
                 query fell back to a full scan. got {} docs, expected {}. \
                 unexpected(outside window)={:?}",
                got_b.len(),
                expected_b.len(),
                got_b.difference(&expected_b).collect::<Vec<_>>()
            );

            // Test C: the TIGHTEST interior window [X_mid, X_{mid+1}) must
            // return exactly one document.
            let mid = k / 2;
            let window_c = FeedRange::new(
                points[mid].1.min_inclusive().clone(),
                points[mid + 1].1.min_inclusive().clone(),
            )?;
            let expected_c: BTreeSet<String> = std::iter::once(points[mid].0.clone()).collect();
            let got_c = drain_ids(&container, FeedScope::range(window_c)).await?;
            assert_eq!(
                got_c,
                expected_c,
                "SCOPE BUG (tight window): FeedScope::range was ignored. \
                 got {} docs, expected 1. unexpected(outside window)={:?}",
                got_c.len(),
                got_c.difference(&expected_c).collect::<Vec<_>>()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}
