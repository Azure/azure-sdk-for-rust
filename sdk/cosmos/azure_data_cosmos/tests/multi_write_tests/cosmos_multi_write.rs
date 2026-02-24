// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

use super::framework;

use framework::TestOptions;
use std::borrow::Cow;
use std::error::Error;
use std::sync::{Arc, Mutex};

use azure_data_cosmos::regions::RegionName;
use azure_data_cosmos::{
    clients::DatabaseClient,
    models::{ContainerProperties, ThroughputProperties},
};
use framework::{TestClient, TestRunContext, HUB_REGION, SATELLITE_REGION};
use tracing_subscriber::layer::SubscriberExt;
/// A simple layer that captures log messages into a shared buffer
struct CaptureLayer {
    buffer: Arc<Mutex<Vec<String>>>,
}

impl<S> tracing_subscriber::Layer<S> for CaptureLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = StringVisitor::default();
        event.record(&mut visitor);
        let message = format!("{}: {}", event.metadata().target(), visitor.message);
        self.buffer.lock().unwrap().push(message);
    }
}

#[derive(Default)]
struct StringVisitor {
    message: String,
}

impl tracing::field::Visit for StringVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        } else {
            self.message
                .push_str(&format!(" {}={:?}", field.name(), value));
        }
    }
}

/// Finds log lines containing the expected operation and returns them
fn find_upsert_document_logs(logs: &[String]) -> Vec<String> {
    logs.iter()
        .filter(|line| {
            line.contains("azure_data_cosmos::retry_handler")
                && line.contains("Upsert")
                && line.contains("Document")
        })
        .cloned()
        .collect()
}

// Helper to avoid duplicating the same preferred-locations setup.
fn options_with_preferred_locations(locations: Vec<RegionName>) -> TestOptions {
    TestOptions::new().with_client_preferred_regions(locations)
}

async fn create_container_and_write_item(
    db_client: &DatabaseClient,
    run_context: &TestRunContext,
    container_id: &str,
    _expected_region: &str,
) -> Result<(), Box<dyn Error>> {
    let properties = ContainerProperties::new(Cow::Owned(String::from(container_id)), "/id".into());

    let throughput = ThroughputProperties::manual(400);

    let container_client = run_context
        .create_container_with_throughput(&db_client, properties, throughput)
        .await?;

    // This upsert operation should be logged by the retry_handler
    container_client
        .upsert_item(
            "item1",
            &serde_json::json!({"id": "item1", "value": "test"}),
            None,
        )
        .await?;

    Ok(())
}

#[tokio::test]
pub async fn multi_write_preferred_locations() -> Result<(), Box<dyn Error>> {
    // Create a buffer to capture log messages
    let log_buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let capture_layer = CaptureLayer {
        buffer: log_buffer.clone(),
    };

    // Set up tracing subscriber with our capture layer
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::DEBUG)
        .with(capture_layer);

    // Use this subscriber for the duration of the test
    let _guard = tracing::subscriber::set_default(subscriber);

    const CONTAINER_ID: &str = "MultiWritePreferredLocations";

    // Clear buffer before first test
    log_buffer.lock().unwrap().clear();

    // write to hub region
    TestClient::run_with_unique_db(
        async |run_context, _db_client| {
            create_container_and_write_item(
                _db_client,
                run_context,
                CONTAINER_ID,
                HUB_REGION.as_str(),
            )
            .await
        },
        Some(options_with_preferred_locations(vec![
            HUB_REGION.into(),
            SATELLITE_REGION.into(),
        ])),
    )
    .await?;

    // Check that the upsert went to the hub region
    {
        let logs = log_buffer.lock().unwrap();
        let upsert_logs = find_upsert_document_logs(&logs);
        println!("Hub region upsert logs: {:?}", upsert_logs);

        assert!(
            !upsert_logs.is_empty(),
            "Expected to find upsert document log entries"
        );

        // Verify the endpoint contains the hub region
        let hub_log = upsert_logs.iter().find(|log| {
            log.to_lowercase()
                .contains(&HUB_REGION.as_str().to_lowercase().replace(" ", ""))
        });
        assert!(
            hub_log.is_some(),
            "Expected upsert to go to hub region ({}), but got: {:?}",
            HUB_REGION,
            upsert_logs
        );
    }

    // Clear buffer before second test
    log_buffer.lock().unwrap().clear();

    // write to satellite region
    TestClient::run_with_unique_db(
        async |run_context, _db_client| {
            create_container_and_write_item(
                _db_client,
                run_context,
                CONTAINER_ID,
                SATELLITE_REGION.as_str(),
            )
            .await
        },
        Some(options_with_preferred_locations(vec![
            SATELLITE_REGION.into(),
            HUB_REGION.into(),
        ])),
    )
    .await?;

    // Check that the upsert went to the satellite region
    {
        let logs = log_buffer.lock().unwrap();
        let upsert_logs = find_upsert_document_logs(&logs);
        println!("Satellite region upsert logs: {:?}", upsert_logs);

        assert!(
            !upsert_logs.is_empty(),
            "Expected to find upsert document log entries"
        );

        // Verify the endpoint contains the satellite region
        let satellite_log = upsert_logs.iter().find(|log| {
            log.to_lowercase()
                .contains(&SATELLITE_REGION.as_str().to_lowercase().replace(" ", ""))
        });
        assert!(
            satellite_log.is_some(),
            "Expected upsert to go to satellite region ({}), but got: {:?}",
            SATELLITE_REGION,
            upsert_logs
        );
    }

    Ok(())
}
