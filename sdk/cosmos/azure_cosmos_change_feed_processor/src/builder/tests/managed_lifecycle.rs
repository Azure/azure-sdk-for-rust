// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{account_binding, Credential};
use crate::{ChangeFeedProcessor, ManagedProcessorOptions, ProcessorLifecycleState};
use azure_data_cosmos_driver::{
    models::{CosmosOperation, ItemReference, PartitionKey},
    options::OperationOptions,
};
use serde_json::{json, Value};
use std::{
    error::Error,
    num::NonZeroU32,
    sync::{Arc, Mutex},
    task::Poll,
    time::Duration,
};
use tokio::{sync::oneshot, time::timeout};

#[tokio::test]
async fn public_lifecycle_retains_cancelled_stop_and_reuses_prepared_resources(
) -> Result<(), Box<dyn Error>> {
    timeout(
        Duration::from_secs(15),
        Box::pin(async {
            let (feed, feed_traffic) = account_binding(
                "https://feed.emulator.local",
                Credential::new("feed"),
                vec![],
            )?;
            let (leases, lease_traffic) = account_binding(
                "https://leases.emulator.local",
                Credential::new("leases"),
                vec![],
            )?;
            let writer = feed.clone().prepare().await?;
            writer
                .driver
                .execute_singleton_operation(
                    CosmosOperation::create_item(ItemReference::from_name(
                        &writer.container,
                        PartitionKey::from("doc"),
                        "doc",
                    ))
                    .with_body(serde_json::to_vec(&json!({"id":"doc","workload":"doc"}))?),
                    OperationOptions::default(),
                )
                .await?;
            let processor = ChangeFeedProcessor::builder()
                .build("projection", feed, leases)
                .await?;
            let initial_metadata = (
                feed_traffic.metadata_reads(),
                lease_traffic.metadata_reads(),
            );
            let options = ManagedProcessorOptions::new("host")
                .with_host_capacity(NonZeroU32::new(2).unwrap())
                .with_callback_concurrency(NonZeroU32::new(1).unwrap())
                .with_balance_interval(Duration::from_millis(10));
            let (entered, arrival) = oneshot::channel();
            let (resume, released) = oneshot::channel();
            let entered = Arc::new(Mutex::new(Some(entered)));
            let released = Arc::new(Mutex::new(Some(released)));
            processor
                .start::<Value, _, _>(options.clone(), move |page| {
                    assert_eq!(page.items().len(), 1);
                    let entered = entered.lock().unwrap().take().unwrap();
                    let released = released.lock().unwrap().take().unwrap();
                    async move {
                        entered.send(()).unwrap();
                        released.await.unwrap();
                        Ok(())
                    }
                })
                .await?;
            arrival.await?;
            assert!(processor
                .start::<Value, _, _>(options, |_| async { Ok(()) })
                .await
                .is_err());
            let mut stopping = Box::pin(processor.stop());
            assert!(matches!(futures::poll!(stopping.as_mut()), Poll::Pending));
            drop(stopping);
            assert!(matches!(
                processor.state().await,
                ProcessorLifecycleState::Stopping(_)
            ));
            resume.send(()).unwrap();
            let report = processor.stop().await.unwrap();
            assert!(
                report.is_clean(),
                "all tasks and releases must be confirmed"
            );
            assert_eq!(report.leases().len(), 1);
            assert!(matches!(
                processor.state().await,
                ProcessorLifecycleState::Stopped(_)
            ));
            assert!(Arc::ptr_eq(&report, &processor.stop().await.unwrap()));
            assert_eq!(
                (
                    feed_traffic.metadata_reads(),
                    lease_traffic.metadata_reads()
                ),
                initial_metadata
            );
            Ok::<_, Box<dyn Error>>(())
        }),
    )
    .await??;
    Ok(())
}
