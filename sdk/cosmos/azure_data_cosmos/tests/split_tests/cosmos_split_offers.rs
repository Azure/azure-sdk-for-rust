// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

use super::framework;

use std::error::Error;
use std::time::Duration;

use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use framework::{TestClient, TestOptions};
use futures::TryStreamExt;

use azure_data_cosmos::CreateContainerOptions;

#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn container_throughput_stream_polling() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("StreamPollContainer", "/id".into());

            let throughput = ThroughputProperties::manual(400);

            let container_client = run_context
                .create_container(
                    db_client,
                    properties.clone(),
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            // Use a high RU value (11000) to trigger async processing, which may take ~5 minutes
            let new_throughput = ThroughputProperties::manual(11000);
            let mut poller = container_client
                .begin_replace_throughput(new_throughput, None)
                .await?;

            let mut count = 0;
            let mut last_throughput = None;
            while let Some(status) = poller.try_next().await? {
                count += 1;
                assert!(status.status().is_success());
                last_throughput = Some(status.into_model()?);
            }

            assert!(count >= 1, "stream should yield at least one response");
            let final_throughput =
                last_throughput.expect("stream should have yielded a throughput response");
            assert_eq!(Some(11000), final_throughput.throughput());

            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(1800))),
    )
    .await
}
