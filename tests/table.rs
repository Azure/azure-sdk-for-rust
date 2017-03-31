#![cfg(all(test,feature = "e2e_test"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::table::Table;
use azure_sdk_for_rust::azure::core::errors::AzureError;

extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Entry {
    pk: String,
    c: String,
}

#[test]
fn insert_get() {
    let client = create_storage_client();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    Table::insert(&client, "rtest1", "a62", s.as_str(), "{\"c\":\"mot1\"}").unwrap();
    let result = Table::get(&client, "rtest1", "a62", s.as_str()).unwrap();
    let entry: Entry = json::decode(result.as_str()).unwrap();
    assert_eq!("mot1", entry.c);
}

#[test]
fn query_range() {
    // env_logger::init().unwrap();
    let client = create_storage_client();
    let utc = chrono::UTC::now();
    let s = utc.to_string();
    for i in 1..5 {
        let key = format!("b{}0", i);
        let tc = Entry {
            c: format!("val{}", i),
            pk: key.clone(),
        };

        let body = json::encode(&tc).unwrap();
        Table::insert(&client, "rtest1", key.as_str(), s.as_str(), body.as_str()).unwrap();
    }

    let ec = test_query_range(&client, "rtest1", "b20", s.as_str(), false, 3).unwrap();
    for item in ec {
        println!("{:?}", item);
    }
    // assert_eq!("mot1", entry.c);
}

fn test_query_range(client: &Client,
                        table_name: &str,
                        partition_key: &str,
                        row_key: &str,
                        ge: bool,
                        limit: u16)
                        -> Result<Vec<Entry>, AzureError> {
    Table::query_range_entry(client,
                       table_name,
                       Some(format!("$filter=PartitionKey {} '{}' and RowKey ge '{}'&$top={}",
                               if ge { "ge" } else { "le" },
                               partition_key,
                               row_key,
                               limit)
                               .as_str()))
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
