#![cfg(all(test,feature = "test_e2e"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
extern crate time;

mod util;

use time::Duration;
use azure_sdk_for_rust::azure;

use std::io::{Write, Seek};

#[test]
fn send_events_to_event_hub() {
    let mut eh_client = create_client();


    for i in 0..2 {
        info!("Sending message {}", i);
        send_event(&mut eh_client);
    }
}


fn send_event(cli: &mut azure::service_bus::event_hub::Client) {
    debug!("running send_event");

    let mut cursor = std::io::Cursor::new(vec![0; 255]);
    cursor.write(b"{ numero: 100, testo: \"sample\" }").unwrap();
    cursor.flush().unwrap();
    cursor.seek(std::io::SeekFrom::Start(0)).unwrap();

    cli.send_event(&mut (&mut cursor, 255), Duration::hours(1))
        .unwrap();
}


fn create_client() -> azure::service_bus::event_hub::Client {
    let policy_name = std::env::var("AZURE_POLICY_NAME")
        .expect("Please set AZURE_POLICY_NAME env variable first!");

    let policy_key = std::env::var("AZURE_POLICY_KEY")
        .expect("Please set AZURE_POLICY_KEY env variable first!");

    let sb_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE")
        .expect("Please set AZURE_SERVICE_BUS_NAMESPACE env variable first!");

    let ev_name = std::env::var("AZURE_EVENT_HUB_NAME")
        .expect("Please set AZURE_EVENT_HUB_NAME env variable first!");

    azure::service_bus::event_hub::Client::new(&sb_namespace, &ev_name, &policy_name, &policy_key)

}
