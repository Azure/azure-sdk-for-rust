#![cfg(all(test,feature = "e2e_test"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::table::Table;

extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Entry {
    c: String,
}

#[test]
fn insert_query() {
    let client = create_storage_client();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    Table::insert(&client, "rtest1", "a62", s.as_str(), "{\"c\":\"mot1\"}").unwrap();
    let result = Table::query(&client, "rtest1", "a62", s.as_str()).unwrap();
    let entry: Entry = json::decode(result.as_str()).unwrap();
    assert_eq!("mot1", entry.c);
}

#[test]
fn query_range() {
    env_logger::init().unwrap();

    let client = create_storage_client();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    for i in 0..5 {
        let key = format!("b{}0", i);
        let body = format!("{{\"c\":\"val{}\"}}", i);
        Table::insert(&client, "rtest1", key.as_str(), s.as_str(), body.as_str()).unwrap();
    }

    let result = Table::query_range(&client, "rtest1", "b20", true, 3).unwrap();
    // let entry: Entry = json::decode(result.as_str()).unwrap();
    // assert_eq!("mot1", entry.c);
}

fn create_storage_client() -> Client {
    let azure_storage_account = get_from_env("AZURE_STORAGE_ACCOUNT");
    let azure_storage_key = get_from_env("AZURE_STORAGE_KEY");
    Client::new(&azure_storage_account, &azure_storage_key, true)
}

fn get_from_env(varname: &str) -> String {
    match std::env::var(varname) {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set {} env variable first!", varname);
        }
    }
}
