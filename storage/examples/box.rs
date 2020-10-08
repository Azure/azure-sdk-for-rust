use azure_sdk_storage::core::prelude::*;
use std::sync::Arc;
use std::thread;

fn get_box() -> Box<dyn Client> {
    Box::new(client::with_access_key("fake", "fake"))
}

fn get_box_send() -> Box<dyn Client> {
    Box::new(client::with_access_key("send", "fake"))
}

fn get_arc() -> Arc<dyn Client> {
    Arc::new(client::with_access_key("arc", "fake"))
}

pub fn main() {
    let client = get_box();
    println!("client.blob_uri() == {}", client.blob_uri());

    let client_send = get_box_send();

    let handler = thread::spawn(move || {
        println!("client_send.blob_uri() == {}", client_send.blob_uri());
    });
    handler.join().unwrap();

    let client_arc = get_arc();
    println!("client_arc.blob_uri() == {}", client_arc.blob_uri());
}
