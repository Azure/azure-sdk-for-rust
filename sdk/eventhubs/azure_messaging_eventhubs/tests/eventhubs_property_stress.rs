// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::ProducerClient;
use std::{env, error::Error, sync::Arc};
use tracing::info;

#[recorded::test(live)]
async fn get_lots_of_properties_one_thread(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ProducerClient::builder()
        .with_application_id("test_get_properties".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    let mut property_futures = Vec::new();
    for _ in 0..600 {
        property_futures.push(client.get_eventhub_properties());
    }

    let properties = futures::future::join_all(property_futures).await;
    assert_eq!(properties.len(), 600);
    for result in properties {
        match result {
            Ok(_properties) => {
                //                info!("Eventhub Properties: {:?}", properties);
            }
            Err(err) => {
                info!("Error getting properties: {:?}", err);
            }
        }
    }

    Ok(())
}

#[recorded::test(live)]
async fn get_lots_of_properties_multiple_threads(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    const THREAD_COUNT: usize = 100;
    const OPERATION_COUNT: usize = 10;

    let credential = recording.credential();

    let client = Arc::new(
        ProducerClient::builder()
            .with_application_id("test_get_properties".to_string())
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await?,
    );

    let mut property_futures = Vec::new();
    for _ in 0..THREAD_COUNT {
        let client = client.clone();
        property_futures.push(tokio::task::spawn(async move {
            let mut results = Vec::new();
            for _ in 0..OPERATION_COUNT {
                results.push(client.get_eventhub_properties().await);
            }
            Ok::<_, Box<dyn Error + Send + Sync>>(results)
        }));
    }

    assert_eq!(property_futures.len(), THREAD_COUNT);

    let properties = futures::future::join_all(property_futures).await;

    assert_eq!(properties.len(), THREAD_COUNT);
    for result in properties {
        match result {
            Ok(_properties) => {
                //                info!("Eventhub Properties: {:?}", properties);
            }
            Err(err) => {
                info!("Error getting properties: {:?}", err);
            }
        }
    }

    Ok(())
}

#[recorded::test(live)]
async fn get_lots_of_properties_multiple_connections1(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    const CONNECTION_COUNT: usize = 50;

    let credential = recording.credential();

    let mut property_futures = Vec::new();
    let mut property_clients = Vec::new();
    for _ in 0..CONNECTION_COUNT {
        let client = Arc::new(
            ProducerClient::builder()
                .with_application_id("test_get_properties".to_string())
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );
        property_clients.push(client.clone());
        property_futures.push(tokio::task::spawn(async move {
            client.get_eventhub_properties().await
        }));
    }

    assert_eq!(property_futures.len(), CONNECTION_COUNT);

    let properties = futures::future::join_all(property_futures).await;
    for client in property_clients {
        if let Ok(client) = Arc::try_unwrap(client) {
            client.close().await?;
        } else {
            panic!("Could not unwrap client");
        }
    }

    assert_eq!(properties.len(), CONNECTION_COUNT);
    for result in properties {
        match result {
            Ok(_properties) => {
                //                info!("Eventhub Properties: {:?}", properties);
            }
            Err(err) => {
                info!("Error getting properties: {:?}", err);
            }
        }
    }

    Ok(())
}

#[allow(unused_attributes)]
#[ignore = "This test creates a lot of threads and connections, so it is ignored by default."]
#[recorded::test(live)]
async fn get_lots_of_properties_multiple_connections2(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    use azure_messaging_eventhubs::EventHubsError;

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    const CONNECTION_COUNT: usize = 50;

    let credential = recording.credential();

    let mut property_futures: Vec<
        tokio::task::JoinHandle<
            Result<azure_messaging_eventhubs::models::EventHubProperties, EventHubsError>,
        >,
    > = Vec::new();
    for _ in 0..CONNECTION_COUNT {
        let credential = credential.clone();
        let host = host.clone();
        let eventhub = eventhub.clone();
        property_futures.push(tokio::task::spawn(async move {
            let client = ProducerClient::builder()
                .with_application_id("test_get_properties".to_string())
                .open(host.as_str(), eventhub.as_str(), credential)
                .await?;
            let property = client.get_eventhub_properties().await?;
            client.close().await?;
            Ok(property)
        }));
    }

    assert_eq!(property_futures.len(), CONNECTION_COUNT);

    let properties = futures::future::join_all(property_futures).await;

    assert_eq!(properties.len(), CONNECTION_COUNT);
    for result in properties {
        match result {
            Ok(_properties) => {
                //                info!("Eventhub Properties: {:?}", properties);
            }
            Err(err) => {
                info!("Error getting properties: {:?}", err);
            }
        }
    }

    Ok(())
}

#[allow(unused_attributes)]
#[ignore = "This test creates a lot of threads and connections, so it is ignored by default."]
#[recorded::test(live)]
async fn get_lots_of_properties_multiple_blocking_threads(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    const THREAD_COUNT: usize = 50;

    let credential = recording.credential();

    let mut property_futures: Vec<
        std::thread::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>,
    > = Vec::new();
    for _ in 0..THREAD_COUNT {
        let host = host.clone();
        let eventhub = eventhub.clone();
        let credential = credential.clone();
        property_futures.push(std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(10)
                .enable_all()
                .build()?;
            let client = runtime.block_on(
                ProducerClient::builder()
                    .with_application_id("test_get_properties".to_string())
                    .open(host.as_str(), eventhub.as_str(), credential.clone()),
            )?;
            for _ in 0..50 {
                runtime.block_on(client.get_eventhub_properties())?;
            }
            runtime.block_on(client.close())?;
            Ok(())
        }));
    }

    assert_eq!(property_futures.len(), THREAD_COUNT);
    for thread in property_futures {
        let result = thread.join();
        match result {
            Ok(_) => {}
            Err(err) => {
                info!("Error joining thread: {:?}", err);
            }
        }
    }

    Ok(())
}
