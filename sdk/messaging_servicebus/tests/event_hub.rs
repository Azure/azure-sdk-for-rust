// #![cfg(all(test, feature = "test_e2e"))]
// #[macro_use]
// extern crate log;

// use azure_service_bus::event_hub::Client;
// use chrono::Duration;

// #[tokio::test]
// async fn send_events_to_event_hub() {
//     let mut eh_client = create_client().unwrap();

//     for i in 0..2 {
//         info!("Sending message {}", i);
//         send_event(&mut eh_client).await;
//     }
// }

// async fn send_event(cli: &mut Client) {
//     debug!("running send_event");

//     let text_to_send = "{ numero: 100, testo: \"sample\" }";
//     cli.send_event(&text_to_send, Duration::hours(1))
//         .await
//         .unwrap();
// }

// fn create_client() -> Result<Client, azure_core::Error> {
//     let policy_name = std::env::var("AZURE_POLICY_NAME")
//         .expect("Please set AZURE_POLICY_NAME env variable first!");

//     let policy_key =
//         std::env::var("AZURE_POLICY_KEY").expect("Please set AZURE_POLICY_KEY env variable first!");

//     let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE")
//         .expect("Please set AZURE_SERVICE_BUS_NAMESPACE env variable first!");

//     let event_hub_name = std::env::var("AZURE_EVENT_HUB_NAME")
//         .expect("Please set AZURE_EVENT_HUB_NAME env variable first!");

//     Ok(Client::new(
//         service_bus_namespace,
//         event_hub_name,
//         policy_name,
//         policy_key,
//     )?)
// }
