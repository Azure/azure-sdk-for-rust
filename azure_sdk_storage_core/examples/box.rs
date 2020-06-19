use azure_sdk_storage_core::prelude::*;
use std::thread;

fn get_box() -> Box<dyn Client> {
    Box::new(client::with_access_key("fake", "fake"))
}

fn get_box_send() -> Box<dyn Client + Send + Sync> {
    Box::new(client::with_access_key("fake", "fake"))
}

pub fn main() {
    let client = get_box();
    println!("client.blob_uri() == {}", client.blob_uri());

    let client_send = get_box_send();

    let handler = thread::spawn(move || {
        println!("client_send.blob_uri() == {}", client_send.blob_uri());
    });
    handler.join().unwrap();
}
