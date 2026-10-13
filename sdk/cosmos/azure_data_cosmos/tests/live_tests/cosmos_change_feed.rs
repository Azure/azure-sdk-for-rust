// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::framework::{self, TestClient, TestOptions};
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::{ChangeFeedPageIterator, ContinuationToken, FeedScope},
    models::{ChangeFeedItem, ContainerProperties},
    options::{ChangeFeedMode, ChangeFeedOptions, ChangeFeedStartFrom, MaxItemCountHint},
    PartitionKey,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, error::Error, num::NonZeroU32, time::Duration};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Item {
    id: String,
    tenant: String,
    user: String,
    device: String,
}

impl Item {
    fn partition_key(&self) -> PartitionKey {
        (&self.tenant, &self.user, &self.device).into()
    }
}

fn require_live_account() -> TestResult {
    if framework::resolve_connection_string().is_none() || framework::targets_emulator() {
        return Err("change-feed live controls require a real account connection string".into());
    }
    if std::env::var(framework::test_client::TEST_MODE_ENV_VAR)
        .is_ok_and(|mode| mode.eq_ignore_ascii_case("skipped"))
    {
        return Err("explicitly selected live controls cannot be skipped".into());
    }
    Ok(())
}

fn options(token: Option<ContinuationToken>) -> ChangeFeedOptions {
    let options = ChangeFeedOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()));
    match token {
        Some(token) => options.with_continuation_token(token),
        None => options,
    }
}

async fn collect_exact(
    pages: &mut ChangeFeedPageIterator<ChangeFeedItem<Item>>,
    expected: &[&str],
    physical_ranges: usize,
) -> TestResult {
    collect_exact_with_resumes(pages, expected, physical_ranges, None).await
}

async fn collect_exact_with_resumes(
    pages: &mut ChangeFeedPageIterator<ChangeFeedItem<Item>>,
    expected: &[&str],
    physical_ranges: usize,
    resume: Option<(&ContainerClient, &FeedScope)>,
) -> TestResult {
    let mut ids = Vec::new();
    let mut idle = 0;
    let mut transports = BTreeSet::new();
    for _ in 0..100 {
        let page = pages
            .next()
            .await
            .ok_or("change feed unexpectedly ended")??;
        for request in page.diagnostics().requests().iter() {
            transports.insert(format!("{:?}", request.transport_kind()));
        }
        if page.items().is_empty() {
            idle += 1;
        } else {
            idle = 0;
            for item in page.items() {
                ids.push(
                    item.current()
                        .ok_or("create envelope has no current item")?
                        .id
                        .clone(),
                );
            }
        }
        if idle >= physical_ranges * 2 {
            ids.sort();
            let mut expected: Vec<_> = expected.iter().map(|id| (*id).to_owned()).collect();
            expected.sort();
            if ids != expected {
                return Err(format!(
                    "change-feed membership: expected {expected:?}, received {ids:?}"
                )
                .into());
            }
            println!("change-feed transports: {transports:?}");
            return Ok(());
        }
        if let Some((container, scope)) = resume {
            *pages = container
                .query_change_feed::<Item>(
                    scope.clone(),
                    ChangeFeedStartFrom::Now,
                    Some(options(Some(pages.to_continuation_token()?))),
                )
                .await?;
        }
    }
    Err("change feed did not reach the expected bounded idle state".into())
}

async fn create_seed_item(container: &ContainerClient, item: &Item) -> TestResult {
    for attempt in 0..60 {
        match container.create_item(item.partition_key(), &item.id, item, None).await {
            Ok(_) => return Ok(()),
            Err(error) if attempt < 59
                && error.status().status_code() == azure_core::http::StatusCode::NotFound
                && error.status().sub_status() == Some(
                    azure_data_cosmos_driver::error::status_codes::substatus::COLLECTION_CREATE_IN_PROGRESS,
                ) => {
                    println!("waiting for seed container data-plane readiness (404/1013)");
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            Err(error) => return Err(error.into()),
        }
    }
    unreachable!("the last attempt returns its result")
}

#[tokio::test]
#[cfg_attr(not(test_category = "live"), ignore = "requires live account")]
async fn checkpoint_before_first_poll_ignores_conflicting_start() -> TestResult {
    require_live_account()?;
    for classic_gateway in [true, false] {
        TestClient::run_with_unique_db(
            async |context, database| {
                let container = context
                    .create_container(
                        database,
                        ContainerProperties::new(
                            "ChangeFeedCheckpoint",
                            ("/tenant", "/user", "/device").into(),
                        ),
                        None,
                    )
                    .await
                    .inspect_err(|_| eprintln!("checkpoint fixture container setup failed"))?;
                let item = Item {
                    id: "history".into(),
                    tenant: "A".into(),
                    user: "B".into(),
                    device: "C".into(),
                };
                create_seed_item(&container, &item)
                    .await
                    .inspect_err(|_| eprintln!("checkpoint fixture seed setup failed"))?;
                let ranges = container
                    .read_feed_ranges(None)
                    .await
                    .inspect_err(|_| eprintln!("checkpoint fixture topology setup failed"))?
                    .len();
                for scope in [
                    FeedScope::partition(item.partition_key()),
                    FeedScope::full_container(),
                ] {
                    for start in [
                        ChangeFeedStartFrom::Beginning,
                        ChangeFeedStartFrom::PointInTime(
                            time::OffsetDateTime::now_utc() - time::Duration::minutes(1),
                        ),
                    ] {
                        let feed = container
                            .query_change_feed::<Item>(scope.clone(), start, Some(options(None)))
                            .await
                            .inspect_err(|_| eprintln!("unpolled checkpoint planning failed"))?;
                        let token = feed.to_continuation_token()?;
                        drop(feed);
                        let mut resumed = container
                            .query_change_feed::<Item>(
                                scope.clone(),
                                ChangeFeedStartFrom::Now,
                                Some(options(Some(token))),
                            )
                            .await
                            .inspect_err(|_| eprintln!("checkpoint resume planning failed"))?;
                        collect_exact(&mut resumed, &["history"], ranges)
                            .await
                            .inspect_err(|_| eprintln!("checkpoint resume drain failed"))?;
                    }
                }
                Ok(())
            },
            Some(
                TestOptions::new()
                    .with_gateway_v2_disabled(classic_gateway)
                    .with_timeout(Duration::from_secs(180)),
            ),
        )
        .await?;
    }
    Ok(())
}

async fn seed(container: &ContainerClient) -> TestResult {
    for (id, tenant, user, device) in [
        ("a1", "A", "one", "x"),
        ("a2", "A", "one", "y"),
        ("a3", "A", "two", "z"),
        ("outside", "B", "one", "x"),
    ] {
        let item = Item {
            id: id.into(),
            tenant: tenant.into(),
            user: user.into(),
            device: device.into(),
        };
        create_seed_item(container, &item).await?;
    }
    Ok(())
}

#[tokio::test]
#[cfg_attr(not(test_category = "live"), ignore = "requires live account")]
async fn hierarchical_prefix_filters_and_resumes_on_one_range() -> TestResult {
    require_live_account()?;
    for classic_gateway in [true, false] {
        TestClient::run_with_unique_db(
            async |context, database| {
                let container = context
                    .create_container(
                        database,
                        ContainerProperties::new(
                            "ChangeFeedPrefix",
                            ("/tenant", "/user", "/device").into(),
                        ),
                        None,
                    )
                    .await?;
                let ranges = container.read_feed_ranges(None).await?.len();
                if ranges != 1 {
                    return Err(format!(
                        "single-range prefix control requires one physical range, found {ranges}"
                    )
                    .into());
                }
                seed(&container).await?;
                for (key, expected) in [
                    (PartitionKey::from("A"), vec!["a1", "a2", "a3"]),
                    (PartitionKey::from(("A", "one")), vec!["a1", "a2"]),
                ] {
                    let scope = FeedScope::partition(key);
                    let feed = container
                        .query_change_feed::<Item>(
                            scope.clone(),
                            ChangeFeedStartFrom::Beginning,
                            Some(options(None)),
                        )
                        .await?;
                    let token = feed.to_continuation_token()?;
                    drop(feed);
                    let mut resumed = container
                        .query_change_feed::<Item>(
                            scope.clone(),
                            ChangeFeedStartFrom::Now,
                            Some(options(Some(token))),
                        )
                        .await?;
                    collect_exact_with_resumes(
                        &mut resumed,
                        &expected,
                        ranges,
                        Some((&container, &scope)),
                    )
                    .await?;
                    let token = resumed.to_continuation_token()?;
                    drop(resumed);
                    let mut idle = container
                        .query_change_feed::<Item>(
                            scope,
                            ChangeFeedStartFrom::Beginning,
                            Some(options(Some(token))),
                        )
                        .await?;
                    collect_exact(&mut idle, &[], ranges).await?;
                }
                Ok(())
            },
            Some(
                TestOptions::new()
                    .with_gateway_v2_disabled(classic_gateway)
                    .with_timeout(Duration::from_secs(180)),
            ),
        )
        .await?;
    }
    Ok(())
}

#[tokio::test]
#[ignore = "requires a live account with AllVersionsAndDeletes already enabled"]
async fn all_versions_and_deletes_preserves_checkpoint_start_and_mode() -> TestResult {
    require_live_account()?;
    TestClient::run_with_unique_db(
        async |context, database| {
            let container = context
                .create_container(
                    database,
                    ContainerProperties::new(
                        "ChangeFeedFullFidelity",
                        ("/tenant", "/user", "/device").into(),
                    ),
                    None,
                )
                .await?;
            seed(&container).await?;
            let ranges = container.read_feed_ranges(None).await?.len();
            for scope in [
                FeedScope::partition(("A", "one", "x")),
                FeedScope::partition("A"),
                FeedScope::full_container(),
            ] {
                let feed = container
                    .query_change_feed::<Item>(
                        scope.clone(),
                        ChangeFeedStartFrom::Now,
                        Some(options(None).with_mode(ChangeFeedMode::AllVersionsAndDeletes)),
                    )
                    .await?;
                let token = feed.to_continuation_token()?;
                drop(feed);
                match container
                    .query_change_feed::<Item>(
                        scope.clone(),
                        ChangeFeedStartFrom::Now,
                        Some(options(Some(token.clone()))),
                    )
                    .await
                {
                    Err(error) if error.to_string().contains("mode") => {}
                    Err(error) => return Err(error.into()),
                    Ok(_) => return Err("change-feed checkpoint accepted a different mode".into()),
                }
                let mut pages = container
                    .query_change_feed::<Item>(
                        scope.clone(),
                        ChangeFeedStartFrom::Beginning,
                        Some(options(Some(token)).with_mode(ChangeFeedMode::AllVersionsAndDeletes)),
                    )
                    .await?;
                collect_exact(&mut pages, &[], ranges).await?;
                let token = pages.to_continuation_token()?;
                drop(pages);
                let mut pages = container
                    .query_change_feed::<Item>(
                        scope,
                        ChangeFeedStartFrom::Beginning,
                        Some(options(Some(token)).with_mode(ChangeFeedMode::AllVersionsAndDeletes)),
                    )
                    .await?;
                collect_exact(&mut pages, &[], ranges).await?;
            }
            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(180))),
    )
    .await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(not(test_category = "live"), ignore = "requires live account")]
async fn now_checkpoint_keeps_idle_etags_and_reads_later_writes() -> TestResult {
    require_live_account()?;
    TestClient::run_with_unique_db(
        async |context, database| {
            let container = context
                .create_container(
                    database,
                    ContainerProperties::new(
                        "ChangeFeedNow",
                        ("/tenant", "/user", "/device").into(),
                    ),
                    None,
                )
                .await?;
            seed(&container).await?;
            let ranges = container.read_feed_ranges(None).await?.len();
            for (index, scope) in [
                FeedScope::partition(("A", "one", "x")),
                FeedScope::partition("A"),
                FeedScope::partition(("A", "one")),
                FeedScope::full_container(),
            ]
            .into_iter()
            .enumerate()
            {
                let feed = container
                    .query_change_feed::<Item>(
                        scope.clone(),
                        ChangeFeedStartFrom::Now,
                        Some(options(None)),
                    )
                    .await?;
                let token = feed.to_continuation_token()?;
                drop(feed);
                let mut pages = container
                    .query_change_feed::<Item>(
                        scope.clone(),
                        ChangeFeedStartFrom::Beginning,
                        Some(options(Some(token))),
                    )
                    .await?;
                collect_exact(&mut pages, &[], ranges).await?;
                let token = pages.to_continuation_token()?;
                drop(pages);
                let item = Item {
                    id: format!("after-idle-{index}"),
                    tenant: "A".into(),
                    user: "one".into(),
                    device: "x".into(),
                };
                container
                    .create_item(item.partition_key(), &item.id, &item, None)
                    .await?;
                let mut pages = container
                    .query_change_feed::<Item>(
                        scope.clone(),
                        ChangeFeedStartFrom::Beginning,
                        Some(options(Some(token))),
                    )
                    .await?;
                collect_exact_with_resumes(
                    &mut pages,
                    &[&item.id],
                    ranges,
                    Some((&container, &scope)),
                )
                .await?;
                let token = pages.to_continuation_token()?;
                drop(pages);
                let mut pages = container
                    .query_change_feed::<Item>(
                        scope,
                        ChangeFeedStartFrom::Beginning,
                        Some(options(Some(token))),
                    )
                    .await?;
                collect_exact(&mut pages, &[], ranges).await?;
            }
            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(180))),
    )
    .await?;
    Ok(())
}
