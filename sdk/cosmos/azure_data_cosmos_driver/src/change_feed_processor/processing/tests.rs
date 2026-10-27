// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    balancing, bootstrap, namespace, now_millis, Batch, BootstrapOptions, ChangeFeedProcessorMode,
    Context, Identity, Lease, ProcessingLease, Processor, ProcessorOptions, Update, SCHEMA_VERSION,
};
use crate::{
    change_feed_processor::bootstrap::Record,
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, ChangeFeedStartFrom, CosmosOperation, FeedRange, ItemReference,
        MaxItemCountHint, PartitionKey,
    },
    options::{DriverOptions, OperationOptions},
    Result,
};
use std::{num::NonZeroU32, sync::Arc, time::Duration};

fn options(worker: &str) -> ProcessorOptions {
    let mut options = ProcessorOptions::new(BootstrapOptions::new(
        ChangeFeedProcessorMode::LatestVersion,
        "workload",
        worker,
    ))
    .with_start(ChangeFeedStartFrom::Beginning);
    options.acquire_interval = Duration::from_millis(20);
    options.poll_delay = Duration::from_millis(10);
    options
}

async fn fixture() -> Result<Arc<Context>> {
    let endpoint = "https://eastus.emulator.local";
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(
        vec![VirtualRegion::new("East US", endpoint.parse()?)],
    )?));
    let store = emulator.store();
    store.create_database("polling");
    let pk =
        serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))?;
    store.create_container_with_config(
        "polling",
        "source",
        pk,
        ContainerConfig::default().with_partition_count(3),
    );
    store.create_container_with_properties_internal(
        "polling",
        "leases",
        serde_json::from_value(serde_json::json!({"paths":["/id"],"kind":"Hash","version":2}))?,
        ContainerConfig::default(),
        serde_json::from_value(serde_json::json!({"defaultTtl":-1}))?,
    );
    let driver = emulator
        .runtime_builder()
        .build()
        .await?
        .create_driver(
            DriverOptions::builder(AccountReference::with_account_key(
                endpoint.parse()?,
                "dGVzdGtleQ==",
            ))
            .build(),
        )
        .await?;
    let source = driver
        .resolve_container("polling", "source", OperationOptions::default())
        .await?;
    let leases = driver
        .resolve_container("polling", "leases", OperationOptions::default())
        .await?;
    let options = options("first");
    bootstrap(&driver, &source, &driver, &leases, &options.bootstrap).await?;
    let identity = Identity {
        version: SCHEMA_VERSION,
        account: source.account().endpoint().origin().ascii_serialization(),
        container_rid: source.rid().into(),
        processor_name: "workload".into(),
        mode: ChangeFeedProcessorMode::LatestVersion,
    };
    Ok(Arc::new(Context {
        source_driver: driver.clone(),
        source,
        lease_driver: driver,
        leases,
        namespace: namespace(&identity)?,
        identity,
        options,
    }))
}

async fn start(context: &Context, options: ProcessorOptions) -> Result<Processor> {
    Processor::start(
        context.source_driver.clone(),
        context.source.clone(),
        context.lease_driver.clone(),
        context.leases.clone(),
        options,
    )
    .await
}

async fn insert(context: &Context, id: &str) -> Result<()> {
    context
        .source_driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &context.source,
                PartitionKey::from("same"),
                id.to_owned(),
            ))
            .with_body(serde_json::to_vec(
                &serde_json::json!({"id":id,"pk":"same"}),
            )?),
            OperationOptions::default(),
        )
        .await?;
    Ok(())
}

async fn next(processor: &Processor) -> Batch {
    tokio::time::timeout(Duration::from_secs(5), processor.next_batch())
        .await
        .expect("batch timeout")
        .expect("processor stopped")
}

fn leases(owners: &[Option<&str>]) -> Vec<Lease> {
    owners
        .iter()
        .enumerate()
        .map(|(index, owner)| Lease {
            base: ProcessingLease {
                record: Record {
                    id: index.to_string(),
                    workload: Identity {
                        version: 1,
                        account: "account".into(),
                        container_rid: "rid".into(),
                        processor_name: "workload".into(),
                        mode: ChangeFeedProcessorMode::LatestVersion,
                    },
                    ttl: -1,
                },
                range: FeedRange::full(),
                owner: owner.map(str::to_owned),
                continuation: None,
            },
            incarnation: Some(index.to_string()),
            renewed_at: Some(1000),
        })
        .collect()
}

#[test]
fn equal_balancing_scale_out_down_and_limits() {
    let all = leases(&[Some("a"), Some("a"), Some("a"), Some("a")]);
    let selected = balancing::select(&all, &options("b"), 1000);
    assert_eq!(selected.len(), 1);
    assert_eq!(all[selected[0]].base.owner.as_deref(), Some("a"));
    let balanced = leases(&[Some("a"), Some("a"), Some("b"), Some("b")]);
    assert!(balancing::select(&balanced, &options("a"), 1000).is_empty());
    let mut expired = balanced;
    expired[2].renewed_at = Some(-100_000);
    expired[3].renewed_at = Some(-100_000);
    assert_eq!(balancing::select(&expired, &options("a"), 1000).len(), 2);
    assert!(balancing::select(&expired, &options("a").with_lease_counts(0, 2), 1000).is_empty());
    assert_eq!(
        balancing::select(&leases(&[None, None, None]), &options("a"), 1000).len(),
        3
    );
}

#[test]
fn donor_threshold_does_not_oscillate_uneven_distribution() {
    let all = leases(&[Some("a"), Some("a"), Some("a"), Some("b"), Some("b")]);
    assert!(balancing::select(&all, &options("b"), 1000).is_empty());
    assert_eq!(balancing::select(&all, &options("c"), 1000).len(), 1);
}

#[tokio::test]
async fn workers_converge_on_scale_out_and_reclaim_on_scale_down() -> Result<()> {
    let context = fixture().await?;
    let mut first_options = options("first");
    first_options.renew_interval = Duration::from_millis(20);
    let first = start(&context, first_options).await?;
    let wait_counts = |first_count, second_count| {
        let context = context.clone();
        async move {
            tokio::time::timeout(Duration::from_secs(5), async {
                loop {
                    let leases = context.list().await.unwrap();
                    let first = leases
                        .iter()
                        .filter(|lease| lease.base.owner.as_deref() == Some("first"))
                        .count();
                    let second = leases
                        .iter()
                        .filter(|lease| lease.base.owner.as_deref() == Some("second"))
                        .count();
                    if (first, second) == (first_count, second_count) {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .unwrap();
        }
    };
    wait_counts(3, 0).await;
    let mut second_options = options("second");
    second_options.renew_interval = Duration::from_millis(20);
    let second = start(&context, second_options).await?;
    wait_counts(2, 1).await;
    second.stop().await;
    wait_counts(3, 0).await;
    first.stop().await;
    Ok(())
}

#[tokio::test]
async fn stealing_fences_old_renewal_and_checkpoint() -> Result<()> {
    let context = fixture().await?;
    let lease = context.list().await?.remove(0);
    let mut first = context.acquire(&lease).await?;
    let observed = first.value.clone();
    let mut second_context = Context {
        source_driver: context.source_driver.clone(),
        source: context.source.clone(),
        lease_driver: context.lease_driver.clone(),
        leases: context.leases.clone(),
        identity: context.identity.clone(),
        namespace: context.namespace.clone(),
        options: options("second"),
    };
    let second = second_context.acquire(&observed).await?;
    assert!(context
        .update(&mut first, Update::Renew)
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert!(context
        .update(&mut first, Update::Release)
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    // Reacquiring under the old instance name must not revive the old receipt.
    second_context.options.bootstrap.instance_name = "first".into();
    let third = second_context.acquire(&second.value).await?;
    assert_ne!(first.value.incarnation, third.value.incarnation);
    assert!(context
        .update(&mut first, Update::Renew)
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    Ok(())
}

#[tokio::test]
async fn callback_receipt_gates_next_page_and_saved_progress() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    insert(&context, "two").await?;
    let opts =
        options("first").with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()));
    let processor = start(&context, opts).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    let stored = context.store().read::<Lease>(&id).await?.unwrap();
    assert!(stored.value.base.continuation.is_none());
    assert!(
        tokio::time::timeout(Duration::from_millis(60), processor.next_batch())
            .await
            .is_err()
    );
    receipt.checkpoint().await?;
    let saved = context
        .store()
        .read::<Lease>(&id)
        .await?
        .unwrap()
        .value
        .base
        .continuation
        .unwrap();
    let (second_id, _, second_receipt) = next(&processor).await.into_parts();
    assert_eq!(id, second_id);
    assert_eq!(
        context
            .store()
            .read::<Lease>(&id)
            .await?
            .unwrap()
            .value
            .base
            .continuation
            .unwrap()
            .as_str(),
        saved.as_str()
    );
    drop(second_receipt);
    processor.stop().await;
    assert!(!processor.is_running());
    let restarted = start(
        &context,
        options("first").with_start(ChangeFeedStartFrom::Now),
    )
    .await?;
    let (resumed_id, _, resumed_receipt) = next(&restarted).await.into_parts();
    assert_eq!(resumed_id, id);
    resumed_receipt.checkpoint().await?;
    restarted.stop().await;
    Ok(())
}

#[tokio::test]
async fn watchdog_cancels_receipt_without_saving_progress() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "one").await?;
    let mut opts = options("first");
    opts.renew_interval = Duration::from_millis(10);
    opts.acquire_interval = Duration::from_secs(2);
    let processor = start(&context, opts).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    tokio::time::timeout(Duration::from_secs(2), receipt.cancelled())
        .await
        .unwrap();
    assert!(receipt
        .checkpoint()
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert!(context
        .store()
        .read::<Lease>(&id)
        .await?
        .unwrap()
        .value
        .base
        .continuation
        .is_none());
    processor.stop().await;
    Ok(())
}

#[tokio::test]
async fn now_start_pins_idle_position_before_new_changes() -> Result<()> {
    let context = fixture().await?;
    insert(&context, "before").await?;
    let processor = start(
        &context,
        options("first").with_start(ChangeFeedStartFrom::Now),
    )
    .await?;
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let all = context.list().await.unwrap();
            if all.iter().all(|lease| lease.base.continuation.is_some()) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(30), processor.next_batch())
            .await
            .is_err()
    );
    insert(&context, "after").await?;
    let (_, response, receipt) = next(&processor).await.into_parts();
    let envelope: serde_json::Value = response.into_body().into_single()?;
    let items = envelope["Documents"]
        .as_array()
        .expect("change feed envelope");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["current"]["id"], "after");
    receipt.checkpoint().await?;
    processor.stop().await;
    assert!(context
        .list()
        .await?
        .iter()
        .all(|lease| lease.base.owner.is_none()));
    assert!(now_millis() > 0);
    Ok(())
}
